use super::*;

pub(crate) fn get_data_time_tz(
    stmt: &TsurugiOdbcStmt,
    value: (time::Time, time::UtcOffset),
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_time_tz()";

    if target_value_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. target_value_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::InvalidValuePtr,
            "target_value_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    match target_type {
        CDataType::SQL_C_TYPE_TIME | CDataType::SQL_C_TIME => {
            let value = SqlTimeStruct::from(value.0);
            write_time_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        CDataType::SQL_C_CHAR | CDataType::SQL_C_WCHAR => match time_tz_to_string(stmt, value) {
            Ok(value) => get_data_string(
                stmt,
                &value,
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ),
            Err(rc) => return rc,
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

fn time_tz_to_string(
    stmt: &TsurugiOdbcStmt,
    value: (time::Time, time::UtcOffset),
) -> Result<String, SqlReturn> {
    const FUNCTION_NAME: &str = "time_tz_to_string()";

    let value = match do_time_tz_to_string(value) {
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

fn do_time_tz_to_string(
    value: (time::Time, time::UtcOffset),
) -> Result<String, Box<dyn std::error::Error>> {
    let time = do_time_to_string(value.0)?;
    let offset = {
        let format =
            time::macros::format_description!("[offset_hour sign:mandatory]:[offset_minute]");
        value.1.format(format)?
    };
    Ok(format!("{}{}", time, offset))
}
