use crate::ctype::{sql_date_struct::SqlDateStruct, sql_timestamp_struct::SqlTimestampStruct};

use super::*;

pub(crate) fn get_data_timestamp_tz(
    stmt: &TsurugiOdbcStmt,
    value: time::OffsetDateTime,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_timestamp_tz()";

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
        SQL_C_TYPE_DATE | SQL_C_DATE => {
            let value = SqlDateStruct::from(value.date());
            write_date_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        SQL_C_TYPE_TIME | SQL_C_TIME => {
            let value = SqlTimeStruct::from(value.time());
            write_time_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        SQL_C_TYPE_TIMESTAMP | SQL_C_TIMESTAMP => {
            let value = SqlTimestampStruct::from(value);
            write_timestamp_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        SQL_C_CHAR | SQL_C_WCHAR => match timestamp_tz_to_string(stmt, value) {
            Ok(value) => get_data_string(
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
