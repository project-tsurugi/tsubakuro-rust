use std::{ffi::c_char, sync::Arc};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_dispose, cchar_field_set,
    context::TsurugiFfiContextHandle,
    ffi_arg_require_non_null, ffi_exec_core, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::query_result_metadata::TsurugiFfiSqlQueryResultMetadata,
};

use super::query_result_metadata::TsurugiFfiSqlQueryResultMetadataHandle;

pub(crate) struct TsurugiFfiSqlQueryResult {
    query_result: SqlQueryResult,
    runtime: Arc<tokio::runtime::Runtime>,
    character_value: *mut c_char,
}

impl TsurugiFfiSqlQueryResult {
    pub(crate) fn new(
        query_result: SqlQueryResult,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiSqlQueryResult {
        TsurugiFfiSqlQueryResult {
            query_result,
            runtime,
            character_value: std::ptr::null_mut(),
        }
    }

    fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

impl std::ops::Deref for TsurugiFfiSqlQueryResult {
    type Target = SqlQueryResult;

    fn deref(&self) -> &Self::Target {
        &self.query_result
    }
}

impl std::ops::DerefMut for TsurugiFfiSqlQueryResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.query_result
    }
}

pub type TsurugiFfiSqlQueryResultHandle = *mut TsurugiFfiSqlQueryResult;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_get_metadata(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    metadata_out: *mut TsurugiFfiSqlQueryResultMetadataHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_get_metadata()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, metadata_out);

    let query_result = unsafe { &*query_result };

    let metadata = match query_result.get_metadata() {
        Some(value) => value.clone(),
        None => unsafe {
            trace!("{FUNCTION_NAME} end. query_result_metadata=null");
            *metadata_out = std::ptr::null_mut();
            return rc_ok(context);
        },
    };
    let metadata = Box::new(TsurugiFfiSqlQueryResultMetadata::new(metadata));

    let handle = Box::into_raw(metadata);
    unsafe {
        *metadata_out = handle;
    }

    trace!("{FUNCTION_NAME} end. query_result_metadata={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_next_row(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    has_row_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_next_row()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, has_row_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let has_next = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.next_row());

    unsafe {
        *has_row_out = has_next;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_next_column(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    has_column_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_next_column()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, has_column_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let has_next =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.next_column());

    unsafe {
        *has_column_out = has_next;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_is_null(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    is_null_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_is_null()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_null_out);

    let query_result = unsafe { &mut *query_result };
    let is_null = ffi_exec_core!(context, FUNCTION_NAME, query_result.is_null());

    unsafe {
        *is_null_out = is_null;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_int4(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_int4()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_int8(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_int8()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_float4(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut f32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_float4()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_float8(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut f64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_float8()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

// TODO tsurugi_ffi_sql_query_result_fetch_decimal()

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_character(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_character()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: String = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());
    unsafe {
        cchar_field_set!(context, query_result.character_value, value);
    }

    unsafe {
        *value_out = query_result.character_value;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

// TODO tsurugi_ffi_sql_query_result_fetch_octet(), etc

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_dispose(
    query_result: TsurugiFfiSqlQueryResultHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_dispose()";
    trace!("{FUNCTION_NAME} start. query_result={:?}", query_result);

    if query_result.is_null() {
        trace!("{FUNCTION_NAME} end. arg[query_result] is null");
        return;
    }

    unsafe {
        let query_result = Box::from_raw(query_result);

        cchar_field_dispose!(query_result.character_value);
    }

    trace!("{FUNCTION_NAME} end");
}
