use std::{sync::Arc, time::Duration};

use log::warn;
use tonic::async_trait;

use crate::{
    error::TgError,
    service::{
        lob::lob_client::RemoteLob,
        sql::r#type::{blob::TgBlob, clob::TgClob},
    },
};

#[async_trait]
pub(crate) trait LobUploader {
    async fn upload_chunk(&self, value: &[u8], timeout: Duration) -> Result<(), TgError>;
    async fn finish(&self, timeout: Duration) -> Result<RemoteLob, TgError>;
    fn cancel(&self) -> Result<(), TgError>;
}

/// BLOB uploader.
///
/// # Examples
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example(client: &SqlClient, value: &[u8]) -> Result<TgBlob, TgError> {
///     let timeout = std::time::Duration::from_secs(10);
///     let mut uploader = client.create_blob_uploader().await?;
///
///     for chunk in value.chunks(1024 * 1024) {
///         uploader.upload_chunk(chunk, timeout).await?;
///     }
///     let blob = uploader.finish(timeout).await?;
///
///     Ok(blob)
/// }
/// ```
///
/// since 0.10.0
pub struct BlobUploader {
    inner: Arc<dyn LobUploader + Send + Sync>,
    done: bool,
}

impl BlobUploader {
    pub(crate) fn new(inner: Arc<dyn LobUploader + Send + Sync>) -> BlobUploader {
        BlobUploader { inner, done: false }
    }
}

impl BlobUploader {
    /// Uploads a chunk of data.
    pub async fn upload_chunk(&mut self, value: &[u8], timeout: Duration) -> Result<(), TgError> {
        self.inner.upload_chunk(value, timeout).await
    }

    /// Finishes the upload and returns the resulting `TgBlob`.
    pub async fn finish(mut self, timeout: Duration) -> Result<TgBlob, TgError> {
        self.done = true;

        let lob = self.inner.finish(timeout).await?;
        Ok(TgBlob::from_remote_lob(lob))
    }

    /// Cancels the upload.
    pub fn cancel(mut self) -> Result<(), TgError> {
        self.done = true;

        self.inner.cancel()
    }
}

impl Drop for BlobUploader {
    fn drop(&mut self) {
        if self.done {
            return;
        }

        let result = self.inner.cancel();
        if let Err(e) = result {
            warn!("BlobUploader.drop() close error. {}", e);
        }
    }
}

/// CLOB uploader.
///
/// # Examples
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example(client: &SqlClient, value: &str) -> Result<TgClob, TgError> {
///     let timeout = std::time::Duration::from_secs(10);
///     let mut uploader = client.create_clob_uploader().await?;
///
///     let bytes = value.as_bytes();
///     for chunk in bytes.chunks(1024 * 1024) {
///         uploader.upload_chunk_utf8(chunk, timeout).await?;
///     }
///     let clob = uploader.finish(timeout).await?;
///
///     Ok(clob)
/// }
/// ```
///
/// since 0.10.0
pub struct ClobUploader {
    inner: Arc<dyn LobUploader + Send + Sync>,
    done: bool,
}

impl ClobUploader {
    pub(crate) fn new(inner: Arc<dyn LobUploader + Send + Sync>) -> ClobUploader {
        ClobUploader { inner, done: false }
    }
}

impl ClobUploader {
    /// Uploads a chunk of data as UTF-8.
    pub async fn upload_chunk_utf8(
        &mut self,
        value: &[u8],
        timeout: Duration,
    ) -> Result<(), TgError> {
        self.inner.upload_chunk(value, timeout).await
    }

    /// Finishes the upload and returns the resulting `TgClob`.
    pub async fn finish(mut self, timeout: Duration) -> Result<TgClob, TgError> {
        self.done = true;

        let lob = self.inner.finish(timeout).await?;
        Ok(TgClob::from_remote_lob(lob))
    }

    /// Cancels the upload.
    pub fn cancel(mut self) -> Result<(), TgError> {
        self.done = true;

        self.inner.cancel()
    }
}

impl Drop for ClobUploader {
    fn drop(&mut self) {
        if self.done {
            return;
        }

        let result = self.inner.cancel();
        if let Err(e) = result {
            warn!("ClobUploader.drop() close error. {}", e);
        }
    }
}
