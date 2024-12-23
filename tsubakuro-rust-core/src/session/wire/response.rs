use prost::bytes::BytesMut;

use crate::{
    core_service_error, error::TgError, prost_decode_error,
    tateyama::proto::diagnostics::Record as DiagnosticsRecord,
};

#[derive(Debug)]
pub(crate) enum WireResponse {
    ResponseSessionPayload(
        /* slot */ i32,
        /* payload */ Option<BytesMut>,
        Option<WireResponseError>,
    ),
    ResponseResultSetHello(/* slot */ i32, /* ResultSet name */ String),
    ResponseSessionBodyhead(
        /* slot */ i32,
        /* payload */ Option<BytesMut>,
        Option<WireResponseError>,
    ),
    ResponseResultSetPayload(
        /* slot */ i32,
        /* writer */ u8,
        /* payload */ Option<BytesMut>,
    ),
    ResponseResultSetBye(/* slot */ i32),
}

// WireResponseではTgErrorを保持できない。
// （TgErrorで保持しているstd::error::ErrorがSend,Syncを実装していないので、スレッド間で渡せない為）
// そこで、WireResponseで保持する専用のエラーを用意する。
#[derive(Debug)]
pub(crate) enum WireResponseError {
    ProstDecodeError(
        /* function_name */ String,
        /* data_name */ String,
        prost::DecodeError,
    ),
    CoreServiceError(/*function_name*/ String, DiagnosticsRecord),
}

impl WireResponseError {
    pub(crate) fn to_tg_error(&self) -> TgError {
        match self {
            WireResponseError::ProstDecodeError(function_name, data_name, cause) => {
                prost_decode_error!(function_name, data_name, cause.clone())
            }
            WireResponseError::CoreServiceError(function_name, record) => {
                core_service_error!(function_name, record)
            }
        }
    }
}

#[macro_export]
macro_rules! prost_decode_wire_response_error {
    ($function_name:expr, $data_name:expr, $cause:expr) => {
        $crate::session::wire::WireResponseError::ProstDecodeError(
            $function_name.to_string(),
            $data_name.to_string(),
            $cause,
        )
    };
}

#[macro_export]
macro_rules! core_service_wire_response_error {
    ($function_name:expr, $cause:expr) => {
        $crate::session::wire::WireResponseError::CoreServiceError(
            $function_name.to_string(),
            $cause,
        )
    };
}
