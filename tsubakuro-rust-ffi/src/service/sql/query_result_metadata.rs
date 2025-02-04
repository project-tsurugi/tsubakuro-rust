use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_out_initialize, ffi_arg_require_non_null, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::column::TsurugiFfiSqlColumn,
};

use super::column::TsurugiFfiSqlColumnHandle;

pub(crate) struct TsurugiFfiSqlQueryResultMetadata {
    query_result_metadata: SqlQueryResultMetadata,
}

impl TsurugiFfiSqlQueryResultMetadata {
    pub(crate) fn new(
        sql_query_result_metadata: SqlQueryResultMetadata,
    ) -> TsurugiFfiSqlQueryResultMetadata {
        TsurugiFfiSqlQueryResultMetadata {
            query_result_metadata: sql_query_result_metadata,
        }
    }
}

impl std::ops::Deref for TsurugiFfiSqlQueryResultMetadata {
    type Target = SqlQueryResultMetadata;

    fn deref(&self) -> &Self::Target {
        &self.query_result_metadata
    }
}

impl std::ops::DerefMut for TsurugiFfiSqlQueryResultMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.query_result_metadata
    }
}

pub type TsurugiFfiSqlQueryResultMetadataHandle = *mut TsurugiFfiSqlQueryResultMetadata;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_metadata_get_columns_size(
    context: TsurugiFfiContextHandle,
    query_result_metadata: TsurugiFfiSqlQueryResultMetadataHandle,
    size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_metadata_get_columns_size()";
    trace!(
        "{FUNCTION_NAME} start. query_result_metadata={:?}",
        query_result_metadata
    );

    ffi_arg_out_initialize!(size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, size_out);

    let sql_query_result_metadata = unsafe { &*query_result_metadata };
    let columns = sql_query_result_metadata.columns();

    unsafe {
        *size_out = columns.len() as u32;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_metadata_get_columns_value(
    context: TsurugiFfiContextHandle,
    query_result_metadata: TsurugiFfiSqlQueryResultMetadataHandle,
    index: u32,
    sql_column_out: *mut TsurugiFfiSqlColumnHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_metadata_get_columns_value()";
    trace!(
        "{FUNCTION_NAME} start. query_result_metadata={:?}",
        query_result_metadata
    );

    ffi_arg_out_initialize!(sql_column_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result_metadata);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql_column_out);

    let sql_query_result_metadata = unsafe { &mut *query_result_metadata };
    let columns = sql_query_result_metadata.columns();

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
pub extern "C" fn tsurugi_ffi_sql_query_result_metadata_dispose(
    query_result_metadata: TsurugiFfiSqlQueryResultMetadataHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_metadata_dispose()";
    trace!(
        "{FUNCTION_NAME} start. query_result_metadata={:?}",
        query_result_metadata
    );

    if query_result_metadata.is_null() {
        trace!("{FUNCTION_NAME} end. arg[query_result_metadata] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(query_result_metadata);
    }

    trace!("{FUNCTION_NAME} end");
}
