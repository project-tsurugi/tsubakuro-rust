use super::*;

pub(crate) fn get_data_string(
    stmt: &TsurugiOdbcStmt,
    value: &str,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_string()";

    if target_value_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. target_value_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::InvalidValuePtr,
            "target_value_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    use CDataType::*;
    match target_type {
        SQL_C_BIT => match str_to_bool(stmt, value) {
            Ok(v) => write_bool(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_UTINYINT => match str_to_u8(stmt, value) {
            Ok(v) => write_u8(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_STINYINT | SQL_C_TINYINT => match str_to_i8(stmt, value) {
            Ok(v) => write_i8(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_USHORT => match str_to_u16(stmt, value) {
            Ok(v) => write_u16(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_SSHORT | SQL_C_SHORT => match str_to_i16(stmt, value) {
            Ok(v) => write_i16(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_ULONG => match str_to_u32(stmt, value) {
            Ok(v) => write_u32(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_SLONG | SQL_C_LONG => match str_to_i32(stmt, value) {
            Ok(v) => write_i32(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_UBIGINT => match str_to_u64(stmt, value) {
            Ok(v) => write_u64(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_SBIGINT => match str_to_i64(stmt, value) {
            Ok(v) => write_i64(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_FLOAT => match str_to_f32(stmt, value) {
            Ok(v) => write_f32(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_DOUBLE => match str_to_f64(stmt, value) {
            Ok(v) => write_f64(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_NUMERIC => match str_to_numeric_struct(stmt, value) {
            Ok(v) => write_numeric_struct(v, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        SQL_C_CHAR | SQL_C_WCHAR => do_get_data_string(
            stmt,
            value,
            target_type,
            target_value_ptr,
            buffer_length,
            str_len_or_ind_ptr,
        ),
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

pub(crate) fn do_get_data_string(
    stmt: &TsurugiOdbcStmt,
    value: &str,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    use CDataType::*;
    match target_type {
        SQL_C_CHAR => write_char_len(
            value,
            target_value_ptr as *mut SqlChar,
            buffer_length,
            str_len_or_ind_ptr,
            &stmt.diag_collection(),
        ),
        SQL_C_WCHAR => write_wchar_len(
            value,
            target_value_ptr as *mut SqlWChar,
            buffer_length,
            str_len_or_ind_ptr,
            &stmt.diag_collection(),
        ),
        _ => unreachable!(),
    }
}

pub(crate) fn get_data_string_opt<S: AsRef<str>>(
    stmt: &TsurugiOdbcStmt,
    value: Option<S>,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    match value {
        Some(value) => get_data_string(
            stmt,
            value.as_ref(),
            target_type,
            target_value_ptr,
            buffer_length,
            str_len_or_ind_ptr,
        ),
        None => get_data_null(stmt, str_len_or_ind_ptr),
    }
}

pub(crate) fn str_to_bool(stmt: &TsurugiOdbcStmt, value: &str) -> Result<bool, SqlReturn> {
    const FUNCTION_NAME: &str = "str_to_bool()";

    let value = match value.parse::<bool>() {
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
    Ok(value)
}

fn str_to_i8(stmt: &TsurugiOdbcStmt, value: &str) -> Result<i8, SqlReturn> {
    let value = str_to_i128(stmt, value)?;
    Ok(value as i8)
}

fn str_to_u8(stmt: &TsurugiOdbcStmt, value: &str) -> Result<u8, SqlReturn> {
    let value = str_to_i128(stmt, value)?;
    Ok(value as u8)
}

fn str_to_i16(stmt: &TsurugiOdbcStmt, value: &str) -> Result<i16, SqlReturn> {
    let value = str_to_i128(stmt, value)?;
    Ok(value as i16)
}

fn str_to_u16(stmt: &TsurugiOdbcStmt, value: &str) -> Result<u16, SqlReturn> {
    let value = str_to_i128(stmt, value)?;
    Ok(value as u16)
}

pub(crate) fn str_to_i32(stmt: &TsurugiOdbcStmt, value: &str) -> Result<i32, SqlReturn> {
    let value = str_to_i128(stmt, value)?;
    Ok(value as i32)
}

fn str_to_u32(stmt: &TsurugiOdbcStmt, value: &str) -> Result<u32, SqlReturn> {
    let value = str_to_i128(stmt, value)?;
    Ok(value as u32)
}

pub(crate) fn str_to_i64(stmt: &TsurugiOdbcStmt, value: &str) -> Result<i64, SqlReturn> {
    let value = str_to_i128(stmt, value)?;
    Ok(value as i64)
}

fn str_to_u64(stmt: &TsurugiOdbcStmt, value: &str) -> Result<u64, SqlReturn> {
    let value = str_to_i128(stmt, value)?;
    Ok(value as u64)
}

fn str_to_i128(stmt: &TsurugiOdbcStmt, value: &str) -> Result<i128, SqlReturn> {
    const FUNCTION_NAME: &str = "str_to_i128()";

    let value = match value.parse::<i128>() {
        Ok(value) => value,
        Err(e) => match str_to_f64(stmt, value) {
            Ok(value) => {
                if value.is_finite() {
                    value as i128
                } else {
                    debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
                    stmt.add_diag(
                        TsurugiOdbcError::ConvertError,
                        format!("convert error. {}", e),
                    );
                    return Err(SqlReturn::SQL_ERROR);
                }
            }
            Err(_) => {
                debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
                stmt.add_diag(
                    TsurugiOdbcError::ConvertError,
                    format!("convert error. {}", e),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        },
    };
    Ok(value)
}

pub(crate) fn str_to_f32(stmt: &TsurugiOdbcStmt, value: &str) -> Result<f32, SqlReturn> {
    const FUNCTION_NAME: &str = "str_to_f32()";

    let ret = match value.parse::<f32>() {
        Ok(v) => v,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ConvertError,
                format!("convert error. {}", e),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };
    Ok(ret)
}

pub(crate) fn str_to_f64(stmt: &TsurugiOdbcStmt, value: &str) -> Result<f64, SqlReturn> {
    const FUNCTION_NAME: &str = "str_to_f64()";

    let ret = match value.parse::<f64>() {
        Ok(v) => v,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ConvertError,
                format!("convert error. {}", e),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };
    Ok(ret)
}

fn str_to_numeric_struct(
    stmt: &TsurugiOdbcStmt,
    value: &str,
) -> Result<SqlNumericStruct, SqlReturn> {
    const FUNCTION_NAME: &str = "str_to_numeric_struct()";

    let ret = match SqlNumericStruct::try_from(value) {
        Ok(v) => v,
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ConvertError,
                format!("convert error. {}", e),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };
    Ok(ret)
}
