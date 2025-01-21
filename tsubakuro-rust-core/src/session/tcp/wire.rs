use std::sync::{atomic::AtomicI64, Arc};

use log::{debug, trace};
use tokio::sync::Mutex;

use crate::{
    client_error,
    error::TgError,
    invalid_response_error,
    session::{
        tcp::link::TcpLink,
        wire::{
            data_channel::DataChannelWire, link::LinkMessage, response::WireResponse,
            response_box::ResponseBox, skip_framework_header,
        },
    },
};

use super::{
    data_channel_box::TcpDataChannelBox,
    data_channel_wire::TcpDataChannelWire,
    r#enum::{TcpRequestInfo, TcpResponseInfo},
    Wire,
};

const SESSION_ID_IS_NOT_ASSIGNED: i64 = i64::MAX;

#[derive(Debug)]
pub(crate) struct TcpWire {
    link: TcpLink,
    session_id: AtomicI64,
    response_box: Arc<ResponseBox>,
    data_channel_box: TcpDataChannelBox,
    // send_lock: Mutex<bool>, // TcpLink::send()内でロックしているので、呼び出し側では排他不要
    pull_lock: Mutex<bool>, // TcpLink::recv()してからaddResponse()するまでの排他が必要
}

impl TcpWire {
    pub(crate) fn new(link: TcpLink) -> TcpWire {
        TcpWire {
            link,
            session_id: AtomicI64::new(SESSION_ID_IS_NOT_ASSIGNED),
            response_box: Arc::new(ResponseBox::new()),
            data_channel_box: TcpDataChannelBox::new(),
            // send_lock: Mutex::new(true),
            pull_lock: Mutex::new(true),
        }
    }
}

// impl DelegateWire for TcpWire
impl TcpWire {
    pub(crate) fn set_session_id(&self, session_id: i64) -> Result<(), TgError> {
        if let Err(_) = self.session_id.compare_exchange(
            SESSION_ID_IS_NOT_ASSIGNED,
            session_id,
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
        ) {
            return Err(client_error!("Wire: session ID is already assigned"));
        }

        Ok(())
    }

    pub(crate) fn session_id(&self) -> i64 {
        self.session_id.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub(crate) fn response_box(&self) -> Arc<ResponseBox> {
        self.response_box.clone()
    }

    pub(crate) async fn send(
        &self,
        slot: i32,
        frame_header: &Vec<u8>,
        payload: &Vec<u8>,
    ) -> Result<(), TgError> {
        // let _lock = self.send_lock.lock().await;
        self.link.send(slot, frame_header, payload).await
    }

    pub(crate) async fn pull1(&self) -> Result<bool, TgError> {
        // trace!("TcpWire::pull1() start");
        let _lock = match self.pull_lock.try_lock() {
            Ok(lock) => lock,
            Err(_) => {
                trace!("TcpWire::pull1() end. try_lock fail");
                return Ok(false);}
        };

        let link_message = {
            if let Some(link_message) = self.link.recv().await? {
                link_message
            } else {
                // trace!("TcpWire::pull1() end. recv none");
                return Ok(false);
            }
        };
        trace!("TcpWire::pull1(): link_message={:?}", link_message);

        let info = TcpResponseInfo::from(link_message.info());
        if Self::is_result_set_response(info) {
            self.process_result_set_response(link_message).await?;
            trace!("TcpWire::pull1() end. process_result_set_response");
            return Ok(true);
        }

        let slot = link_message.slot();
        let is_slot_end = Self::is_slot_end(info);
        let response = tcp_convert_wire_response(link_message).await?;
        self.response_box
            .set_response_to_slot_handle(slot, response, is_slot_end);
        trace!("TcpWire::pull1() end. set_response_to_slot_handle");
        Ok(true)
    }

    pub(crate) fn create_data_channel_wire(
        &self,
        wire: Arc<Wire>,
        dc_name: &String,
    ) -> Result<Arc<dyn DataChannelWire>, TgError> {
        let dc_wire = Arc::new(TcpDataChannelWire::new(wire));
        self.data_channel_box
            .register_data_channel_wire(dc_name, dc_wire.clone())?;
        Ok(dc_wire)
    }

    pub(crate) async fn close(&self) -> Result<(), TgError> {
        self.link.close().await
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.link.is_closed()
    }
}

impl TcpWire {
    fn is_result_set_response(info: TcpResponseInfo) -> bool {
        match info {
            TcpResponseInfo::ResponseResultSetHello => true,
            TcpResponseInfo::ResponseResultSetPayload => true,
            TcpResponseInfo::ResponseResultSetBye => true,
            _ => false,
        }
    }

    fn is_slot_end(info: TcpResponseInfo) -> bool {
        match info {
            TcpResponseInfo::ResponseSessionPayload => true,
            TcpResponseInfo::ResponseResultSetHello => false,
            TcpResponseInfo::ResponseSessionBodyhead => false,
            TcpResponseInfo::ResponseResultSetPayload => false,
            TcpResponseInfo::ResponseResultSetBye => true,
            _ => {
                debug!("is_slot_end() error: unknown info({info:?})");
                false
            }
        }
    }

    pub(crate) async fn send_result_set_bye_ok(&self, slot: i32) -> Result<(), TgError> {
        // let _lock = self.send_lock.lock().await;
        self.link
            .send_header_only(TcpRequestInfo::RequestResultSetByeOk, slot)
            .await
    }

    async fn process_result_set_response(&self, link_message: LinkMessage) -> Result<(), TgError> {
        let response = tcp_convert_wire_response(link_message).await?;
        match response {
            WireResponse::ResponseResultSetHello(rs_slot, name) => {
                self.data_channel_box.set_data_channel_name(name, rs_slot);
            }
            WireResponse::ResponseResultSetPayload(rs_slot, _writer, ref _payload) => {
                let dc_wire = self.data_channel_box.get_data_channel_wire(rs_slot)?;
                dc_wire.add_response(response);
            }
            WireResponse::ResponseResultSetBye(rs_slot) => {
                let dc_wire = self.data_channel_box.release_data_channel_wire(rs_slot)?;
                dc_wire.add_response(response);
                self.send_result_set_bye_ok(rs_slot).await?;
            }
            _ => {
                return Err(client_error!(format!(
                    "response {:?} is not ResponseResultSetXXX",
                    response
                )));
            }
        }

        Ok(())
    }
}

async fn tcp_convert_wire_response(link_message: LinkMessage) -> Result<WireResponse, TgError> {
    const FUNCTION_NAME: &str = "tcp_convert_wire_response()";

    match TcpResponseInfo::from(link_message.info()) {
        TcpResponseInfo::ResponseSessionPayload => {
            // trace!("{}: RESPONSE_SESSION_PAYLOAD", FUNCTION_NAME);
            let slot = link_message.slot();
            let mut payload = link_message.take_payload().await;
            let error = skip_framework_header(&mut payload);
            Ok(WireResponse::ResponseSessionPayload(slot, payload, error))
        }
        TcpResponseInfo::ResponseSessionBodyhead => {
            // trace!("{}: RESPONSE_SESSION_BODYHEAD", FUNCTION_NAME);
            let slot = link_message.slot();
            let mut payload = link_message.take_payload().await;
            let error = skip_framework_header(&mut payload);
            Ok(WireResponse::ResponseSessionBodyhead(slot, payload, error))
        }
        TcpResponseInfo::ResponseResultSetPayload => {
            // trace!("{}: RESPONSE_RESULT_SET_PAYLOAD", FUNCTION_NAME);
            let slot = link_message.slot();
            let writer = link_message.writer();
            let payload = link_message.take_payload().await;

            Ok(WireResponse::ResponseResultSetPayload(
                slot, writer, payload,
            ))
        }
        TcpResponseInfo::ResponseResultSetHello => {
            // trace!("{}: RESPONSE_RESULT_SET_HELLO", FUNCTION_NAME);
            let slot = link_message.slot();
            let payload = link_message.take_payload().await;
            let payload = payload.ok_or(invalid_response_error!(
                FUNCTION_NAME,
                "RESPONSE_RESULT_SET_HELLO.payload is None",
            ))?;
            let message = String::from_utf8_lossy(&payload);
            let message = String::from(message);

            Ok(WireResponse::ResponseResultSetHello(slot, message))
        }
        TcpResponseInfo::ResponseResultSetBye => {
            // trace!("{}: RESPONSE_RESULT_SET_BYE", FUNCTION_NAME);
            let slot = link_message.slot();

            Ok(WireResponse::ResponseResultSetBye(slot))
        }
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            &format!(
                "invalid info in the response. info={:?}",
                link_message.info()
            ),
        )),
    }
}
