use crate::ctype::{sql_date_struct::SqlDateStruct, sql_timestamp_struct::SqlTimestampStruct};

use super::*;

pub(crate) fn get_data_timestamp_tz(
    stmt: &TsurugiOdbcStmt,
    arg: GetDataArguments,
    value: time::OffsetDateTime,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_timestamp_tz()";

    if let Err(rc) = check_target_value_ptr(FUNCTION_NAME, stmt, &arg) {
        return rc;
    }

    use CDataType::*;
    let target_type = arg.target_type;
    match target_type {
        SQL_C_TYPE_DATE | SQL_C_DATE => {
            let value = SqlDateStruct::from(value.date());
            write_date_struct(arg, value)
        }
        SQL_C_TYPE_TIME | SQL_C_TIME => {
            let value = SqlTimeStruct::from(value.time());
            write_time_struct(arg, value)
        }
        SQL_C_TYPE_TIMESTAMP | SQL_C_TIMESTAMP => {
            let value = SqlTimestampStruct::from(value);
            write_timestamp_struct(arg, value)
        }
        SQL_C_CHAR | SQL_C_WCHAR => match timestamp_tz_to_string(stmt, value) {
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
                format!("Unsupported target type {:?}", target_type),
            );
            SqlReturn::SQL_ERROR
        }
    }
}

fn timestamp_tz_to_string(
    stmt: &TsurugiOdbcStmt,
    value: time::OffsetDateTime,
) -> Result<String, SqlReturn> {
    const FUNCTION_NAME: &str = "timestamp_tz_to_string()";

    let value = match do_timestamp_tz_to_string(value) {
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

fn do_timestamp_tz_to_string(
    value: time::OffsetDateTime,
) -> Result<String, Box<dyn std::error::Error>> {
    let format = time::macros::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]"
    );
    Ok(value.format(format)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_string() {
        {
            let date = time::Date::from_calendar_date(2025, time::Month::December, 31).unwrap();
            let time = time::Time::from_hms_nano(12, 34, 56, 123456789).unwrap();
            let offset = time::UtcOffset::from_hms(0, 0, 0).unwrap();
            let src = time::OffsetDateTime::new_in_offset(date, time, offset);
            let actual = do_timestamp_tz_to_string(src).unwrap();
            let expected = "2025-12-31 12:34:56.123456789+00:00";
            assert_eq!(expected, actual);
        }
        {
            let date = time::Date::from_calendar_date(2025, time::Month::December, 31).unwrap();
            let time = time::Time::from_hms_nano(12, 34, 56, 123456789).unwrap();
            let offset = time::UtcOffset::from_hms(9, 0, 0).unwrap();
            let src = time::OffsetDateTime::new_in_offset(date, time, offset);
            let actual = do_timestamp_tz_to_string(src).unwrap();
            let expected = "2025-12-31 12:34:56.123456789+09:00";
            assert_eq!(expected, actual);
        }
        {
            let date = time::Date::from_calendar_date(2025, time::Month::December, 31).unwrap();
            let time = time::Time::from_hms_milli(12, 34, 56, 780).unwrap();
            let offset = time::UtcOffset::from_hms(-9, 0, 0).unwrap();
            let src = time::OffsetDateTime::new_in_offset(date, time, offset);
            let actual = do_timestamp_tz_to_string(src).unwrap();
            let expected = "2025-12-31 12:34:56.78-09:00";
            assert_eq!(expected, actual);
        }
    }
}
