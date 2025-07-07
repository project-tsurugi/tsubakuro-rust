use super::*;

pub(crate) fn get_data_decimal(
    stmt: &TsurugiOdbcStmt,
    value: TgDecimalResult,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_decimal()";

    if target_value_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. target_value_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::InvalidValuePtr,
            "target_value_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    match target_type {
        CDataType::SQL_C_BIT => match decimal_to_i128(stmt, value) {
            Ok(v) => write_bool(v != 0, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_UTINYINT => match decimal_to_i128(stmt, value) {
            Ok(v) => write_u8(v as u8, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_STINYINT | CDataType::SQL_C_TINYINT => {
            match decimal_to_i128(stmt, value) {
                Ok(v) => write_i8(v as i8, target_value_ptr, str_len_or_ind_ptr),
                Err(rc) => rc,
            }
        }
        CDataType::SQL_C_USHORT => match decimal_to_i128(stmt, value) {
            Ok(v) => write_u16(v as u16, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_SSHORT | CDataType::SQL_C_SHORT => match decimal_to_i128(stmt, value) {
            Ok(v) => write_i16(v as i16, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_ULONG => match decimal_to_i128(stmt, value) {
            Ok(v) => write_u32(v as u32, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_SLONG | CDataType::SQL_C_LONG => match decimal_to_i128(stmt, value) {
            Ok(v) => write_i32(v as i32, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_UBIGINT => match decimal_to_i128(stmt, value) {
            Ok(v) => write_u64(v as u64, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_SBIGINT => match decimal_to_i128(stmt, value) {
            Ok(v) => write_i64(v as i64, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_FLOAT => match decimal_to_f32(stmt, value) {
            Ok(v) => write_f32(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_DOUBLE => match decimal_to_f64(stmt, value) {
            Ok(v) => write_f64(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_NUMERIC => {
            let v = SqlNumericStruct::from(value);
            write_numeric_struct(v, target_value_ptr, str_len_or_ind_ptr)
        }
        CDataType::SQL_C_CHAR | CDataType::SQL_C_WCHAR => match decimal_to_string(stmt, value) {
            Ok(value) => do_get_data_string(
                stmt,
                &value,
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ),
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
    } else {
        if value.exponent > 0 {
            value.unscaled_value * 10_i128.pow(value.exponent as u32)
        } else {
            value.unscaled_value / 10_i128.pow(-value.exponent as u32)
        }
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
