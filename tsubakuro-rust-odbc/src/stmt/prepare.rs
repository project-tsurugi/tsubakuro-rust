use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::{SqlParameter, SqlPlaceholder, SqlPreparedStatement};

use crate::{
    check_sql_client_or_err, check_stmt,
    ctype::{SqlChar, SqlInteger, SqlReturn, SqlWChar},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    util::{char_to_string_integer, wchar_to_string_integer},
};

#[no_mangle]
pub extern "system" fn SQLPrepare(
    hstmt: HStmt,
    statement_text: *const SqlChar,
    text_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLPrepare()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, statement_text={:?}, text_length={:?}",
        hstmt,
        statement_text,
        text_length
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLPrepare");

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

    let rc = prepare(&mut stmt, statement);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLPrepareW(
    hstmt: HStmt,
    statement_text: *const SqlWChar,
    text_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLPrepareW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, statement_text={:?}, text_length={:?}",
        hstmt,
        statement_text,
        text_length
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLPrepareW");

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

    let rc = prepare(&mut stmt, statement);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn prepare(stmt: &mut TsurugiOdbcStmt, statement: String) -> SqlReturn {
    // TODO PreparedStatementの作成を遅延させない

    let prepare = TsurugiOdbcPrepare::new(statement);
    stmt.set_prepare(prepare);

    SqlReturn::SQL_SUCCESS
}

#[derive(Debug)]
pub(crate) struct TsurugiOdbcPrepare {
    statement: String,
    prepared_statement: Option<SqlPreparedStatement>,
}

impl TsurugiOdbcPrepare {
    pub(crate) fn new(statement: String) -> TsurugiOdbcPrepare {
        TsurugiOdbcPrepare {
            statement,
            prepared_statement: None,
        }
    }

    pub(crate) fn prepared_statement(
        &mut self,
        stmt: &mut TsurugiOdbcStmt,
    ) -> Result<(&SqlPreparedStatement, Vec<SqlParameter>), SqlReturn> {
        const FUNCTION_NAME: &str = "prepared_statement()";

        if self.prepared_statement.is_none() {
            let sql_client = check_sql_client_or_err!(stmt);
            let runtime = stmt.runtime();

            let statement = &self.statement;
            let placeholders = self.placeholders(stmt)?;
            let ps = match runtime.block_on(sql_client.prepare(statement, placeholders)) {
                Ok(ps) => {
                    debug!("{stmt}.{FUNCTION_NAME}: prepare() succeeded");
                    ps
                }
                Err(e) => {
                    warn!("{stmt}.{FUNCTION_NAME}: prepare() error. {:?}", e);
                    stmt.add_diag(
                        TsurugiOdbcError::PrepareError,
                        format!("prepare error. {}", e),
                    );
                    return Err(SqlReturn::SQL_ERROR);
                }
            };

            self.prepared_statement = Some(ps);
        }

        let parameters = self.parameters(stmt)?;

        if let Some(ps) = &self.prepared_statement {
            Ok((ps, parameters))
        } else {
            unreachable!()
        }
    }

    fn placeholders(&self, stmt: &TsurugiOdbcStmt) -> Result<Vec<SqlPlaceholder>, SqlReturn> {
        const FUNCTION_NAME: &str = "placeholders()";

        let parameters = stmt.parameters();
        let mut vec = Vec::with_capacity(parameters.len());

        for (i, parameter) in parameters.iter().enumerate() {
            let parameter = match parameter {
                Some(value) => value,
                None => {
                    debug!(
                        "{stmt}.{FUNCTION_NAME} error. parameter is None. {:?}",
                        parameters
                    );
                    stmt.add_diag(
                        TsurugiOdbcError::BindParameterError,
                        format!("parameter is not set. parameter_number={}", i + 1),
                    );
                    return Err(SqlReturn::SQL_ERROR);
                }
            };

            let placeholder = parameter.tg_placeholder();
            vec.push(placeholder);
        }

        Ok(vec)
    }

    pub(crate) fn parameters(
        &self,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<Vec<SqlParameter>, SqlReturn> {
        const FUNCTION_NAME: &str = "parameters()";

        let parameters = stmt.parameters();
        let mut vec = Vec::with_capacity(parameters.len());

        for (i, parameter) in parameters.iter().enumerate() {
            let parameter = match parameter {
                Some(value) => value,
                None => {
                    debug!(
                        "{stmt}.{FUNCTION_NAME} error. parameter is None. {:?}",
                        parameters
                    );
                    stmt.add_diag(
                        TsurugiOdbcError::BindParameterError,
                        format!("parameter is not set. parameter_number={}", i + 1),
                    );
                    return Err(SqlReturn::SQL_ERROR);
                }
            };

            let parameter = parameter.tg_parameter(stmt)?;
            vec.push(parameter);
        }

        Ok(vec)
    }
}

impl TsurugiOdbcPrepare {
    pub(crate) fn close(&mut self, stmt: &TsurugiOdbcStmt) -> SqlReturn {
        const FUNCTION_NAME: &str = "ps_close()";

        let ps = self.prepared_statement.take();
        if let Some(ps) = ps {
            let runtime = stmt.runtime();
            match runtime.block_on(ps.close()) {
                Ok(_) => {
                    debug!("{stmt}.{FUNCTION_NAME}: prepared_statement.close() succeeded");
                    SqlReturn::SQL_SUCCESS
                }
                Err(e) => {
                    warn!(
                        "{stmt}.{FUNCTION_NAME}: prepared_statement.close() error. {:?}",
                        e
                    );
                    stmt.add_diag(
                        TsurugiOdbcError::PreparedStatementCloseError,
                        format!("prepared_statement close error. {}", e),
                    );
                    SqlReturn::SQL_SUCCESS_WITH_INFO
                }
            }
        } else {
            SqlReturn::SQL_SUCCESS
        }
    }
}
