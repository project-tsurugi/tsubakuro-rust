use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use log::trace;
use prost::{
    bytes::{Buf, BytesMut},
    Message as ProstMessage,
};

use crate::{
    core_service_wire_response_error,
    error::TgError,
    job::Job,
    prelude::{
        endpoint::endpoint_broker::{EndpointBroker, HandshakeResult},
        ServiceMessageVersion,
    },
    prost_decode_wire_response_error, return_err_if_timeout,
    session::{tcp::wire::TcpWire, wire::crypto::Crypto},
    tateyama::proto::{
        diagnostics::Record as DiagnosticsRecord,
        framework::{
            common::{BlobInfo, RepeatedBlobInfo},
            request::Header as FrameworkRequestHeader,
            response::{header::BlobOpt, Header as FrameworkResponseHeader},
        },
    },
    util::Timeout,
};

use super::{
    data_channel::{DataChannel, DataChannelWire},
    response::{WireResponse, WireResponseError},
    response_box::{ResponseBox, SlotEntryHandle},
};

/// The major service message version for FrameworkRequest.Header.
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 0;

/// The minor service message version for FrameworkRequest.Header.
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 1;

/// Client of session wire.
pub struct WireClient;

impl ServiceMessageVersion for WireClient {
    fn service_message_version() -> String {
        format!(
            "wire-{}.{}",
            SERVICE_MESSAGE_VERSION_MAJOR, SERVICE_MESSAGE_VERSION_MINOR
        )
    }
}

pub(crate) struct Wire {
    wire: DelegateWire,
    crypto: tokio::sync::Mutex<Option<Crypto>>,
    user_name: Mutex<Option<String>>,
}

impl std::fmt::Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.wire)
    }
}

impl Wire {
    pub(crate) fn new(wire: DelegateWire) -> Arc<Wire> {
        Arc::new(Wire {
            wire,
            crypto: tokio::sync::Mutex::new(None),
            user_name: Mutex::new(None),
        })
    }

    pub(crate) fn get_delegate_wire(&self) -> &DelegateWire {
        &self.wire
    }

    pub(crate) fn initialize(&self, handshake: HandshakeResult) -> Result<(), TgError> {
        self.wire.set_session_id(handshake.session_id())?;
        *self.user_name.lock().unwrap() = handshake.user_name();
        Ok(())
    }

    pub(crate) fn session_id(&self) -> i64 {
        self.wire.session_id()
    }

    pub(crate) fn user_name(&self) -> Option<String> {
        self.user_name.lock().unwrap().clone()
    }
}

impl Wire {
    pub(crate) async fn has_encryption_key(&self) -> bool {
        let self_crypto = self.crypto.lock().await;
        if let Some(crypto) = &*self_crypto {
            crypto.has_public_key()
        } else {
            false
        }
    }

    pub(crate) async fn encrypt(
        &self,
        plain_text: &str,
        timeout: Duration,
    ) -> Result<Option<String>, TgError> {
        let mut self_crypto = self.crypto.lock().await;
        if self_crypto.is_none() {
            let pem = EndpointBroker::encryption_key(self, timeout).await?;
            let mut crypto = Crypto::from_pem(pem)?;
            let encrypted = crypto.encrypt(plain_text)?;

            *self_crypto = Some(crypto);

            Ok(encrypted)
        } else {
            let crypto = self_crypto.as_mut().unwrap();
            let encrypted = crypto.encrypt(plain_text)?;
            Ok(encrypted)
        }
    }
}

impl Wire {
    pub(crate) async fn send_only<R: ProstMessage>(
        &self,
        service_id: i32,
        request: R,
        lobs: Option<Vec<BlobInfo>>,
    ) -> Result<Arc<SlotEntryHandle>, TgError> {
        let slot_handle = self.send_internal(service_id, request, lobs).await?;
        Ok(slot_handle)
    }

    pub(crate) async fn send_and_pull_response<R: ProstMessage>(
        &self,
        service_id: i32,
        request: R,
        lobs: Option<Vec<BlobInfo>>,
        timeout: Duration,
    ) -> Result<(Arc<SlotEntryHandle>, WireResponse), TgError> {
        let timeout = Timeout::new(timeout);
        let slot_handle = self.send_internal(service_id, request, lobs).await?;
        let response = self.pull_response(&slot_handle, &timeout).await?;
        Ok((slot_handle, response))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn send_and_pull_async<R: ProstMessage, T: 'static>(
        self: &Arc<Wire>,
        job_name: &str,
        service_id: i32,
        request: R,
        lobs: Option<Vec<BlobInfo>>,
        converter: Box<dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send>,
        default_timeout: Duration,
        fail_on_drop_error: bool,
    ) -> Result<Job<T>, TgError> {
        let slot_handle = self.send_internal(service_id, request, lobs).await?;

        let wire = self.clone();
        let job = Job::new(
            job_name,
            wire,
            slot_handle,
            converter,
            default_timeout,
            fail_on_drop_error,
        );
        Ok(job)
    }

    async fn send_internal<T: ProstMessage>(
        &self,
        service_id: i32,
        request: T,
        lobs: Option<Vec<BlobInfo>>,
    ) -> Result<Arc<SlotEntryHandle>, TgError> {
        let blob_opt = lobs.map(|blobs| {
            crate::tateyama::proto::framework::request::header::BlobOpt::Blobs(RepeatedBlobInfo {
                blobs,
            })
        });
        let header = FrameworkRequestHeader {
            service_message_version_major: SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: SERVICE_MESSAGE_VERSION_MINOR,
            service_id: service_id as u64,
            session_id: self.session_id() as u64,
            blob_opt,
        };
        let header = header.encode_length_delimited_to_vec();

        let payload = request.encode_length_delimited_to_vec();

        let slot_handle = self.wire.response_box().create_slot_handle();
        let slot = slot_handle.slot();

        trace!(
            "Wire::send_internal() start. slot={}, request={:?}",
            slot,
            request
        );
        self.wire.send(slot, &header, &payload).await?;
        trace!("Wire::send_internal() end");

        Ok(slot_handle)
    }

    pub(crate) async fn send_to_slot<R: ProstMessage>(
        &self,
        slot: i32,
        service_id: i32,
        request: R,
    ) -> Result<(), TgError> {
        let header = FrameworkRequestHeader {
            service_message_version_major: SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: SERVICE_MESSAGE_VERSION_MINOR,
            service_id: service_id as u64,
            session_id: self.session_id() as u64,
            blob_opt: None,
        };
        let header = header.encode_length_delimited_to_vec();

        let payload = request.encode_length_delimited_to_vec();

        self.wire.send(slot, &header, &payload).await?;
        Ok(())
    }

    pub(crate) async fn pull_response(
        &self,
        slot_handle: &Arc<SlotEntryHandle>,
        timeout: &Timeout,
    ) -> Result<WireResponse, TgError> {
        const FUNCTION_NAME: &str = "Wire::pull()";
        let slot = slot_handle.slot();
        trace!("{FUNCTION_NAME} slot={slot} start");

        if let Some(response) = slot_handle.take_wire_response() {
            trace!("{FUNCTION_NAME} slot={slot} end. response={:?}", response);
            return Ok(response);
        }

        let function_name = format!("{FUNCTION_NAME} slot={slot}");
        loop {
            self.wire.pull1().await?;

            if let Some(response) = slot_handle.take_wire_response() {
                trace!("{function_name} end. response={:?}", response);
                return Ok(response);
            }

            return_err_if_timeout!(timeout, &function_name);

            tokio::task::yield_now().await;
        }
    }

    pub(crate) async fn wait_response(
        &self,
        slot_handle: Arc<SlotEntryHandle>,
        timeout: &Timeout,
    ) -> Result<bool, TgError> {
        const FUNCTION_NAME: &str = "Wire::wait()";
        let slot = slot_handle.slot();
        trace!("{FUNCTION_NAME} slot={slot} start");

        if slot_handle.exists_wire_response() {
            trace!("{FUNCTION_NAME} slot={slot} end. response exists");
            return Ok(true);
        }

        loop {
            self.wire.pull1().await?;

            if slot_handle.exists_wire_response() {
                trace!("{FUNCTION_NAME} slot={slot} end. response exists");
                return Ok(true);
            }

            if timeout.is_timeout() {
                trace!("{FUNCTION_NAME} slot={slot} end. timeout");
                return Ok(false);
            }
            tokio::task::yield_now().await;
        }
    }

    pub(crate) async fn check_response(
        &self,
        slot_handle: Arc<SlotEntryHandle>,
    ) -> Result<bool, TgError> {
        const FUNCTION_NAME: &str = "Wire::check()";
        let slot = slot_handle.slot();
        trace!("{FUNCTION_NAME} slot={slot} start");

        if slot_handle.exists_wire_response() {
            trace!("{FUNCTION_NAME} slot={slot} end. response exists");
            return Ok(true);
        }

        // 一度だけ受信状態を確認する
        if !self.wire.pull1().await? {
            trace!("{FUNCTION_NAME} slot={slot} end. response not exists");
            return Ok(false);
        }

        let exists = slot_handle.exists_wire_response();
        if exists {
            trace!("{FUNCTION_NAME} slot={slot} end. response exists");
        } else {
            trace!("{FUNCTION_NAME} slot={slot} end. response not exists");
        }
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

    async fn send(&self, slot: i32, frame_header: &[u8], payload: &[u8]) -> Result<(), TgError> {
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

pub(crate) fn skip_framework_header(
    payload: &mut Option<BytesMut>,
) -> (Option<HashMap<String, BlobInfo>>, Option<WireResponseError>) {
    if let Some(payload) = payload.as_mut() {
        let mut slice = payload.as_ref();
        let before_length = slice.len();

        let result = skip_framework_header_body(&mut slice);

        let after_length = slice.len();
        payload.advance(before_length - after_length);

        return match result {
            Ok(Some(blob_opt)) => (Some(blob_opt_to_map(blob_opt)), None),
            Ok(None) => (None, None),
            Err(e) => (None, Some(e)),
        };
    }

    (None, None)
}

fn skip_framework_header_body(slice: &mut &[u8]) -> Result<Option<BlobOpt>, WireResponseError> {
    const FUNCTION_NAME: &str = "skip_framework_header()";

    let header = FrameworkResponseHeader::decode_length_delimited(&mut *slice).map_err(|e| {
        prost_decode_wire_response_error!(FUNCTION_NAME, "FrameworkResponseHeader", e)
    })?;
    match header.payload_type() {
        crate::tateyama::proto::framework::response::header::PayloadType::ServerDiagnostics => {
            let record = DiagnosticsRecord::decode_length_delimited(&mut *slice).map_err(|e| {
                prost_decode_wire_response_error!(FUNCTION_NAME, "DiagnosticsRecord", e)
            })?;
            Err(core_service_wire_response_error!(FUNCTION_NAME, record))
        }
        _ => Ok(header.blob_opt),
    }
}

fn blob_opt_to_map(blob_opt: BlobOpt) -> HashMap<String, BlobInfo> {
    match blob_opt {
        BlobOpt::Blobs(repeated_blob_info) => {
            let blobs = repeated_blob_info.blobs;

            let mut map = HashMap::with_capacity(blobs.len());
            for blob in blobs {
                map.insert(blob.channel_name.clone(), blob);
            }

            map
        }
    }
}
