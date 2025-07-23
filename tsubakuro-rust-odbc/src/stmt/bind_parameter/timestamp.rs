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
                            TsurugiOdbcError::BindParameterConvertDateError,
                            format!("SQL_DATE_STRUCT to timestamp convert error, {}", e),
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
                            TsurugiOdbcError::BindParameterConvertTimestampError,
                            format!("SQL_TIMESTAMP_STRUCT to timestamp convert error, {}", e),
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
    match do_string_to_timestamp_tz(value) {
        Ok(value) => Ok(time::PrimitiveDateTime::new(value.date(), value.time())),
        Err(e) => {
            debug!("{stmt}.{function_name}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::BindParameterConvertTimestampError,
                format!("string to timestamp convert error. {}", e),
            );
            Err(SqlReturn::SQL_ERROR)
        }
    }
}
