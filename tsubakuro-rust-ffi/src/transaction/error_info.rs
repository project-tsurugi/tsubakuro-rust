//! Transaction error information.

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
pub struct TsurugiFfiTransactionErrorInfo {
    transaction_error_info: TransactionErrorInfo,
    error_name: Option<CString>,
    error_message: Option<CString>,
    diagnostic_category_str: Option<CString>,
    diagnostic_structured_code: Option<CString>,
}

impl TsurugiFfiTransactionErrorInfo {
    pub(crate) fn new(
        transaction_error_info: TransactionErrorInfo,
    ) -> TsurugiFfiTransactionErrorInfo {
        TsurugiFfiTransactionErrorInfo {
            transaction_error_info,
            error_name: None,
            error_message: None,
            diagnostic_category_str: None,
            diagnostic_structured_code: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiTransactionErrorInfo {
    type Target = TransactionErrorInfo;

    fn deref(&self) -> &Self::Target {
        &self.transaction_error_info
    }
}

impl std::ops::DerefMut for TsurugiFfiTransactionErrorInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction_error_info
    }
}

/// Transaction error information.
pub type TsurugiFfiTransactionErrorInfoHandle = *mut TsurugiFfiTransactionErrorInfo;

/// TransactionErrorInfo: Whether the status is normal.
///
/// See [`TransactionErrorInfo::is_normal`].
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
///
/// # Returns
/// - `is_normal_out` - `true`: No error / `false`: Error occurred in transaction.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_is_normal(
    context: TsurugiFfiContextHandle,
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
    is_normal_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_error_info_is_normal()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_error_info={:?}, is_normal_out={:?}",
        context,
        transaction_error_info,
        is_normal_out
    );

    ffi_arg_out_initialize!(is_normal_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_error_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_normal_out);

    let status = unsafe { &mut *transaction_error_info };

    let value = status.is_normal();

    unsafe {
        *is_normal_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (is_normal={:?})", rc, value);
    rc
}

/// TransactionErrorInfo: Whether the status is error.
///
/// See [`TransactionErrorInfo::is_error`].
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
///
/// # Returns
/// - `is_error_out` - `true`: Error occurred in transaction / `false`: No error.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_is_error(
    context: TsurugiFfiContextHandle,
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
    is_error_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_error_info_is_error()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_error_info={:?}, is_error_out={:?}",
        context,
        transaction_error_info,
        is_error_out
    );

    ffi_arg_out_initialize!(is_error_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_error_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_error_out);

    let status = unsafe { &mut *transaction_error_info };

    let value = status.is_error();

    unsafe {
        *is_error_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (is_error={:?})", rc, value);
    rc
}

/// TransactionErrorInfo: Get server error name.
///
/// See [`TransactionErrorInfo::server_error`].
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
///
/// # Returns
/// - `error_name_out` - error name (`null` if no error).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_get_server_error_name(
    context: TsurugiFfiContextHandle,
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
    error_name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_error_info_get_server_error_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_error_info={:?}, error_name_out={:?}",
        context,
        transaction_error_info,
        error_name_out
    );

    ffi_arg_out_initialize!(error_name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_error_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, error_name_out);

    let status = unsafe { &mut *transaction_error_info };

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
        *error_name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (error_name={:?})", rc, ptr);
    rc
}

/// TransactionErrorInfo: Get server error message.
///
/// See [`TransactionErrorInfo::server_error`].
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
///
/// # Returns
/// - `error_message_out` - error message (`null` if no error).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_get_server_error_message(
    context: TsurugiFfiContextHandle,
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
    error_message_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_error_info_get_server_error_message()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_error_info={:?}, error_message_out={:?}",
        context,
        transaction_error_info,
        error_message_out
    );

    ffi_arg_out_initialize!(error_message_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_error_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, error_message_out);

    let status = unsafe { &mut *transaction_error_info };

    if status.error_message.is_none() {
        let value = match status.server_error() {
            Some(TgError::ServerError(_, _, _, server_message)) => server_message.clone(),
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

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (error_message={:?})", rc, ptr);
    rc
}

/// TransactionErrorInfo: Get server error category.
///
/// See [`TransactionErrorInfo::server_error`].
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
///
/// # Returns
/// - `category_number_out` - error category (0 if no error).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_get_server_error_category_number(
    context: TsurugiFfiContextHandle,
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
    category_number_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str =
        "tsurugi_ffi_transaction_error_info_get_server_error_category_number()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_error_info={:?}, category_number_out={:?}",
        context,
        transaction_error_info,
        category_number_out
    );

    ffi_arg_out_initialize!(category_number_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_error_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, category_number_out);

    let status = unsafe { &mut *transaction_error_info };

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

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (category_number={:?})",
        rc,
        value
    );
    rc
}

/// TransactionErrorInfo: Get server error category.
///
/// See [`TransactionErrorInfo::server_error`].
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
///
/// # Returns
/// - `category_str_out` - error category (`null` if no error).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_get_server_error_category_str(
    context: TsurugiFfiContextHandle,
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
    category_str_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str =
        "tsurugi_ffi_transaction_error_info_get_server_error_category_str()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_error_info={:?}, category_str_out={:?}",
        context,
        transaction_error_info,
        category_str_out
    );

    ffi_arg_out_initialize!(category_str_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_error_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, category_str_out);

    let status = unsafe { &mut *transaction_error_info };

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

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (category_str={:?})", rc, ptr);
    rc
}

/// TransactionErrorInfo: Get server error code.
///
/// See [`TransactionErrorInfo::server_error`].
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
///
/// # Returns
/// - `code_number_out` - error code (0 if no error).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_get_server_error_code_number(
    context: TsurugiFfiContextHandle,
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
    code_number_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_error_info_get_server_error_code_number()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_error_info={:?}, code_number_out={:?}",
        context,
        transaction_error_info,
        code_number_out
    );

    ffi_arg_out_initialize!(code_number_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_error_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, code_number_out);

    let status = unsafe { &mut *transaction_error_info };

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

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (code_number={:?})", rc, value);
    rc
}

/// TransactionErrorInfo: Get server error structured code.
///
/// See [`TransactionErrorInfo::server_error`].
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
///
/// # Returns
/// - `structured_code_out` - structured error code (`null` if no error).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_get_server_error_structured_code(
    context: TsurugiFfiContextHandle,
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
    structured_code_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str =
        "tsurugi_ffi_transaction_error_info_get_server_error_structured_code()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_error_info={:?}, structured_code_out={:?}",
        context,
        transaction_error_info,
        structured_code_out
    );

    ffi_arg_out_initialize!(structured_code_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_error_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, structured_code_out);

    let status = unsafe { &mut *transaction_error_info };

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

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (structured_code={:?})",
        rc,
        ptr
    );
    rc
}

/// TransactionErrorInfo: Dispose.
///
/// # Receiver
/// - `transaction_error_info` - Transaction error information.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_error_info_dispose(
    transaction_error_info: TsurugiFfiTransactionErrorInfoHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_error_info_dispose()";
    trace!(
        "{FUNCTION_NAME} start. transaction_error_info={:?}",
        transaction_error_info
    );

    if transaction_error_info.is_null() {
        trace!("{FUNCTION_NAME} end. arg[transaction_error_info] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(transaction_error_info);
    }

    trace!("{FUNCTION_NAME} end");
}
