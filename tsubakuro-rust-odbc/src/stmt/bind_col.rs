use log::{debug, trace};

use crate::{
    check_stmt,
    ctype::{CDataType, SqlLen, SqlPointer, SqlReturn, SqlSmallInt, SqlUSmallInt},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::get_data::TsurugiOdbcGetDataArguments,
};

#[no_mangle]
pub extern "system" fn SQLBindCol(
    hstmt: HStmt,
    column_number: SqlUSmallInt,
    target_type: SqlSmallInt,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLBindCol()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, column_number={:?}, target_type={:?}, target_value_ptr={:?}, buffer_length={:?}, str_len_or_ind_ptr={:?}",
        hstmt,
        column_number,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = bind_col(
        &mut stmt,
        column_number,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn bind_col(
    stmt: &mut TsurugiOdbcStmt,
    column_number: SqlUSmallInt,
    target_type: SqlSmallInt,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLBindCol()";

    let target_type = match CDataType::try_from(target_type) {
        Ok(value) => value,
        Err(target_type) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported CDataType {:?}",
                target_type
            );
            let odbc_function_name = "SQLBindCol()";
            stmt.add_diag(
                TsurugiOdbcError::BindColUnsupportedTargetType,
                format!(
                    "{odbc_function_name}: Unsupported target_type {:?}",
                    target_type
                ),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    if str_len_or_ind_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. str_len_or_ind_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::BindColInvalidStrLenOrIndPtr,
            "str_len_or_ind_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    };

    let bind_column = TsurugiOdbcGetDataArguments::new(
        column_number,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    );

    stmt.set_bind_column(bind_column);

    SqlReturn::SQL_SUCCESS
}
