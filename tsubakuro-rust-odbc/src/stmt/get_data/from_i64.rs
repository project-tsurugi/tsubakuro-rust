use super::*;

pub(crate) fn get_data_i64(
    stmt: &TsurugiOdbcStmt,
    value: i64,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_i64()";

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
        SQL_C_BIT => write_bool(value != 0, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_UTINYINT => write_u8(value as u8, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_STINYINT | SQL_C_TINYINT => {
            write_i8(value as i8, target_value_ptr, str_len_or_ind_ptr)
        }
        SQL_C_USHORT => write_u16(value as u16, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_SSHORT | SQL_C_SHORT => write_i16(value as i16, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_ULONG => write_u32(value as u32, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_SLONG | SQL_C_LONG => write_i32(value as i32, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_UBIGINT => write_u64(value as u64, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_SBIGINT => write_i64(value, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_FLOAT => write_f32(value as f32, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_DOUBLE => write_f64(value as f64, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_NUMERIC => write_numeric_i128(value as i128, target_value_ptr, str_len_or_ind_ptr),
        SQL_C_CHAR | SQL_C_WCHAR => {
            let value = value.to_string();
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
