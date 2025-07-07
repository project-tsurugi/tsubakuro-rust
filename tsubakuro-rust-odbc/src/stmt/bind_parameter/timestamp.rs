use log::debug;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

use crate::{
    ctype::{
        sql_date_struct::SqlDateStruct, sql_timestamp_struct::SqlTimestampStruct, CDataType,
        SqlReturn,
    },
    handle::{diag::TsurugiOdbcError, hstmt::TsurugiOdbcStmt},
    stmt::bind_parameter::{
        date::do_string_to_date, time::do_string_to_time, TsurugiOdbcBindParameter,
    },
};

impl TsurugiOdbcBindParameter {
    pub(super) fn tg_parameter_timestamp(
        &self,
        name: String,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "sql_parameter_timestamp()";

        let value_type = self.value_type;
        let value_ptr = self.parameter_value_ptr;

        use CDataType::*;
        let value: time::PrimitiveDateTime = match value_type {
            SQL_C_CHAR => {
                let s = self.char_ptr_to_string(FUNCTION_NAME, stmt)?;
                string_to_timestamp(FUNCTION_NAME, stmt, &s)?
            }
            SQL_C_WCHAR => {
                let s = self.wchar_ptr_to_string(FUNCTION_NAME, stmt)?;
                string_to_timestamp(FUNCTION_NAME, stmt, &s)?
            }
            SQL_C_TYPE_DATE | SQL_C_DATE => unsafe {
                let ptr = value_ptr as *const SqlDateStruct;
                match time::PrimitiveDateTime::try_from(&*ptr) {
                    Ok(value) => value,
                    Err(e) => {
                        debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
                        stmt.add_diag(
                            TsurugiOdbcError::ConvertError,
                            format!("convert error, {}", e),
                        );
                        return Err(SqlReturn::SQL_ERROR);
                    }
                }
            },
            SQL_C_TYPE_TIMESTAMP | SQL_C_TIMESTAMP => unsafe {
                let ptr = value_ptr as *const SqlTimestampStruct;
                match time::PrimitiveDateTime::try_from(&*ptr) {
                    Ok(value) => value,
                    Err(e) => {
                        debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
                        stmt.add_diag(
                            TsurugiOdbcError::ConvertError,
                            format!("convert error, {}", e),
                        );
                        return Err(SqlReturn::SQL_ERROR);
                    }
                }
            },
            _ => {
                debug!(
                    "{stmt}.{FUNCTION_NAME} error. Unsupported value_type. {:?}",
                    self
                );
                stmt.add_diag(
                    TsurugiOdbcError::UnsupportedCDataType,
                    format!("Unsupported value_type {:?} for TIMESTAMP", value_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        let parameter = SqlParameter::of(&name, value);
        Ok(parameter)
    }
}

fn string_to_timestamp(
    function_name: &str,
    stmt: &TsurugiOdbcStmt,
    value: &str,
) -> Result<time::PrimitiveDateTime, SqlReturn> {
    match do_string_to_timestamp(value) {
        Ok(value) => Ok(value),
        Err(e) => {
            debug!("{stmt}.{function_name}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ConvertError,
                format!("convert error. {}", e),
            );
            Err(SqlReturn::SQL_ERROR)
        }
    }
}

fn do_string_to_timestamp(
    value: &str,
) -> Result<time::PrimitiveDateTime, Box<dyn std::error::Error>> {
    let value = if let Some((left, right)) = value.split_once(' ') {
        let date = do_string_to_date(left)?;
        let time = do_string_to_time(right)?;
        time::PrimitiveDateTime::new(date, time)
    } else if let Some((left, right)) = value.split_once('T') {
        let date = do_string_to_date(left)?;
        let time = do_string_to_time(right)?;
        time::PrimitiveDateTime::new(date, time)
    } else {
        let format = time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
        );
        time::PrimitiveDateTime::parse(value, format)?
    };
    Ok(value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        {
            let actual = do_string_to_timestamp("2025-07-03 12:34:56.123456789").unwrap();
            let date = time::Date::from_calendar_date(2025, time::Month::July, 3).unwrap();
            let time = time::Time::from_hms_nano(12, 34, 56, 123456789).unwrap();
            let expected = time::PrimitiveDateTime::new(date, time);
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_timestamp("2025-07-03 01:02:03.4").unwrap();
            let date = time::Date::from_calendar_date(2025, time::Month::July, 3).unwrap();
            let time = time::Time::from_hms_milli(1, 2, 3, 400).unwrap();
            let expected = time::PrimitiveDateTime::new(date, time);
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_timestamp("2025-07-03 01:02:03").unwrap();
            let date = time::Date::from_calendar_date(2025, time::Month::July, 3).unwrap();
            let time = time::Time::from_hms(1, 2, 3).unwrap();
            let expected = time::PrimitiveDateTime::new(date, time);
            assert_eq!(expected, actual);
        }

        {
            let actual = do_string_to_timestamp("2025-07-03T12:34:56.123456789").unwrap();
            let date = time::Date::from_calendar_date(2025, time::Month::July, 3).unwrap();
            let time = time::Time::from_hms_nano(12, 34, 56, 123456789).unwrap();
            let expected = time::PrimitiveDateTime::new(date, time);
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_timestamp("2025-07-03T01:02:03.4").unwrap();
            let date = time::Date::from_calendar_date(2025, time::Month::July, 3).unwrap();
            let time = time::Time::from_hms_milli(1, 2, 3, 400).unwrap();
            let expected = time::PrimitiveDateTime::new(date, time);
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_timestamp("2025-07-03T01:02:03").unwrap();
            let date = time::Date::from_calendar_date(2025, time::Month::July, 3).unwrap();
            let time = time::Time::from_hms(1, 2, 3).unwrap();
            let expected = time::PrimitiveDateTime::new(date, time);
            assert_eq!(expected, actual);
        }
    }
}
