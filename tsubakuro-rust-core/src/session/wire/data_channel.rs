use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use async_trait::async_trait;
use prost::bytes::{Buf, BytesMut};
use tokio::sync::{Mutex, MutexGuard};

use crate::{error::TgError, return_err_if_timeout, util::Timeout};

#[async_trait]
pub(crate) trait DataChannelWire: std::fmt::Debug + Send + Sync {
    async fn pull1(&self, data_channel: &DataChannel, timeout: &Timeout) -> Result<(), TgError>;
    fn is_end(&self) -> bool;
}

pub(crate) struct DataChannel {
    name: String,
    dc_wire: Arc<dyn DataChannelWire>,
    bytes_list: Mutex<VecDeque<BytesMut>>,
    writer_map: Mutex<HashMap<u8, Vec<BytesMut>>>,
    current: Mutex<Option<BytesMut>>,
}

impl std::fmt::Debug for DataChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataChannel")
            .field("name", &self.name)
            .field("dc_wire", &self.dc_wire)
            .finish()
    }
}

impl DataChannel {
    pub(crate) fn new(dc_name: &str, dc_wire: Arc<dyn DataChannelWire>) -> DataChannel {
        DataChannel {
            name: dc_name.to_string(),
            dc_wire,
            bytes_list: Mutex::new(VecDeque::new()),
            writer_map: Mutex::new(HashMap::new()),
            current: Mutex::new(None),
        }
    }
}

impl DataChannel {
    // TODO EOFのときはOk(None)でなくErr(EOF)を返すべき？
    pub(crate) async fn read_u8(&mut self, timeout: &Timeout) -> Result<Option<u8>, TgError> {
        let mut current = self.current.lock().await;

        if !self.exists_current(&mut current, timeout).await? {
            return Ok(None);
        }

        let bytes = current.as_mut().unwrap();
        let value = bytes[0];
        bytes.advance(1);
        Ok(Some(value))
    }

    pub(crate) async fn read_all(
        &mut self,
        size: usize,
        timeout: &Timeout,
    ) -> Result<Option<BytesMut>, TgError> {
        let mut current = self.current.lock().await;

        if !self.exists_current(&mut current, timeout).await? {
            return Ok(None);
        }
        let bytes = current.as_mut().unwrap();
        let bytes_len = bytes.len();
        if bytes_len == size {
            let bytes = current.take();
            return Ok(bytes);
        }
        if bytes_len > size {
            let bytes = bytes.split_to(size);
            return Ok(Some(bytes));
        }

        let mut buffer = current.take().unwrap();
        let mut remain_size = size - bytes_len;
        buffer.reserve(remain_size);
        loop {
            if !self.exists_current(&mut current, timeout).await? {
                return Ok(None);
            }
            let bytes = current.as_mut().unwrap();
            let bytes_len = bytes.len();

            if bytes_len == remain_size {
                let bytes = current.take().unwrap();
                buffer.unsplit(bytes);
                return Ok(Some(buffer));
            }
            if bytes_len > remain_size {
                let bytes = bytes.split_to(remain_size);
                buffer.unsplit(bytes);
                return Ok(Some(buffer));
            }

            let bytes = current.take().unwrap();
            buffer.unsplit(bytes);
            remain_size -= bytes_len;
        }
    }

    async fn exists_current<'a>(
        &self,
        current: &mut MutexGuard<'a, Option<BytesMut>>,
        timeout: &Timeout,
    ) -> Result<bool, TgError> {
        if current.as_ref().is_some_and(|b| !b.is_empty()) {
            return Ok(true);
        }

        let bytes = self.pull(timeout).await?;
        let exists = bytes.is_some();
        **current = bytes;

        Ok(exists)
    }
}

impl DataChannel {
    async fn pull(&self, timeout: &Timeout) -> Result<Option<BytesMut>, TgError> {
        loop {
            let bytes = {
                let mut bytes_list = self.bytes_list.lock().await;
                bytes_list.pop_front()
            };
            if bytes.is_some() {
                return Ok(bytes);
            }

            if self.dc_wire.is_end() {
                return Ok(None);
            }

            return_err_if_timeout!(timeout, "DataChannel::pull()");

            self.dc_wire.pull1(self, timeout).await?;
        }
    }

    pub(crate) async fn add_writer_payload(&self, writer: u8, payload: BytesMut) {
        let mut writer_map = self.writer_map.lock().await;
        if let Some(list) = writer_map.get_mut(&writer) {
            list.push(payload);
        } else {
            let list = vec![payload];
            writer_map.insert(writer, list);
        }
    }

    pub(crate) async fn flush_writer(&self, writer: u8) {
        if let Some(list) = {
            let mut writer_map = self.writer_map.lock().await;
            writer_map.remove(&writer)
        } {
            let mut bytes_list = self.bytes_list.lock().await;
            for bytes in list {
                bytes_list.push_back(bytes);
            }
        }
    }
}
