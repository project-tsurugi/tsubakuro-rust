use log::trace;

use crate::{
    context::{TsurugiFfiContext, TsurugiFfiContextHandle},
    error::TsurugiFfiError,
};

pub type TsurugiFfiRc = u32;
pub const TSURUGI_FFI_RC_OK: TsurugiFfiRc = 0;
pub const TSURUGI_FFI_RC_NG_FFI_HEADER: u32 = 0xc000_0000;
pub const TSURUGI_FFI_RC_NG_FFI_ARG0: TsurugiFfiRc = TSURUGI_FFI_RC_NG_FFI_HEADER | 0;
// pub const TSURUGI_FFI_RC_NG_FFI_ARG1: TsurugiFfiRc = TSURUGI_FFI_RC_NG_FFI_HEADER | 1;
// pub const TSURUGI_FFI_RC_NG_FFI_ARG2: TsurugiFfiRc = TSURUGI_FFI_RC_NG_FFI_HEADER | 2;

pub(crate) fn rc_ffi_arg_error(
    context: TsurugiFfiContextHandle,
    function_name: &str,
    arg_index: u32,
    arg_name: &str,
    fail_message: &str,
) -> TsurugiFfiRc {
    let message = format!("{function_name} error. arg[{arg_name}] {fail_message}");
    trace!("{message}");

    let rc = TSURUGI_FFI_RC_NG_FFI_HEADER | arg_index;
    let error = TsurugiFfiError::FfiError(rc, message);
    TsurugiFfiContext::set_error(context, rc, error)
}
