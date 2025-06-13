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
    description: Option<CString>,
    sql_type_name: Option<CString>,
    sql_type: Option<CString>,
}

impl TsurugiFfiSqlColumn {
    pub(crate) fn new(sql_column: SqlColumn) -> TsurugiFfiSqlColumn {
        TsurugiFfiSqlColumn {
            sql_column,
            name: None,
            description: None,
            sql_type_name: None,
            sql_type: None,
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
        let column_name = sql_column.name().clone();
        cchar_field_set!(context, sql_column.name, column_name);
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

/// SqlColumn: Get length for data types.
///
/// See [`SqlColumn::length`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `provided_out` - Whether length or arbitrary_length is provided.
/// - `length_out` - defined length. Valid when `arbitrary_length` is `false`.
/// - `arbitrary_length_out` - arbitrary length (*).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_length(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    provided_out: *mut bool,
    length_out: *mut u32,
    arbitrary_length_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_length()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, provided_out={:?}, length_out={:?}, arbitrary_length_out={:?}",
        context,
        sql_column,
        provided_out,
        length_out,
        arbitrary_length_out,
    );

    ffi_arg_out_initialize!(provided_out, false);
    ffi_arg_out_initialize!(length_out, 0);
    ffi_arg_out_initialize!(arbitrary_length_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, provided_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, length_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, arbitrary_length_out);

    let sql_column = unsafe { &mut *sql_column };

    let (provided, length, arbitrary_length) = match sql_column.length() {
        Some((length, false)) => (true, length, false),
        Some((_, true)) => (true, 0, true),
        None => (false, 0, false),
    };

    unsafe {
        *provided_out = provided;
        *length_out = length;
        *arbitrary_length_out = arbitrary_length;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (provided={:?}, length={:?}, arbitrary_length={:?})",
        rc,
        provided,
        length,
        arbitrary_length,
    );
    rc
}

/// SqlColumn: Get precision for decimal types.
///
/// See [`SqlColumn::precision`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `provided_out` - Whether precision or arbitrary_precision is provided.
/// - `precision_out` - defined precision. Valid when `arbitrary_precision` is `false`.
/// - `arbitrary_precision_out` - arbitrary precision (*).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_precision(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    provided_out: *mut bool,
    precision_out: *mut u32,
    arbitrary_precision_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_precision()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, provided_out={:?}, precision_out={:?}, arbitrary_precision_out={:?}",
        context,
        sql_column,
        provided_out,
        precision_out,
        arbitrary_precision_out,
    );

    ffi_arg_out_initialize!(provided_out, false);
    ffi_arg_out_initialize!(precision_out, 0);
    ffi_arg_out_initialize!(arbitrary_precision_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, provided_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, precision_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, arbitrary_precision_out);

    let sql_column = unsafe { &mut *sql_column };

    let (provided, precision, arbitrary_precision) = match sql_column.precision() {
        Some((precision, false)) => (true, precision, false),
        Some((_, true)) => (true, 0, true),
        None => (false, 0, false),
    };

    unsafe {
        *provided_out = provided;
        *precision_out = precision;
        *arbitrary_precision_out = arbitrary_precision;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (provided={:?}, precision={:?}, arbitrary_precision={:?})",
        rc,
        provided,
        precision,
        arbitrary_precision,
    );
    rc
}

/// SqlColumn: Get scale for decimal types.
///
/// See [`SqlColumn::scale`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `provided_out` - Whether scale or arbitrary_scale is provided.
/// - `scale_out` - defined scale. Valid when `arbitrary_scale` is `false`.
/// - `arbitrary_scale_out` - arbitrary scale (*).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_scale(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    provided_out: *mut bool,
    scale_out: *mut u32,
    arbitrary_scale_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_scale()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, provided_out={:?}, scale_out={:?}, arbitrary_scale_out={:?}",
        context,
        sql_column,
        provided_out,
        scale_out,
        arbitrary_scale_out,
    );

    ffi_arg_out_initialize!(provided_out, false);
    ffi_arg_out_initialize!(scale_out, 0);
    ffi_arg_out_initialize!(arbitrary_scale_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, provided_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, scale_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, arbitrary_scale_out);

    let sql_column = unsafe { &mut *sql_column };

    let (provided, scale, arbitrary_scale) = match sql_column.scale() {
        Some((scale, false)) => (true, scale, false),
        Some((_, true)) => (true, 0, true),
        None => (false, 0, false),
    };

    unsafe {
        *provided_out = provided;
        *scale_out = scale;
        *arbitrary_scale_out = arbitrary_scale;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (provided={:?}, scale={:?}, arbitrary_scale={:?})",
        rc,
        provided,
        scale,
        arbitrary_scale,
    );
    rc
}

/// SqlColumn: Whether the column type is nullable.
///
/// See [`SqlColumn::nullable`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `provided_out` - Whether nullable is provided.
/// - `nullable_out` - Whether the column is nullable.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_nullable(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    provided_out: *mut bool,
    nullable_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_nullable()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, provided_out={:?}, nullable_out={:?}",
        context,
        sql_column,
        provided_out,
        nullable_out,
    );

    ffi_arg_out_initialize!(provided_out, false);
    ffi_arg_out_initialize!(nullable_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, provided_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, nullable_out);

    let sql_column = unsafe { &mut *sql_column };

    let (provided, nullable) = match sql_column.nullable() {
        Some(nullable) => (true, nullable),
        None => (false, false),
    };

    unsafe {
        *provided_out = provided;
        *nullable_out = nullable;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (provided={:?}, nullable={:?}",
        rc,
        provided,
        nullable,
    );
    rc
}

/// SqlColumn: Whether the column type is varying.
///
/// See [`SqlColumn::varying`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `provided_out` - Whether varying is provided.
/// - `varying_out` - Whether the column is varying.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_varying(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    provided_out: *mut bool,
    varying_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_varying()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, provided_out={:?}, varying_out={:?}",
        context,
        sql_column,
        provided_out,
        varying_out,
    );

    ffi_arg_out_initialize!(provided_out, false);
    ffi_arg_out_initialize!(varying_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, provided_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, varying_out);

    let sql_column = unsafe { &mut *sql_column };

    let (provided, varying) = match sql_column.varying() {
        Some(varying) => (true, varying),
        None => (false, false),
    };

    unsafe {
        *provided_out = provided;
        *varying_out = varying;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (provided={:?}, varying={:?}",
        rc,
        provided,
        varying,
    );
    rc
}

/// SqlColumn: Get description.
///
/// See [`SqlColumn::description`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `description_out` - column description (nullable).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_description(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    description_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_description()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, description_out={:?}",
        context,
        sql_column,
        description_out
    );

    ffi_arg_out_initialize!(description_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, description_out);

    let sql_column = unsafe { &mut *sql_column };

    if sql_column.description.is_none() {
        if let Some(description) = sql_column.description() {
            cchar_field_set!(context, sql_column.description, description.clone());
        }
    }

    let ptr = cstring_to_cchar!(sql_column.description);
    unsafe {
        *description_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (description={:?})", rc, ptr);
    rc
}

/// SqlColumn: Get SQL type name.
///
/// See [`SqlColumn::sql_type_name`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `sql_type_name_out` - SQL type name (nullable).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_sql_type_name(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    sql_type_name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_sql_type_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, sql_type_name_out={:?}",
        context,
        sql_column,
        sql_type_name_out
    );

    ffi_arg_out_initialize!(sql_type_name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, sql_type_name_out);

    let sql_column = unsafe { &mut *sql_column };

    if sql_column.sql_type_name.is_none() {
        if let Some(sql_type_name) = sql_column.sql_type_name() {
            cchar_field_set!(context, sql_column.sql_type_name, sql_type_name);
        }
    }

    let ptr = cstring_to_cchar!(sql_column.sql_type_name);
    unsafe {
        *sql_type_name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (sql_type_name={:?})", rc, ptr);
    rc
}

/// SqlColumn: Get SQL type.
///
/// See [`SqlColumn::sql_type`].
///
/// # Receiver
/// - `sql_column` - Sql column.
///
/// # Returns
/// - `sql_type_out` - SQL type (nullable).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_column_get_sql_type(
    context: TsurugiFfiContextHandle,
    sql_column: TsurugiFfiSqlColumnHandle,
    sql_type_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_column_get_sql_type()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_column={:?}, sql_type_out={:?}",
        context,
        sql_column,
        sql_type_out
    );

    ffi_arg_out_initialize!(sql_type_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_column);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, sql_type_out);

    let sql_column = unsafe { &mut *sql_column };

    if sql_column.sql_type.is_none() {
        if let Some(sql_type) = sql_column.sql_type() {
            cchar_field_set!(context, sql_column.sql_type, sql_type);
        }
    }

    let ptr = cstring_to_cchar!(sql_column.sql_type);
    unsafe {
        *sql_type_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (sql_type={:?})", rc, ptr);
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
