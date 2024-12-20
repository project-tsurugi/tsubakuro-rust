use std::{sync::Arc, time::Duration};

use data_channel::{DataChannel, DataChannelWire};
use prost::{
    bytes::{Buf, BytesMut},
    Message as ProstMessage,
};
use response_box::{ResponseBox, SlotEntryHandle};

use crate::{
    core_service_error,
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
    util::Timeout,
};
use crate::{error::TgError, job::Job, prost_decode_error};

use super::tcp::wire::TcpWire;

pub(crate) mod data_channel;
pub(crate) mod link;
pub(crate) mod response_box;

/// The major service message version for FrameworkRequest.Header.
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 0;

/// The minor service message version for FrameworkRequest.Header.
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 0;

const POLLING_INTERVAL: Duration = Duration::from_millis(20);

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
        let timeout = Timeout::new(timeout);
        let slot_handle = self.send_internal(service_id, request).await?;
        let response = self.pull_response(slot_handle, &timeout).await?;
        Ok(response)
    }

    pub async fn send_and_pull_async<R: ProstMessage, T: 'static>(
        self: Arc<Wire>,
        job_name: &str,
        service_id: i32,
        request: R,
        converter: Box<dyn Fn(WireResponse) -> Result<T, TgError> + Send>,
        default_timeout: Duration,
    ) -> Result<Job<T>, TgError> {
        let slot_handle = self.send_internal(service_id, request).await?;

        let wire = self.clone();
        let job = Job::new(job_name, wire, slot_handle, converter, default_timeout);
        Ok(job)
    }

    async fn send_internal<T: ProstMessage>(
        &self,
        service_id: i32,
        request: T,
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

        self.wire.send(slot, &header, &payload).await?;

        Ok(slot_handle)
    }

    pub(crate) async fn pull_response(
        &self,
        slot_handle: Arc<SlotEntryHandle>,
        timeout: &Timeout,
    ) -> Result<WireResponse, TgError> {
        if let Some(response) = slot_handle.take_wire_response() {
            return Ok(response);
        }

        let mut interval = tokio::time::interval(POLLING_INTERVAL);
        loop {
            interval.tick().await;

            self.wire.pull1().await?;

            if let Some(response) = slot_handle.take_wire_response() {
                return Ok(response);
            }

            timeout.return_err_if_timeout("Wire.pull()")?;
        }
    }

    pub(crate) async fn wait_response(
        &self,
        slot_handle: Arc<SlotEntryHandle>,
        timeout: &Timeout,
    ) -> Result<bool, TgError> {
        if slot_handle.exists_wire_response() {
            return Ok(true);
        }

        let mut interval = tokio::time::interval(POLLING_INTERVAL);
        loop {
            interval.tick().await;

            self.wire.pull1().await?;

            if slot_handle.exists_wire_response() {
                return Ok(true);
            }

            if timeout.is_timeout() {
                return Ok(false);
            }
        }
    }

    pub(crate) async fn check_response(
        &self,
        slot_handle: Arc<SlotEntryHandle>,
    ) -> Result<bool, TgError> {
        if slot_handle.exists_wire_response() {
            return Ok(true);
        }

        // 一度だけ受信状態を確認する
        if !self.wire.pull1().await? {
            return Ok(false);
        }

        let exists = slot_handle.exists_wire_response();
        Ok(exists)
    }

    pub(crate) fn create_data_channel(
        self: &Arc<Self>,
        dc_name: &String,
    ) -> Result<DataChannel, TgError> {
        let dc_wire = self.wire.create_data_channel_wire(self.clone(), dc_name)?;
        let data_channel = DataChannel::new(dc_name, dc_wire);
        Ok(data_channel)
    }

    pub async fn close(&self) -> Result<(), TgError> {
        self.wire.close().await
    }

    pub fn is_closed(&self) -> bool {
        self.wire.is_closed()
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
    ) -> Result<(), TgError> {
        match self {
            DelegateWire::Tcp(wire) => wire.send(slot, frame_header, payload).await,
            _ => todo!("DelegateWire"),
        }
    }

    async fn pull1(&self) -> Result<bool, TgError> {
        match self {
            DelegateWire::Tcp(wire) => wire.pull1().await,
            _ => todo!("DelegateWire"),
        }
    }

    fn create_data_channel_wire(
        &self,
        wire: Arc<Wire>,
        dc_name: &String,
    ) -> Result<Arc<dyn DataChannelWire>, TgError> {
        match self {
            DelegateWire::Tcp(tcp_wire) => tcp_wire.create_data_channel_wire(wire, dc_name),
            _ => todo!("DelegateWire"),
        }
    }

    async fn close(&self) -> Result<(), TgError> {
        match self {
            DelegateWire::Tcp(tcp_wire) => tcp_wire.close().await,
            _ => todo!("DelegateWire"),
        }
    }

    fn is_closed(&self) -> bool {
        match self {
            DelegateWire::Tcp(tcp_wire) => tcp_wire.is_closed(),
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
