use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TsurugiFfiSqlCounterType {
    /// the un-categorized counter type.
    Unspecified = 0,
    /// The number of rows inserted in the execution.
    InsertedRows = 10,
    /// The number of rows updated in the execution.
    UpdatedRows = 20,
    /// The number of rows merged in the execution.
    MergedRows = 30,
    /// The number of rows deleted in the execution.
    DeletedRows = 40,
}

impl From<SqlCounterType> for TsurugiFfiSqlCounterType {
    fn from(value: SqlCounterType) -> Self {
        match value {
            SqlCounterType::Unspecified => TsurugiFfiSqlCounterType::Unspecified,
            SqlCounterType::InsertedRows => TsurugiFfiSqlCounterType::InsertedRows,
            SqlCounterType::UpdatedRows => TsurugiFfiSqlCounterType::UpdatedRows,
            SqlCounterType::MergedRows => TsurugiFfiSqlCounterType::MergedRows,
            SqlCounterType::DeletedRows => TsurugiFfiSqlCounterType::DeletedRows,
        }
    }
}

pub(crate) struct TsurugiFfiSqlExecuteResult {
    execute_result: SqlExecuteResult,
    counters_keys: Option<Vec<TsurugiFfiSqlCounterType>>,
    counters_rows: Option<Vec<i64>>,
}

impl TsurugiFfiSqlExecuteResult {
    pub(crate) fn new(execute_result: SqlExecuteResult) -> TsurugiFfiSqlExecuteResult {
        TsurugiFfiSqlExecuteResult {
            execute_result,
            counters_keys: None,
            counters_rows: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiSqlExecuteResult {
    type Target = SqlExecuteResult;

    fn deref(&self) -> &Self::Target {
        &self.execute_result
    }
}

impl std::ops::DerefMut for TsurugiFfiSqlExecuteResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.execute_result
    }
}

pub type TsurugiFfiSqlExecuteResultHandle = *mut TsurugiFfiSqlExecuteResult;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_execute_result_get_counters(
    context: TsurugiFfiContextHandle,
    execute_result: TsurugiFfiSqlExecuteResultHandle,
    counters_keys_out: *mut *const TsurugiFfiSqlCounterType,
    counters_rows_out: *mut *const i64,
    counters_size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_execute_result_counters()";
    trace!("{FUNCTION_NAME} start. execute_result={:?}", execute_result);

    ffi_arg_out_initialize!(counters_keys_out, std::ptr::null());
    ffi_arg_out_initialize!(counters_rows_out, std::ptr::null());
    ffi_arg_out_initialize!(counters_size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, execute_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, counters_keys_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, counters_rows_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, counters_size_out);

    let execute_result = unsafe { &mut *execute_result };
    let counters = execute_result.counters();

    let size = counters.len();

    if execute_result.counters_keys.is_none() || execute_result.counters_rows.is_none() {
        let mut keys = Vec::with_capacity(size);
        let mut rows = Vec::with_capacity(size);
        for counter in counters {
            keys.push((*counter.0).into());
            rows.push(*counter.1);
        }
        execute_result.counters_keys = Some(keys);
        execute_result.counters_rows = Some(rows);
    }

    unsafe {
        if size != 0 {
            *counters_keys_out = execute_result.counters_keys.as_ref().unwrap().as_ptr();
            *counters_rows_out = execute_result.counters_rows.as_ref().unwrap().as_ptr();
        } else {
            *counters_keys_out = std::ptr::null();
            *counters_rows_out = std::ptr::null();
        }
        *counters_size_out = size as u32;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_execute_result_get_inserted_rows(
    context: TsurugiFfiContextHandle,
    execute_result: TsurugiFfiSqlExecuteResultHandle,
    rows_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_execute_result_get_inserted_rows()";
    get_rows(
        context,
        execute_result,
        rows_out,
        FUNCTION_NAME,
        SqlExecuteResult::inserted_rows,
    )
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_execute_result_get_updated_rows(
    context: TsurugiFfiContextHandle,
    execute_result: TsurugiFfiSqlExecuteResultHandle,
    rows_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_execute_result_get_updated_rows()";
    get_rows(
        context,
        execute_result,
        rows_out,
        FUNCTION_NAME,
        SqlExecuteResult::updated_rows,
    )
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_execute_result_get_merged_rows(
    context: TsurugiFfiContextHandle,
    execute_result: TsurugiFfiSqlExecuteResultHandle,
    rows_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_execute_result_get_merged_rows()";
    get_rows(
        context,
        execute_result,
        rows_out,
        FUNCTION_NAME,
        SqlExecuteResult::merged_rows,
    )
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_execute_result_get_deleted_rows(
    context: TsurugiFfiContextHandle,
    execute_result: TsurugiFfiSqlExecuteResultHandle,
    rows_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_execute_result_get_deleted_rows()";
    get_rows(
        context,
        execute_result,
        rows_out,
        FUNCTION_NAME,
        SqlExecuteResult::deleted_rows,
    )
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_execute_result_get_rows(
    context: TsurugiFfiContextHandle,
    execute_result: TsurugiFfiSqlExecuteResultHandle,
    rows_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_execute_result_get_rows()";
    get_rows(
        context,
        execute_result,
        rows_out,
        FUNCTION_NAME,
        SqlExecuteResult::rows,
    )
}

fn get_rows(
    context: TsurugiFfiContextHandle,
    execute_result: TsurugiFfiSqlExecuteResultHandle,
    rows_out: *mut i64,
    function_name: &str,
    getter: fn(&SqlExecuteResult) -> i64,
) -> TsurugiFfiRc {
    trace!("{function_name} start. execute_result={:?}", execute_result);

    ffi_arg_out_initialize!(rows_out, 0);
    ffi_arg_require_non_null!(context, function_name, 1, execute_result);
    ffi_arg_require_non_null!(context, function_name, 2, rows_out);

    let execute_result = unsafe { &*execute_result };
    let rows = getter(execute_result);

    unsafe {
        *rows_out = rows;
    }

    trace!("{function_name} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_execute_result_dispose(
    execute_result: TsurugiFfiSqlExecuteResultHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_execute_result_dispose()";
    trace!("{FUNCTION_NAME} start. execute_result={:?}", execute_result);

    if execute_result.is_null() {
        trace!("{FUNCTION_NAME} end. arg[execute_result] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(execute_result);
    }

    trace!("{FUNCTION_NAME} end");
}
