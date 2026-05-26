use std::{sync::Mutex, time::Duration};

use tokio::{
    sync::mpsc::{self, error::SendTimeoutError},
    task::JoinHandle,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, Request, Response};

use crate::{
    client_error,
    data_relay_grpc::proto::blob_relay::blob_relay_streaming::{
        blob_relay_streaming_client::BlobRelayStreamingClient, PutStreamingRequest,
        PutStreamingResponse,
    },
    error::TgError,
    io_error,
    service::lob::{lob_client::RemoteLob, relay::client::RelayLobClient, uploader::LobUploader},
    timeout_error,
};

struct PutHandle {
    tx: mpsc::Sender<PutStreamingRequest>,
    request_handle: JoinHandle<Result<Response<PutStreamingResponse>, tonic::Status>>,
}

pub(crate) struct RelayLobUploader {
    handle: tokio::sync::Mutex<Option<PutHandle>>,
    cancel_tx: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
}

impl RelayLobUploader {
    pub(crate) async fn new(
        mut grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
        blob_session_id: u64,
    ) -> Result<RelayLobUploader, TgError> {
        let (tx, rx) = mpsc::channel(4);
        let (cancel_tx, cancel_rx) = tokio::sync::oneshot::channel::<()>();
        let stream = ReceiverStream::new(rx);
        let request_handle = tokio::spawn(async move {
            tokio::select! {
                result = grpc_client.put(Request::new(stream)) => result,
                _ = cancel_rx => Err(tonic::Status::cancelled("Upload cancelled")),
            }
        });

        let first_request = RelayLobClient::create_upload_metadata_request(blob_session_id, None);
        tx.send(first_request)
            .await
            .map_err(|e| io_error!("Failed to send first request", e))?;

        Ok(RelayLobUploader {
            handle: tokio::sync::Mutex::new(Some(PutHandle { tx, request_handle })),
            cancel_tx: Mutex::new(Some(cancel_tx)),
        })
    }
}

#[async_trait]
impl LobUploader for RelayLobUploader {
    async fn upload_chunk(&self, value: &[u8], timeout: Duration) -> Result<(), TgError> {
        let request = RelayLobClient::create_upload_chunk_request(value.to_vec());
        if let Some(put_handle) = self.handle.lock().await.as_ref() {
            match put_handle.tx.send_timeout(request, timeout).await {
                Ok(()) => Ok(()),
                Err(SendTimeoutError::Timeout(_)) => {
                    Err(timeout_error!("RelayLobUploader::upload_chunk()"))
                }
                Err(SendTimeoutError::Closed(_)) => Err(io_error!("Upload stream closed")),
            }
        } else {
            Err(client_error!(
                "RelayLobUploader::upload_chunk() called after finish() or cancel()"
            ))
        }
    }

    async fn finish(&self, timeout: Duration) -> Result<RemoteLob, TgError> {
        if let Some(put_handle) = self.handle.lock().await.take() {
            drop(put_handle.tx); // close the sender to indicate the end of the stream

            let result = if timeout.is_zero() {
                put_handle.request_handle.await
            } else {
                match tokio::time::timeout(timeout, put_handle.request_handle).await {
                    Ok(result) => result,
                    Err(_) => {
                        let _ = self.cancel();
                        return Err(timeout_error!("RelayLobUploader::finish()"));
                    }
                }
            };
            let result = result.map_err(|e| io_error!("Failed to receive upload response", e))?;
            match result {
                Ok(response) => {
                    let response = response.into_inner();
                    let lob = response.blob.ok_or_else(|| {
                        io_error!("Failed to upload to blob relay service: missing blob reference in response")
                    })?;
                    Ok(RemoteLob::LobReference(
                        lob.storage_id,
                        lob.object_id,
                        lob.tag,
                    ))
                }
                Err(status) => {
                    return Err(io_error!("Upload failed", status));
                }
            }
        } else {
            Err(client_error!(
                "RelayLobUploader::finish() called multiple times"
            ))
        }
    }

    fn cancel(&self) -> Result<(), TgError> {
        if let Some(cancel_tx) = self.cancel_tx.lock().unwrap().take() {
            let _ = cancel_tx.send(());
        }
        Ok(())
    }
}
