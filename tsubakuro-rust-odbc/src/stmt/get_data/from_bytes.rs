use super::*;

pub(crate) fn get_data_bytes(
    stmt: &TsurugiOdbcStmt,
    arg: &TsurugiOdbcGetDataArguments,
    value: &[u8],
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_bytes()";

    if let Err(rc) = check_target_value_ptr(FUNCTION_NAME, stmt, arg) {
        return rc;
    }

    use CDataType::*;
    let target_type = arg.target_type;
    match target_type {
        SQL_C_BINARY => write_bytes(stmt, arg, value),
        _ => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported target type {:?}",
                target_type
            );
            stmt.add_diag(
                TsurugiOdbcError::GetDataUnsupportedTargetType,
                format!("Unsupported target type {:?}", target_type),
            );
            SqlReturn::SQL_ERROR
        }
    }
}
