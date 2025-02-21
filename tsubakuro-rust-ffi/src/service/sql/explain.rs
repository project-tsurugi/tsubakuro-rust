use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc, TSURUGI_FFI_RC_OK},
    service::sql::column::TsurugiFfiSqlColumn,
    TsurugiFfiStringHandle,
};

use super::column::TsurugiFfiSqlColumnHandle;

#[derive(Debug)]
pub(crate) struct TsurugiFfiSqlExplainResult {
    explain_result: SqlExplainResult,
    format_id: Option<CString>,
    contents: Option<CString>,
}

impl TsurugiFfiSqlExplainResult {
    pub(crate) fn new(explain_result: SqlExplainResult) -> TsurugiFfiSqlExplainResult {
        TsurugiFfiSqlExplainResult {
            explain_result,
            format_id: None,
            contents: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiSqlExplainResult {
    type Target = SqlExplainResult;

    fn deref(&self) -> &Self::Target {
        &self.explain_result
    }
}

pub type TsurugiFfiSqlExplainResultHandle = *mut TsurugiFfiSqlExplainResult;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_explain_result_get_format_id(
    context: TsurugiFfiContextHandle,
    explain_result: TsurugiFfiSqlExplainResultHandle,
    format_id_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_explain_result_get_format_id()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, explain_result={:?}, format_id_out={:?}",
        context,
        explain_result,
        format_id_out
    );

    ffi_arg_out_initialize!(format_id_out, std::ptr::null());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, explain_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, format_id_out);

    let explain_result = unsafe { &mut *explain_result };

    if explain_result.format_id.is_none() {
        let format_id = explain_result.format_id().clone();
        cchar_field_set!(context, explain_result.format_id, format_id);
    }

    let ptr = cstring_to_cchar!(explain_result.format_id);
    unsafe {
        *format_id_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (format_id={:?})", rc, ptr);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_explain_result_get_format_version(
    context: TsurugiFfiContextHandle,
    explain_result: TsurugiFfiSqlExplainResultHandle,
    format_version_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_explain_result_get_format_version()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, explain_result={:?}, format_version_out={:?}",
        context,
        explain_result,
        format_version_out
    );

    ffi_arg_out_initialize!(format_version_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, explain_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, format_version_out);

    let explain_result = unsafe { &*explain_result };

    let value = explain_result.format_version();

    unsafe {
        *format_version_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (format_version={:?})",
        rc,
        value
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_explain_result_get_contents(
    context: TsurugiFfiContextHandle,
    explain_result: TsurugiFfiSqlExplainResultHandle,
    contents_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_explain_result_get_contents()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, explain_result={:?}, contents_out={:?}",
        context,
        explain_result,
        contents_out
    );

    ffi_arg_out_initialize!(contents_out, std::ptr::null());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, explain_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, contents_out);

    let explain_result = unsafe { &mut *explain_result };

    if explain_result.contents.is_none() {
        let contents = explain_result.contents().clone();
        cchar_field_set!(context, explain_result.contents, contents);
    }

    let ptr = cstring_to_cchar!(explain_result.contents);
    unsafe {
        *contents_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (contents={:?})", rc, ptr);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_explain_result_get_columns_size(
    context: TsurugiFfiContextHandle,
    explain_result: TsurugiFfiSqlExplainResultHandle,
    size_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_explain_result_get_columns_size()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, explain_result={:?}, size_out={:?}",
        context,
        explain_result,
        size_out
    );

    ffi_arg_out_initialize!(size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, explain_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, size_out);

    let explain_result = unsafe { &*explain_result };
    let columns = explain_result.columns();

    let value = columns.len() as u32;
    unsafe {
        *size_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (size={:?})", rc, value);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_explain_result_get_columns_value(
    context: TsurugiFfiContextHandle,
    explain_result: TsurugiFfiSqlExplainResultHandle,
    index: u32,
    sql_column_out: *mut TsurugiFfiSqlColumnHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_explain_result_get_columns_value()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, explain_result={:?}, index={:?}, sql_column_out={:?}",
        context,
        explain_result,
        index,
        sql_column_out
    );

    ffi_arg_out_initialize!(sql_column_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, explain_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql_column_out);

    let explain_result = unsafe { &mut *explain_result };
    let columns = explain_result.columns();

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

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_explain_result_dispose(
    explain_result: TsurugiFfiSqlExplainResultHandle,
) {
    explain_result_dispose(explain_result);
}

fn explain_result_dispose(explain_result: TsurugiFfiSqlExplainResultHandle) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_explain_result_dispose()";
    trace!("{FUNCTION_NAME} start. explain_result={:?}", explain_result);

    if explain_result.is_null() {
        trace!("{FUNCTION_NAME} end. arg[explain_result] is null");
        return TSURUGI_FFI_RC_OK;
    }

    unsafe {
        let _ = Box::from_raw(explain_result);
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}
