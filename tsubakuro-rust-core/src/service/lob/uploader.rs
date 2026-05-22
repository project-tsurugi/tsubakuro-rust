use std::{sync::Arc, time::Duration};

use log::{error, warn};
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
    async fn cancel(&self) -> Result<(), TgError>;
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

    pub async fn cancel(mut self) -> Result<(), TgError> {
        self.done = true;

        self.inner.cancel().await
    }
}

impl Drop for BlobUploader {
    fn drop(&mut self) {
        if self.done {
            return;
        }

        let inner = self.inner.clone();
        std::thread::scope(|scope| {
            scope.spawn(move || {
                let runtime = {
                    match tokio::runtime::Runtime::new() {
                        Ok(runtime) => runtime,
                        Err(e) => {
                            error!("BlobUploader.drop() runtime::new error. {}", e);
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    let result = inner.cancel().await;
                    if let Err(e) = result {
                        warn!("BlobUploader.drop() close error. {}", e);
                    }
                })
            });
        });
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

    pub async fn cancel(mut self) -> Result<(), TgError> {
        self.done = true;

        self.inner.cancel().await
    }
}

impl Drop for ClobUploader {
    fn drop(&mut self) {
        if self.done {
            return;
        }

        let inner = self.inner.clone();
        std::thread::scope(|scope| {
            scope.spawn(move || {
                let runtime = {
                    match tokio::runtime::Runtime::new() {
                        Ok(runtime) => runtime,
                        Err(e) => {
                            error!("ClobUploader.drop() runtime::new error. {}", e);
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    let result = inner.cancel().await;
                    if let Err(e) = result {
                        warn!("ClobUploader.drop() close error. {}", e);
                    }
                })
            });
        });
    }
}
