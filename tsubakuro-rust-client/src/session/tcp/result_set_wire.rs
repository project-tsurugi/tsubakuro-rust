use std::{
    collections::{HashMap, VecDeque},
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::{Duration, Instant},
};

use async_trait::async_trait;
use prost::bytes::BytesMut;

use crate::{
    error::TgError,
    invalid_response_error,
    prelude::sql::query_result::result_set_wire::ResultSetWire,
    session::wire::{Wire, WireResponse},
    timeout_error,
};

use super::{wire::TcpWire, DelegateWire};

#[derive(Debug)]
pub(crate) struct TcpResultSetWire {
    tcp_wire: Arc<TcpWire>,
    response: Mutex<VecDeque<WireResponse>>,
    end: AtomicBool,
    bytes_list: Mutex<VecDeque<BytesMut>>,
    writer_map: Mutex<HashMap<u8, Vec<BytesMut>>>,
}

impl TcpResultSetWire {
    pub(crate) fn new(wire: Arc<Wire>) -> TcpResultSetWire {
        let tcp_wire = if let DelegateWire::Tcp(wire) = wire.get_delegate_wire() {
            wire.clone()
        } else {
            panic!();
        };

        TcpResultSetWire {
            tcp_wire,
            response: Mutex::new(VecDeque::new()),
            end: AtomicBool::new(false),
            bytes_list: Mutex::new(VecDeque::new()),
            writer_map: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn add_result_set_response(&self, response: WireResponse) {
        let mut queue = self.response.lock().unwrap();
        queue.push_back(response);
    }

    fn take_response(&self) -> Option<WireResponse> {
        let mut queue = self.response.lock().unwrap();
        queue.pop_front()
    }

    pub(crate) fn set_end(&self) {
        self.end.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn is_end(&self) -> bool {
        self.end.load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait]
impl ResultSetWire for TcpResultSetWire {
    async fn pull(&self, timeout: Duration) -> Result<Option<BytesMut>, TgError> {
        let start = Instant::now();
        loop {
            let bytes = {
                let mut queue = self.bytes_list.lock().unwrap();
                queue.pop_front()
            };
            if bytes.is_some() {
                return Ok(bytes);
            }

            if self.is_end() {
                return Ok(None);
            }

            if !timeout.is_zero() {
                let elapsed = start.elapsed();
                if elapsed > timeout {
                    return Err(timeout_error!("TcpResultSetWire.pull() timeout"));
                }
            }

            self.pull1(timeout).await?;
        }
    }
}

impl TcpResultSetWire {
    async fn pull1(&self, timeout: Duration) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "TcpResultSetWire.pull1()";

        self.tcp_wire.pull1(timeout).await?;

        let response = self.take_response();
        match response {
            Some(WireResponse::ResponseResultSetPayload(_slot, writer, payload)) => {
                self.set_writer_payload(writer, payload)
            }
            Some(WireResponse::ResponseResultSetBye(slot)) => {
                self.set_end();
                self.tcp_wire.send_result_set_bye_ok(slot, timeout).await
            }
            None => Ok(()),
            _ => Err(invalid_response_error!(
                FUNCTION_NAME,
                format!("response {response:?} is not ResponseResultSetXXX"),
            )),
        }
    }

    fn set_writer_payload(&self, writer: u8, payload: Option<BytesMut>) -> Result<(), TgError> {
        if let Some(payload) = payload {
            let mut writer_map = self.writer_map.lock().unwrap();
            if let Some(list) = writer_map.get_mut(&writer) {
                list.push(payload);
            } else {
                let list = vec![payload];
                writer_map.insert(writer, list);
            }
        } else {
            if let Some(list) = {
                let mut writer_map = self.writer_map.lock().unwrap();
                writer_map.remove(&writer)
            } {
                let mut bytes_list = self.bytes_list.lock().unwrap();
                for bytes in list {
                    bytes_list.push_back(bytes);
                }
            }
        }
        Ok(())
    }
}
