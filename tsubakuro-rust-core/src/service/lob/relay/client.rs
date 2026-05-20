use std::{path::Path, time::Duration};

use log::debug;
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
    service::lob::lob_client::{LobClient, RemoteLob},
    tateyama::proto::endpoint::response::BlobRelayServiceInfo,
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
        debug!("{} start", FUNCTION_NAME);

        let value =
            std::fs::read(path).map_err(|e| io_error!("Failed to read blob file: {}", e))?;
        let lob_ref = self.upload(&value, timeout).await?;

        debug!("{} end", FUNCTION_NAME);
        Ok(RemoteLob::RelayLobReference(lob_ref))
    }

    async fn upload_lob(&self, value: &[u8], timeout: Duration) -> Result<RemoteLob, TgError> {
        const FUNCTION_NAME: &str = "RelayLobClient::upload_lob()";
        debug!("{} start", FUNCTION_NAME);

        let lob_ref = self.upload(value, timeout).await?;

        debug!("{} end", FUNCTION_NAME);
        Ok(RemoteLob::RelayLobReference(lob_ref))
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
                .map_err(|_| io_error!("Timeout uploading to blob relay service"))?
        };
        let response =
            result.map_err(|e| io_error!("Failed to upload to blob relay service: {}", e))?;
        let lob_ref = response.into_inner().blob.ok_or_else(|| {
            io_error!("Failed to upload to blob relay service: missing blob reference in response")
        })?;
        Ok(lob_ref)
    }
}
