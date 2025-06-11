//! table metadata.

use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_array_field_set_if_none, cstring_array_field_to_ptr, cstring_to_cchar,
    ffi_arg_out_initialize, ffi_arg_require_non_null, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::column::TsurugiFfiSqlColumn,
    util::cchar::TsurugiFfiCStringArray,
    TsurugiFfiStringArrayHandle, TsurugiFfiStringHandle,
};

use super::column::TsurugiFfiSqlColumnHandle;

pub(crate) struct TsurugiFfiTableMetadata {
    table_metadata: TableMetadata,
    database_name: Option<CString>,
    schema_name: Option<CString>,
    table_name: Option<CString>,
    description: Option<CString>,
    primary_keys: Option<TsurugiFfiCStringArray>,
}

impl TsurugiFfiTableMetadata {
    pub(crate) fn new(table_metadata: TableMetadata) -> TsurugiFfiTableMetadata {
        TsurugiFfiTableMetadata {
            table_metadata,
            database_name: None,
            schema_name: None,
            table_name: None,
            description: None,
            primary_keys: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiTableMetadata {
    type Target = TableMetadata;

    fn deref(&self) -> &Self::Target {
        &self.table_metadata
    }
}

impl std::ops::DerefMut for TsurugiFfiTableMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table_metadata
    }
}

/// Table metadata.
pub type TsurugiFfiTableMetadataHandle = *mut TsurugiFfiTableMetadata;

/// TableMetadata: Get database name.
///
/// See [`TableMetadata::database_name`].
///
/// # Receiver
/// - `table_metadata` - Table metadata.
///
/// # Returns
/// - `database_name_out` - database name (nullable).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_metadata_get_database_name(
    context: TsurugiFfiContextHandle,
    table_metadata: TsurugiFfiTableMetadataHandle,
    database_name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_metadata_get_database_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, table_metadata={:?}, database_name_out={:?}",
        context,
        table_metadata,
        database_name_out
    );

    ffi_arg_out_initialize!(database_name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, database_name_out);

    let table_metadata = unsafe { &mut *table_metadata };

    if table_metadata.database_name.is_none() {
        let database_name = table_metadata.database_name().clone();
        cchar_field_set!(context, table_metadata.database_name, database_name);
    }

    let ptr = cstring_to_cchar!(table_metadata.database_name);
    unsafe {
        *database_name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (database_name={:?})", rc, ptr);
    rc
}

/// TableMetadata: Get schema name.
///
/// See [`TableMetadata::schema_name`].
///
/// # Receiver
/// - `table_metadata` - Table metadata.
///
/// # Returns
/// - `schema_name_out` - schema name (nullable).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_metadata_get_schema_name(
    context: TsurugiFfiContextHandle,
    table_metadata: TsurugiFfiTableMetadataHandle,
    schema_name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_metadata_get_schema_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, table_metadata={:?}, schema_name_out={:?}",
        context,
        table_metadata,
        schema_name_out
    );

    ffi_arg_out_initialize!(schema_name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, schema_name_out);

    let table_metadata = unsafe { &mut *table_metadata };

    if table_metadata.schema_name.is_none() {
        let schema_name = table_metadata.schema_name().clone();
        cchar_field_set!(context, table_metadata.schema_name, schema_name);
    }

    let ptr = cstring_to_cchar!(table_metadata.schema_name);
    unsafe {
        *schema_name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (schema_name={:?})", rc, ptr);
    rc
}

/// TableMetadata: Get table name.
///
/// See [`TableMetadata::table_name`].
///
/// # Receiver
/// - `table_metadata` - Table metadata.
///
/// # Returns
/// - `table_name_out` - table name.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_metadata_get_table_name(
    context: TsurugiFfiContextHandle,
    table_metadata: TsurugiFfiTableMetadataHandle,
    table_name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_metadata_get_table_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, table_metadata={:?}, table_name_out={:?}",
        context,
        table_metadata,
        table_name_out
    );

    ffi_arg_out_initialize!(table_name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_name_out);

    let table_metadata = unsafe { &mut *table_metadata };

    if table_metadata.table_name.is_none() {
        let table_name = table_metadata.table_name().clone();
        cchar_field_set!(context, table_metadata.table_name, table_name);
    }

    let ptr = cstring_to_cchar!(table_metadata.table_name);
    unsafe {
        *table_name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (table_name={:?})", rc, ptr);
    rc
}

/// TableMetadata: Get description.
///
/// See [`TableMetadata::description`].
///
/// # Receiver
/// - `table_metadata` - Table metadata.
///
/// # Returns
/// - `description_out` - description (nullable).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_metadata_get_description(
    context: TsurugiFfiContextHandle,
    table_metadata: TsurugiFfiTableMetadataHandle,
    description_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_metadata_get_description()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, table_metadata={:?}, description_out={:?}",
        context,
        table_metadata,
        description_out
    );

    ffi_arg_out_initialize!(description_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, description_out);

    let table_metadata = unsafe { &mut *table_metadata };

    if table_metadata.description.is_none() {
        if let Some(description) = table_metadata.description() {
            cchar_field_set!(context, table_metadata.description, description.clone());
        }
    }

    let ptr = cstring_to_cchar!(table_metadata.description);
    unsafe {
        *description_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (description={:?})", rc, ptr);
    rc
}

/// TableMetadata: Get columns size.
///
/// See [`TableMetadata::columns`].
///
/// # Receiver
/// - `table_metadata` - Table metadata.
///
/// # Returns
/// - `size_out` - number of columns \[number of columns\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_metadata_get_columns_size(
    context: TsurugiFfiContextHandle,
    table_metadata: TsurugiFfiTableMetadataHandle,
    size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_metadata_get_columns_size()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, table_metadata={:?}, size_out={:?}",
        context,
        table_metadata,
        size_out
    );

    ffi_arg_out_initialize!(size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, size_out);

    let table_metadata = unsafe { &*table_metadata };
    let columns = table_metadata.columns();

    let value = columns.len() as u32;
    unsafe {
        *size_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (size={:?})", rc, value);
    rc
}

/// TableMetadata: Get columns value.
///
/// See [`TableMetadata::columns`].
///
/// # Receiver
/// - `table_metadata` - Table metadata.
///
/// # Parameters
/// - `index` - column index \[0..tsurugi_ffi_table_metadata_get_columns_size()-1\].
///
/// # Returns
/// - `sql_column_out` - column. To dispose, call [`tsurugi_ffi_sql_column_dispose`](crate::service::sql::column::tsurugi_ffi_sql_column_dispose).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_metadata_get_columns_value(
    context: TsurugiFfiContextHandle,
    table_metadata: TsurugiFfiTableMetadataHandle,
    index: u32,
    sql_column_out: *mut TsurugiFfiSqlColumnHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_metadata_get_columns_value()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, table_metadata={:?}, index={:?}, sql_column_out={:?}",
        context,
        table_metadata,
        index,
        sql_column_out
    );

    ffi_arg_out_initialize!(sql_column_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql_column_out);

    let table_metadata = unsafe { &mut *table_metadata };
    let columns = table_metadata.columns();

    let index = index as usize;
    if index >= columns.len() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "index", "out of bounds");
    }

    let column = columns[index].clone();
    let column = Box::new(TsurugiFfiSqlColumn::new(column));

    let handle = Box::into_raw(column);
    unsafe {
        *sql_column_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. sql_column={:?}", rc, handle);
    rc
}

/// TableMetadata: Get primary keys.
///
/// See [`TableMetadata::primary_keys`].
///
/// # Receiver
/// - `table_metadata` - Table metadata.
///
/// # Returns
/// - `primary_keys_out` - primary keys (string array).
/// - `primary_keys_size_out` - `primary_keys_out` size (number of keys).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_metadata_get_primary_keys(
    context: TsurugiFfiContextHandle,
    table_metadata: TsurugiFfiTableMetadataHandle,
    primary_keys_out: *mut TsurugiFfiStringArrayHandle,
    primary_keys_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_metadata_get_primary_keys()";
    trace!("{FUNCTION_NAME} start. context={:?}, table_list={:?}, primary_keys_out={:?}, primary_keys_size_out={:?}",
        context,
        table_metadata,
        primary_keys_out,
        primary_keys_size_out
    );

    ffi_arg_out_initialize!(primary_keys_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(primary_keys_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, table_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, primary_keys_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, primary_keys_size_out);

    let table_metadata = unsafe { &mut *table_metadata };
    let primary_keys = table_metadata.primary_keys();

    let size = primary_keys.len();

    cstring_array_field_set_if_none!(context, table_metadata.primary_keys, primary_keys);

    let ptr = cstring_array_field_to_ptr!(table_metadata.primary_keys);
    unsafe {
        *primary_keys_out = ptr;
        *primary_keys_size_out = size as u32;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (primary_keys={:?}, primary_keys_size={:?})",
        rc,
        ptr,
        size as u32
    );
    rc
}

/// TableMetadata: Dispose.
///
/// # Receiver
/// - `table_metadata` - Table metadata.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_table_metadata_dispose(
    table_metadata: TsurugiFfiTableMetadataHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_table_metadata_dispose()";
    trace!("{FUNCTION_NAME} start. table_metadata={:?}", table_metadata);

    if table_metadata.is_null() {
        trace!("{FUNCTION_NAME} end. arg[table_metadata] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(table_metadata);
    }

    trace!("{FUNCTION_NAME} end");
}
