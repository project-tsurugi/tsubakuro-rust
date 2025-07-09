use log::{debug, trace};

use crate::{
    check_stmt,
    ctype::SqlReturn,
    handle::hstmt::HStmt,
    stmt::get_data::{do_get_data, GetDataArguments},
};

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
    debug!("{stmt}.{FUNCTION_NAME}: fetch={:?}", rc);

    let rc1 = if stmt.has_bind_columns() && rc.is_success() {
        let mut rc = SqlReturn::SQL_SUCCESS;
        let bind_columns = stmt.bind_columns();
        for bind_column in bind_columns.iter().flatten() {
            let arg = GetDataArguments::new(
                bind_column.column_number(),
                bind_column.target_type(),
                bind_column.target_value_ptr(),
                bind_column.buffer_length(),
                bind_column.str_len_or_ind_ptr(),
            );
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
