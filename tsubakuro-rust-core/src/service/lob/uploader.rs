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
    pub async fn upload_chunk(&mut self, value: &[u8], timeout: Duration) -> Result<(), TgError> {
        self.inner.upload_chunk(value, timeout).await
    }

    pub async fn finish(mut self, timeout: Duration) -> Result<TgBlob, TgError> {
        self.done = true;

        let lob = self.inner.finish(timeout).await?;
        Ok(TgBlob::RemoteLob(lob))
    }

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
    pub async fn upload_chunk(&mut self, value: &[u8], timeout: Duration) -> Result<(), TgError> {
        self.inner.upload_chunk(value, timeout).await
    }

    pub async fn finish(mut self, timeout: Duration) -> Result<TgClob, TgError> {
        self.done = true;

        let lob = self.inner.finish(timeout).await?;
        Ok(TgClob::RemoteLob(lob))
    }

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
