use log::{debug, trace};

use crate::{check_stmt, ctype::SqlReturn, handle::hstmt::HStmt};

#[no_mangle]
pub extern "system" fn SQLMoreResults(hstmt: HStmt) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLMoreResults()";
    trace!("{FUNCTION_NAME} start. hstmt={:?}", hstmt);

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = SqlReturn::SQL_NO_DATA;
    debug!("{stmt}.{FUNCTION_NAME}: returns {:?}", rc);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}
