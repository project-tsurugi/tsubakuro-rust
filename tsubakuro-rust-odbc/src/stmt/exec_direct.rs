use log::{debug, trace};

use crate::{
    check_stmt,
    ctype::{SqlChar, SqlInteger, SqlReturn, SqlWChar},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::{
        execute::{prepared_execute, prepared_query},
        prepare::TsurugiOdbcPrepare,
    },
    util::{char_to_string_integer, wchar_to_string_integer},
};

#[no_mangle]
pub extern "system" fn SQLExecDirect(
    hstmt: HStmt,
    statement_text: *const SqlChar,
    text_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLExecDirect()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, statement_text={:?}, text_length={:?}",
        hstmt,
        statement_text,
        text_length
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLExecDirect");

    let statement = match char_to_string_integer(statement_text, text_length) {
        Ok(s) => s,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME} statement_text error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::StringError,
                format!("statement_text error. {}", e),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    let rc = if stmt.parameters().is_empty() {
        exec_direct(&mut stmt, statement)
    } else {
        exec_direct_prepared(&mut stmt, statement)
    };

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLExecDirectW(
    hstmt: HStmt,
    statement_text: *const SqlWChar,
    text_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLExecDirectW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, statement_text={:?}, text_length={:?}",
        hstmt,
        statement_text,
        text_length
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLExecDirectW");

    let statement = match wchar_to_string_integer(statement_text, text_length) {
        Ok(s) => s,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME} statement_text error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::StringError,
                format!("statement_text error. {}", e),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    let rc = if stmt.parameters().is_empty() {
        exec_direct(&mut stmt, statement)
    } else {
        exec_direct_prepared(&mut stmt, statement)
    };

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn exec_direct(stmt: &mut TsurugiOdbcStmt, statement: String) -> SqlReturn {
    // TODO SQL direct execute
    exec_direct_prepared(stmt, statement)
}

fn exec_direct_prepared(stmt: &mut TsurugiOdbcStmt, statement: String) -> SqlReturn {
    let mut prepare = TsurugiOdbcPrepare::new(statement);

    let (rc, close_ps) = {
        let (ps, parameters) = {
            match prepare.prepared_statement(stmt) {
                Ok(value) => value,
                Err(rc) => return rc,
            }
        };

        if ps.has_result_records() {
            stmt.set_name("SQLExecDirect.PreparedQuery");
            match prepared_query(stmt, ps, parameters, true) {
                Ok(_) => (SqlReturn::SQL_SUCCESS, false),
                Err(rc) => (rc, true),
            }
        } else {
            stmt.set_name("SQLExecDirect.PreparedExecute");
            let rc = prepared_execute(stmt, ps, parameters);
            (rc, true)
        }
    };

    let rc1 = if close_ps {
        prepare.close(stmt)
    } else {
        stmt.set_prepare(prepare);
        SqlReturn::SQL_SUCCESS
    };

    rc.or(rc1)
}
