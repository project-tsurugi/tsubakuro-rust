use log::{debug, trace};

use crate::{
    check_stmt,
    ctype::{SqlLen, SqlReturn},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
};

#[no_mangle]
pub extern "system" fn SQLRowCount(hstmt: HStmt, row_count_ptr: *mut SqlLen) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLRowCount()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, row_count_ptr={:?}",
        hstmt,
        row_count_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = row_count(&stmt, row_count_ptr);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn row_count(stmt: &TsurugiOdbcStmt, row_count_ptr: *mut SqlLen) -> SqlReturn {
    const FUNCTION_NAME: &str = "row_count()";

    let processor = match stmt.processor(FUNCTION_NAME) {
        Ok(processor) => processor,
        Err(rc) => return rc,
    };
    let processor = processor.borrow();

    let row_count = processor.row_count();
    debug!("{stmt}.{FUNCTION_NAME}: row_count={}", row_count);

    if row_count_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. row_count_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::InvalidArgumentPtr,
            "row_count_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    unsafe {
        *row_count_ptr = row_count;
    }

    SqlReturn::SQL_SUCCESS
}
