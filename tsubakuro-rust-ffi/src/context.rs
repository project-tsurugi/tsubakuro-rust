use std::ffi::c_char;

use log::trace;

use crate::{
    cchar_field_clear, cchar_field_dispose, cchar_field_set,
    error::TsurugiFfiError,
    rc_ffi_arg_error,
    return_code::{
        get_rc_type_from_rc, TsurugiFfiRc, TSURUGI_FFI_RC_FFI_ARG0_ERROR, TSURUGI_FFI_RC_OK,
    },
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiContext {
    rc: TsurugiFfiRc,
    error: Option<TsurugiFfiError>,
    error_message: *mut c_char,
}

impl TsurugiFfiContext {
    pub(crate) fn clear(context: TsurugiFfiContextHandle) {
        Self::set(context, TSURUGI_FFI_RC_OK, None);
    }

    pub(crate) fn set_error(
        context: TsurugiFfiContextHandle,
        rc: TsurugiFfiRc,
        error: TsurugiFfiError,
    ) -> TsurugiFfiRc {
        Self::set(context, rc, Some(error));
        rc
    }

    fn set(context: TsurugiFfiContextHandle, rc: TsurugiFfiRc, error: Option<TsurugiFfiError>) {
        if context.is_null() {
            return;
        }

        let context = unsafe { &mut *context };

        context.rc = rc;
        context.error = error;
        unsafe {
            cchar_field_clear!(context.error_message);
        }
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
        return TSURUGI_FFI_RC_FFI_ARG0_ERROR;
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
pub extern "C" fn tsurugi_ffi_context_get_return_code(
    context: TsurugiFfiContextHandle,
    rc_out: *mut TsurugiFfiRc,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_return_code()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    if context.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 0, "context", "is null");
    }
    if rc_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "rc_out", "is null");
    }

    unsafe {
        let context = &*context;

        *rc_out = context.rc;
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_get_error_type(
    context: TsurugiFfiContextHandle,
    error_type_out: *mut TsurugiFfiRc,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_return_code()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    if context.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 0, "context", "is null");
    }
    if error_type_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "error_type_out", "is null");
    }

    unsafe {
        let context = &*context;

        *error_type_out = get_rc_type_from_rc(context.rc);
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_get_error_message(
    context: TsurugiFfiContextHandle,
    error_message_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_error_message()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    if context.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 0, "context", "is null");
    }
    if error_message_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "error_message_out", "is null");
    }

    let context = unsafe { &mut *context };

    if !context.error_message.is_null() {
        unsafe {
            *error_message_out = context.error_message;
        }
        trace!(
            "{FUNCTION_NAME} end. error_message={:?}",
            context.error_message
        );
        return TSURUGI_FFI_RC_OK;
    }
    match &context.error {
        Some(error) => {
            let error_message = error.message().as_str();
            unsafe {
                cchar_field_set!(std::ptr::null_mut(), context.error_message, error_message);
                *error_message_out = context.error_message;
            }
        }
        None => unsafe {
            *error_message_out = std::ptr::null_mut();
        },
    }

    trace!(
        "{FUNCTION_NAME} end. error_message={:?}",
        context.error_message
    );
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

        cchar_field_dispose!(context.error_message);
    }

    trace!("{FUNCTION_NAME} end");
}
