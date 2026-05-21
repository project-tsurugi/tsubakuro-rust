use std::{sync::Arc, time::Duration};

use tonic::async_trait;

use crate::{
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
};

pub(crate) struct ValueJob<T: Send> {
    supplier: Arc<dyn Fn() -> Result<T, TgError> + Send + Sync>,
}

impl<T: Send> ValueJob<T> {
    pub(crate) fn new(supplier: Arc<dyn Fn() -> Result<T, TgError> + Send + Sync>) -> Self {
        ValueJob { supplier }
    }
}

#[async_trait]
impl<T: Send> InnerJob<T> for ValueJob<T> {
    async fn wait(&self, _timeout: Duration) -> Result<bool, TgError> {
        Ok(true)
    }

    async fn is_done(&self) -> Result<bool, TgError> {
        Ok(true)
    }

    async fn take_for(&self, _timeout: Duration) -> Result<T, TgError> {
        (self.supplier)()
    }

    async fn cancel_async(&self) -> Result<Option<CancelJob>, TgError> {
        Ok(None)
    }

    async fn send_cancel(&self) -> Result<(), TgError> {
        Ok(())
    }

    fn dispose(&self, _name: &str, _fail_on_error: bool) {
        // do nothing
    }
}
