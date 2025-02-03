use std::{ffi::CString, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_clear, cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null,
    rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    vec_cchar_field_clear, vec_cchar_field_set_if_none, vec_cchar_field_to_ptr, TsurugiFfiDuration,
    TsurugiFfiStringArrayHandle, TsurugiFfiStringHandle,
};

use super::r#type::{TsurugiFfiTransactionPriority, TsurugiFfiTransactionType};

#[derive(Debug)]
pub(crate) struct TsurugiFfiTransactionOption {
    transaction_option: TransactionOption,
    transaction_label: Option<CString>,
    write_preserve: Option<Vec<CString>>,
    write_preserve_ptr: Option<Vec<TsurugiFfiStringHandle>>,
    inclusive_read_area: Option<Vec<CString>>,
    inclusive_read_area_ptr: Option<Vec<TsurugiFfiStringHandle>>,
    exclusive_read_area: Option<Vec<CString>>,
    exclusive_read_area_ptr: Option<Vec<TsurugiFfiStringHandle>>,
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

    ffi_arg_out_initialize!(transaction_option_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option_out);

    let transaction_option = Box::new(TsurugiFfiTransactionOption {
        transaction_option: TransactionOption::new(),
        transaction_label: None,
        write_preserve: None,
        write_preserve_ptr: None,
        inclusive_read_area: None,
        inclusive_read_area_ptr: None,
        exclusive_read_area: None,
        exclusive_read_area_ptr: None,
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

    ffi_arg_out_initialize!(transaction_type_out, TsurugiFfiTransactionType::Unspecified);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_type_out);

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
    label: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_transaction_label()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, label);

    let transaction_option = unsafe { &mut *transaction_option };
    let label = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, label);

    transaction_option.set_transaction_label(label);

    cchar_field_clear!(transaction_option.transaction_label);

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_transaction_label(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    label_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_transaction_label()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_out_initialize!(label_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, label_out);

    let transaction_option = unsafe { &mut *transaction_option };

    if transaction_option.transaction_label.is_none() {
        match transaction_option.transaction_label() {
            Some(value) => {
                let label = value.clone();
                cchar_field_set!(context, transaction_option.transaction_label, label);
            }
            None => {}
        }
    }
    unsafe {
        *label_out = cstring_to_cchar!(transaction_option.transaction_label);
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_modifies_definitions(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    modifies_definitions: bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_modifies_definitions()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);

    let transaction_option = unsafe { &mut *transaction_option };

    transaction_option.set_modifies_definitions(modifies_definitions);

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_modifies_definitions(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    modifies_definitions_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_modifies_definitions()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_out_initialize!(modifies_definitions_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, modifies_definitions_out);

    let transaction_option = unsafe { &mut *transaction_option };

    let modifies_definitions = transaction_option.modifies_definitions();
    unsafe {
        *modifies_definitions_out = modifies_definitions;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

macro_rules! convert_table_names {
    ($context:expr, $function_name:expr, $arg_index:expr, $table_names:expr, $table_names_size:expr) => {
        if $table_names_size > 0 {
            let src =
                unsafe { std::slice::from_raw_parts($table_names, $table_names_size as usize) };
            let mut dst = Vec::with_capacity(src.len());
            for &talbe_name in src {
                ffi_arg_require_non_null!($context, $function_name, $arg_index, talbe_name);
                let talbe_name =
                    ffi_arg_cchar_to_str!($context, $function_name, $arg_index, talbe_name)
                        .to_string();
                dst.push(talbe_name);
            }
            dst
        } else {
            vec![]
        }
    };
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_write_preserve(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names: *const TsurugiFfiStringHandle,
    table_names_size: u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_write_preserve()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names: Vec<String> =
        convert_table_names!(context, FUNCTION_NAME, 2, table_names, table_names_size);

    transaction_option.set_write_preserve(&table_names);

    vec_cchar_field_clear!(
        transaction_option.write_preserve,
        transaction_option.write_preserve_ptr
    );

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_write_preserve(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names_out: *mut TsurugiFfiStringArrayHandle,
    table_names_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_write_preserve_size()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_out_initialize!(table_names_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(table_names_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_names_size_out);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names = transaction_option.write_preserve();

    let size = table_names.len();

    // TODO mutex.lock transaction_option.write_preserve
    vec_cchar_field_set_if_none!(
        context,
        transaction_option.write_preserve,
        transaction_option.write_preserve_ptr,
        table_names
    );

    unsafe {
        *table_names_out = vec_cchar_field_to_ptr!(transaction_option.write_preserve_ptr);
        *table_names_size_out = size as u32;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_inclusive_read_area(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names: *const TsurugiFfiStringHandle,
    table_names_size: u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_inclusive_read_area()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names: Vec<String> =
        convert_table_names!(context, FUNCTION_NAME, 2, table_names, table_names_size);

    transaction_option.set_inclusive_read_area(&table_names);

    vec_cchar_field_clear!(
        transaction_option.inclusive_read_area,
        transaction_option.inclusive_read_area_ptr
    );

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_inclusive_read_area(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names_out: *mut TsurugiFfiStringArrayHandle,
    table_names_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_inclusive_read_area()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_out_initialize!(table_names_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(table_names_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_names_size_out);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names = transaction_option.inclusive_read_area();

    let size = table_names.len();

    // TODO mutex.lock transaction_option.inclusive_read_area
    vec_cchar_field_set_if_none!(
        context,
        transaction_option.inclusive_read_area,
        transaction_option.inclusive_read_area_ptr,
        table_names
    );

    unsafe {
        *table_names_out = vec_cchar_field_to_ptr!(transaction_option.inclusive_read_area_ptr);
        *table_names_size_out = size as u32;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_exclusive_read_area(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names: *const TsurugiFfiStringHandle,
    table_names_size: u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_exclusive_read_area()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names: Vec<String> =
        convert_table_names!(context, FUNCTION_NAME, 2, table_names, table_names_size);

    transaction_option.set_exclusive_read_area(&table_names);

    vec_cchar_field_clear!(
        transaction_option.exclusive_read_area,
        transaction_option.exclusive_read_area_ptr
    );

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_exclusive_read_area(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names_out: *mut TsurugiFfiStringArrayHandle,
    table_names_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_exclusive_read_area()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_out_initialize!(table_names_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(table_names_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_names_size_out);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names = transaction_option.exclusive_read_area();

    let size = table_names.len();

    // TODO mutex.lock transaction_option.exclusive_read_area
    vec_cchar_field_set_if_none!(
        context,
        transaction_option.exclusive_read_area,
        transaction_option.exclusive_read_area_ptr,
        table_names
    );

    unsafe {
        *table_names_out = vec_cchar_field_to_ptr!(transaction_option.exclusive_read_area_ptr);
        *table_names_size_out = size as u32;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_priority(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    priority: TsurugiFfiTransactionPriority,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_priority()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    if !TsurugiFfiTransactionPriority::is_valid(priority as i32) {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "priority", "is invalid");
    }

    let transaction_option = unsafe { &mut *transaction_option };

    transaction_option.set_priority(priority.into());

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_priority(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    priority_out: *mut TsurugiFfiTransactionPriority,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_priority()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_out_initialize!(priority_out, TsurugiFfiTransactionPriority::Unspecified);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, priority_out);

    let transaction_option = unsafe { &mut *transaction_option };

    let priority = transaction_option.priority();
    unsafe {
        *priority_out = priority.into();
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_close_timeout(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_close_timeout()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);

    let transaction_option = unsafe { &mut *transaction_option };
    let timeout = Duration::from_nanos(timeout);

    transaction_option.set_close_timeout(timeout);

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_close_timeout(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    close_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_close_timeout()";
    trace!(
        "{FUNCTION_NAME} start. transaction_option={:?}",
        transaction_option
    );

    ffi_arg_out_initialize!(close_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, close_timeout_out);

    let transaction_option = unsafe { &mut *transaction_option };

    let timeout = match transaction_option.close_timeout() {
        Some(value) => value.as_nanos() as TsurugiFfiDuration,
        None => 0, // FIXME close_timeout None
    };

    unsafe {
        *close_timeout_out = timeout;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

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
        let _ = Box::from_raw(transaction_option);
    }

    trace!("{FUNCTION_NAME} end");
}
