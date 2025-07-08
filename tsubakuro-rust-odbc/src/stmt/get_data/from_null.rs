use super::*;

pub(crate) fn get_data_null(stmt: &TsurugiOdbcStmt, str_len_or_ind_ptr: *mut SqlLen) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_null()";

    if str_len_or_ind_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. str_len_or_ind_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::GetDataInvalidStrLenOrIndPtr,
            "SQLGetData.str_len_or_ind_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    unsafe {
        *str_len_or_ind_ptr = SQL_NULL_DATA;
    }
    SqlReturn::SQL_SUCCESS
}
