use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle, ffi_arg_out_initialize, ffi_arg_require_non_null, return_code::{rc_ok, TsurugiFfiRc}
};

pub(crate) struct TsurugiFfiSqlExecuteResult {
    execute_result: SqlExecuteResult,
}

impl TsurugiFfiSqlExecuteResult {
    pub(crate) fn new(execute_result: SqlExecuteResult) -> TsurugiFfiSqlExecuteResult {
        TsurugiFfiSqlExecuteResult { execute_result }
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

// TODO tsurugi_ffi_sql_execute_result_get_counters_size()
// TODO tsurugi_ffi_sql_execute_result_get_counters_key(index)
// TODO tsurugi_ffi_sql_execute_result_get_counters_value(key)

// #[no_mangle]
// pub extern "C" fn tsurugi_ffi_sql_execute_result_get_counters_size(
//     context: TsurugiFfiContextHandle,
//     execute_result: TsurugiFfiSqlExecuteResultHandle,
//     size_out: *mut u32,
// ) -> TsurugiFfiRc {
//     const FUNCTION_NAME: &str = "tsurugi_ffi_sql_execute_result_counters_size()";
//     trace!("{FUNCTION_NAME} start. execute_result={:?}", execute_result);

//     ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, execute_result);
//     ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, size_out);

//     let execute_result = unsafe { &*execute_result };
//     let counters = execute_result.counters();

//     unsafe {
//         *size_out = counters.len() as u32;
//     }

//     trace!("{FUNCTION_NAME} end");
//     rc_ok(context)
// }

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

fn get_rows<F>(
    context: TsurugiFfiContextHandle,
    execute_result: TsurugiFfiSqlExecuteResultHandle,
    rows_out: *mut i64,
    function_name: &str,
    getter: F,
) -> TsurugiFfiRc
where
    F: FnOnce(&SqlExecuteResult) -> i64,
{
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
