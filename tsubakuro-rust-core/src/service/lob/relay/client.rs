use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use log::{debug, trace};
use tonic::async_trait;

use crate::{
    data_relay_grpc::proto::blob_relay::{
        blob_reference::BlobReference as RelayLobReference,
        blob_relay_streaming::{
            blob_relay_streaming_client::BlobRelayStreamingClient, PutStreamingRequest,
        },
    },
    error::TgError,
    io_error,
    job::Job,
    service::{
        lob::{
            lob_client::{LobClient, RemoteLob},
            storage_id,
        },
        sql::r#type::large_object::TgLargeObjectReference,
    },
    tateyama::proto::endpoint::response::BlobRelayServiceInfo,
    timeout_error,
    transaction::Transaction,
};

const API_VERSION: u64 = 1;

pub(crate) struct RelayLobClient {
    info: BlobRelayServiceInfo,
    grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
}

impl RelayLobClient {
    pub(crate) async fn new(info: BlobRelayServiceInfo) -> Result<RelayLobClient, TgError> {
        let url = Self::grpc_url(&info);
        debug!("Connecting to blob relay service at URL: {}", url);

        let grpc_client = BlobRelayStreamingClient::connect(url)
            .await
            .map_err(|e| io_error!("Failed to connect to blob relay service: {}", e))?;

        Ok(RelayLobClient { info, grpc_client })
    }

    fn grpc_url(info: &BlobRelayServiceInfo) -> String {
        let endpoint = &info.endpoint;

        if let Some(rest) = endpoint.strip_prefix("dns:///") {
            let scheme = if info.secure { "https://" } else { "http://" };
            format!("{scheme}{rest}")
        } else {
            endpoint.clone()
        }
    }
}

#[async_trait]
impl LobClient for RelayLobClient {
    async fn upload_lob_file(&self, path: &Path, timeout: Duration) -> Result<RemoteLob, TgError> {
        const FUNCTION_NAME: &str = "RelayLobClient::upload_lob_file()";
        trace!("{} start", FUNCTION_NAME);

        let value =
            std::fs::read(path).map_err(|e| io_error!("Failed to read blob file: {}", e))?;
        let lob_ref = self.upload(&value, timeout).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(RemoteLob::RelayLobReference(lob_ref))
    }

    async fn upload_lob(&self, value: &[u8], timeout: Duration) -> Result<RemoteLob, TgError> {
        const FUNCTION_NAME: &str = "RelayLobClient::upload_lob()";
        trace!("{} start", FUNCTION_NAME);

        let lob_ref = self.upload(value, timeout).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(RemoteLob::RelayLobReference(lob_ref))
    }

    fn support_download_lob_file(&self) -> bool {
        false
    }

    async fn download_lob_file(
        &self,
        _transaction: &Transaction,
        _lob: &dyn TgLargeObjectReference,
        _timeout: Duration,
    ) -> Result<PathBuf, TgError> {
        Err(io_error!("Not supported in blob relay service"))
    }

    async fn download_lob_file_async(
        &self,
        _transaction: &Transaction,
        _lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<PathBuf>, TgError> {
        Err(io_error!("Not supported in blob relay service"))
    }

    async fn download_lob(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        timeout: Duration,
    ) -> Result<Vec<u8>, TgError> {
        let tx_handle = transaction.transaction_handle()?;
        Self::download(
            self.grpc_client.clone(),
            tx_handle.handle,
            storage_id(lob)?,
            lob.object_id(),
            lob.reference_tag(),
            timeout,
        )
        .await
    }

    async fn download_lob_async(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<Vec<u8>>, TgError> {
        let grpc_client = self.grpc_client.clone();
        let tx_handle = transaction.transaction_handle()?.handle;
        let storage_id = storage_id(lob)?;
        let object_id = lob.object_id();
        let tag = lob.reference_tag();
        let job = Job::supplier(
            "RelayLobDownload",
            Arc::new(move |timeout| {
                let future = Self::download(
                    grpc_client.clone(),
                    tx_handle,
                    storage_id,
                    object_id,
                    tag,
                    timeout,
                );
                Box::pin(future)
            }),
        );

        Ok(job)
    }
}

impl RelayLobClient {
    async fn upload(&self, value: &[u8], timeout: Duration) -> Result<RelayLobReference, TgError> {
        use crate::data_relay_grpc::proto::blob_relay::blob_relay_streaming::put_streaming_request::{Payload, Metadata, metadata::BlobSizeOpt};

        let metadata = Metadata {
            api_version: API_VERSION,
            session_id: self.info.blob_session_id,
            blob_size_opt: Some(BlobSizeOpt::BlobSize(value.len() as u64)),
        };
        let first_request = PutStreamingRequest {
            payload: Some(Payload::Metadata(metadata)),
        };

        const CHUNK_SIZE: usize = 64 * 1024;

        let chunks: Vec<PutStreamingRequest> = value
            .chunks(CHUNK_SIZE)
            .map(|c| PutStreamingRequest {
                payload: Some(Payload::Chunk(c.to_vec())),
            })
            .collect();
        let stream = tokio_stream::iter(std::iter::once(first_request).chain(chunks));

        let mut grpc_client = self.grpc_client.clone();
        let result = if timeout.is_zero() {
            grpc_client.put(tonic::Request::new(stream)).await
        } else {
            tokio::time::timeout(timeout, grpc_client.put(tonic::Request::new(stream)))
                .await
                .map_err(|_| timeout_error!("RelayLobClient::upload()"))?
        };
        let response =
            result.map_err(|e| io_error!("Failed to upload to blob relay service: {}", e))?;
        let lob_ref = response.into_inner().blob.ok_or_else(|| {
            io_error!("Failed to upload to blob relay service: missing blob reference in response")
        })?;
        Ok(lob_ref)
    }

    async fn download(
        mut grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
        transaction_handle: u64,
        storage_id: u64,
        object_id: u64,
        tag: u64,
        timeout: Duration,
    ) -> Result<Vec<u8>, TgError> {
        use crate::data_relay_grpc::proto::blob_relay::blob_relay_streaming::{
            get_streaming_request::ContextId,
            get_streaming_response::{metadata::BlobSizeOpt, Payload},
            GetStreamingRequest,
        };

        let context_id = ContextId::TransactionId(transaction_handle);
        let lob_ref = RelayLobReference {
            storage_id,
            object_id,
            tag,
        };
        let request = GetStreamingRequest {
            api_version: API_VERSION,
            blob: Some(lob_ref),
            context_id: Some(context_id),
        };

        let result = if timeout.is_zero() {
            grpc_client.get(request).await
        } else {
            tokio::time::timeout(timeout, grpc_client.get(request))
                .await
                .map_err(|_| timeout_error!("RelayLobClient::download()"))?
        };
        let mut stream = result
            .map_err(|e| io_error!("Failed to download from blob relay service: {}", e))?
            .into_inner();

        let mut buffer = Vec::new();
        while let Some(response) = stream
            .message()
            .await
            .map_err(|e| io_error!("Failed to receive chunk from blob relay service: {}", e))?
        {
            match response.payload {
                Some(Payload::Metadata(metadata)) => {
                    if let Some(BlobSizeOpt::BlobSize(lob_size)) = metadata.blob_size_opt {
                        buffer.reserve(lob_size as usize);
                    }
                }
                Some(Payload::Chunk(chunk)) => {
                    buffer.extend_from_slice(&chunk);
                }
                _ => {
                    return Err(io_error!(
                        "Failed to download from blob relay service: missing chunk in response"
                    ));
                }
            }
        }

        Ok(buffer)
    }
}
