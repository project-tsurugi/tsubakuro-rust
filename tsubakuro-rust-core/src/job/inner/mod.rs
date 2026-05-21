use std::time::Duration;

use super::cancel_job::CancelJob;
use tonic::async_trait;

use crate::error::TgError;

pub(crate) mod convert_job;
pub(crate) mod supplier_job;
pub(crate) mod value_job;
pub(crate) mod wire_slot_job;

#[async_trait]
pub(crate) trait InnerJob<T: Send> {
    async fn wait(&self, timeout: Duration) -> Result<bool, TgError>;
    async fn is_done(&self) -> Result<bool, TgError>;
    async fn take_for(&self, timeout: Duration) -> Result<T, TgError>;
    async fn cancel_async(&self) -> Result<Option<CancelJob>, TgError>;
    async fn send_cancel(&self) -> Result<(), TgError>;
    fn dispose(&self, name: &str, fail_on_error: bool);
}
