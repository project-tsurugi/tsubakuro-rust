//! Sql query result.

use std::{ffi::CString, sync::Arc, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core,
    ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::{
        query_result_metadata::TsurugiFfiSqlQueryResultMetadata,
        r#type::{blob::TsurugiFfiBlobReference, clob::TsurugiFfiClobReference},
    },
    vec_u8_to_field, TsurugiFfiByteArrayHandle, TsurugiFfiDuration, TsurugiFfiStringHandle,
};

use super::{
    query_result_metadata::TsurugiFfiSqlQueryResultMetadataHandle,
    r#type::{blob::TsurugiFfiBlobReferenceHandle, clob::TsurugiFfiClobReferenceHandle},
};

pub(crate) struct TsurugiFfiSqlQueryResult {
    query_result: SqlQueryResult,
    runtime: Arc<tokio::runtime::Runtime>,
    unscaled_value_bytes: Option<Vec<u8>>,
    character_value: Option<CString>,
    octet_value: Option<Vec<u8>>,
}

impl TsurugiFfiSqlQueryResult {
    pub(crate) fn new(
        query_result: SqlQueryResult,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiSqlQueryResult {
        TsurugiFfiSqlQueryResult {
            query_result,
            runtime,
            unscaled_value_bytes: None,
            character_value: None,
            octet_value: None,
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

/// Sql query result (sql result set).
pub type TsurugiFfiSqlQueryResultHandle = *mut TsurugiFfiSqlQueryResult;

/// SqlQueryResult: Set default timeout.
///
/// See [`SqlQueryResult::set_default_timeout`].
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_set_default_timeout(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_set_default_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}",
        context,
        query_result,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);

    let query_result = unsafe { &mut *query_result };
    let default_timeout = Duration::from_nanos(timeout);

    query_result.set_default_timeout(default_timeout);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// SqlQueryResult: Get default timeout.
///
/// See [`SqlQueryResult::default_timeout`].
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `default_timeout_out` - timeout time \[nanoseconds\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_get_default_timeout(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    default_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_get_default_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, default_timeout_out={:?}",
        context,
        query_result,
        default_timeout_out
    );

    ffi_arg_out_initialize!(default_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, default_timeout_out);

    let query_result = unsafe { &mut *query_result };

    let default_timeout = query_result.default_timeout();

    let value = default_timeout.as_nanos() as TsurugiFfiDuration;
    unsafe {
        *default_timeout_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (default_timeout={:?})",
        rc,
        value
    );
    rc
}

/// SqlQueryResult: Get metadata.
///
/// See [`SqlQueryResult::get_metadata`].
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `query_result_metadata_out` - metadata. To dispose, call [`tsurugi_ffi_sql_query_result_metadata_dispose`](crate::service::sql::query_result_metadata::tsurugi_ffi_sql_query_result_metadata_dispose).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_get_metadata(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    query_result_metadata_out: *mut TsurugiFfiSqlQueryResultMetadataHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_get_metadata()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, query_result_metadata_out={:?}",
        context,
        query_result,
        query_result_metadata_out
    );

    ffi_arg_out_initialize!(query_result_metadata_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, query_result_metadata_out);

    let query_result = unsafe { &*query_result };

    let metadata = match query_result.get_metadata() {
        Some(value) => value.clone(),
        None => unsafe {
            trace!("{FUNCTION_NAME} end. query_result_metadata=null");
            *query_result_metadata_out = std::ptr::null_mut();
            return rc_ok(context);
        },
    };
    let metadata = Box::new(TsurugiFfiSqlQueryResultMetadata::new(metadata));

    let handle = Box::into_raw(metadata);
    unsafe {
        *query_result_metadata_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. query_result_metadata={:?}",
        rc,
        handle
    );
    rc
}

/// SqlQueryResult: Next row.
///
/// See [`SqlQueryResult::next_row`].
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `has_row_out` - `true`: Has next row / `false`: No next row.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_next_row(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    has_row_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_next_row()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, has_row_out={:?}",
        context,
        query_result,
        has_row_out
    );

    ffi_arg_out_initialize!(has_row_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, has_row_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.next_row());

    unsafe {
        *has_row_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (has_row={:?})", rc, value);
    rc
}

/// SqlQueryResult: Next row.
///
/// See [`SqlQueryResult::next_row_for`].
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `has_row_out` - `true`: Has next row / `false`: No next row.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_next_row_for(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    has_row_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_next_row_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, has_row_out={:?}",
        context,
        query_result,
        timeout,
        has_row_out
    );

    ffi_arg_out_initialize!(has_row_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, has_row_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.next_row_for(timeout)
    );

    unsafe {
        *has_row_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (has_row={:?})", rc, value);
    rc
}

/// SqlQueryResult: Next column.
///
/// See [`SqlQueryResult::next_column`].
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `has_column_out` - `true`: Has next column / `false`: No next column.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_next_column(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    has_column_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_next_column()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, has_column_out={:?}",
        context,
        query_result,
        has_column_out
    );

    ffi_arg_out_initialize!(has_column_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, has_column_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.next_column());

    unsafe {
        *has_column_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (has_column={:?})", rc, value);
    rc
}

/// SqlQueryResult: Next column.
///
/// See [`SqlQueryResult::next_column_for`].
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `has_column_out` - `true`: Has next column / `false`: No next column.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_next_column_for(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    has_column_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_next_column_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, has_column_out={:?}",
        context,
        query_result,
        timeout,
        has_column_out
    );

    ffi_arg_out_initialize!(has_column_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, has_column_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.next_column_for(timeout)
    );

    unsafe {
        *has_column_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (has_column={:?})", rc, value);
    rc
}

/// SqlQueryResult: Whether the column on this cursor is `NULL` or not.
///
/// See [`SqlQueryResult::is_null`].
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `is_null_out` - `true`: Is null / `false`: Not null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_is_null(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    is_null_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_is_null()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, is_null_out={:?}",
        context,
        query_result,
        is_null_out
    );

    ffi_arg_out_initialize!(is_null_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_null_out);

    let query_result = unsafe { &mut *query_result };
    let value = ffi_exec_core!(context, FUNCTION_NAME, query_result.is_null());

    unsafe {
        *is_null_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (is_null={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch int4 (int).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_int4(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_int4()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}",
        context,
        query_result,
        value_out
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch int4 (int).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_int4(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_int4()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}",
        context,
        query_result,
        timeout,
        value_out
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch int8 (bigint).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_int8(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_int8()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}",
        context,
        query_result,
        value_out
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch int8 (bigint).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_int8(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_int8()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}",
        context,
        query_result,
        timeout,
        value_out
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch float4 (real).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_float4(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut f32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_float4()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}",
        context,
        query_result,
        value_out
    );

    ffi_arg_out_initialize!(value_out, 0f32);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch float4 (real).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_float4(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut f32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_float4()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}",
        context,
        query_result,
        timeout,
        value_out
    );

    ffi_arg_out_initialize!(value_out, 0f32);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch float8 (double).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_float8(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut f64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_float8()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}",
        context,
        query_result,
        value_out
    );

    ffi_arg_out_initialize!(value_out, 0f64);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch float8 (double).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_float8(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut f64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_float8()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}",
        context,
        query_result,
        timeout,
        value_out
    );

    ffi_arg_out_initialize!(value_out, 0f64);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch decimal.
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `unscaled_value_bytes_out` - unscaled value of decimal.
/// - `unscaled_value_bytes_size_out` - `unscaled_value_bytes_out` size \[byte\].
/// - `unscaled_value_out` - unscaled value of decimal if `unscaled_value_bytes_out` is null (`unscaled_value_bytes_size_out` = 0).
/// - `exponent_out` - exponent of decimal.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_decimal(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    unscaled_value_bytes_out: *mut TsurugiFfiByteArrayHandle,
    unscaled_value_bytes_size_out: *mut u32,
    unscaled_value_out: *mut i64,
    exponent_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_decimal()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, unscaled_value_bytes_out={:?}, unscaled_value_bytes_size_out={:?}, unscaled_value_out={:?}, exponent_out={:?}",
        context,
        query_result,
        unscaled_value_bytes_out,
        unscaled_value_bytes_size_out,
        unscaled_value_out,
        exponent_out,
    );

    ffi_arg_out_initialize!(unscaled_value_bytes_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(unscaled_value_bytes_size_out, 0);
    ffi_arg_out_initialize!(unscaled_value_out, 0);
    ffi_arg_out_initialize!(exponent_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, unscaled_value_bytes_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, unscaled_value_bytes_size_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, unscaled_value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, exponent_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgDecimalResult =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    let (ptr, size) = if let Some(vec) = value.unscaled_value_bytes {
        vec_u8_to_field!(query_result.unscaled_value_bytes, vec)
    } else {
        (std::ptr::null(), 0)
    };
    let unscaled_value_bytes_size = size as u32;
    let unscaled_value = value.unscaled_value;
    let exponent = value.exponent;

    unsafe {
        *unscaled_value_bytes_out = ptr;
        *unscaled_value_bytes_size_out = unscaled_value_bytes_size;
        *unscaled_value_out = unscaled_value;
        *exponent_out = exponent;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (unscaled_value_bytes={:?}, unscaled_value_bytes_size={:?}, unscaled_value={:?}, exponent={:?})",
        rc,
        ptr,
        unscaled_value_bytes_size_out,
        unscaled_value,
        exponent
    );
    rc
}

/// SqlQueryResult: fetch decimal.
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `unscaled_value_bytes_out` - unscaled value of decimal.
/// - `unscaled_value_bytes_size_out` - `unscaled_value_bytes_out` size \[byte\].
/// - `unscaled_value_out` - unscaled value of decimal if `unscaled_value_bytes_out` is null (`unscaled_value_bytes_size_out` = 0).
/// - `exponent_out` - exponent of decimal.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_decimal(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    unscaled_value_bytes_out: *mut TsurugiFfiByteArrayHandle,
    unscaled_value_bytes_size_out: *mut u32,
    unscaled_value_out: *mut i64,
    exponent_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_decimal()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, unscaled_value_bytes_out={:?}, unscaled_value_bytes_size_out={:?}, unscaled_value_out={:?}, exponent_out={:?}",
        context,
        query_result,
        timeout,
        unscaled_value_bytes_out,
        unscaled_value_bytes_size_out,
        unscaled_value_out,
        exponent_out,
    );

    ffi_arg_out_initialize!(unscaled_value_bytes_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(unscaled_value_bytes_size_out, 0);
    ffi_arg_out_initialize!(unscaled_value_out, 0);
    ffi_arg_out_initialize!(exponent_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, unscaled_value_bytes_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, unscaled_value_bytes_size_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, unscaled_value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 6, exponent_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgDecimalResult = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    let (ptr, size) = if let Some(vec) = value.unscaled_value_bytes {
        vec_u8_to_field!(query_result.unscaled_value_bytes, vec)
    } else {
        (std::ptr::null(), 0)
    };
    let unscaled_value_bytes_size = size as u32;
    let unscaled_value = value.unscaled_value;
    let exponent = value.exponent;

    unsafe {
        *unscaled_value_bytes_out = ptr;
        *unscaled_value_bytes_size_out = unscaled_value_bytes_size;
        *unscaled_value_out = unscaled_value;
        *exponent_out = exponent;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (unscaled_value_bytes={:?}, unscaled_value_bytes_size={:?}, unscaled_value={:?}, exponent={:?})",
        rc,
        ptr,
        unscaled_value_bytes_size_out,
        unscaled_value,
        exponent
    );
    rc
}

/// SqlQueryResult: fetch decimal.
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `unscaled_value_high_out` - unscaled value of decimal (high 64bit).
/// - `unscaled_value_low_out` - unscaled value of decimal (low 64bit).
/// - `exponent_out` - exponent of decimal.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_decimal_i128(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    unscaled_value_high_out: *mut i64,
    unscaled_value_low_out: *mut u64,
    exponent_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_decimal_i128()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, unscaled_value_high_out={:?}, unscaled_value_low_out={:?}, exponent_out={:?}",
        context,
        query_result,
        unscaled_value_high_out,
        unscaled_value_low_out,
        exponent_out,
    );

    ffi_arg_out_initialize!(unscaled_value_high_out, 0);
    ffi_arg_out_initialize!(unscaled_value_low_out, 0);
    ffi_arg_out_initialize!(exponent_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, unscaled_value_high_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, unscaled_value_low_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, exponent_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgDecimalI128 =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    let unscaled_value = value.unscaled_value;
    let high_value = (unscaled_value >> 64) as i64;
    let low_value = unscaled_value as u64;
    let exponent = value.exponent;

    unsafe {
        *unscaled_value_high_out = high_value;
        *unscaled_value_low_out = low_value;
        *exponent_out = exponent;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (unscaled_value_high={:?}, unscaled_value_low={:?}, exponent={:?})",
        rc,
        high_value,
        low_value,
        exponent
    );
    rc
}

/// SqlQueryResult: fetch decimal.
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `unscaled_value_high_out` - unscaled value of decimal (high 64bit).
/// - `unscaled_value_low_out` - unscaled value of decimal (low 64bit).
/// - `exponent_out` - exponent of decimal.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_decimal_i128(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    unscaled_value_high_out: *mut i64,
    unscaled_value_low_out: *mut u64,
    exponent_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_decimal_i128()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, unscaled_value_high_out={:?}, unscaled_value_low_out={:?}, exponent_out={:?}",
        context,
        query_result,
        timeout,
        unscaled_value_high_out,
        unscaled_value_low_out,
        exponent_out,
    );

    ffi_arg_out_initialize!(unscaled_value_high_out, 0);
    ffi_arg_out_initialize!(unscaled_value_low_out, 0);
    ffi_arg_out_initialize!(exponent_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, unscaled_value_high_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, unscaled_value_low_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, exponent_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgDecimalI128 = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    let unscaled_value = value.unscaled_value;
    let high_value = (unscaled_value >> 64) as i64;
    let low_value = unscaled_value as u64;
    let exponent = value.exponent;

    unsafe {
        *unscaled_value_high_out = high_value;
        *unscaled_value_low_out = low_value;
        *exponent_out = exponent;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (unscaled_value_high={:?}, unscaled_value_low={:?}, exponent={:?})",
        rc,
        high_value,
        low_value,
        exponent
    );
    rc
}

/// SqlQueryResult: fetch character (char/varchar).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_character(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_character()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}",
        context,
        query_result,
        value_out
    );

    ffi_arg_out_initialize!(value_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: String = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());
    cchar_field_set!(context, query_result.character_value, value);

    let ptr = cstring_to_cchar!(query_result.character_value);
    unsafe {
        *value_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, ptr);
    rc
}

/// SqlQueryResult: fetch character (char/varchar).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - value.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_character(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_character()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}",
        context,
        query_result,
        timeout,
        value_out
    );

    ffi_arg_out_initialize!(value_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: String = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );
    cchar_field_set!(context, query_result.character_value, value);

    let ptr = cstring_to_cchar!(query_result.character_value);
    unsafe {
        *value_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, ptr);
    rc
}

/// SqlQueryResult: fetch octet.
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - value.
/// - `size_out` - `value_out` size \[byte\].
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_octet(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut TsurugiFfiByteArrayHandle,
    size_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_octet()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}, size_out={:?}",
        context,
        query_result,
        value_out,
        size_out
    );

    ffi_arg_out_initialize!(value_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, size_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: Vec<u8> =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    let (ptr, size) = vec_u8_to_field!(query_result.octet_value, value);
    unsafe {
        *value_out = ptr;
        *size_out = size;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, size={:?})",
        rc,
        ptr,
        size
    );
    rc
}

/// SqlQueryResult: fetch octet.
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - value.
/// - `size_out` - `value_out` size \[byte\].
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_octet(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut TsurugiFfiByteArrayHandle,
    size_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_octet()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}",
        context,
        query_result,
        timeout,
        value_out
    );

    ffi_arg_out_initialize!(value_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, size_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: Vec<u8> = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );
    let size = value.len();
    query_result.octet_value = Some(value);

    let ptr = if size != 0 {
        query_result.octet_value.as_ref().unwrap().as_ptr()
    } else {
        std::ptr::null()
    };
    unsafe {
        *value_out = ptr;
        *size_out = size as u64;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, size={:?})",
        rc,
        ptr,
        size as u64
    );
    rc
}

/// SqlQueryResult: fetch date.
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - number of days offset of epoch 1970-01-01.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_date(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_date()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}",
        context,
        query_result,
        value_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgDate = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    let value = value.epoch_days;
    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch date.
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - number of days offset of epoch 1970-01-01.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_date(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut i64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_date()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}",
        context,
        query_result,
        timeout,
        value_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgDate = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    let value = value.epoch_days;
    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch time of day (time).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - nanoseconds since 00:00:00.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_time_of_day(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_time_of_day()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}",
        context,
        query_result,
        value_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgTimeOfDay =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    let value = value.offset_nanoseconds;
    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch time of day (time).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - nanoseconds since 00:00:00.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_time_of_day(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_time_of_day()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}",
        context,
        query_result,
        timeout,
        value_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgTimeOfDay = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    let value = value.offset_nanoseconds;
    unsafe {
        *value_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (value={:?})", rc, value);
    rc
}

/// SqlQueryResult: fetch time point (timestamp).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - number of seconds offset of epoch 1970-01-01.
/// - `nanos_out` - nanoseconds adjustment \[0, 10^9-1\].
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_time_point(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut i64,
    nanos_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_time_point()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}, nanos_out={:?}",
        context,
        query_result,
        value_out,
        nanos_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, nanos_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgTimePoint =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value.offset_seconds;
        *nanos_out = value.nano_adjustment;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, nanos={:?})",
        rc,
        value.offset_seconds,
        value.nano_adjustment
    );
    rc
}

/// SqlQueryResult: fetch time point (timestamp).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - number of seconds offset of epoch 1970-01-01.
/// - `nanos_out` - nanoseconds adjustment \[0, 10^9-1\].
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_time_point(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut i64,
    nanos_out: *mut u32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_time_point()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}, nanos_out={:?}",
        context,
        query_result,
        timeout,
        value_out,
        nanos_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, nanos_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgTimePoint = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    unsafe {
        *value_out = value.offset_seconds;
        *nanos_out = value.nano_adjustment;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, nanos={:?})",
        rc,
        value.offset_seconds,
        value.nano_adjustment
    );
    rc
}

/// SqlQueryResult: fetch time of day with time zone (time with time zone).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - nanoseconds since 00:00:00.
/// - `time_zone_offset_out` - timezone offset in minute.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_time_of_day_with_time_zone(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut u64,
    time_zone_offset_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_time_of_day_with_time_zone()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}, time_zone_offset_out={:?}",
        context,
        query_result,
        value_out,
        time_zone_offset_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, time_zone_offset_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgTimeOfDayWithTimeZone =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value.offset_nanoseconds;
        *time_zone_offset_out = value.time_zone_offset;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, time_zone_offset={:?})",
        rc,
        value.offset_nanoseconds,
        value.time_zone_offset
    );
    rc
}

/// SqlQueryResult: fetch time of day with time zone (time with time zone).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - nanoseconds since 00:00:00.
/// - `time_zone_offset_out` - timezone offset in minute.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_time_of_day_with_time_zone(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut u64,
    time_zone_offset_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str =
        "tsurugi_ffi_sql_query_result_fetch_for_time_of_day_with_time_zone()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}, time_zone_offset_out={:?}",
        context,
        query_result,
        timeout,
        value_out,
        time_zone_offset_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, time_zone_offset_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgTimeOfDayWithTimeZone = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    unsafe {
        *value_out = value.offset_nanoseconds;
        *time_zone_offset_out = value.time_zone_offset;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, time_zone_offset={:?})",
        rc,
        value.offset_nanoseconds,
        value.time_zone_offset
    );
    rc
}

/// SqlQueryResult: fetch time point with time zone (timestamp with time zone).
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `value_out` - number of seconds offset of epoch 1970-01-01.
/// - `nanos_out` - nanoseconds adjustment \[0, 10^9-1\].
/// - `time_zone_offset_out` - timezone offset in minute.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_time_point_with_time_zone(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    value_out: *mut i64,
    nanos_out: *mut u32,
    time_zone_offset_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_time_point_with_time_zone()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, value_out={:?}, nanos_out={:?}, time_zone_offset_out={:?}",
        context,
        query_result,
        value_out,
        nanos_out,
        time_zone_offset_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, nanos_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, time_zone_offset_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgTimePointWithTimeZone =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());

    unsafe {
        *value_out = value.offset_seconds;
        *nanos_out = value.nano_adjustment;
        *time_zone_offset_out = value.time_zone_offset;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, nanos={:?}, time_zone_offset={:?})",
        rc,
        value.offset_seconds,
        value.nano_adjustment,
        value.time_zone_offset
    );
    rc
}

/// SqlQueryResult: fetch time point with time zone (timestamp with time zone).
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - number of seconds offset of epoch 1970-01-01.
/// - `nanos_out` - nanoseconds adjustment \[0, 10^9-1\].
/// - `time_zone_offset_out` - timezone offset in minute.
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_time_point_with_time_zone(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut i64,
    nanos_out: *mut u32,
    time_zone_offset_out: *mut i32,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str =
        "tsurugi_ffi_sql_query_result_fetch_for_time_point_with_time_zone()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, value_out={:?}, nanos_out={:?}, time_zone_offset_out={:?}",
        context,
        query_result,
        timeout,
        value_out,
        nanos_out,
        time_zone_offset_out,
    );

    ffi_arg_out_initialize!(value_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, nanos_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, time_zone_offset_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgTimePointWithTimeZone = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );

    unsafe {
        *value_out = value.offset_seconds;
        *nanos_out = value.nano_adjustment;
        *time_zone_offset_out = value.time_zone_offset;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, nanos={:?}, time_zone_offset={:?})",
        rc,
        value.offset_seconds,
        value.nano_adjustment,
        value.time_zone_offset
    );
    rc
}

/// SqlQueryResult: fetch blob.
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `blob_reference_out` - blob reference. To dispose, call [`tsurugi_ffi_blob_reference_dispose`](crate::service::sql::type::blob::tsurugi_ffi_blob_reference_dispose).
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_blob(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    blob_reference_out: *mut TsurugiFfiBlobReferenceHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_blob()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, blob_reference_out={:?}",
        context,
        query_result,
        blob_reference_out,
    );

    ffi_arg_out_initialize!(blob_reference_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, blob_reference_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgBlobReference =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());
    let value = Box::new(TsurugiFfiBlobReference::new(value));

    let ptr = Box::into_raw(value);
    unsafe {
        *blob_reference_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. blob_reference={:?}", rc, ptr);
    rc
}

/// SqlQueryResult: fetch blob.
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `blob_reference_out` - blob reference. To dispose, call [`tsurugi_ffi_blob_reference_dispose`](crate::service::sql::type::blob::tsurugi_ffi_blob_reference_dispose).
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_blob(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    blob_reference_out: *mut TsurugiFfiBlobReferenceHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_blob()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, blob_reference_out={:?}",
        context,
        query_result,
        timeout,
        blob_reference_out,
    );

    ffi_arg_out_initialize!(blob_reference_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, blob_reference_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgBlobReference = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );
    let value = Box::new(TsurugiFfiBlobReference::new(value));

    let ptr = Box::into_raw(value);
    unsafe {
        *blob_reference_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. blob_reference={:?}", rc, ptr);
    rc
}

/// SqlQueryResult: fetch clob.
///
/// See [`SqlQueryResult::fetch`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Returns
/// - `clob_reference_out` - clob reference. To dispose, call [`tsurugi_ffi_clob_reference_dispose`](crate::service::sql::type::clob::tsurugi_ffi_clob_reference_dispose).
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_clob(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    clob_reference_out: *mut TsurugiFfiClobReferenceHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_clob()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, clob_reference_out={:?}",
        context,
        query_result,
        clob_reference_out,
    );

    ffi_arg_out_initialize!(clob_reference_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, clob_reference_out);

    let query_result = unsafe { &mut *query_result };

    let runtime = query_result.runtime().clone();
    let value: TgClobReference =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, query_result.fetch());
    let value = Box::new(TsurugiFfiClobReference::new(value));

    let ptr = Box::into_raw(value);
    unsafe {
        *clob_reference_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. clob_reference={:?}", rc, ptr);
    rc
}

/// SqlQueryResult: fetch clob.
///
/// See [`SqlQueryResult::fetch_for`].
///
/// Retrieves a value on the column of the cursor position.
///
/// Need to call [`tsurugi_ffi_sql_query_result_next_column`] first.
/// You can only take once to retrieve the value on the column.
///
/// # Receiver
/// - `query_result` - Sql query result.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `clob_reference_out` - clob reference. To dispose, call [`tsurugi_ffi_clob_reference_dispose`](crate::service::sql::type::clob::tsurugi_ffi_clob_reference_dispose).
///
/// Return value is not null. Call [`tsurugi_ffi_sql_query_result_is_null`] to check null.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_query_result_fetch_for_clob(
    context: TsurugiFfiContextHandle,
    query_result: TsurugiFfiSqlQueryResultHandle,
    timeout: TsurugiFfiDuration,
    clob_reference_out: *mut TsurugiFfiClobReferenceHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_query_result_fetch_for_clob()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, query_result={:?}, timeout={:?}, clob_reference_out={:?}",
        context,
        query_result,
        timeout,
        clob_reference_out,
    );

    ffi_arg_out_initialize!(clob_reference_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, query_result);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, clob_reference_out);

    let query_result = unsafe { &mut *query_result };
    let timeout = Duration::from_nanos(timeout);

    let runtime = query_result.runtime().clone();
    let value: TgClobReference = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        query_result.fetch_for(timeout)
    );
    let value = Box::new(TsurugiFfiClobReference::new(value));

    let ptr = Box::into_raw(value);
    unsafe {
        *clob_reference_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. clob_reference={:?}", rc, ptr);
    rc
}

/// SqlQueryResult: Dispose.
///
/// # Receiver
/// - `query_result` - Sql query result.
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
        let _ = Box::from_raw(query_result);
    }

    trace!("{FUNCTION_NAME} end");
}
