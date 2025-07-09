use super::*;

pub(crate) fn get_data_f64(stmt: &TsurugiOdbcStmt, arg: GetDataArguments, value: f64) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_f64()";

    if let Err(rc) = check_target_value_ptr(FUNCTION_NAME, stmt, &arg) {
        return rc;
    }

    use CDataType::*;
    let target_type = arg.target_type;
    match target_type {
        SQL_C_BIT => write_bool(arg, value != 0f64),
        SQL_C_UTINYINT => write_u8(arg, value as u8),
        SQL_C_STINYINT | SQL_C_TINYINT => write_i8(arg, value as i8),
        SQL_C_USHORT => write_u16(arg, value as u16),
        SQL_C_SSHORT | SQL_C_SHORT => write_i16(arg, value as i16),
        SQL_C_ULONG => write_u32(arg, value as u32),
        SQL_C_SLONG | SQL_C_LONG => write_i32(arg, value as i32),
        SQL_C_UBIGINT => write_u64(arg, value as u64),
        SQL_C_SBIGINT => write_i64(arg, value as i64),
        SQL_C_FLOAT => write_f32(arg, value as f32),
        SQL_C_DOUBLE => write_f64(arg, value),
        SQL_C_NUMERIC => match f64_to_decimal(stmt, value) {
            Ok(value) => write_numeric_struct(arg, value),
            Err(rc) => rc,
        },
        SQL_C_CHAR | SQL_C_WCHAR => {
            let value = f64_to_string(value);
            do_get_data_string(stmt, arg, value)
        }
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

fn f64_to_decimal(stmt: &TsurugiOdbcStmt, value: f64) -> Result<SqlNumericStruct, SqlReturn> {
    const FUNCTION_NAME: &str = "f64_to_decimal()";

    match SqlNumericStruct::try_from(value) {
        Ok(value) => Ok(value),
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME}: convert error {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::DecimalError,
                format!("convert error. {}", e),
            );
            Err(SqlReturn::SQL_ERROR)
        }
    }
}

pub(crate) fn f64_to_string(value: f64) -> String {
    let mut buffer = dtoa::Buffer::new();
    buffer.format(value).into()
}
