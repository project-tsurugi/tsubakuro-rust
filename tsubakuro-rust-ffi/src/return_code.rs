use tsubakuro_rust_core::error::TgError;

use crate::context::{TsurugiFfiContext, TsurugiFfiContextHandle};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TsurugiFfiRcType {
    Ok = 0,
    FfiError = 1,
    CoreClientError = 2,
    CoreServerError = 3,
}

impl From<TsurugiFfiRc> for TsurugiFfiRcType {
    fn from(value: TsurugiFfiRc) -> Self {
        match value >> 30 {
            0 => Self::Ok,
            1 => Self::FfiError,
            2 => Self::CoreClientError,
            3 => Self::CoreServerError,
            _ => panic!("unsupported rc={:x}", value),
        }
    }
}

// pub const TSURUGI_FFI_RC_TYPE_OK: u32 = TsurugiFfiRcType::Ok as u32;
pub const TSURUGI_FFI_RC_TYPE_FFI_ERROR: u32 = TsurugiFfiRcType::FfiError as u32;
pub const TSURUGI_FFI_RC_TYPE_CORE_CLIENT_ERROR: u32 = TsurugiFfiRcType::CoreClientError as u32;
pub const TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR: u32 = TsurugiFfiRcType::CoreServerError as u32;

pub type TsurugiFfiRc = u32;
pub const TSURUGI_FFI_RC_OK: TsurugiFfiRc = 0;

pub const TSURUGI_FFI_RC_FFI_BASE: u32 = TSURUGI_FFI_RC_TYPE_FFI_ERROR << 30;
pub const TSURUGI_FFI_RC_FFI_ARG_ERROR: u32 = TSURUGI_FFI_RC_FFI_BASE | (0 << 24);
pub const TSURUGI_FFI_RC_FFI_ARG0_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 0;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_FFI_ARG1_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 1;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_FFI_ARG2_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 2;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_FFI_ARG3_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 3;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_FFI_ARG4_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 4;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_FFI_ARG5_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 5;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_FFI_ARG6_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 6;
pub const TSURUGI_FFI_RC_FFI_JOB_ERROR: u32 = TSURUGI_FFI_RC_FFI_BASE | (1 << 24);
pub const TSURUGI_FFI_RC_FFI_JOB_ALREADY_CLOSED: u32 = TSURUGI_FFI_RC_FFI_JOB_ERROR | 1;

pub const TSURUGI_FFI_RC_FFI_ERROR: u32 = TSURUGI_FFI_RC_FFI_BASE | (1 << 24);
pub const TSURUGI_FFI_RC_FFI_NUL_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ERROR | 1;

pub const TSURUGI_FFI_RC_CORE_CLIENT_ERROR: u32 = TSURUGI_FFI_RC_TYPE_CORE_CLIENT_ERROR << 30;
pub const TSURUGI_FFI_RC_CORE_CLIENT_CLIENT_ERROR: TsurugiFfiRc =
    TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (1 << 16);
pub const TSURUGI_FFI_RC_CORE_CLIENT_TIMEOUT_ERROR: TsurugiFfiRc =
    TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (2 << 16);
pub const TSURUGI_FFI_RC_CORE_CLIENT_IO_ERROR: TsurugiFfiRc =
    TSURUGI_FFI_RC_CORE_CLIENT_ERROR | (3 << 16);

pub const TSURUGI_FFI_RC_CORE_SERVER_ERROR: u32 = TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR << 30;

pub(crate) fn rc_ok(context: TsurugiFfiContextHandle) -> TsurugiFfiRc {
    TsurugiFfiContext::clear(context);
    TSURUGI_FFI_RC_OK
}

#[macro_export]
macro_rules! rc_ffi_arg_error {
    ($context:expr, $function_name:expr, $arg_index:expr, $arg_name:expr, $fail_message:expr) => {{
        let message = format!(
            "{} error. arg[{}] {}",
            $function_name, $arg_name, $fail_message
        );
        log::trace!("{message}");

        let rc = $crate::return_code::TSURUGI_FFI_RC_FFI_ARG_ERROR | $arg_index;
        let error = $crate::error::TsurugiFfiError::FfiError(rc, message);
        $crate::context::TsurugiFfiContext::set_error($context, rc, error)
    }};
}

#[macro_export]
macro_rules! rc_core_error {
    ($context:expr, $function_name:expr, $error:expr) => {{
        log::trace!("{} error. {}", $function_name, $error.message());

        let rc = $crate::return_code::get_rc_from_core_error(&$error);
        let error = $crate::error::TsurugiFfiError::CoreError(rc, $error);
        $crate::context::TsurugiFfiContext::set_error($context, rc, error)
    }};
}

pub(crate) fn get_rc_from_core_error(error: &TgError) -> TsurugiFfiRc {
    match error {
        TgError::ClientError(_, _) => TSURUGI_FFI_RC_CORE_CLIENT_CLIENT_ERROR,
        TgError::TimeoutError(_) => TSURUGI_FFI_RC_CORE_CLIENT_TIMEOUT_ERROR,
        TgError::IoError(_, _) => TSURUGI_FFI_RC_CORE_CLIENT_IO_ERROR,
        TgError::ServerError(_, code, _) => {
            TSURUGI_FFI_RC_CORE_SERVER_ERROR
                | ((code.category_number() as TsurugiFfiRc) << 20)
                | (code.code_number() as TsurugiFfiRc)
        }
    }
}
