use std::{sync::Arc, time::Duration};

use prost::bytes::{Buf, BytesMut};

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

    // TODO EOFのときはOk(None)でなくErr(EOF)を返すべき？
    pub(crate) async fn read_u8(&mut self, timeout: Duration) -> Result<Option<u8>, TgError> {
        if let Some(bytes) = self.get_bytes(timeout).await? {
            let value = bytes[0];
            bytes.advance(1);
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
        let bytes = {
            if let Some(bytes) = self.get_bytes(timeout).await? {
                bytes
            } else {
                return Ok(None);
            }
        };
        let bytes_len = bytes.len();
        if bytes_len == size {
            let bytes = self.current.take();
            return Ok(bytes);
        }
        if bytes_len > size {
            let bytes = bytes.split_to(size);
            return Ok(Some(bytes));
        }

        let mut buffer = self.current.take().unwrap();
        let mut remain_size = size - bytes_len;
        buffer.reserve(remain_size);
        loop {
            let bytes = {
                if let Some(bytes) = self.get_bytes(timeout).await? {
                    bytes
                } else {
                    return Ok(None);
                }
            };
            let bytes_len = bytes.len();

            if bytes_len == remain_size {
                let bytes = self.current.take().unwrap();
                buffer.unsplit(bytes);
                return Ok(Some(buffer));
            }
            if bytes_len > remain_size {
                let bytes = bytes.split_to(remain_size);
                buffer.unsplit(bytes);
                return Ok(Some(buffer));
            }

            let bytes = self.current.take().unwrap();
            buffer.unsplit(bytes);
            remain_size -= bytes_len;
        }
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
