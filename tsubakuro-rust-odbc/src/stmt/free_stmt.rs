use log::{debug, trace};

use crate::{
    check_stmt,
    ctype::{SqlReturn, SqlUSmallInt},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{free_handle_stmt, HStmt},
    },
};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum FreeStmtOption {
    SQL_CLOSE = 0,
    SQL_DROP = 1,
    SQL_UNBIND = 2,
    SQL_RESET_PARAMS = 3,
}

impl TryFrom<u16> for FreeStmtOption {
    type Error = TsurugiOdbcError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use FreeStmtOption::*;
        match value {
            0 => Ok(SQL_CLOSE),
            1 => Ok(SQL_DROP),
            2 => Ok(SQL_UNBIND),
            3 => Ok(SQL_RESET_PARAMS),
            _ => Err(TsurugiOdbcError::UnsupportedFreeStmtOption),
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLFreeStmt(hstmt: HStmt, option: SqlUSmallInt) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLFreeStmt()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, option={:?}",
        hstmt,
        option
    );

    let rc = free_stmt(hstmt, option);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn free_stmt(hstmt: HStmt, option: SqlUSmallInt) -> SqlReturn {
    const FUNCTION_NAME: &str = "free_stmt()";

    let option = match FreeStmtOption::try_from(option) {
        Ok(value) => value,
        Err(e) => {
            let stmt = check_stmt!(hstmt);
            let stmt = stmt.lock().unwrap();
            stmt.clear_diag();

            debug!(
                "{stmt}.{FUNCTION_NAME} error: Unsupported option {}",
                option
            );
            stmt.add_diag(e, format!("SQLFreeStmt(): Unsupported option {}", option));
            return SqlReturn::SQL_ERROR;
        }
    };

    if option == FreeStmtOption::SQL_DROP {
        return free_handle_stmt(hstmt);
    }

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    debug!("{stmt}.{FUNCTION_NAME}: option={:?}", option);

    match option {
        FreeStmtOption::SQL_CLOSE => stmt.close_processor(), // FIXME QueryResultのときのみクローズすべきか？
        FreeStmtOption::SQL_DROP => unreachable!(),
        FreeStmtOption::SQL_UNBIND => stmt.clear_bind_columns(),
        FreeStmtOption::SQL_RESET_PARAMS => stmt.clear_parameters(),
    }
}
