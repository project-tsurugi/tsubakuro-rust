use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::column::TsurugiFfiSqlColumn,
    TsurugiFfiStringHandle,
};

use super::column::TsurugiFfiSqlColumnHandle;

pub(crate) struct TsurugiFfiTableMetadata {
    table_metadata: TableMetadata,
    database_name: Option<CString>,
    schema_name: Option<CString>,
    table_name: Option<CString>,
}

impl TsurugiFfiTableMetadata {
    pub(crate) fn new(table_metadata: TableMetadata) -> TsurugiFfiTableMetadata {
        TsurugiFfiTableMetadata {
            table_metadata,
            database_name: None,
            schema_name: None,
            table_name: None,
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

pub type TsurugiFfiTableMetadataHandle = *mut TsurugiFfiTableMetadata;

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

    trace!("{FUNCTION_NAME} end. (database_name={:?})", ptr);
    rc_ok(context)
}

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

    trace!("{FUNCTION_NAME} end. (schema_name={:?})", ptr);
    rc_ok(context)
}

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

    trace!("{FUNCTION_NAME} end. (table_name={:?})", ptr);
    rc_ok(context)
}

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

    unsafe {
        *size_out = columns.len() as u32;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

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

    trace!("{FUNCTION_NAME} end. sql_column={:?}", handle);
    rc_ok(context)
}

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
