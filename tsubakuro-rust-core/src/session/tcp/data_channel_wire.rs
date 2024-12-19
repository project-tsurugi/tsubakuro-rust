use std::{
    collections::VecDeque,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};

use async_trait::async_trait;
use tokio::time::Instant;

use crate::{
    error::TgError,
    invalid_response_error,
    session::wire::{
        data_channel::{DataChannel, DataChannelWire},
        Wire, WireResponse,
    },
    util::calculate_timeout,
};

use super::{wire::TcpWire, DelegateWire};

#[derive(Debug)]
pub(crate) struct TcpDataChannelWire {
    tcp_wire: Arc<TcpWire>,
    response: Mutex<VecDeque<WireResponse>>,
    end: AtomicBool,
}

impl TcpDataChannelWire {
    pub(crate) fn new(wire: Arc<Wire>) -> TcpDataChannelWire {
        let tcp_wire = if let DelegateWire::Tcp(tcp_wire) = wire.get_delegate_wire() {
            tcp_wire.clone()
        } else {
            panic!();
        };

        TcpDataChannelWire {
            tcp_wire,
            response: Mutex::new(VecDeque::new()),
            end: AtomicBool::new(false),
        }
    }

    pub(crate) fn add_response(&self, response: WireResponse) {
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
}

#[async_trait]
impl DataChannelWire for TcpDataChannelWire {
    async fn pull1(&self, data_channel: &DataChannel, timeout: Duration) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "TcpDataChannelWire.pull1()";

        let start = Instant::now();
        loop {
            let timeout = calculate_timeout(FUNCTION_NAME, timeout, start)?;
            self.tcp_wire.pull1(timeout).await?;

            let response = self.take_response();
            match response {
                Some(WireResponse::ResponseResultSetPayload(_slot, writer, payload)) => {
                    if let Some(payload) = payload {
                        data_channel.add_writer_payload(writer, payload).await;
                    } else {
                        data_channel.flush_writer(writer).await;
                        break;
                    }
                }
                Some(WireResponse::ResponseResultSetBye(slot)) => {
                    self.set_end();
                    self.tcp_wire.send_result_set_bye_ok(slot, timeout).await?;
                    break;
                }
                None => (),
                _ => {
                    return Err(invalid_response_error!(
                        FUNCTION_NAME,
                        format!("response {response:?} is not ResponseResultSetXXX"),
                    ))
                }
            }
        }
        Ok(())
    }

    fn is_end(&self) -> bool {
        self.end.load(std::sync::atomic::Ordering::SeqCst)
    }
}
