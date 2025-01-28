use std::sync::Arc;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc, TSURUGI_FFI_RC_OK},
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
pub extern "C" fn tsurugi_ffi_sql_prepared_statement_close(
    context: TsurugiFfiContextHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_prepared_statement_close()";
    trace!(
        "{FUNCTION_NAME} start. prepared_statement={:?}",
        prepared_statement
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, prepared_statement);

    let prepared_statement = unsafe { &mut *prepared_statement };

    let runtime = prepared_statement.runtime();
    ffi_exec_core_async!(context, FUNCTION_NAME, runtime, prepared_statement.close());

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
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
