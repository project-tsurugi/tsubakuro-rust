use log::debug;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

use crate::{
    ctype::{CDataType, SqlReturn},
    handle::{diag::TsurugiOdbcError, hstmt::TsurugiOdbcStmt},
    stmt::bind_parameter::TsurugiOdbcBindParameter,
};

impl TsurugiOdbcBindParameter {
    pub(super) fn tg_parameter_octet(
        &self,
        name: String,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "sql_parameter_octet()";

        let value_type = self.value_type;
        let value_ptr = self.parameter_value_ptr;

        use CDataType::*;
        let value: Vec<u8> = match value_type {
            SQL_C_BINARY => unsafe {
                // TODO length < 0 ならエラー
                let ptr = value_ptr as *const u8;
                let length = self.length_or_ind as usize;
                std::slice::from_raw_parts(ptr, length).to_vec()
            },
            _ => {
                debug!(
                    "{stmt}.{FUNCTION_NAME} error. Unsupported value_type. {:?}",
                    self
                );
                stmt.add_diag(
                    TsurugiOdbcError::BindParameterUnsupportedValueType,
                    format!("Unsupported value_type {:?} for BINARY", value_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        let parameter = SqlParameter::of(&name, value);
        Ok(parameter)
    }
}
