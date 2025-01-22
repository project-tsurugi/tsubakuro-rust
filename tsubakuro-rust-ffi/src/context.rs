use std::ffi::{c_char, CString};

use log::{debug, trace};

use crate::{
    error::TsurugiFfiError,
    return_code::{TsurugiFfiRc, TSURUGI_FFI_RC_NG_FFI_ARG0, TSURUGI_FFI_RC_OK},
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiContext {
    rc: TsurugiFfiRc,
    error: Option<TsurugiFfiError>,
    error_message: *mut c_char,
}

impl TsurugiFfiContext {
    pub(crate) fn set_error(
        context: TsurugiFfiContextHandle,
        rc: TsurugiFfiRc,
        error: TsurugiFfiError,
    ) -> TsurugiFfiRc {
        if !context.is_null() {
            let context = unsafe { &mut *context };

            context.rc = rc;

            if !context.error_message.is_null() {
                let _ = unsafe { CString::from_raw(context.error_message) };
                context.error_message = std::ptr::null_mut();
            }

            // TODO contextからerror_messageを取得するときにcontext.error_messageにセットする
            let message = error.message();
            match CString::new(message.as_str()) {
                Ok(message) => context.error_message = message.into_raw(),
                Err(e) => {
                    debug!("TsurugiFfiContext::set_error_message() error. {:?}", e);
                }
            }

            context.error = Some(error);
        }

        rc
    }
}

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

    let context = Box::new(TsurugiFfiContext {
        rc: TSURUGI_FFI_RC_OK,
        error: None,
        error_message: std::ptr::null_mut(),
    });

    let handle = Box::into_raw(context);
    unsafe {
        *context_out = handle;
    }

    trace!("{FUNCTION_NAME} end. context={:?}", handle);
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_dispose(context: TsurugiFfiContextHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_dispose()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    if context.is_null() {
        trace!("{FUNCTION_NAME} end. arg[context] is null");
        return;
    }

    unsafe {
        let context = Box::from_raw(context);

        if !context.error_message.is_null() {
            let _ = CString::from_raw(context.error_message);
        }
    }

    trace!("{FUNCTION_NAME} end");
}
