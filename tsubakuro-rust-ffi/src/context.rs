use log::trace;

use crate::{TsurugiFfiRc, TSURUGI_FFI_RC_NG_FFI_ARG0, TSURUGI_FFI_RC_OK};

pub(crate) struct TsurugiFfiContext {}

pub type TsurugiFfiContextHandle = *mut TsurugiFfiContext;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_create(
    context_out: *mut TsurugiFfiContextHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_create()";
    trace!("{FUNCTION_NAME} start");

    if context_out.is_null() {
        trace!("{FUNCTION_NAME} error. arg[context_out] is null");
        return TSURUGI_FFI_RC_NG_FFI_ARG0;
    }

    let context = Box::new(TsurugiFfiContext {});
    unsafe {
        *context_out = Box::into_raw(context);
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_dispose(context: TsurugiFfiContextHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_dispose()";
    trace!("{FUNCTION_NAME} start");

    if context.is_null() {
        trace!("{FUNCTION_NAME} end. arg[context] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(context);
    }

    trace!("{FUNCTION_NAME} end");
}
