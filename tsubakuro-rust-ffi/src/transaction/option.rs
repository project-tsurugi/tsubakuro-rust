use std::ffi::c_char;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_clear, cchar_field_dispose, cchar_field_set,
    context::TsurugiFfiContextHandle,
    ffi_arg_cchar_to_str, ffi_arg_require_non_null, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
#[allow(dead_code)]
pub enum TsurugiFfiTransactionType {
    /// use default transaction type.
    Unspecified = 0,
    /// short transactions (optimistic concurrency control).
    Short = 1,
    /// long transactions (pessimistic concurrency control).
    Long = 2,
    /// read only transactions (may be abort-free).
    ReadOnly = 3,
}

impl TsurugiFfiTransactionType {
    fn is_valid(value: i32) -> bool {
        matches!(value, 0 | 1 | 2 | 3)
    }
}

impl From<TransactionType> for TsurugiFfiTransactionType {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Unspecified => TsurugiFfiTransactionType::Unspecified,
            TransactionType::Short => TsurugiFfiTransactionType::Short,
            TransactionType::Long => TsurugiFfiTransactionType::Long,
            TransactionType::ReadOnly => TsurugiFfiTransactionType::ReadOnly,
        }
    }
}

impl From<TsurugiFfiTransactionType> for TransactionType {
    fn from(value: TsurugiFfiTransactionType) -> Self {
        match value {
            TsurugiFfiTransactionType::Unspecified => Self::Unspecified,
            TsurugiFfiTransactionType::Short => Self::Short,
            TsurugiFfiTransactionType::Long => Self::Long,
            TsurugiFfiTransactionType::ReadOnly => Self::ReadOnly,
        }
    }
}

#[derive(Debug)]
pub(crate) struct TsurugiFfiTransactionOption {
    transaction_option: TransactionOption,
    transaction_label: *mut c_char,
}

impl std::ops::Deref for TsurugiFfiTransactionOption {
    type Target = TransactionOption;

    fn deref(&self) -> &Self::Target {
        &self.transaction_option
    }
}

impl std::ops::DerefMut for TsurugiFfiTransactionOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction_option
    }
}

pub type TsurugiFfiTransactionOptionHandle = *mut TsurugiFfiTransactionOption;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_create(
    context: TsurugiFfiContextHandle,
    transaction_option_out: *mut TsurugiFfiTransactionOptionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_create()";
    trace!("{FUNCTION_NAME} start");

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option_out);
    unsafe {
        *transaction_option_out = std::ptr::null_mut();
    }

    let transaction_option = Box::new(TsurugiFfiTransactionOption {
        transaction_option: TransactionOption::new(),
        transaction_label: std::ptr::null_mut(),
    });

    let handle = Box::into_raw(transaction_option);
    unsafe {
        *transaction_option_out = handle;
    }

    trace!("{FUNCTION_NAME} end. transaction_option={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_transaction_type(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_type: TsurugiFfiTransactionType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_transaction_type()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    if !TsurugiFfiTransactionType::is_valid(transaction_type as i32) {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "transaction_type", "is invalid");
    }

    let transaction_option = unsafe { &mut *transaction_option };

    transaction_option.set_transaction_type(transaction_type.into());

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_transaction_type(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_type_out: *mut TsurugiFfiTransactionType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_transaction_type()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_type_out);
    unsafe {
        *transaction_type_out = TsurugiFfiTransactionType::Unspecified;
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);

    let transaction_option = unsafe { &mut *transaction_option };

    let transaction_type = transaction_option.transaction_type();
    unsafe {
        *transaction_type_out = transaction_type.into();
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_transaction_label(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    label: *const c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_transaction_label()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, label);

    let label = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, label);

    let transaction_option = unsafe { &mut *transaction_option };
    transaction_option.set_transaction_label(label);

    unsafe {
        cchar_field_clear!(transaction_option.transaction_label);
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_transaction_label(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    label_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_transaction_label()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, label_out);
    unsafe {
        *label_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);

    let transaction_option = unsafe { &mut *transaction_option };

    if transaction_option.transaction_label.is_null() {
        match transaction_option.transaction_label() {
            Some(value) => unsafe {
                let label = value.clone();
                cchar_field_set!(context, transaction_option.transaction_label, label);
            },
            None => {}
        }
    }
    unsafe {
        *label_out = transaction_option.transaction_label;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

// TODO tsurugi_ffi_transaction_option_set_modifies_definitions(), etc

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_dispose(
    transaction_option: TsurugiFfiTransactionOptionHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_dispose()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    if transaction_option.is_null() {
        trace!("{FUNCTION_NAME} end. arg[transaction_option] is null");
        return;
    }

    unsafe {
        let transaction_option = Box::from_raw(transaction_option);

        cchar_field_dispose!(transaction_option.transaction_label);
    }

    trace!("{FUNCTION_NAME} end");
}
