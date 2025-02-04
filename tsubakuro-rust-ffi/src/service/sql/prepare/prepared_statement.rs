use std::{sync::Arc, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc, TSURUGI_FFI_RC_OK},
    TsurugiFfiDuration,
};

#[derive(Debug)]
pub(crate) struct TsurugiFfiSqlPreparedStatement {
    prepared_statement: SqlPreparedStatement,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl TsurugiFfiSqlPreparedStatement {
    pub(crate) fn new(
        prepared_statement: SqlPreparedStatement,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiSqlPreparedStatement {
        TsurugiFfiSqlPreparedStatement {
            prepared_statement,
            runtime,
        }
    }

    fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

impl std::ops::Deref for TsurugiFfiSqlPreparedStatement {
    type Target = SqlPreparedStatement;

    fn deref(&self) -> &Self::Target {
        &self.prepared_statement
    }
}

impl std::ops::DerefMut for TsurugiFfiSqlPreparedStatement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.prepared_statement
    }
}

pub type TsurugiFfiSqlPreparedStatementHandle = *mut TsurugiFfiSqlPreparedStatement;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_prepared_statement_has_result_records(
    context: TsurugiFfiContextHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    has_result_records_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_prepared_statement_has_result_records()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, prepared_statement={:?}, has_result_records_out={:?}",
        context,
        prepared_statement,
        has_result_records_out
    );

    ffi_arg_out_initialize!(has_result_records_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, prepared_statement);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, has_result_records_out);

    let prepared_statement = unsafe { &*prepared_statement };

    let value = prepared_statement.has_result_records();

    unsafe {
        *has_result_records_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (has_result_records={:?})",
        rc,
        value
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_prepared_statement_set_close_timeout(
    context: TsurugiFfiContextHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_prepared_statement_set_close_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, prepared_statement={:?}, timeout={:?}",
        context,
        prepared_statement,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, prepared_statement);

    let prepared_statement = unsafe { &mut *prepared_statement };
    let timeout = Duration::from_nanos(timeout);

    prepared_statement.set_close_timeout(timeout);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_prepared_statement_get_close_timeout(
    context: TsurugiFfiContextHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    close_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_prepared_statement_get_close_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, prepared_statement={:?}, close_timeout_out={:?}",
        context,
        prepared_statement,
        close_timeout_out
    );

    ffi_arg_out_initialize!(close_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, prepared_statement);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, close_timeout_out);

    let prepared_statement = unsafe { &mut *prepared_statement };

    let close_timeout = prepared_statement.close_timeout();
    let close_timeout = close_timeout.as_nanos() as TsurugiFfiDuration;

    unsafe {
        *close_timeout_out = close_timeout;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (close_timeout={:?})",
        rc,
        close_timeout
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_prepared_statement_close(
    context: TsurugiFfiContextHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_prepared_statement_close()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, prepared_statement={:?}",
        context,
        prepared_statement
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, prepared_statement);

    let prepared_statement = unsafe { &mut *prepared_statement };

    let runtime = prepared_statement.runtime();
    ffi_exec_core_async!(context, FUNCTION_NAME, runtime, prepared_statement.close());

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_prepared_statement_close_for(
    context: TsurugiFfiContextHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_prepared_statement_close_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, prepared_statement={:?}, timeout={:?}",
        context,
        prepared_statement,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, prepared_statement);

    let prepared_statement = unsafe { &mut *prepared_statement };
    let timeout = Duration::from_nanos(timeout);

    let runtime = prepared_statement.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        prepared_statement.close_for(timeout)
    );

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_prepared_statement_is_closed(
    context: TsurugiFfiContextHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    is_closed_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_prepared_statement_is_closed()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, prepared_statement={:?}, is_closed_out={:?}",
        context,
        prepared_statement,
        is_closed_out
    );

    ffi_arg_out_initialize!(is_closed_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, prepared_statement);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_closed_out);

    let prepared_statement = unsafe { &*prepared_statement };

    let is_closed = prepared_statement.is_closed();

    unsafe {
        *is_closed_out = is_closed;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (is_closed={:?})",
        rc,
        is_closed
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_prepared_statement_dispose(
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
) {
    prepared_statement_dispose(prepared_statement);
}

fn prepared_statement_dispose(
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_prepared_statement_dispose()";
    trace!(
        "{FUNCTION_NAME} start. prepared_statement={:?}",
        prepared_statement
    );

    if prepared_statement.is_null() {
        trace!("{FUNCTION_NAME} end. arg[prepared_statement] is null");
        return TSURUGI_FFI_RC_OK;
    }

    unsafe {
        let prepared_statement = Box::from_raw(prepared_statement);

        if !prepared_statement.is_closed() {
            let context = std::ptr::null_mut();

            let runtime = prepared_statement.runtime();
            ffi_exec_core_async!(context, FUNCTION_NAME, runtime, prepared_statement.close());
        }
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}
