use super::*;

pub(crate) fn get_data_f32(
    stmt: &TsurugiOdbcStmt,
    value: f32,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_f32()";

    if target_value_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. target_value_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::InvalidValuePtr,
            "target_value_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    match target_type {
        CDataType::SQL_C_BIT => write_bool(value != 0f32, target_value_ptr, str_len_or_ind_ptr),
        CDataType::SQL_C_UTINYINT => write_u8(value as u8, target_value_ptr, str_len_or_ind_ptr),
        CDataType::SQL_C_STINYINT | CDataType::SQL_C_TINYINT => {
            write_i8(value as i8, target_value_ptr, str_len_or_ind_ptr)
        }
        CDataType::SQL_C_USHORT => write_u16(value as u16, target_value_ptr, str_len_or_ind_ptr),
        CDataType::SQL_C_SSHORT | CDataType::SQL_C_SHORT => {
            write_i16(value as i16, target_value_ptr, str_len_or_ind_ptr)
        }
        CDataType::SQL_C_ULONG => write_u32(value as u32, target_value_ptr, str_len_or_ind_ptr),
        CDataType::SQL_C_SLONG | CDataType::SQL_C_LONG => {
            write_i32(value as i32, target_value_ptr, str_len_or_ind_ptr)
        }
        CDataType::SQL_C_UBIGINT => write_u64(value as u64, target_value_ptr, str_len_or_ind_ptr),
        CDataType::SQL_C_SBIGINT => write_i64(value as i64, target_value_ptr, str_len_or_ind_ptr),
        CDataType::SQL_C_FLOAT => write_f32(value as f32, target_value_ptr, str_len_or_ind_ptr),
        CDataType::SQL_C_DOUBLE => write_f64(value as f64, target_value_ptr, str_len_or_ind_ptr),
        CDataType::SQL_C_NUMERIC => match f32_to_decimal(stmt, value) {
            Ok(value) => write_numeric_struct(value, target_value_ptr, str_len_or_ind_ptr),
            Err(rc) => rc,
        },
        CDataType::SQL_C_CHAR | CDataType::SQL_C_WCHAR => {
            let value = f32_to_string(value);
            do_get_data_string(
                stmt,
                &value,
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            )
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

fn f32_to_decimal(stmt: &TsurugiOdbcStmt, value: f32) -> Result<SqlNumericStruct, SqlReturn> {
    const FUNCTION_NAME: &str = "f32_to_decimal()";

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

pub(crate) fn f32_to_string(value: f32) -> String {
    let mut buffer = dtoa::Buffer::new();
    buffer.format(value).into()
}
