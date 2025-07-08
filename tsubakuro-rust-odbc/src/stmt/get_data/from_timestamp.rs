use crate::ctype::{sql_date_struct::SqlDateStruct, sql_timestamp_struct::SqlTimestampStruct};

use super::*;

pub(crate) fn get_data_timestamp(
    stmt: &TsurugiOdbcStmt,
    value: time::PrimitiveDateTime,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_timestamp()";

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
            let value = SqlDateStruct::from(value);
            write_date_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        SQL_C_TYPE_TIME | SQL_C_TIME => {
            let value = SqlTimeStruct::from(value);
            write_time_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        SQL_C_TYPE_TIMESTAMP | SQL_C_TIMESTAMP => {
            let value = SqlTimestampStruct::from(value);
            write_timestamp_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        SQL_C_CHAR | SQL_C_WCHAR => match timestamp_to_string(stmt, value) {
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

fn timestamp_to_string(
    stmt: &TsurugiOdbcStmt,
    value: time::PrimitiveDateTime,
) -> Result<String, SqlReturn> {
    const FUNCTION_NAME: &str = "timestamp_to_string()";

    let value = match do_timestamp_to_string(value) {
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

fn do_timestamp_to_string(
    value: time::PrimitiveDateTime,
) -> Result<String, Box<dyn std::error::Error>> {
    let format = time::macros::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
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
            let src = time::PrimitiveDateTime::new(date, time);
            let actual = do_timestamp_to_string(src).unwrap();
            let expected = "2025-12-31 12:34:56.123456789";
            assert_eq!(expected, actual);
        }
        {
            let date = time::Date::from_calendar_date(-2025, time::Month::April, 1).unwrap();
            let time = time::Time::from_hms_milli(1, 2, 3, 400).unwrap();
            let src = time::PrimitiveDateTime::new(date, time);
            let actual = do_timestamp_to_string(src).unwrap();
            let expected = "-2025-04-01 01:02:03.4";
            assert_eq!(expected, actual);
        }
        {
            let date = time::Date::from_calendar_date(2025, time::Month::April, 1).unwrap();
            let time = time::Time::from_hms(1, 2, 3).unwrap();
            let src = time::PrimitiveDateTime::new(date, time);
            let actual = do_timestamp_to_string(src).unwrap();
            let expected = "2025-04-01 01:02:03.0";
            assert_eq!(expected, actual);
        }
    }
}
