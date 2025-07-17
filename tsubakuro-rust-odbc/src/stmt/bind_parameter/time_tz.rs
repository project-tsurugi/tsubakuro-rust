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
    pub(super) fn tg_parameter_time_tz(
        &self,
        name: String,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "sql_parameter_time_tz()";

        let value_type = self.value_type;
        let value_ptr = self.parameter_value_ptr;

        use CDataType::*;
        let value: (time::Time, time::UtcOffset) = match value_type {
            SQL_C_CHAR => {
                let s = self.char_ptr_to_string(FUNCTION_NAME, stmt)?;
                string_to_time_tz(FUNCTION_NAME, stmt, &s)?
            }
            SQL_C_WCHAR => {
                let s = self.wchar_ptr_to_string(FUNCTION_NAME, stmt)?;
                string_to_time_tz(FUNCTION_NAME, stmt, &s)?
            }
            SQL_C_TYPE_TIME | SQL_C_TIME => unsafe {
                let ptr = value_ptr as *const SqlTimeStruct;
                match time::Time::try_from(&*ptr) {
                    Ok(value) => (value, time::UtcOffset::UTC),
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
                match time::Time::try_from(&*ptr) {
                    Ok(value) => (value, time::UtcOffset::UTC),
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
                    format!("Unsupported value_type {:?} for TIME", value_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        let parameter = SqlParameter::of(&name, value);
        Ok(parameter)
    }
}

fn string_to_time_tz(
    function_name: &str,
    stmt: &TsurugiOdbcStmt,
    value: &str,
) -> Result<(time::Time, time::UtcOffset), SqlReturn> {
    match do_string_to_timestamp_tz(value) {
        Ok(value) => Ok((value.time(), value.offset())),
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
