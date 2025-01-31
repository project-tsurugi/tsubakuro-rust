use std::ffi::c_char;

use log::trace;

use crate::{
    cchar_field_clear, cchar_field_dispose, cchar_field_set,
    error::TsurugiFfiError,
    ffi_arg_require_non_null,
    return_code::{
        rc_to_name, TsurugiFfiRc, TsurugiFfiRcType, TSURUGI_FFI_RC_FFI_ARG0_ERROR,
        TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND, TSURUGI_FFI_RC_OK,
    },
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiContext {
    rc: TsurugiFfiRc,
    error: Option<TsurugiFfiError>,
    error_name: *mut c_char,
    error_message: *mut c_char,
    diagnostic_category_str: *mut c_char,
    diagnostic_structured_code: *mut c_char,
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
            cchar_field_clear!(context.error_name);
            cchar_field_clear!(context.error_message);
            cchar_field_clear!(context.diagnostic_category_str);
            cchar_field_clear!(context.diagnostic_structured_code);
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
    unsafe {
        *context_out = std::ptr::null_mut();
    }

    let context = Box::new(TsurugiFfiContext {
        rc: TSURUGI_FFI_RC_OK,
        error: None,
        error_name: std::ptr::null_mut(),
        error_message: std::ptr::null_mut(),
        diagnostic_category_str: std::ptr::null_mut(),
        diagnostic_structured_code: std::ptr::null_mut(),
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

    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 1, rc_out);
    unsafe {
        *rc_out = TSURUGI_FFI_RC_FFI_ARG0_ERROR;
    }
    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 0, context);

    unsafe {
        let context = &*context;

        *rc_out = context.rc;
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_get_error_name(
    context: TsurugiFfiContextHandle,
    name_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_error_name()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 1, name_out);
    unsafe {
        *name_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 0, context);

    let context = unsafe { &mut *context };

    if context.error_name.is_null() {
        let value = match &context.error {
            Some(e) => e.name(),
            None => rc_to_name(TSURUGI_FFI_RC_OK).to_string(),
        };
        unsafe {
            cchar_field_set!(std::ptr::null_mut(), context.error_name, value);
        }
    }

    unsafe {
        *name_out = context.error_name;
    }
    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_get_error_type(
    context: TsurugiFfiContextHandle,
    error_type_out: *mut TsurugiFfiRcType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_error_type()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 1, error_type_out);
    unsafe {
        *error_type_out = TsurugiFfiRcType::FfiError;
    }
    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 0, context);

    unsafe {
        let context = &*context;

        *error_type_out = context.rc.into();
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

    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 1, error_message_out);
    unsafe {
        *error_message_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 0, context);

    let context = unsafe { &mut *context };

    if context.error_message.is_null() {
        match &context.error {
            Some(error) => {
                let error_message = error.message();
                unsafe {
                    cchar_field_set!(std::ptr::null_mut(), context.error_message, error_message);
                }
            }
            None => {}
        }
    }

    unsafe {
        *error_message_out = context.error_message;
    }
    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_get_server_error_category_number(
    context: TsurugiFfiContextHandle,
    category_number_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_server_error_category_number()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 1, category_number_out);
    unsafe {
        *category_number_out = 0;
    }
    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 0, context);

    let context = unsafe { &mut *context };

    let value = match &context.error {
        Some(e) => match e.diagnostic_code() {
            Some(code) => code.category_number(),
            None => return TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND,
        },
        None => return TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND,
    };

    unsafe {
        *category_number_out = value;
    }
    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_get_server_error_category_str(
    context: TsurugiFfiContextHandle,
    category_str_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_server_error_category_str()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 1, category_str_out);
    unsafe {
        *category_str_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 0, context);

    let context = unsafe { &mut *context };

    if context.diagnostic_category_str.is_null() {
        match &context.error {
            Some(e) => match e.diagnostic_code() {
                Some(code) => {
                    let value = code.category_str().clone();
                    unsafe {
                        cchar_field_set!(
                            std::ptr::null_mut(),
                            context.diagnostic_category_str,
                            value
                        );
                    }
                }
                None => return TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND,
            },
            None => return TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND,
        }
    }

    unsafe {
        *category_str_out = context.diagnostic_category_str;
    }
    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_get_server_error_code_number(
    context: TsurugiFfiContextHandle,
    code_number_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_server_error_code_number()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 1, code_number_out);
    unsafe {
        *code_number_out = 0;
    }
    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 0, context);

    let context = unsafe { &mut *context };

    let value = match &context.error {
        Some(e) => match e.diagnostic_code() {
            Some(code) => code.code_number(),
            None => return TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND,
        },
        None => return TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND,
    };

    unsafe {
        *code_number_out = value;
    }
    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_context_get_server_error_structured_code(
    context: TsurugiFfiContextHandle,
    structured_code_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_context_get_server_error_structured_code()";
    trace!("{FUNCTION_NAME} start. context={:?}", context);

    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 1, structured_code_out);
    unsafe {
        *structured_code_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(std::ptr::null_mut(), FUNCTION_NAME, 0, context);

    let context = unsafe { &mut *context };

    if context.diagnostic_structured_code.is_null() {
        match &context.error {
            Some(e) => match e.diagnostic_code() {
                Some(code) => {
                    let value = code.structured_code();
                    unsafe {
                        cchar_field_set!(
                            std::ptr::null_mut(),
                            context.diagnostic_structured_code,
                            value
                        );
                    }
                }
                None => return TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND,
            },
            None => return TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND,
        }
    }

    unsafe {
        *structured_code_out = context.diagnostic_structured_code;
    }
    trace!("{FUNCTION_NAME} end");
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

        cchar_field_dispose!(context.error_name);
        cchar_field_dispose!(context.error_message);
        cchar_field_dispose!(context.diagnostic_category_str);
        cchar_field_dispose!(context.diagnostic_structured_code);
    }

    trace!("{FUNCTION_NAME} end");
}
