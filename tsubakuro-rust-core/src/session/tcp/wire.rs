use std::{
    sync::{atomic::AtomicI64, Arc},
    time::Duration,
};

use log::debug;

use crate::{
    client_error,
    error::TgError,
    invalid_response_error,
    prelude::sql::query_result::result_set_wire::ResultSetWire,
    session::{
        tcp::link::TcpLink,
        wire::{link::LinkMessage, response_box::ResponseBox, skip_framework_header, WireResponse},
    },
};

use super::{
    r#enum::{TcpRequestInfo, TcpResponseInfo},
    result_set_box::TcpResultSetBox,
    result_set_wire::TcpResultSetWire,
    Wire,
};

const SESSION_ID_IS_NOT_ASSIGNED: i64 = i64::MAX;

#[derive(Debug)]
pub(crate) struct TcpWire {
    link: TcpLink,
    session_id: AtomicI64,
    response_box: Arc<ResponseBox>,
    result_set_box: TcpResultSetBox,
}

impl TcpWire {
    pub(crate) fn new(link: TcpLink) -> TcpWire {
        TcpWire {
            link,
            session_id: AtomicI64::new(SESSION_ID_IS_NOT_ASSIGNED),
            response_box: Arc::new(ResponseBox::new()),
            result_set_box: TcpResultSetBox::new(),
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
        timeout: Duration,
    ) -> Result<(), TgError> {
        self.link.send(slot, frame_header, payload, timeout).await
    }

    pub(crate) async fn pull1(&self, timeout: Duration) -> Result<(), TgError> {
        let link_message = {
            if let Some(link_message) = self.link.recv(timeout).await? {
                link_message
            } else {
                return Ok(());
            }
        };

        let info = TcpResponseInfo::from(link_message.info());
        if Self::is_result_set_response(info) {
            self.process_result_set_response(link_message)?;
            return Ok(());
        }

        let slot = link_message.slot();
        let is_slot_end = Self::is_slot_end(info);
        let response = tcp_convert_wire_response(link_message)?;
        self.response_box
            .set_response_to_slot_handle(slot, response, is_slot_end);
        Ok(())
    }

    pub(crate) fn create_result_set_wire(
        &self,
        wire: Arc<Wire>,
        result_set_name: &String,
    ) -> Result<Arc<dyn ResultSetWire>, TgError> {
        let rs_wire = Arc::new(TcpResultSetWire::new(wire));
        self.result_set_box
            .register_result_set_wire(result_set_name, rs_wire.clone())?;
        Ok(rs_wire)
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

    pub(crate) async fn send_result_set_bye_ok(
        &self,
        slot: i32,
        timeout: Duration,
    ) -> Result<(), TgError> {
        self.link
            .send_header_only(TcpRequestInfo::RequestResultSetByeOk, slot, timeout)
            .await
    }

    fn process_result_set_response(&self, link_message: LinkMessage) -> Result<(), TgError> {
        let response = tcp_convert_wire_response(link_message)?;
        match response {
            WireResponse::ResponseResultSetHello(rs_slot, name) => {
                self.result_set_box.set_result_set_name(name, rs_slot);
            }
            WireResponse::ResponseResultSetPayload(rs_slot, _writer, ref _payload) => {
                let rs_wire = self.result_set_box.get_result_set_wire(rs_slot)?;
                rs_wire.add_result_set_response(response);
            }
            WireResponse::ResponseResultSetBye(rs_slot) => {
                let rs_wire = self.result_set_box.release_result_set_wire(rs_slot)?;
                rs_wire.add_result_set_response(response);
                rs_wire.set_end();
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

fn tcp_convert_wire_response(link_message: LinkMessage) -> Result<WireResponse, TgError> {
    const FUNCTION_NAME: &str = "tcp_convert_wire_response()";

    match TcpResponseInfo::from(link_message.info()) {
        TcpResponseInfo::ResponseSessionPayload => {
            // trace!("{}: RESPONSE_SESSION_PAYLOAD", FUNCTION_NAME);
            let slot = link_message.slot();
            let mut payload = link_message.take_payload();
            if let Some(before) = payload {
                payload = Some(skip_framework_header(before)?);
            }

            Ok(WireResponse::ResponseSessionPayload(slot, payload))
        }
        TcpResponseInfo::ResponseSessionBodyhead => {
            // trace!("{}: RESPONSE_SESSION_BODYHEAD", FUNCTION_NAME);
            let slot = link_message.slot();
            let mut payload = link_message.take_payload();
            if let Some(before) = payload {
                payload = Some(skip_framework_header(before)?);
            }

            Ok(WireResponse::ResponseSessionBodyhead(slot, payload))
        }
        TcpResponseInfo::ResponseResultSetPayload => {
            // trace!("{}: RESPONSE_RESULT_SET_PAYLOAD", FUNCTION_NAME);
            let slot = link_message.slot();
            let writer = link_message.writer();
            let payload = link_message.take_payload();

            Ok(WireResponse::ResponseResultSetPayload(
                slot, writer, payload,
            ))
        }
        TcpResponseInfo::ResponseResultSetHello => {
            // trace!("{}: RESPONSE_RESULT_SET_HELLO", FUNCTION_NAME);
            let slot = link_message.slot();
            let payload = link_message.take_payload();
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
