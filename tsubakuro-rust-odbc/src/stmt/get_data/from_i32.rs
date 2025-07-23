use super::*;

pub(crate) fn get_data_i32(
    stmt: &TsurugiOdbcStmt,
    arg: &TsurugiOdbcGetDataArguments,
    value: i32,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_i32()";

    if let Err(rc) = check_target_value_ptr(FUNCTION_NAME, stmt, arg) {
        return rc;
    }

    use CDataType::*;
    let target_type = arg.target_type;
    match target_type {
        SQL_C_BIT => write_bool(arg, value != 0),
        SQL_C_UTINYINT => write_u8(arg, value as u8),
        SQL_C_STINYINT | SQL_C_TINYINT => write_i8(arg, value as i8),
        SQL_C_USHORT => write_u16(arg, value as u16),
        SQL_C_SSHORT | SQL_C_SHORT => write_i16(arg, value as i16),
        SQL_C_ULONG => write_u32(arg, value as u32),
        SQL_C_SLONG | SQL_C_LONG => write_i32(arg, value),
        SQL_C_UBIGINT => write_u64(arg, value as u64),
        SQL_C_SBIGINT => write_i64(arg, value as i64),
        SQL_C_FLOAT => write_f32(arg, value as f32),
        SQL_C_DOUBLE => write_f64(arg, value as f64),
        SQL_C_NUMERIC => write_numeric_i128(arg, value as i128),
        SQL_C_CHAR | SQL_C_WCHAR => {
            let value = value.to_string();
            do_get_data_string(stmt, arg, value)
        }
        _ => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported target type {:?}",
                target_type
            );
            stmt.add_diag(
                TsurugiOdbcError::GetDataUnsupportedTargetType,
                format!(
                    "{ODBC_FUNCTION_NAME}: Unsupported target type {:?} from int",
                    target_type
                ),
            );
            SqlReturn::SQL_ERROR
        }
    }
}

pub(crate) fn get_data_i32_opt(
    stmt: &TsurugiOdbcStmt,
    arg: &TsurugiOdbcGetDataArguments,
    value: Option<i32>,
) -> SqlReturn {
    match value {
        Some(value) => get_data_i32(stmt, arg, value),
        None => get_data_null(stmt, arg),
    }
}
