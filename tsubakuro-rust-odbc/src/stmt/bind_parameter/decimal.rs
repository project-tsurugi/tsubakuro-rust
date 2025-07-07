use std::str::FromStr;

use log::debug;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgDecimal, TgDecimalI128};

use crate::{
    ctype::{CDataType, SqlReturn},
    handle::{diag::TsurugiOdbcError, hstmt::TsurugiOdbcStmt},
    stmt::bind_parameter::{numeric_ptr_to_decimal, TsurugiOdbcBindParameter},
};

impl TsurugiOdbcBindParameter {
    pub(super) fn tg_parameter_decimal(
        &self,
        name: String,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "sql_parameter_decimal()";

        let value_type = self.value_type;
        let value_ptr = self.parameter_value_ptr;

        use CDataType::*;
        let value: TgDecimal = match value_type {
            SQL_C_TINYINT | SQL_C_STINYINT => unsafe {
                let ptr = value_ptr as *mut i8;
                self.i128_to_decimal(*ptr as i128)
            },
            SQL_C_UTINYINT => unsafe {
                let ptr = value_ptr as *mut u8;
                self.i128_to_decimal(*ptr as i128)
            },
            SQL_C_SHORT | SQL_C_SSHORT => unsafe {
                let ptr = value_ptr as *mut i16;
                self.i128_to_decimal(*ptr as i128)
            },
            SQL_C_USHORT => unsafe {
                let ptr = value_ptr as *mut u16;
                self.i128_to_decimal(*ptr as i128)
            },
            SQL_C_LONG | SQL_C_SLONG => unsafe {
                let ptr = value_ptr as *mut i32;
                self.i128_to_decimal(*ptr as i128)
            },
            SQL_C_ULONG => unsafe {
                let ptr = value_ptr as *mut u32;
                self.i128_to_decimal(*ptr as i128)
            },
            SQL_C_SBIGINT => unsafe {
                let ptr = value_ptr as *mut i64;
                self.i128_to_decimal(*ptr as i128)
            },
            SQL_C_UBIGINT => unsafe {
                let ptr = value_ptr as *mut u64;
                self.i128_to_decimal(*ptr as i128)
            },
            SQL_C_FLOAT => unsafe {
                let ptr = value_ptr as *mut f32;
                self.f32_to_decimal(FUNCTION_NAME, stmt, *ptr)?
            },
            SQL_C_DOUBLE => unsafe {
                let ptr = value_ptr as *mut f64;
                self.f64_to_decimal(FUNCTION_NAME, stmt, *ptr)?
            },
            SQL_C_NUMERIC => numeric_ptr_to_decimal(value_ptr),
            SQL_C_CHAR => self.char_ptr_to_decimal(FUNCTION_NAME, stmt)?,
            SQL_C_WCHAR => self.wchar_ptr_to_decimal(FUNCTION_NAME, stmt)?,
            _ => {
                debug!(
                    "{stmt}.{FUNCTION_NAME} error. Unsupported value_type. {:?}",
                    self
                );
                stmt.add_diag(
                    TsurugiOdbcError::UnsupportedCDataType,
                    format!("Unsupported value_type {:?} for DECIMAL", value_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        };

        let parameter = SqlParameter::of(&name, value);
        Ok(parameter)
    }
}

impl TsurugiOdbcBindParameter {
    fn i128_to_decimal(&self, value: i128) -> TgDecimal {
        i128_to_decimal(value, self.decimal_digits as i32).into()
    }

    fn f32_to_decimal(
        &self,
        function_name: &str,
        stmt: &TsurugiOdbcStmt,
        value: f32,
    ) -> Result<TgDecimal, SqlReturn> {
        let s = value.to_string();
        let value = string_to_decimal(function_name, stmt, &s)?;
        Ok(value.into())
    }

    fn f64_to_decimal(
        &self,
        function_name: &str,
        stmt: &TsurugiOdbcStmt,
        value: f64,
    ) -> Result<TgDecimal, SqlReturn> {
        let s = value.to_string();
        let value = string_to_decimal(function_name, stmt, &s)?;
        Ok(value.into())
    }

    fn char_ptr_to_decimal(
        &self,
        function_name: &str,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<TgDecimal, SqlReturn> {
        let s = self.char_ptr_to_string(function_name, stmt)?;
        let value = string_to_decimal(function_name, stmt, &s)?;
        Ok(value.into())
    }

    fn wchar_ptr_to_decimal(
        &self,
        function_name: &str,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<TgDecimal, SqlReturn> {
        let s = self.wchar_ptr_to_string(function_name, stmt)?;
        let value = string_to_decimal(function_name, stmt, &s)?;
        Ok(value.into())
    }
}

fn i128_to_decimal(value: i128, scale: i32) -> TgDecimalI128 {
    if scale == 0 {
        TgDecimalI128::new(value, scale)
    } else if scale > 0 {
        let unscaled_value = value * 10_i128.pow(scale as u32);
        TgDecimalI128::new(unscaled_value, -scale)
    } else {
        let unscaled_value = value / 10_i128.pow(-scale as u32);
        TgDecimalI128::new(unscaled_value, -scale)
    }
}

fn string_to_decimal(
    function_name: &str,
    stmt: &TsurugiOdbcStmt,
    value: &str,
) -> Result<TgDecimalI128, SqlReturn> {
    match TgDecimalI128::from_str(value) {
        Ok(value) => Ok(value),
        Err(e) => {
            debug!("{stmt}.{function_name}: convert error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::DecimalError,
                format!("convert error. {}", e),
            );
            Err(SqlReturn::SQL_ERROR)
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn i128_to_decimal() {
        {
            let value = super::i128_to_decimal(123_i128, 0);
            assert_eq!(123_i128, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = super::i128_to_decimal(123_i128, 2);
            assert_eq!(12300_i128, value.unscaled_value);
            assert_eq!(-2, value.exponent);
        }
        {
            let value = super::i128_to_decimal(12345_i128, -2);
            assert_eq!(123_i128, value.unscaled_value);
            assert_eq!(2, value.exponent);
        }
    }
}
