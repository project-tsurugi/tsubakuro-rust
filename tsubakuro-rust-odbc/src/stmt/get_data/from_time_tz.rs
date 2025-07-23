use super::*;

pub(crate) fn get_data_time_tz(
    stmt: &TsurugiOdbcStmt,
    arg: &TsurugiOdbcGetDataArguments,
    value: (time::Time, time::UtcOffset),
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_time_tz()";

    if let Err(rc) = check_target_value_ptr(FUNCTION_NAME, stmt, arg) {
        return rc;
    }

    use CDataType::*;
    let target_type = arg.target_type;
    match target_type {
        SQL_C_TYPE_TIME | SQL_C_TIME => {
            let value = SqlTimeStruct::from(value.0);
            write_time_struct(arg, value)
        }
        SQL_C_CHAR | SQL_C_WCHAR => match time_tz_to_string(stmt, value) {
            Ok(value) => do_get_data_string(stmt, arg, value),
            Err(rc) => rc,
        },
        _ => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported target type {:?}",
                target_type
            );
            stmt.add_diag(
                TsurugiOdbcError::GetDataUnsupportedTargetType,
                format!(
                    "{ODBC_FUNCTION_NAME}: Unsupported target type {:?} from time_tz",
                    target_type
                ),
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
                TsurugiOdbcError::GetDataConvertTimeError,
                format!("{ODBC_FUNCTION_NAME}: time to string convert error. {}", e),
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
