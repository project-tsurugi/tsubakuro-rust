use log::{debug, trace};

use crate::{check_stmt, ctype::SqlReturn, handle::hstmt::HStmt, stmt::get_data::do_get_data};

#[no_mangle]
pub extern "system" fn SQLFetch(hstmt: HStmt) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLFetch()";
    trace!("{FUNCTION_NAME} start. hstmt={:?}", hstmt);

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = {
        let processor = match stmt.processor(FUNCTION_NAME) {
            Ok(processor) => processor,
            Err(rc) => return rc,
        };
        let mut processor = processor.borrow_mut();

        processor.fetch(&mut stmt)
    };

    let rows_fetched = if rc.is_success() { 1 } else { 0 };
    stmt.set_rows_fetched(rows_fetched);

    debug!(
        "{stmt}.{FUNCTION_NAME}: fetch={:?}, rows_fetched={}",
        rc, rows_fetched
    );

    let rc1 = if stmt.has_bind_columns() && rc.is_success() {
        let mut rc = SqlReturn::SQL_SUCCESS;
        let bind_columns = stmt.bind_columns();
        for arg in bind_columns.iter().flatten() {
            let rc1 = do_get_data(&stmt, arg);
            rc = rc.or(rc1);
        }
        rc
    } else {
        SqlReturn::SQL_SUCCESS
    };

    let rc = rc.or(rc1);
    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}
