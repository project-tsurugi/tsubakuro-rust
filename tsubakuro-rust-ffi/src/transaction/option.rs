use std::{ffi::CString, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_clear, cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_array_field_clear, cstring_array_field_set_if_none, cstring_array_field_to_ptr,
    cstring_to_cchar, ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
    util::cchar::TsurugiFfiCStringArray,
    TsurugiFfiDuration, TsurugiFfiStringArrayHandle, TsurugiFfiStringHandle,
};

use super::r#type::{TsurugiFfiTransactionPriority, TsurugiFfiTransactionType};

#[derive(Debug)]
pub(crate) struct TsurugiFfiTransactionOption {
    transaction_option: TransactionOption,
    transaction_label: Option<CString>,
    write_preserve: Option<TsurugiFfiCStringArray>,
    inclusive_read_area: Option<TsurugiFfiCStringArray>,
    exclusive_read_area: Option<TsurugiFfiCStringArray>,
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

/// Transaction option.
pub type TsurugiFfiTransactionOptionHandle = *mut TsurugiFfiTransactionOption;

/// TransactionOption: Creates a new instance.
///
/// See [`TransactionOption::new`].
///
/// # Returns
/// - `transaction_option_out` - transaction option. To dispose, call `tsurugi_ffi_transaction_option_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_create(
    context: TsurugiFfiContextHandle,
    transaction_option_out: *mut TsurugiFfiTransactionOptionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_create()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option_out={:?}",
        context,
        transaction_option_out
    );

    ffi_arg_out_initialize!(transaction_option_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option_out);

    let transaction_option = Box::new(TsurugiFfiTransactionOption {
        transaction_option: TransactionOption::new(),
        transaction_label: None,
        write_preserve: None,
        inclusive_read_area: None,
        exclusive_read_area: None,
    });

    let handle = Box::into_raw(transaction_option);
    unsafe {
        *transaction_option_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. transaction_option={:?}",
        rc,
        handle
    );
    rc
}

/// TransactionOption: Set transaction type.
///
/// See [`TransactionOption::set_transaction_type`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Parameters
/// - `transaction_type` - transaction type.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_transaction_type(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_type: TsurugiFfiTransactionType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_transaction_type()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, transaction_type={:?}",
        context,
        transaction_option,
        transaction_type as i32
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);

    let transaction_option = unsafe { &mut *transaction_option };

    transaction_option.set_transaction_type(transaction_type.into());

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// TransactionOption: Get transaction type.
///
/// See [`TransactionOption::transaction_type`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Returns
/// - `transaction_type_out` - transaction type.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_transaction_type(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_type_out: *mut TsurugiFfiTransactionType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_transaction_type()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, transaction_type_out={:?}",
        context,
        transaction_option,
        transaction_type_out
    );

    ffi_arg_out_initialize!(transaction_type_out, TsurugiFfiTransactionType::Unspecified);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_type_out);

    let transaction_option = unsafe { &mut *transaction_option };

    let transaction_type = transaction_option.transaction_type();

    let value = transaction_type.into();
    unsafe {
        *transaction_type_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (transaction_type={:?})",
        rc,
        value as i32
    );
    rc
}

/// TransactionOption: Set transaction label.
///
/// See [`TransactionOption::set_transaction_label`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Parameters
/// - `transaction_label` - transaction label.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_transaction_label(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_label: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_transaction_label()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, transaction_label={:?}",
        context,
        transaction_option,
        transaction_label
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_label);

    let transaction_option = unsafe { &mut *transaction_option };
    let label = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, transaction_label);

    transaction_option.set_transaction_label(label);

    cchar_field_clear!(transaction_option.transaction_label);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// TransactionOption: Get transaction label.
///
/// See [`TransactionOption::transaction_label`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Returns
/// - `transaction_label_out` - transaction label.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_transaction_label(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_label_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_transaction_label()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, transaction_label_out={:?}",
        context,
        transaction_option,
        transaction_label_out
    );

    ffi_arg_out_initialize!(transaction_label_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_label_out);

    let transaction_option = unsafe { &mut *transaction_option };

    if transaction_option.transaction_label.is_none() {
        if let Some(value) = transaction_option.transaction_label() {
            let label = value.clone();
            cchar_field_set!(context, transaction_option.transaction_label, label);
        }
    }

    let ptr = cstring_to_cchar!(transaction_option.transaction_label);
    unsafe {
        *transaction_label_out = ptr;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (transaction_label={:?})",
        rc,
        ptr
    );
    rc
}

/// TransactionOption: Set modifies definitions.
///
/// See [`TransactionOption::set_modifies_definitions`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Parameters
/// - `modifies_definitions` - modifies definitions.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_modifies_definitions(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    modifies_definitions: bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_modifies_definitions()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, modifies_definitions={:?}",
        context,
        transaction_option,
        modifies_definitions
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);

    let transaction_option = unsafe { &mut *transaction_option };

    transaction_option.set_modifies_definitions(modifies_definitions);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// TransactionOption: Get modifies definitions.
///
/// See [`TransactionOption::modifies_definitions`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Returns
/// - `modifies_definitions_out` - modifies definitions.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_modifies_definitions(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    modifies_definitions_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_modifies_definitions()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, modifies_definitions_out={:?}",
        context,
        transaction_option,
        modifies_definitions_out
    );

    ffi_arg_out_initialize!(modifies_definitions_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, modifies_definitions_out);

    let transaction_option = unsafe { &mut *transaction_option };

    let value = transaction_option.modifies_definitions();

    unsafe {
        *modifies_definitions_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (modifies_definitions={:?})",
        rc,
        value
    );
    rc
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

/// TransactionOption: Set write preserve.
///
/// See [`TransactionOption::set_write_preserve`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Parameters
/// - `table_names` - table names (String array).
/// - `table_names_size` - `table_names` size \[number of tables\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_write_preserve(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names: *const TsurugiFfiStringHandle,
    table_names_size: u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_write_preserve()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, table_names={:?}, table_names_size={:?}",
        context,
        transaction_option,
        table_names,
        table_names_size
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names: Vec<String> =
        convert_table_names!(context, FUNCTION_NAME, 2, table_names, table_names_size);

    transaction_option.set_write_preserve(&table_names);

    cstring_array_field_clear!(transaction_option.write_preserve);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// TransactionOption: Get write preserve.
///
/// See [`TransactionOption::write_preserve`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Returns
/// - `table_names_out` - table names (String array).
/// - `table_names_size_out` - `table_names_out` size \[number of tables\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_write_preserve(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names_out: *mut TsurugiFfiStringArrayHandle,
    table_names_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_write_preserve_size()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, table_names_out={:?}, table_names_size_out={:?}",
        context,
        transaction_option,
        table_names_out,
        table_names_size_out
    );

    ffi_arg_out_initialize!(table_names_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(table_names_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_names_size_out);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names = transaction_option.write_preserve();

    let size = table_names.len();

    cstring_array_field_set_if_none!(context, transaction_option.write_preserve, table_names);

    let ptr = cstring_array_field_to_ptr!(transaction_option.write_preserve);
    unsafe {
        *table_names_out = ptr;
        *table_names_size_out = size as u32;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (table_names={:?}, table_names_size={:?})",
        rc,
        ptr,
        size as u32
    );
    rc
}

/// TransactionOption: Set inclusive read area.
///
/// See [`TransactionOption::set_inclusive_read_area`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Parameters
/// - `table_names` - table names (String array).
/// - `table_names_size` - `table_names` size \[number of tables\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_inclusive_read_area(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names: *const TsurugiFfiStringHandle,
    table_names_size: u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_inclusive_read_area()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, table_names={:?}, table_names_size={:?}",
        context,
        transaction_option,
        table_names,
        table_names_size
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names: Vec<String> =
        convert_table_names!(context, FUNCTION_NAME, 2, table_names, table_names_size);

    transaction_option.set_inclusive_read_area(&table_names);

    cstring_array_field_clear!(transaction_option.inclusive_read_area);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// TransactionOption: Get inclusive read area.
///
/// See [`TransactionOption::inclusive_read_area`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Returns
/// - `table_names_out` - table names (String array).
/// - `table_names_size_out` - `table_names_out` size \[number of tables\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_inclusive_read_area(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names_out: *mut TsurugiFfiStringArrayHandle,
    table_names_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_inclusive_read_area()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, table_names_out={:?}, table_names_size_out={:?}",
        context,
        transaction_option,
        table_names_out,
        table_names_size_out
    );

    ffi_arg_out_initialize!(table_names_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(table_names_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_names_size_out);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names = transaction_option.inclusive_read_area();

    let size = table_names.len();

    cstring_array_field_set_if_none!(context, transaction_option.inclusive_read_area, table_names);

    let ptr = cstring_array_field_to_ptr!(transaction_option.inclusive_read_area);
    unsafe {
        *table_names_out = ptr;
        *table_names_size_out = size as u32;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (table_names={:?}, table_names_size={:?})",
        rc,
        ptr,
        size as u32
    );
    rc
}

/// TransactionOption: Set exclusive read area.
///
/// See [`TransactionOption::set_exclusive_read_area`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Parameters
/// - `table_names` - table names (String array).
/// - `table_names_size` - `table_names` size \[number of tables\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_exclusive_read_area(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names: *const TsurugiFfiStringHandle,
    table_names_size: u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_exclusive_read_area()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, table_names={:?}, table_names_size={:?}",
        context,
        transaction_option,
        table_names,
        table_names_size
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names: Vec<String> =
        convert_table_names!(context, FUNCTION_NAME, 2, table_names, table_names_size);

    transaction_option.set_exclusive_read_area(&table_names);

    cstring_array_field_clear!(transaction_option.exclusive_read_area);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// TransactionOption: Get exclusive read area.
///
/// See [`TransactionOption::exclusive_read_area`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Returns
/// - `table_names_out` - table names (String array).
/// - `table_names_size_out` - `table_names_out` size \[number of tables\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_exclusive_read_area(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    table_names_out: *mut TsurugiFfiStringArrayHandle,
    table_names_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_exclusive_read_area()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, table_names_out={:?}, table_names_size_out={:?}",
        context,
        transaction_option,
        table_names_out,
        table_names_size_out
    );

    ffi_arg_out_initialize!(table_names_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(table_names_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_names_size_out);

    let transaction_option = unsafe { &mut *transaction_option };
    let table_names = transaction_option.exclusive_read_area();

    let size = table_names.len();

    cstring_array_field_set_if_none!(context, transaction_option.exclusive_read_area, table_names);

    let ptr = cstring_array_field_to_ptr!(transaction_option.exclusive_read_area);
    unsafe {
        *table_names_out = ptr;
        *table_names_size_out = size as u32;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (table_names={:?}, table_names_size={:?})",
        rc,
        ptr,
        size as u32
    );
    rc
}

/// TransactionOption: Set priority.
///
/// See [`TransactionOption::set_priority`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Parameters
/// - `priority` - priority.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_priority(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    priority: TsurugiFfiTransactionPriority,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_priority()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, priority={:?}",
        context,
        transaction_option,
        priority as i32
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);

    let transaction_option = unsafe { &mut *transaction_option };

    transaction_option.set_priority(priority.into());

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// TransactionOption: Get priority.
///
/// See [`TransactionOption::priority`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Returns
/// - `priority_out` - priority.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_priority(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    priority_out: *mut TsurugiFfiTransactionPriority,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_priority()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, priority_out={:?}",
        context,
        transaction_option,
        priority_out
    );

    ffi_arg_out_initialize!(priority_out, TsurugiFfiTransactionPriority::Unspecified);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, priority_out);

    let transaction_option = unsafe { &mut *transaction_option };

    let priority = transaction_option.priority();

    let value = priority.into();
    unsafe {
        *priority_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (priority={:?})",
        rc,
        value as i32
    );
    rc
}

/// TransactionOption: Set close timeout.
///
/// See [`TransactionOption::set_close_timeout`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Parameters
/// - `close_timeout` - close timeout \[nanoseconds\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_set_close_timeout(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    close_timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_set_close_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, close_timeout={:?}",
        context,
        transaction_option,
        close_timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);

    let transaction_option = unsafe { &mut *transaction_option };
    let close_timeout = Duration::from_nanos(close_timeout);

    transaction_option.set_close_timeout(close_timeout);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// TransactionOption: Get close timeout.
///
/// See [`TransactionOption::close_timeout`].
///
/// # Receiver
/// - `transaction_option` - Transaction option.
///
/// # Returns
/// - `close_timeout_out` - close timeout \[nanoseconds\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_option_get_close_timeout(
    context: TsurugiFfiContextHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    close_timeout_exists_out: *mut bool,
    close_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_option_get_close_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_option={:?}, close_timeout_exists_out={:?}, close_timeout_out={:?}",
        context,
        transaction_option,
        close_timeout_exists_out,
        close_timeout_out
    );

    ffi_arg_out_initialize!(close_timeout_exists_out, false);
    ffi_arg_out_initialize!(close_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, close_timeout_exists_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, close_timeout_out);

    let transaction_option = unsafe { &mut *transaction_option };

    let (exists, close_timeout) = match transaction_option.close_timeout() {
        Some(value) => (true, value),
        None => (false, Duration::ZERO),
    };

    let value = close_timeout.as_nanos() as TsurugiFfiDuration;
    unsafe {
        *close_timeout_exists_out = exists;
        *close_timeout_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (close_timeout_exists={:?}, close_timeout={:?})",
        rc,
        exists,
        value
    );
    rc
}

/// TransactionOption: Dispose.
///
/// # Receiver
/// - `transaction_option` - Transaction option.
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
