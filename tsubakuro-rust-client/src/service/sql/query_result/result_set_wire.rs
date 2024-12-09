use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use prost::bytes::BytesMut;

use crate::error::TgError;

#[async_trait]
pub(crate) trait ResultSetWire: Debug {
    async fn pull(&self, timeout: Duration) -> Result<Option<BytesMut>, TgError>;
}
