//! Sql column.

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

use super::atom_type::TsurugiFfiAtomType;

pub(crate) struct TsurugiFfiSqlColumn {
    sql_column: SqlColumn,
    name: Option<CString>,
}

impl TsurugiFfiSqlColumn {
    pub(crate) fn new(sql_column: SqlColumn) -> TsurugiFfiSqlColumn {
        TsurugiFfiSqlColumn {
            sql_column,
            name: None,
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

/// Sql column.
pub type TsurugiFfiSqlColumnHandle = *mut TsurugiFfiSqlColumn;

/// SqlColumn: Get name.
///
/// See [`SqlColumn::name`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `name_out` - column name.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_name(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, name_out={:?}",
        context,
        sql_column,
        name_out
    );

    ffi_arg_out_initialize!(name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, name_out);

    let sql_column = unsafe { &mut *sql_column };

    if sql_column.name.is_none() {
        let table_name = sql_column.name().clone();
        cchar_field_set!(context, sql_column.name, table_name);
    }

    let ptr = cstring_to_cchar!(sql_column.name);
    unsafe {
        *name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (name={:?})", rc, ptr);
    rc
}

/// SqlColumn: Get AtomType.
///
/// See [`SqlColumn::atom_type`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `atom_type_out` - column type.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_atom_type(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    atom_type_out: *mut TsurugiFfiAtomType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_atom_type()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, atom_type_out={:?}",
        context,
        sql_column,
        atom_type_out
    );

    ffi_arg_out_initialize!(atom_type_out, TsurugiFfiAtomType::Unrecognized);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, atom_type_out);

    let sql_column = unsafe { &mut *sql_column };

    let atom_type = match sql_column.atom_type() {
        Some(value) => value.into(),
        None => TsurugiFfiAtomType::Unrecognized,
    };

    unsafe {
        *atom_type_out = atom_type;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (atom_type={:?})",
        rc,
        atom_type as i32
    );
    rc
}

/// SqlColumn: Dispose.
///
/// # Receiver
/// - `sql_column` - Sql column.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_dispose(sql_column: TsurugiFfiSqlColumnHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_dispose()";
    trace!("{FUNCTION_NAME} start. sql_column={:?}", sql_column);

    if sql_column.is_null() {
        trace!("{FUNCTION_NAME} end. arg[sql_column] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(sql_column);
    }

    trace!("{FUNCTION_NAME} end");
}
