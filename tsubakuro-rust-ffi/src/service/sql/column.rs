use std::ffi::c_char;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_dispose, cchar_field_set,
    context::TsurugiFfiContextHandle,
    rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
};

pub(crate) struct TsurugiFfiSqlColumn {
    sql_column: SqlColumn,
    name: *mut c_char,
}

impl TsurugiFfiSqlColumn {
    pub(crate) fn new(sql_column: SqlColumn) -> TsurugiFfiSqlColumn {
        TsurugiFfiSqlColumn {
            sql_column,
            name: std::ptr::null_mut(),
        }
    }
}

impl std::ops::Deref for TsurugiFfiSqlColumn {
    type Target = SqlColumn;

    fn deref(&self) -> &Self::Target {
        &self.sql_column
    }
}

impl std::ops::DerefMut for TsurugiFfiSqlColumn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sql_column
    }
}

pub type TsurugiFfiSqlColumnHandle = *mut TsurugiFfiSqlColumn;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_name(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    name_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_name()";
    trace!("{FUNCTION_NAME} start. sql_column={:?}", sql_column);

    if sql_column.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "sql_column", "is null");
    }
    if name_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "name_out", "is null");
    }

    let sql_column = unsafe { &mut *sql_column };

    if sql_column.name.is_null() {
        let table_name = sql_column.name().clone();
        unsafe {
            cchar_field_set!(context, sql_column.name, table_name);
        }
    }

    unsafe {
        *name_out = sql_column.name;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_dispose(sql_column: TsurugiFfiSqlColumnHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_dispose()";
    trace!("{FUNCTION_NAME} start. sql_column={:?}", sql_column);

    if sql_column.is_null() {
        trace!("{FUNCTION_NAME} end. arg[sql_column] is null");
        return;
    }

    unsafe {
        let sql_column = Box::from_raw(sql_column);

        cchar_field_dispose!(sql_column.name);
    }

    trace!("{FUNCTION_NAME} end");
}
