use log::{debug, trace};

use crate::{
    check_stmt,
    ctype::{SqlReturn, SqlSmallInt},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
};

#[no_mangle]
pub extern "system" fn SQLNumResultCols(
    hstmt: HStmt,
    column_count_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLNumResultCols()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, column_count_ptr={:?}",
        hstmt,
        column_count_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = num_result_cols(&stmt, column_count_ptr);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn num_result_cols(stmt: &TsurugiOdbcStmt, column_count_ptr: *mut i16) -> SqlReturn {
    const FUNCTION_NAME: &str = "num_result_cols()";

    let processor = match stmt.processor(FUNCTION_NAME) {
        Ok(processor) => processor,
        Err(rc) => return rc,
    };
    let processor = processor.borrow();

    let count = processor.number_of_columns();
    debug!("{stmt}.{FUNCTION_NAME}: number_of_columns={}", count);

    if column_count_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME}: column_count_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::InvalidValuePtr,
            "column_count_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    unsafe {
        *column_count_ptr = count as SqlSmallInt;
    }

    SqlReturn::SQL_SUCCESS
}
