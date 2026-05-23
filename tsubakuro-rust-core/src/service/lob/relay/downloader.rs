use std::time::Duration;

use bytes::BytesMut;
use tonic::{async_trait, Streaming};

use crate::{
    data_relay_grpc::proto::blob_relay::blob_relay_streaming::{
        blob_relay_streaming_client::BlobRelayStreamingClient, GetStreamingResponse,
    },
    error::TgError,
    io_error,
    service::{
        lob::{downloader::LobDownloader, relay::client::RelayLobClient, storage_id},
        sql::r#type::large_object::TgLargeObjectReference,
    },
    transaction::Transaction,
};

pub(crate) struct RelayLobDownloader {
    stream: Streaming<GetStreamingResponse>,
}

impl RelayLobDownloader {
    pub(crate) async fn new(
        grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        timeout: Duration,
    ) -> Result<Self, TgError> {
        let tx_handle = transaction.transaction_handle()?;
        let stream = RelayLobClient::start_download(
            grpc_client,
            tx_handle.handle,
            storage_id(lob)?,
            lob.object_id(),
            lob.reference_tag(),
            timeout,
        )
        .await?;

        Ok(Self { stream })
    }
}

#[async_trait]
impl LobDownloader for RelayLobDownloader {
    async fn download_chunk(
        &mut self,
        buffer: &mut BytesMut,
        _timeout: Duration,
    ) -> Result<usize, TgError> {
        use crate::data_relay_grpc::proto::blob_relay::blob_relay_streaming::get_streaming_response::{
            Payload,
        };

        while let Some(response) = self
            .stream
            .message()
            .await
            .map_err(|e| io_error!("Failed to receive chunk from blob relay service: {}", e))?
        {
            match response.payload {
                Some(Payload::Metadata(_metadata)) => {
                    continue;
                }
                Some(Payload::Chunk(chunk)) => {
                    buffer.extend_from_slice(&chunk);
                    return Ok(buffer.len());
                }
                _ => {
                    return Err(io_error!(
                        "Failed to download from blob relay service: missing chunk in response"
                    ));
                }
            }
        }

        Ok(0)
    }
}
