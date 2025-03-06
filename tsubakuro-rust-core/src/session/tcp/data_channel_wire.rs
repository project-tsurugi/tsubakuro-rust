use std::{
    collections::VecDeque,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use async_trait::async_trait;

use crate::{
    error::TgError,
    invalid_response_error, return_err_if_timeout,
    session::wire::{
        data_channel::{DataChannel, DataChannelWire},
        response::WireResponse,
        DelegateWire, Wire,
    },
    util::Timeout,
};

use super::wire::TcpWire;

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
}

#[async_trait]
impl DataChannelWire for TcpDataChannelWire {
    async fn pull1(&self, data_channel: &DataChannel, timeout: &Timeout) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "TcpDataChannelWire.pull1()";

        loop {
            return_err_if_timeout!(timeout, FUNCTION_NAME);

            self.tcp_wire.pull1().await?;

            let response = self.take_response();
            match response {
                Some(WireResponse::ResponseResultSetPayload(_rs_slot, writer, payload)) => {
                    if let Some(payload) = payload {
                        data_channel.add_writer_payload(writer, payload).await;
                    } else {
                        data_channel.flush_writer(writer).await;
                        break;
                    }
                }
                Some(WireResponse::ResponseResultSetBye(_rs_slot)) => {
                    self.set_end();
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

            tokio::task::yield_now().await;
        }
        Ok(())
    }

    fn is_end(&self) -> bool {
        self.end.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl TcpDataChannelWire {
    pub(crate) fn add_response(&self, response: WireResponse) {
        let mut queue = self.response.lock().unwrap();
        queue.push_back(response);
    }

    fn take_response(&self) -> Option<WireResponse> {
        let mut queue = self.response.lock().unwrap();
        queue.pop_front()
    }

    fn set_end(&self) {
        self.end.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
