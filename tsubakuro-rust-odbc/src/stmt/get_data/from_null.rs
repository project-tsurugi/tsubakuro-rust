use super::*;

pub(crate) fn get_data_null(
    stmt: &TsurugiOdbcStmt,
    arg: &TsurugiOdbcGetDataArguments,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_null()";

    let str_len_or_ind_ptr = arg.str_len_or_ind_ptr;
    if str_len_or_ind_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. str_len_or_ind_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::GetDataInvalidStrLenOrIndPtr,
            format!("{ODBC_FUNCTION_NAME}: str_len_or_ind_ptr is null"),
        );
        return SqlReturn::SQL_ERROR;
    }

    unsafe {
        *str_len_or_ind_ptr = SQL_NULL_DATA;
    }
    SqlReturn::SQL_SUCCESS
}
