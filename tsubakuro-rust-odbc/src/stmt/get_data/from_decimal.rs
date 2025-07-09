use super::*;

pub(crate) fn get_data_decimal(
    stmt: &TsurugiOdbcStmt,
    arg: GetDataArguments,
    value: TgDecimalResult,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_decimal()";

    if let Err(rc) = check_target_value_ptr(FUNCTION_NAME, stmt, &arg) {
        return rc;
    }

    use CDataType::*;
    let target_type = arg.target_type;
    match target_type {
        SQL_C_BIT => match decimal_to_i128(stmt, value) {
            Ok(value) => write_bool(arg, value != 0),
            Err(rc) => rc,
        },
        SQL_C_UTINYINT => match decimal_to_i128(stmt, value) {
            Ok(value) => write_u8(arg, value as u8),
            Err(rc) => rc,
        },
        SQL_C_STINYINT | SQL_C_TINYINT => match decimal_to_i128(stmt, value) {
            Ok(value) => write_i8(arg, value as i8),
            Err(rc) => rc,
        },
        SQL_C_USHORT => match decimal_to_i128(stmt, value) {
            Ok(value) => write_u16(arg, value as u16),
            Err(rc) => rc,
        },
        SQL_C_SSHORT | SQL_C_SHORT => match decimal_to_i128(stmt, value) {
            Ok(value) => write_i16(arg, value as i16),
            Err(rc) => rc,
        },
        SQL_C_ULONG => match decimal_to_i128(stmt, value) {
            Ok(value) => write_u32(arg, value as u32),
            Err(rc) => rc,
        },
        SQL_C_SLONG | SQL_C_LONG => match decimal_to_i128(stmt, value) {
            Ok(value) => write_i32(arg, value as i32),
            Err(rc) => rc,
        },
        SQL_C_UBIGINT => match decimal_to_i128(stmt, value) {
            Ok(value) => write_u64(arg, value as u64),
            Err(rc) => rc,
        },
        SQL_C_SBIGINT => match decimal_to_i128(stmt, value) {
            Ok(value) => write_i64(arg, value as i64),
            Err(rc) => rc,
        },
        SQL_C_FLOAT => match decimal_to_f32(stmt, value) {
            Ok(value) => write_f32(arg, value),
            Err(rc) => rc,
        },
        SQL_C_DOUBLE => match decimal_to_f64(stmt, value) {
            Ok(value) => write_f64(arg, value),
            Err(rc) => rc,
        },
        SQL_C_NUMERIC => {
            let value = SqlNumericStruct::from(value);
            write_numeric_struct(arg, value)
        }
        SQL_C_CHAR | SQL_C_WCHAR => match decimal_to_string(stmt, value) {
            Ok(value) => do_get_data_string(stmt, arg, &value),
            Err(rc) => rc,
        },
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

fn decimal_to_i128(stmt: &TsurugiOdbcStmt, value: TgDecimalResult) -> Result<i128, SqlReturn> {
    const FUNCTION_NAME: &str = "decimal_to_i128()";

    let value = match TgDecimalI128::try_from(value) {
        Ok(value) => value,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME} error. convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ConvertError,
                format!("convert error. {}", e),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };

    let value = if value.exponent == 0 {
        value.unscaled_value
    } else if value.exponent > 0 {
        value.unscaled_value * 10_i128.pow(value.exponent as u32)
    } else {
        value.unscaled_value / 10_i128.pow(-value.exponent as u32)
    };

    Ok(value)
}

fn decimal_to_f32(stmt: &TsurugiOdbcStmt, value: TgDecimalResult) -> Result<f32, SqlReturn> {
    const FUNCTION_NAME: &str = "decimal_to_f32()";

    let value = match TgDecimalI128::try_from(value) {
        Ok(value) => value,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ConvertError,
                format!("convert error. {}", e),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };

    let value = if value.exponent == 0 {
        value.unscaled_value as f32
    } else {
        value.unscaled_value as f32 * 10_f32.powi(value.exponent)
    };

    Ok(value)
}

fn decimal_to_f64(stmt: &TsurugiOdbcStmt, value: TgDecimalResult) -> Result<f64, SqlReturn> {
    const FUNCTION_NAME: &str = "decimal_to_f64()";

    let value = match TgDecimalI128::try_from(value) {
        Ok(value) => value,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ConvertError,
                format!("convert error. {}", e),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };

    let value = if value.exponent == 0 {
        value.unscaled_value as f64
    } else {
        value.unscaled_value as f64 * 10_f64.powi(value.exponent)
    };

    Ok(value)
}

fn decimal_to_string(stmt: &TsurugiOdbcStmt, value: TgDecimalResult) -> Result<String, SqlReturn> {
    const FUNCTION_NAME: &str = "decimal_to_string()";

    let value = match TgDecimalI128::try_from(value) {
        Ok(value) => value,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ConvertError,
                format!("convert error. {}", e),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };

    let value = value.to_string();

    Ok(value)
}
