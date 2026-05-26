use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use log::{debug, trace, warn};
use tonic::{async_trait, Streaming};

use crate::{
    data_relay_grpc::proto::blob_relay::{
        blob_reference::BlobReference as RelayLobReference,
        blob_relay_streaming::{
            blob_relay_streaming_client::BlobRelayStreamingClient, GetStreamingResponse,
            PutStreamingRequest,
        },
    },
    error::TgError,
    io_error,
    job::Job,
    service::{
        lob::{
            downloader::LobDownloader,
            lob_client::{LobClient, LobClientMethod, RemoteLob},
            relay::{downloader::RelayLobDownloader, uploader::RelayLobUploader},
            storage_id,
            uploader::LobUploader,
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
    stream_chunk_size: usize,
}

impl RelayLobClient {
    pub(crate) async fn new(
        info: BlobRelayServiceInfo,
        endpoint: Option<&String>,
    ) -> Result<RelayLobClient, TgError> {
        let url = if let Some(endpoint) = endpoint {
            endpoint.clone()
        } else {
            Self::grpc_url(&info)
        };
        debug!("Connecting to blob relay service at URL: {}", url);

        let grpc_client = BlobRelayStreamingClient::connect(url)
            .await
            .map_err(|e| io_error!("Failed to connect to blob relay service", e))?;

        let stream_chunk_size = Self::stream_chunk_size(&info);

        Ok(RelayLobClient {
            info,
            grpc_client,
            stream_chunk_size,
        })
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

    fn stream_chunk_size(info: &BlobRelayServiceInfo) -> usize {
        if let Some(size) = info.parameters.get("stream_chunk_size") {
            if let Ok(size) = size.parse::<usize>() {
                return size;
            }
            warn!(
                "Invalid stream_chunk_size parameter in BlobRelayServiceInfo: {}",
                size
            );
        }
        1024 * 1024 // default 1MB
    }
}

#[async_trait]
impl LobClient for RelayLobClient {
    fn supports_method(&self, method: LobClientMethod) -> bool {
        use LobClientMethod::*;
        match method {
            UploadLobFile | UploadLob | CreateLobUploader => true,
            DownloadLobFile => false,
            DownloadLob | CreateLobDownloader => true,
        }
    }

    async fn upload_lob_file(&self, path: &Path, timeout: Duration) -> Result<RemoteLob, TgError> {
        const FUNCTION_NAME: &str = "RelayLobClient::upload_lob_file()";
        trace!("{} start", FUNCTION_NAME);

        let grpc_client = self.grpc_client.clone();
        let blob_session_id = self.info.blob_session_id;

        let value = tokio::fs::read(path)
            .await
            .map_err(|e| io_error!("Failed to read lob file", e))?;
        let lob = Self::upload(
            grpc_client,
            blob_session_id,
            &value,
            self.stream_chunk_size,
            timeout,
        )
        .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(RemoteLob::LobReference(
            lob.storage_id,
            lob.object_id,
            lob.tag,
        ))
    }

    async fn upload_lob_file_async(&self, path: &Path) -> Result<Job<RemoteLob>, TgError> {
        const FUNCTION_NAME: &str = "RelayLobClient::upload_lob_file_async()";
        trace!("{} start", FUNCTION_NAME);

        let grpc_client = self.grpc_client.clone();
        let blob_session_id = self.info.blob_session_id;
        let path = path.to_path_buf();
        let chunk_size = self.stream_chunk_size;
        let job = Job::supplier(
            "RelayLobUpload",
            Arc::new(move |timeout| {
                let future = Self::upload_file_for_async(
                    grpc_client.clone(),
                    blob_session_id,
                    path.clone(),
                    chunk_size,
                    timeout,
                );
                Box::pin(future)
            }),
            Duration::ZERO,
        );

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    async fn upload_lob(&self, value: &[u8], timeout: Duration) -> Result<RemoteLob, TgError> {
        const FUNCTION_NAME: &str = "RelayLobClient::upload_lob()";
        trace!("{} start", FUNCTION_NAME);

        let grpc_client = self.grpc_client.clone();
        let blob_session_id = self.info.blob_session_id;
        let lob = Self::upload(
            grpc_client,
            blob_session_id,
            value,
            self.stream_chunk_size,
            timeout,
        )
        .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(RemoteLob::LobReference(
            lob.storage_id,
            lob.object_id,
            lob.tag,
        ))
    }

    async fn upload_lob_async(&self, value: &[u8]) -> Result<Job<RemoteLob>, TgError> {
        const FUNCTION_NAME: &str = "RelayLobClient::upload_lob_async()";
        trace!("{} start", FUNCTION_NAME);

        let grpc_client = self.grpc_client.clone();
        let blob_session_id = self.info.blob_session_id;
        let value = value.to_vec();
        let chunk_size = self.stream_chunk_size;
        let job = Job::supplier(
            "RelayLobUpload",
            Arc::new(move |timeout| {
                let future = Self::upload_for_async(
                    grpc_client.clone(),
                    blob_session_id,
                    value.clone(),
                    chunk_size,
                    timeout,
                );
                Box::pin(future)
            }),
            Duration::ZERO,
        );

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    async fn create_lob_uploader(&self) -> Result<Arc<dyn LobUploader + Send + Sync>, TgError> {
        const FUNCTION_NAME: &str = "RelayLobClient::create_lob_uploader()";
        trace!("{} start", FUNCTION_NAME);

        let grpc_client = self.grpc_client.clone();
        let blob_session_id = self.info.blob_session_id;
        let uploader = RelayLobUploader::new(grpc_client, blob_session_id).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(Arc::new(uploader))
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
        let default_timeout = transaction.default_timeout();
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
            default_timeout,
        );

        Ok(job)
    }

    async fn create_lob_downloader(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        timeout: Duration,
    ) -> Result<Box<dyn LobDownloader>, TgError> {
        let grpc_client = self.grpc_client.clone();
        let downloader = RelayLobDownloader::new(grpc_client, transaction, lob, timeout).await?;
        Ok(Box::new(downloader))
    }
}

impl RelayLobClient {
    async fn upload(
        mut grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
        blob_session_id: u64,
        value: &[u8],
        chunk_size: usize,
        timeout: Duration,
    ) -> Result<RelayLobReference, TgError> {
        let first_request =
            Self::create_upload_metadata_request(blob_session_id, Some(value.len() as u64));

        let chunks: Vec<PutStreamingRequest> = value
            .chunks(chunk_size)
            .map(|c| Self::create_upload_chunk_request(c.to_vec()))
            .collect();
        let stream = tokio_stream::iter(std::iter::once(first_request).chain(chunks));

        let result = if timeout.is_zero() {
            grpc_client.put(tonic::Request::new(stream)).await
        } else {
            tokio::time::timeout(timeout, grpc_client.put(tonic::Request::new(stream)))
                .await
                .map_err(|_| timeout_error!("RelayLobClient::upload()"))?
        };
        let response =
            result.map_err(|e| io_error!("Failed to upload to blob relay service", e))?;
        let lob_ref = response.into_inner().blob.ok_or_else(|| {
            io_error!("Failed to upload to blob relay service: missing blob reference in response")
        })?;
        Ok(lob_ref)
    }

    async fn upload_file_for_async(
        grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
        blob_session_id: u64,
        path: PathBuf,
        chunk_size: usize,
        timeout: Duration,
    ) -> Result<RemoteLob, TgError> {
        let value = tokio::fs::read(path)
            .await
            .map_err(|e| io_error!("Failed to read lob file: {}", e))?;
        let lob = Self::upload(
            grpc_client.clone(),
            blob_session_id,
            &value,
            chunk_size,
            timeout,
        )
        .await?;
        let remote_lob = RemoteLob::LobReference(lob.storage_id, lob.object_id, lob.tag);
        Ok(remote_lob)
    }

    async fn upload_for_async(
        grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
        blob_session_id: u64,
        value: Vec<u8>,
        chunk_size: usize,
        timeout: Duration,
    ) -> Result<RemoteLob, TgError> {
        let lob = Self::upload(
            grpc_client.clone(),
            blob_session_id,
            &value,
            chunk_size,
            timeout,
        )
        .await?;
        let remote_lob = RemoteLob::LobReference(lob.storage_id, lob.object_id, lob.tag);
        Ok(remote_lob)
    }

    pub(crate) fn create_upload_metadata_request(
        blob_session_id: u64,
        lob_size: Option<u64>,
    ) -> PutStreamingRequest {
        use crate::data_relay_grpc::proto::blob_relay::blob_relay_streaming::put_streaming_request::{
            metadata::BlobSizeOpt, Metadata, Payload,
        };

        let lob_size_opt = lob_size.map(BlobSizeOpt::BlobSize);

        let metadata = Metadata {
            api_version: API_VERSION,
            session_id: blob_session_id,
            blob_size_opt: lob_size_opt,
        };
        PutStreamingRequest {
            payload: Some(Payload::Metadata(metadata)),
        }
    }

    pub(crate) fn create_upload_chunk_request(chunk: Vec<u8>) -> PutStreamingRequest {
        use crate::data_relay_grpc::proto::blob_relay::blob_relay_streaming::put_streaming_request::Payload;

        PutStreamingRequest {
            payload: Some(Payload::Chunk(chunk)),
        }
    }

    async fn download(
        grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
        transaction_handle: u64,
        storage_id: u64,
        object_id: u64,
        tag: u64,
        timeout: Duration,
    ) -> Result<Vec<u8>, TgError> {
        use crate::data_relay_grpc::proto::blob_relay::blob_relay_streaming::{
            get_streaming_response::{metadata::BlobSizeOpt, Payload},
        };
        let mut stream = Self::start_download(
            grpc_client.clone(),
            transaction_handle,
            storage_id,
            object_id,
            tag,
            timeout,
        )
        .await?;

        let future = async {
            let mut buffer = Vec::new();

            loop {
                let response = stream.message().await.map_err(|e| {
                    io_error!("Failed to receive chunk from blob relay service", e)
                })?;

                match response {
                    Some(response) => match response.payload {
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
                    },
                    None => break, // end of stream
                }
            }

            Ok(buffer)
        };

        if timeout.is_zero() {
            future.await
        } else {
            tokio::time::timeout(timeout, future)
                .await
                .map_err(|_| timeout_error!("RelayLobClient::download()"))?
        }
    }

    pub(crate) async fn start_download(
        mut grpc_client: BlobRelayStreamingClient<tonic::transport::Channel>,
        transaction_handle: u64,
        storage_id: u64,
        object_id: u64,
        tag: u64,
        timeout: Duration,
    ) -> Result<Streaming<GetStreamingResponse>, TgError> {
        use crate::data_relay_grpc::proto::blob_relay::blob_relay_streaming::{
            get_streaming_request::ContextId, GetStreamingRequest,
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
        let stream = result
            .map_err(|e| io_error!("Failed to download from blob relay service", e))?
            .into_inner();

        Ok(stream)
    }
}
