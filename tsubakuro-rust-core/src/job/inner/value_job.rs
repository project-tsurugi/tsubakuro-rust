use std::time::Duration;

use tonic::async_trait;

use crate::{
    client_error,
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
};

pub(crate) struct ValueInnerJob<T: Send> {
    value: tokio::sync::Mutex<Option<T>>,
}

impl<T: Send> ValueInnerJob<T> {
    pub(crate) fn new(value: T) -> Self {
        ValueInnerJob {
            value: tokio::sync::Mutex::new(Some(value)),
        }
    }
}

#[async_trait(?Send)]
impl<T: Send> InnerJob<T> for ValueInnerJob<T> {
    async fn wait(&self, _timeout: Duration) -> Result<bool, TgError> {
        Ok(true)
    }

    async fn is_done(&self) -> Result<bool, TgError> {
        Ok(true)
    }

    async fn take_for(&self, _timeout: Duration) -> Result<T, TgError> {
        let mut self_value = self.value.lock().await;
        match self_value.take() {
            Some(value) => Ok(value),
            None => Err(client_error!("ValueJob::take_for(): value already taken")),
        }
    }

    async fn cancel_async(&self) -> Result<Option<CancelJob>, TgError> {
        Ok(None)
    }

    async fn send_cancel(&self) -> Result<(), TgError> {
        Ok(())
    }

    fn set_fail_on_drop_error(&self, _value: bool) {
        // do nothing
    }
}
