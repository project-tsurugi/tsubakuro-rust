use log::debug;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

use crate::{
    ctype::{
        sql_time_struct::SqlTimeStruct, sql_timestamp_struct::SqlTimestampStruct, CDataType,
        SqlReturn,
    },
    handle::{diag::TsurugiOdbcError, hstmt::TsurugiOdbcStmt},
    stmt::bind_parameter::{timestamp_tz::do_string_to_timestamp_tz, TsurugiOdbcBindParameter},
};

impl TsurugiOdbcBindParameter {
    pub(super) fn tg_parameter_time(
        &self,
        name: String,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "sql_parameter_time()";

        let value_type = self.value_type;
        let value_ptr = self.parameter_value_ptr;

        use CDataType::*;
        let value: time::Time = match value_type {
            SQL_C_CHAR => {
                let s = self.char_ptr_to_string(FUNCTION_NAME, stmt)?;
                string_to_time(FUNCTION_NAME, stmt, &s)?
            }
            SQL_C_WCHAR => {
                let s = self.wchar_ptr_to_string(FUNCTION_NAME, stmt)?;
                string_to_time(FUNCTION_NAME, stmt, &s)?
            }
            SQL_C_TYPE_TIME | SQL_C_TIME => unsafe {
                let ptr = value_ptr as *const SqlTimeStruct;
                match time::Time::try_from(&*ptr) {
                    Ok(value) => value,
                    Err(e) => {
                        debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
                        stmt.add_diag(
                            TsurugiOdbcError::BindParameterConvertTimeError,
                            format!("SQL_TIME_STRUCT to time convert error, {}", e),
                        );
                        return Err(SqlReturn::SQL_ERROR);
                    }
                }
            },
            SQL_C_TYPE_TIMESTAMP | SQL_C_TIMESTAMP => unsafe {
                let ptr = value_ptr as *const SqlTimestampStruct;
                match time::Time::try_from(&*ptr) {
                    Ok(value) => value,
                    Err(e) => {
                        debug!("{stmt}.{FUNCTION_NAME}: convert error. {:?}", e);
                        stmt.add_diag(
                            TsurugiOdbcError::BindParameterConvertTimestampError,
                            format!("SQL_TIMESTAMP_STRUCT to time convert error, {}", e),
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
                    TsurugiOdbcError::BindParameterUnsupportedValueType,
                    format!("Unsupported value_type {:?} for TIME", value_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        let parameter = SqlParameter::of(&name, value);
        Ok(parameter)
    }
}

fn string_to_time(
    function_name: &str,
    stmt: &TsurugiOdbcStmt,
    value: &str,
) -> Result<time::Time, SqlReturn> {
    match do_string_to_timestamp_tz(value) {
        Ok(value) => Ok(value.time()),
        Err(e) => {
            debug!("{stmt}.{function_name}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::BindParameterConvertTimeError,
                format!("string to time convert error. {}", e),
            );
            Err(SqlReturn::SQL_ERROR)
        }
    }
}

pub(crate) fn do_string_to_time(value: &str) -> Result<time::Time, Box<dyn std::error::Error>> {
    let value = truncate_after_char(value, '+');
    let value = truncate_after_char(value, '-');

    let mut count = 0;
    let mut period = false;

    for c in value.chars() {
        if c == ':' {
            count += 1;
        } else if c == '.' {
            period = true;
            break;
        }
    }

    let format = if period {
        time::macros::format_description!("[hour]:[minute]:[second].[subsecond]")
    } else {
        #[allow(clippy::collapsible_else_if)]
        if count == 1 {
            time::macros::format_description!("[hour]:[minute]")
        } else {
            time::macros::format_description!("[hour]:[minute]:[second]")
        }
    };
    let value = time::Time::parse(value, format)?;
    Ok(value)
}

fn truncate_after_char(s: &str, c: char) -> &str {
    if let Some(pos) = s.find(c) {
        &s[..pos]
    } else {
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_truncate_after_char() {
        assert_eq!(truncate_after_char("abc+def", '+'), "abc");
        assert_eq!(truncate_after_char("abc-def", '+'), "abc-def");
        assert_eq!(truncate_after_char("abcdef", '+'), "abcdef");
    }

    #[test]
    fn parse() {
        {
            let actual = do_string_to_time("00:00").unwrap();
            let expected = time::Time::from_hms(0, 0, 0).unwrap();
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_time("01:02").unwrap();
            let expected = time::Time::from_hms(1, 2, 0).unwrap();
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_time("23:59").unwrap();
            let expected = time::Time::from_hms(23, 59, 0).unwrap();
            assert_eq!(expected, actual);
        }

        {
            let actual = do_string_to_time("00:00:00").unwrap();
            let expected = time::Time::from_hms(0, 0, 0).unwrap();
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_time("01:02:03").unwrap();
            let expected = time::Time::from_hms(1, 2, 3).unwrap();
            assert_eq!(expected, actual);
        }

        {
            let actual = do_string_to_time("00:00:00.0").unwrap();
            let expected = time::Time::from_hms(0, 0, 0).unwrap();
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_time("01:02:03.0").unwrap();
            let expected = time::Time::from_hms(1, 2, 3).unwrap();
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_time("01:02:03.4").unwrap();
            let expected = time::Time::from_hms_milli(1, 2, 3, 400).unwrap();
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_time("12:34:56.123456789").unwrap();
            let expected = time::Time::from_hms_nano(12, 34, 56, 123456789).unwrap();
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_time("23:59:59.999999999").unwrap();
            let expected = time::Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn parse_with_timezone() {
        {
            let actual = do_string_to_time("12:34:56.123456789+09:00").unwrap();
            let expected = time::Time::from_hms_nano(12, 34, 56, 123456789).unwrap();
            assert_eq!(expected, actual);
        }
        {
            let actual = do_string_to_time("12:34:56.123456789-09:00").unwrap();
            let expected = time::Time::from_hms_nano(12, 34, 56, 123456789).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
