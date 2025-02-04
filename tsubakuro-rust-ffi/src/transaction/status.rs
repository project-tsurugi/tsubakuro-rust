use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiStringHandle,
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiTransactionStatus {
    transaction_status: TransactionStatus,
    error_name: Option<CString>,
    error_message: Option<CString>,
    diagnostic_category_str: Option<CString>,
    diagnostic_structured_code: Option<CString>,
}

impl TsurugiFfiTransactionStatus {
    pub(crate) fn new(transaction_status: TransactionStatus) -> TsurugiFfiTransactionStatus {
        TsurugiFfiTransactionStatus {
            transaction_status,
            error_name: None,
            error_message: None,
            diagnostic_category_str: None,
            diagnostic_structured_code: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiTransactionStatus {
    type Target = TransactionStatus;

    fn deref(&self) -> &Self::Target {
        &self.transaction_status
    }
}

impl std::ops::DerefMut for TsurugiFfiTransactionStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction_status
    }
}

pub type TsurugiFfiTransactionStatusHandle = *mut TsurugiFfiTransactionStatus;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_is_normal(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusHandle,
    is_normal_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_is_normal()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, is_normal_out={:?}",
        context,
        transaction_status,
        is_normal_out
    );

    ffi_arg_out_initialize!(is_normal_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_normal_out);

    let status = unsafe { &mut *transaction_status };

    let is_normal = status.is_normal();
    unsafe {
        *is_normal_out = is_normal;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_is_error(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusHandle,
    is_error_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_is_error()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, is_error_out={:?}",
        context,
        transaction_status,
        is_error_out
    );

    ffi_arg_out_initialize!(is_error_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_error_out);

    let status = unsafe { &mut *transaction_status };

    let is_error = status.is_error();
    unsafe {
        *is_error_out = is_error;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_get_server_error_name(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusHandle,
    name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_get_server_error_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, name_out={:?}",
        context,
        transaction_status,
        name_out
    );

    ffi_arg_out_initialize!(name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, name_out);

    let status = unsafe { &mut *transaction_status };

    if status.error_name.is_none() {
        let value = match status.diagnostic_code() {
            Some(e) => e.name().clone(),
            None => {
                trace!("{FUNCTION_NAME} end");
                return rc_ok(context);
            }
        };
        cchar_field_set!(context, status.error_name, value);
    }

    let ptr = cstring_to_cchar!(status.error_name);
    unsafe {
        *name_out = ptr;
    }

    trace!("{FUNCTION_NAME} end. (name={:?})", ptr);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_get_server_error_message(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusHandle,
    error_message_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_get_server_error_message()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, error_message_out={:?}",
        context,
        transaction_status,
        error_message_out
    );

    ffi_arg_out_initialize!(error_message_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, error_message_out);

    let status = unsafe { &mut *transaction_status };

    if status.error_message.is_none() {
        let value = match status.server_error() {
            Some(TgError::ServerError(_, _, server_message)) => server_message.clone(),
            Some(e) => e.message().clone(),
            None => {
                trace!("{FUNCTION_NAME} end");
                return rc_ok(context);
            }
        };
        cchar_field_set!(context, status.error_message, value);
    }

    let ptr = cstring_to_cchar!(status.error_message);
    unsafe {
        *error_message_out = ptr;
    }

    trace!("{FUNCTION_NAME} end. (error_message={:?})", ptr);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_get_server_error_category_number(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusHandle,
    category_number_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_get_server_error_category_number()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, category_number_out={:?}",
        context,
        transaction_status,
        category_number_out
    );

    ffi_arg_out_initialize!(category_number_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, category_number_out);

    let status = unsafe { &mut *transaction_status };

    let value = match status.diagnostic_code() {
        Some(code) => code.category_number(),
        None => {
            trace!("{FUNCTION_NAME} end");
            return rc_ok(context);
        }
    };

    unsafe {
        *category_number_out = value;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_get_server_error_category_str(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusHandle,
    category_str_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_get_server_error_category_str()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, category_str_out={:?}",
        context,
        transaction_status,
        category_str_out
    );

    ffi_arg_out_initialize!(category_str_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, category_str_out);

    let status = unsafe { &mut *transaction_status };

    if status.diagnostic_category_str.is_none() {
        let value = match status.diagnostic_code() {
            Some(e) => e.category_str().clone(),
            None => {
                trace!("{FUNCTION_NAME} end");
                return rc_ok(context);
            }
        };
        cchar_field_set!(context, status.diagnostic_category_str, value);
    }

    let ptr = cstring_to_cchar!(status.diagnostic_category_str);
    unsafe {
        *category_str_out = ptr;
    }

    trace!("{FUNCTION_NAME} end. (category_str={:?})", ptr);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_get_server_error_code_number(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusHandle,
    code_number_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_get_server_error_code_number()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, code_number_out={:?}",
        context,
        transaction_status,
        code_number_out
    );

    ffi_arg_out_initialize!(code_number_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, code_number_out);

    let status = unsafe { &mut *transaction_status };

    let value = match status.diagnostic_code() {
        Some(code) => code.code_number(),
        None => {
            trace!("{FUNCTION_NAME} end");
            return rc_ok(context);
        }
    };

    unsafe {
        *code_number_out = value;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_get_server_error_structured_code(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusHandle,
    structured_code_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_get_server_error_structured_code()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, structured_code_out={:?}",
        context,
        transaction_status,
        structured_code_out
    );

    ffi_arg_out_initialize!(structured_code_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, structured_code_out);

    let status = unsafe { &mut *transaction_status };

    if status.diagnostic_structured_code.is_none() {
        let value = match status.diagnostic_code() {
            Some(e) => e.structured_code(),
            None => {
                trace!("{FUNCTION_NAME} end");
                return rc_ok(context);
            }
        };
        cchar_field_set!(context, status.diagnostic_structured_code, value);
    }

    let ptr = cstring_to_cchar!(status.diagnostic_structured_code);
    unsafe {
        *structured_code_out = ptr;
    }

    trace!("{FUNCTION_NAME} end. (structured_code={:?})", ptr);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_dispose(
    transaction_status: TsurugiFfiTransactionStatusHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_dispose()";
    trace!(
        "{FUNCTION_NAME} start. transaction_status={:?}",
        transaction_status
    );

    if transaction_status.is_null() {
        trace!("{FUNCTION_NAME} end. arg[transaction_status] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(transaction_status);
    }

    trace!("{FUNCTION_NAME} end");
}
