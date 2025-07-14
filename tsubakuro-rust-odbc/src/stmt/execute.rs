use std::time::Duration;

use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::*;

use crate::{
    check_sql_client, check_sql_client_or_err, check_stmt,
    ctype::SqlReturn,
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::execute::{
        execute_result::TsurugiOdbcSqlExecuteResult, query_result::TsurugiOdbcQueryResult,
    },
};

mod execute_result;
mod query_result;

#[no_mangle]
pub extern "system" fn SQLExecute(hstmt: HStmt) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLExecute()";
    trace!("{FUNCTION_NAME} start. hstmt={:?}", hstmt);

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = execute(&mut stmt);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn execute(stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
    const FUNCTION_NAME: &str = "execute()";

    let prepare = match stmt.prepare() {
        Some(prepare) => prepare,
        None => {
            debug!("{stmt}.{FUNCTION_NAME} error. prepare not found");
            stmt.add_diag(
                TsurugiOdbcError::BindParameterError,
                "SQLPrepare not called",
            );
            return SqlReturn::SQL_ERROR;
        }
    };
    let mut prepare = prepare.borrow_mut();

    let (ps, parameters) = match prepare.prepared_statement(stmt) {
        Ok(value) => value,
        Err(rc) => return rc,
    };

    if ps.has_result_records() {
        stmt.set_name("SQLExecute.PreparedQuery");
        match prepared_query(stmt, ps, parameters, false) {
            Ok(_) => SqlReturn::SQL_SUCCESS,
            Err(rc) => rc,
        }
    } else {
        stmt.set_name("SQLExecute.PreparedExecute");
        prepared_execute(stmt, ps, parameters)
    }
}

pub(crate) fn prepared_query(
    stmt: &mut TsurugiOdbcStmt,
    ps: &SqlPreparedStatement,
    parameters: Vec<SqlParameter>,
    close_ps: bool,
) -> Result<(), SqlReturn> {
    const FUNCTION_NAME: &str = "prepared_query()";

    let transaction = stmt.dbc().transaction()?;
    stmt.set_auto_commit_from_dbc();

    let sql_client = check_sql_client_or_err!(stmt);
    let runtime = stmt.runtime();

    let timeout = Duration::from_secs(stmt.query_timeout());
    let query_result = match runtime.block_on(sql_client.prepared_query_for(
        &transaction,
        ps,
        parameters,
        timeout,
    )) {
        Ok(result) => {
            debug!("{stmt}.{FUNCTION_NAME}: prepared_query() succeeded");
            result
        }
        Err(e) => {
            warn!("{stmt}.{FUNCTION_NAME}: prepared_query() error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::PreparedQueryError,
                format!("query error. {}", e),
            );
            stmt.rollback_if_auto_commit();
            return Err(SqlReturn::SQL_ERROR);
        }
    };

    let processor = TsurugiOdbcQueryResult::new(query_result, close_ps);
    stmt.set_processor(processor);

    Ok(())
}

pub(crate) fn prepared_execute(
    stmt: &mut TsurugiOdbcStmt,
    ps: &SqlPreparedStatement,
    parameters: Vec<SqlParameter>,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "prepared_execute()";

    let runtime = stmt.runtime().clone();

    let transaction = match stmt.dbc().transaction() {
        Ok(transaction) => transaction,
        Err(rc) => return rc,
    };
    stmt.set_auto_commit_from_dbc();

    let sql_client = check_sql_client!(stmt);

    let timeout = Duration::from_secs(stmt.query_timeout());
    let execute_result = match runtime.block_on(sql_client.prepared_execute_for(
        &transaction,
        ps,
        parameters,
        timeout,
    )) {
        Ok(result) => {
            debug!("{stmt}.{FUNCTION_NAME}: prepared_execute() succeeded");
            result
        }
        Err(e) => {
            warn!("{stmt}.{FUNCTION_NAME}: prepared_execute() error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::PreparedExecuteError,
                format!("SQL execute error. {}", e),
            );
            stmt.rollback_if_auto_commit();
            return SqlReturn::SQL_ERROR;
        }
    };

    let processor = TsurugiOdbcSqlExecuteResult::new(execute_result);
    stmt.set_processor(processor);

    stmt.commit_if_auto_commit()
}
