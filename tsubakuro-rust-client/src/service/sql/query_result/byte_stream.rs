use std::{sync::Arc, time::Duration};

use prost::bytes::{BufMut, BytesMut};

use crate::error::TgError;

use super::result_set_wire::ResultSetWire;

#[derive(Debug)]
pub(crate) struct ResultSetByteStream {
    rs_wire: Arc<dyn ResultSetWire>,
    current: Option<BytesMut>,
}

impl ResultSetByteStream {
    pub(crate) fn new(rs_wire: Arc<dyn ResultSetWire>) -> ResultSetByteStream {
        ResultSetByteStream {
            rs_wire,
            current: None,
        }
    }

    pub(crate) async fn read_u8(&mut self, timeout: Duration) -> Result<Option<u8>, TgError> {
        // TODO 効率の良い方法
        if let Some(bytes) = self.get_bytes(timeout).await? {
            let value = bytes.split_to(1)[0];
            Ok(Some(value))
        } else {
            return Ok(None);
        }
    }

    pub(crate) async fn read_all(
        &mut self,
        size: usize,
        timeout: Duration,
    ) -> Result<Option<BytesMut>, TgError> {
        // TODO 効率の良い方法
        let mut buffer = BytesMut::with_capacity(size);
        for _ in 0..size {
            if let Some(value) = self.read_u8(timeout).await? {
                buffer.put_u8(value);
            } else {
                return Ok(None);
            }
        }

        Ok(Some(buffer))
    }

    async fn get_bytes(&mut self, timeout: Duration) -> Result<Option<&mut BytesMut>, TgError> {
        if self.current.as_ref().is_some_and(|b| !b.is_empty()) {
            return Ok(self.current.as_mut());
        }
        self.current = None;

        let bytes = self.rs_wire.pull(timeout).await?;
        if bytes.is_some() {
            self.current = bytes;
            return Ok(self.current.as_mut());
        }

        return Ok(None);
    }
}
