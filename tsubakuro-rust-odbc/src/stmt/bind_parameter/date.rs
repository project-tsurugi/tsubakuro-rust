use log::debug;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

use crate::{
    ctype::{
        sql_date_struct::SqlDateStruct, sql_timestamp_struct::SqlTimestampStruct, CDataType,
        SqlReturn,
    },
    handle::{diag::TsurugiOdbcError, hstmt::TsurugiOdbcStmt},
    stmt::bind_parameter::{timestamp_tz::do_string_to_timestamp_tz, TsurugiOdbcBindParameter},
};

impl TsurugiOdbcBindParameter {
    pub(super) fn tg_parameter_date(
        &self,
        name: String,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "sql_parameter_date()";

        let value_type = self.value_type;
        let value_ptr = self.parameter_value_ptr;

        use CDataType::*;
        let value: time::Date = match value_type {
            SQL_C_CHAR => {
                let s = self.char_ptr_to_string(FUNCTION_NAME, stmt)?;
                string_to_date(FUNCTION_NAME, stmt, &s)?
            }
            SQL_C_WCHAR => {
                let s = self.wchar_ptr_to_string(FUNCTION_NAME, stmt)?;
                string_to_date(FUNCTION_NAME, stmt, &s)?
            }
            SQL_C_TYPE_DATE | SQL_C_DATE => unsafe {
                let ptr = value_ptr as *const SqlDateStruct;
                match time::Date::try_from(&*ptr) {
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
                match time::Date::try_from(&*ptr) {
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
                    format!("Unsupported value_type {:?} for DATE", value_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        let parameter = SqlParameter::of(&name, value);
        Ok(parameter)
    }
}

fn string_to_date(
    function_name: &str,
    stmt: &TsurugiOdbcStmt,
    value: &str,
) -> Result<time::Date, SqlReturn> {
    match do_string_to_timestamp_tz(value) {
        Ok(value) => Ok(value.date()),
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

pub(crate) fn do_string_to_date(value: &str) -> Result<time::Date, Box<dyn std::error::Error>> {
    let format = time::macros::format_description!("[year]-[month]-[day]");
    time::Date::parse(value, format).map_err(|e| e.into())
}
