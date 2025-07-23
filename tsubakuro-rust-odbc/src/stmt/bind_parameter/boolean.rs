use log::debug;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

use crate::{
    ctype::{CDataType, SqlReturn},
    handle::{diag::TsurugiOdbcError, hstmt::TsurugiOdbcStmt},
    stmt::{
        bind_parameter::{numeric_ptr_to_i128, TsurugiOdbcBindParameter}, get_data::str_to_bool,
    },
};

impl TsurugiOdbcBindParameter {
    pub(super) fn tg_parameter_boolean(
        &self,
        name: String,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "sql_parameter_boolean()";

        let value_type = self.value_type;
        let value_ptr = self.parameter_value_ptr;

        use CDataType::*;
        let value: bool = match value_type {
            SQL_C_TINYINT | SQL_C_STINYINT => unsafe {
                let ptr = value_ptr as *mut i8;
                *ptr != 0
            },
            SQL_C_UTINYINT => unsafe {
                let ptr = value_ptr as *mut u8;
                *ptr != 0
            },
            SQL_C_SHORT | SQL_C_SSHORT => unsafe {
                let ptr = value_ptr as *mut i16;
                *ptr != 0
            },
            SQL_C_USHORT => unsafe {
                let ptr = value_ptr as *mut u16;
                *ptr != 0
            },
            SQL_C_LONG | SQL_C_SLONG => unsafe {
                let ptr = value_ptr as *mut i32;
                *ptr != 0
            },
            SQL_C_ULONG => unsafe {
                let ptr = value_ptr as *mut u32;
                *ptr != 0
            },
            SQL_C_SBIGINT => unsafe {
                let ptr = value_ptr as *mut i64;
                *ptr != 0
            },
            SQL_C_UBIGINT => unsafe {
                let ptr = value_ptr as *mut u64;
                *ptr != 0
            },
            SQL_C_FLOAT => unsafe {
                let ptr = value_ptr as *mut f32;
                *ptr != 0f32
            },
            SQL_C_DOUBLE => unsafe {
                let ptr = value_ptr as *mut f64;
                *ptr != 0f64
            },
            SQL_C_NUMERIC => numeric_ptr_to_i128(value_ptr) != 0,
            SQL_C_CHAR => {
                let value = self.char_ptr_to_string(FUNCTION_NAME, stmt)?;
                str_to_bool(stmt, &value)?
            }
            SQL_C_WCHAR => {
                let value = self.wchar_ptr_to_string(FUNCTION_NAME, stmt)?;
                str_to_bool(stmt, &value)?
            }
            _ => {
                debug!(
                    "{stmt}.{FUNCTION_NAME} error. Unsupported value_type. {:?}",
                    self
                );
                stmt.add_diag(
                    TsurugiOdbcError::BindParameterUnsupportedValueType,
                    format!("Unsupported value_type {:?} for BOOLEAN", value_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        let parameter = SqlParameter::of(&name, value);
        Ok(parameter)
    }
}
