use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use prost::{
    bytes::{Buf, BytesMut},
    Message as ProstMessage,
};
use response_box::{ResponseBox, SlotEntryHandle};

use crate::{
    core_service_error,
    prelude::sql::query_result::result_set_wire::ResultSetWire,
    tateyama::{
        self,
        proto::{
            diagnostics::Record as DiagnosticsRecord,
            framework::{
                request::Header as FrameworkRequestHeader,
                response::Header as FrameworkResponseHeader,
            },
        },
    },
    timeout_error,
};
use crate::{error::TgError, job::Job, prost_decode_error};

use super::tcp::wire::TcpWire;

pub(crate) mod link;
pub(crate) mod response_box;

/// The major service message version for FrameworkRequest.Header.
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 0;

/// The minor service message version for FrameworkRequest.Header.
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 0;

pub(crate) struct Wire {
    wire: DelegateWire,
}

impl std::fmt::Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.wire)
    }
}

impl Wire {
    pub(crate) fn new(wire: DelegateWire) -> Arc<Wire> {
        Arc::new(Wire { wire })
    }

    pub(crate) fn get_delegate_wire<'a>(self: &'a Arc<Self>) -> &'a DelegateWire {
        &self.wire
    }

    pub(crate) fn set_session_id(self: &Arc<Self>, session_id: i64) -> Result<(), TgError> {
        self.wire.set_session_id(session_id)
    }

    pub(crate) fn session_id(&self) -> i64 {
        self.wire.session_id()
    }

    pub async fn send_and_pull_response<R: ProstMessage>(
        &self,
        service_id: i32,
        request: R,
        timeout: Duration,
    ) -> Result<WireResponse, TgError> {
        let slot_handle = self.send_internal(service_id, request, timeout).await?;
        let response = self.pull_internal(slot_handle, timeout).await?;
        Ok(response)
    }

    pub async fn send_and_pull_async<R: ProstMessage, T: 'static>(
        self: Arc<Wire>,
        service_id: i32,
        request: R,
        converter: Box<dyn FnOnce(WireResponse) -> Result<T, TgError> + Send>,
        timeout: Duration,
        default_timeout: Duration,
    ) -> Result<Job<T>, TgError> {
        let slot_handle = self.send_internal(service_id, request, timeout).await?;
        let wire = self.clone();
        let job = Job::new(
            |timeout| {
                Box::pin(async move {
                    let response = wire.pull_internal(slot_handle, timeout).await?;
                    converter(response)
                })
            },
            default_timeout,
        );
        Ok(job)
    }

    async fn send_internal<T: ProstMessage>(
        &self,
        service_id: i32,
        request: T,
        timeout: Duration,
    ) -> Result<Arc<SlotEntryHandle>, TgError> {
        let header = FrameworkRequestHeader {
            service_message_version_major: SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: SERVICE_MESSAGE_VERSION_MINOR,
            service_id: service_id as u64,
            session_id: self.session_id() as u64,
        };
        let header = header.encode_length_delimited_to_vec();

        let payload = request.encode_length_delimited_to_vec();

        let slot_handle = self.wire.response_box().create_slot_handle();
        let slot = slot_handle.slot();

        self.wire.send(slot, &header, &payload, timeout).await?;

        Ok(slot_handle)
    }

    async fn pull_internal(
        &self,
        slot_handle: Arc<SlotEntryHandle>,
        timeout: Duration,
    ) -> Result<WireResponse, TgError> {
        // trace!("wire.pull() loop start");
        let start = Instant::now();
        loop {
            if let Some(response) = slot_handle.take_wire_response() {
                // trace!("wire.pull() loop end");
                return Ok(response);
            }

            if !timeout.is_zero() {
                let elapsed = start.elapsed();
                if elapsed > timeout {
                    return Err(timeout_error!("Wire.pull() timeout"));
                }
            }

            self.wire.pull1(timeout).await?;
        }
    }

    pub(crate) fn create_result_set_wire(
        self: &Arc<Self>,
        result_set_name: &String,
    ) -> Result<Arc<dyn ResultSetWire>, TgError> {
        self.wire
            .create_result_set_wire(self.clone(), result_set_name)
    }
}

// DelegateWireをトレイトにしたいが、downcastが難しいので、enumにしておく
pub(crate) enum DelegateWire {
    Tcp(Arc<TcpWire>),
    _Dummy, // 列挙子がひとつしか存在していないと、enumを使っている個所で警告が出るので、ダミーとして置いておく
}

impl std::fmt::Debug for DelegateWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tcp(tcp_wire) => write!(f, "{:?}", tcp_wire),
            Self::_Dummy => write!(f, "Dummy"),
        }
    }
}

impl DelegateWire {
    fn set_session_id(&self, session_id: i64) -> Result<(), TgError> {
        match self {
            DelegateWire::Tcp(wire) => wire.set_session_id(session_id),
            _ => todo!("DelegateWire"),
        }
    }

    fn session_id(&self) -> i64 {
        match self {
            DelegateWire::Tcp(wire) => wire.session_id(),
            _ => todo!("DelegateWire"),
        }
    }

    fn response_box(&self) -> Arc<ResponseBox> {
        match self {
            DelegateWire::Tcp(wire) => wire.response_box(),
            _ => todo!("DelegateWire"),
        }
    }

    async fn send(
        &self,
        slot: i32,
        frame_header: &Vec<u8>,
        payload: &Vec<u8>,
        timeout: Duration,
    ) -> Result<(), TgError> {
        match self {
            DelegateWire::Tcp(wire) => wire.send(slot, frame_header, payload, timeout).await,
            _ => todo!("DelegateWire"),
        }
    }

    async fn pull1(&self, timeout: Duration) -> Result<(), TgError> {
        match self {
            DelegateWire::Tcp(wire) => wire.pull1(timeout).await,
            _ => todo!("DelegateWire"),
        }
    }

    fn create_result_set_wire(
        &self,
        wire: Arc<Wire>,
        result_set_name: &String,
    ) -> Result<Arc<dyn ResultSetWire>, TgError> {
        match self {
            DelegateWire::Tcp(tcp_wire) => tcp_wire.create_result_set_wire(wire, result_set_name),
            _ => todo!("DelegateWire"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum WireResponse {
    ResponseSessionPayload(/* slot */ i32, /* payload */ Option<BytesMut>),
    ResponseResultSetHello(/* slot */ i32, /* ResultSet name */ String),
    ResponseSessionBodyhead(/* slot */ i32, /* payload */ Option<BytesMut>),
    ResponseResultSetPayload(
        /* slot */ i32,
        /* writer */ u8,
        /* payload */ Option<BytesMut>,
    ),
    ResponseResultSetBye(/* slot */ i32),
}

pub(crate) fn skip_framework_header(mut payload: BytesMut) -> Result<BytesMut, TgError> {
    const FUNCTION_NAME: &str = "skip_framework_header()";

    let mut slice = payload.as_ref();
    let before_length = slice.len();
    let header = FrameworkResponseHeader::decode_length_delimited(&mut slice)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "FrameworkResponseHeader", e))?;

    if header.payload_type()
        == tateyama::proto::framework::response::header::PayloadType::ServerDiagnostics
    {
        let error_response = DiagnosticsRecord::decode_length_delimited(&mut slice)
            .map_err(|e| prost_decode_error!(FUNCTION_NAME, "DiagnosticsRecord", e))?;
        return Err(core_service_error!(FUNCTION_NAME, error_response));
    }

    let after_length = slice.len();
    payload.advance(before_length - after_length);
    Ok(payload)
}
