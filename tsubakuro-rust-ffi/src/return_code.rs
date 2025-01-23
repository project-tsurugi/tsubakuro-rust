use crate::context::{TsurugiFfiContext, TsurugiFfiContextHandle};

#[allow(dead_code)]
pub const TSURUGI_FFI_RC_TYPE_OK: u32 = 0;
pub const TSURUGI_FFI_RC_TYPE_FFI_ERROR: u32 = 1;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_TYPE_CORE_CLIENT_ERROR: u32 = 2;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_TYPE_CORE_SERVER_ERROR: u32 = 3;

pub type TsurugiFfiRc = u32;
pub const TSURUGI_FFI_RC_OK: TsurugiFfiRc = 0;

pub const TSURUGI_FFI_RC_FFI_BASE: u32 = TSURUGI_FFI_RC_TYPE_FFI_ERROR << 28;
pub const TSURUGI_FFI_RC_FFI_ARG_ERROR: u32 = TSURUGI_FFI_RC_FFI_BASE | (0 << 24);
pub const TSURUGI_FFI_RC_FFI_ARG0_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 0;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_FFI_ARG1_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 1;
#[allow(dead_code)]
pub const TSURUGI_FFI_RC_FFI_ARG2_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ARG_ERROR | 2;

pub const TSURUGI_FFI_RC_FFI_ERROR: u32 = TSURUGI_FFI_RC_FFI_BASE | (1 << 24);
pub const TSURUGI_FFI_RC_FFI_NUL_ERROR: TsurugiFfiRc = TSURUGI_FFI_RC_FFI_ERROR | 1;

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
