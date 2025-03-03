//! table list.

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    cstring_array_field_set_if_none, cstring_array_field_to_ptr, ffi_arg_out_initialize,
    ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
    util::cchar::TsurugiFfiCStringArray,
    TsurugiFfiStringArrayHandle,
};

pub(crate) struct TsurugiFfiTableList {
    table_list: TableList,
    table_names: Option<TsurugiFfiCStringArray>,
}

impl TsurugiFfiTableList {
    pub(crate) fn new(table_list: TableList) -> TsurugiFfiTableList {
        TsurugiFfiTableList {
            table_list,
            table_names: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiTableList {
    type Target = TableList;

    fn deref(&self) -> &Self::Target {
        &self.table_list
    }
}

impl std::ops::DerefMut for TsurugiFfiTableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table_list
    }
}

/// Table list.
pub type TsurugiFfiTableListHandle = *mut TsurugiFfiTableList;

/// TableList: Get table names.
///
/// See [`TableList::table_names`].
///
/// # Receiver
/// - `table_list` - Table list.
///
/// # Returns
/// - `table_names_out` - table names (string array).
/// - `table_names_size_out` - `table_names_out` size (number of tables).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_list_get_table_names(
    context: TsurugiFfiContextHandle,
    table_list: TsurugiFfiTableListHandle,
    table_names_out: *mut TsurugiFfiStringArrayHandle,
    table_names_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_list_get_table_names()";
    trace!("{FUNCTION_NAME} start. context={:?}, table_list={:?}, table_names_out={:?}, table_names_size_out={:?}",
        context,
        table_list,
        table_names_out,
        table_names_size_out
    );

    ffi_arg_out_initialize!(table_names_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(table_names_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_list);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_names_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_names_size_out);

    let table_list = unsafe { &mut *table_list };
    let table_names = table_list.table_names();

    let size = table_names.len();

    cstring_array_field_set_if_none!(context, table_list.table_names, table_names);

    let ptr = cstring_array_field_to_ptr!(table_list.table_names);
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

/// TableList: Dispose.
///
/// # Receiver
/// - `table_list` - Table list.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_list_dispose(table_list: TsurugiFfiTableListHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_list_dispose()";
    trace!("{FUNCTION_NAME} start. table_list={:?}", table_list);

    if table_list.is_null() {
        trace!("{FUNCTION_NAME} end. arg[table_list] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(table_list);
    }

    trace!("{FUNCTION_NAME} end");
}
