use std::{time::Duration};

use tonic::async_trait;

use crate::{
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
};

pub(crate) struct ConvertInnerJob<ORG: Send, T: Send> {
    original_job: Box<dyn InnerJob<ORG> + Send>,
    converter: Box<dyn Fn(ORG) -> Result<T, TgError> + Send>,
}

impl<ORG: Send, T: Send> ConvertInnerJob<ORG, T> {
    pub(crate) fn new(
        original_job: Box<dyn InnerJob<ORG> + Send>,
        converter: Box<dyn Fn(ORG) -> Result<T, TgError> + Send>,
    ) -> Self {
        ConvertInnerJob {
            original_job,
            converter,
        }
    }
}

#[async_trait(?Send)]
impl<ORG: Send, T: Send> InnerJob<T> for ConvertInnerJob<ORG, T> {
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

    fn set_fail_on_drop_error(&self, value: bool) {
        self.original_job.set_fail_on_drop_error(value);
    }
}
