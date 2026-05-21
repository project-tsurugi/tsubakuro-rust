use std::{sync::Arc, time::Duration};

use tonic::async_trait;

use crate::{
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
};

pub(crate) struct ConvertJob<ORG: Send, T: Send> {
    original_job: Arc<dyn InnerJob<ORG> + Send + Sync>,
    converter: Arc<dyn Fn(ORG) -> Result<T, TgError> + Send + Sync>,
}

impl<ORG: Send, T: Send> ConvertJob<ORG, T> {
    pub(crate) fn new(
        original_job: Arc<dyn InnerJob<ORG> + Send + Sync>,
        converter: Arc<dyn Fn(ORG) -> Result<T, TgError> + Send + Sync>,
    ) -> Self {
        ConvertJob {
            original_job,
            converter,
        }
    }
}

#[async_trait]
impl<ORG: Send, T: Send> InnerJob<T> for ConvertJob<ORG, T> {
    async fn wait(&self, timeout: Duration) -> Result<bool, TgError> {
        self.original_job.wait(timeout).await
    }

    async fn is_done(&self) -> Result<bool, TgError> {
        self.original_job.is_done().await
    }

    async fn take_for(&self, timeout: Duration) -> Result<T, TgError> {
        let result = self.original_job.take_for(timeout).await?;
        (self.converter)(result)
    }

    async fn cancel_async(&self) -> Result<Option<CancelJob>, TgError> {
        self.original_job.cancel_async().await
    }

    async fn send_cancel(&self) -> Result<(), TgError> {
        self.original_job.send_cancel().await
    }

    fn dispose(&self, name: &str, fail_on_error: bool) {
        self.original_job.dispose(name, fail_on_error);
    }
}
