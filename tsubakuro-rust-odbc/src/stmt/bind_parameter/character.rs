use log::debug;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

use crate::{
    ctype::{CDataType, SqlChar, SqlReturn, SqlSmallInt, SqlWChar},
    handle::{diag::TsurugiOdbcError, hstmt::TsurugiOdbcStmt},
    stmt::{
        bind_parameter::{numeric_ptr_to_string, TsurugiOdbcBindParameter},
        get_data::{f32_to_string, f64_to_string},
    },
    util::{char_to_string, wchar_to_string},
};

impl TsurugiOdbcBindParameter {
    pub(super) fn tg_parameter_character(
        &self,
        name: String,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "sql_parameter_character()";

        let value_type = self.value_type;
        let value_ptr = self.parameter_value_ptr;

        use CDataType::*;
        let value: String = match value_type {
            SQL_C_TINYINT | SQL_C_STINYINT => unsafe {
                let ptr = value_ptr as *mut i8;
                (*ptr).to_string()
            },
            SQL_C_UTINYINT => unsafe {
                let ptr = value_ptr as *mut u8;
                (*ptr).to_string()
            },
            SQL_C_SHORT | SQL_C_SSHORT => unsafe {
                let ptr = value_ptr as *mut i16;
                (*ptr).to_string()
            },
            SQL_C_USHORT => unsafe {
                let ptr = value_ptr as *mut u16;
                (*ptr).to_string()
            },
            SQL_C_LONG | SQL_C_SLONG => unsafe {
                let ptr = value_ptr as *mut i32;
                (*ptr).to_string()
            },
            SQL_C_ULONG => unsafe {
                let ptr = value_ptr as *mut u32;
                (*ptr).to_string()
            },
            SQL_C_SBIGINT => unsafe {
                let ptr = value_ptr as *mut i64;
                (*ptr).to_string()
            },
            SQL_C_UBIGINT => unsafe {
                let ptr = value_ptr as *mut u64;
                (*ptr).to_string()
            },
            SQL_C_FLOAT => unsafe {
                let ptr = value_ptr as *mut f32;
                f32_to_string(*ptr)
            },
            SQL_C_DOUBLE => unsafe {
                let ptr = value_ptr as *mut f64;
                f64_to_string(*ptr)
            },
            SQL_C_NUMERIC => numeric_ptr_to_string(value_ptr),
            SQL_C_CHAR => {
                let ptr = value_ptr as *mut SqlChar;
                let length = self.length_or_ind as SqlSmallInt;
                match char_to_string(ptr, length) {
                    Ok(value) => value,
                    Err(e) => {
                        debug!("{stmt}.{FUNCTION_NAME}: string error. {:?}", e);
                        stmt.add_diag(
                            TsurugiOdbcError::StringError,
                            format!("string error. {}", e),
                        );
                        return Err(SqlReturn::SQL_ERROR);
                    }
                }
            }
            SQL_C_WCHAR => {
                let ptr = value_ptr as *mut SqlWChar;
                let length = self.length_or_ind as SqlSmallInt;
                match wchar_to_string(ptr, length) {
                    Ok(value) => value,
                    Err(e) => {
                        debug!("{stmt}.{FUNCTION_NAME}: string error. {:?}", e);
                        stmt.add_diag(
                            TsurugiOdbcError::StringError,
                            format!("string error. {}", e),
                        );
                        return Err(SqlReturn::SQL_ERROR);
                    }
                }
            }
            _ => {
                debug!(
                    "{stmt}.{FUNCTION_NAME} error. Unsupported value_type. {:?}",
                    self
                );
                stmt.add_diag(
                    TsurugiOdbcError::BindParameterUnsupportedValueType,
                    format!("Unsupported value_type {:?} for CHARACTER", value_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        let parameter = SqlParameter::of(&name, value);
        Ok(parameter)
    }
}
