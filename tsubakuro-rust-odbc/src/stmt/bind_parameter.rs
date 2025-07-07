use log::{debug, trace};
use tsubakuro_rust_core::prelude::{AtomType, SqlParameter, SqlPlaceholder, TgDecimal};

use crate::{
    check_stmt,
    ctype::{
        sql_numeric_struct::SqlNumericStruct, CDataType, SqlChar, SqlDataType, SqlLen, SqlPointer,
        SqlReturn, SqlSmallInt, SqlULen, SqlUSmallInt, SqlWChar, SQL_NULL_DATA,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    util::{char_to_string, wchar_to_string},
};

mod boolean;
mod character;
mod date;
mod decimal;
mod float4;
mod float8;
mod int4;
mod int8;
mod octet;
mod time;
mod timestamp;

#[no_mangle]
pub extern "system" fn SQLBindParameter(
    hstmt: HStmt,
    parameter_number: SqlUSmallInt,
    input_output_type: SqlSmallInt,
    value_type: SqlSmallInt,
    parameter_type: SqlSmallInt,
    column_size: SqlULen,
    decimal_digits: SqlSmallInt,
    parameter_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLBindParameter()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, parameter_number={:?}, input_output_type={:?}, value_type={:?}, parameter_type={:?}, column_size={:?}, decimal_digits={:?}, parameter_value_ptr={:?}, buffer_length={:?}, str_len_or_ind_ptr={:?}",
        hstmt,
        parameter_number,
        input_output_type,
        value_type,
        parameter_type,
        column_size,
        decimal_digits,
        parameter_value_ptr,
        buffer_length,
        str_len_or_ind_ptr
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = bind_parameter(
        &mut stmt,
        parameter_number,
        input_output_type,
        value_type,
        parameter_type,
        column_size,
        decimal_digits,
        parameter_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn bind_parameter(
    stmt: &mut TsurugiOdbcStmt,
    parameter_number: SqlUSmallInt,
    _input_output_type: SqlSmallInt,
    value_type: SqlSmallInt,
    parameter_type: SqlSmallInt,
    column_size: SqlULen,
    decimal_digits: SqlSmallInt,
    parameter_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "bind_parameter()";

    // TODO parameter_numberの上限チェック
    if parameter_number < 1 {
        debug!(
            "{stmt}.{FUNCTION_NAME} error. Out of range. parameter_number={}",
            parameter_number
        );
        stmt.add_diag(
            TsurugiOdbcError::BindParameterError,
            "parameter_number out of range",
        );
        return SqlReturn::SQL_ERROR;
    }

    // TODO input_output_type（ParameterType）がSQL_PARAM_INPUT以外の場合はエラーとする

    let value_type = match CDataType::try_from(value_type) {
        Ok(value) => value,
        Err(e) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported value_type {}",
                value_type
            );
            stmt.add_diag(e, format!("Unsupported value_type {}", value_type));
            return SqlReturn::SQL_ERROR;
        }
    };

    let parameter_type = match SqlDataType::try_from(parameter_type) {
        Ok(value) => value,
        Err(e) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported parameter_type {}",
                parameter_type
            );
            stmt.add_diag(e, format!("Unsupported parameter_type {}", parameter_type));
            return SqlReturn::SQL_ERROR;
        }
    };

    // TODO value_typeがSQL_C_DEFAULTの場合、parameter_typeから決定する

    let atom_type = match AtomType::try_from(parameter_type) {
        Ok(value) => value,
        Err(_) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported parameter_type {:?} convert to AtomType",
                parameter_type
            );
            stmt.add_diag(
                TsurugiOdbcError::BindParameterError,
                format!("Unsupported parameter_type {:?}", parameter_type),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    if str_len_or_ind_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. str_len_or_ind_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::BindParameterError,
            "str_len_or_ind_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    };
    let length_or_ind = unsafe { *str_len_or_ind_ptr };

    let parameter = TsurugiOdbcBindParameter::new(
        parameter_number,
        value_type,
        parameter_type,
        atom_type,
        column_size,
        decimal_digits,
        parameter_value_ptr,
        buffer_length,
        length_or_ind,
    );

    stmt.set_parameter(parameter);

    SqlReturn::SQL_SUCCESS
}

pub(crate) struct TsurugiOdbcBindParameter {
    parameter_number: SqlUSmallInt,
    value_type: CDataType,
    parameter_type: SqlDataType,
    atom_type: AtomType,
    _column_size: SqlULen,
    decimal_digits: SqlSmallInt,
    parameter_value_ptr: SqlPointer,
    _buffer_length: SqlLen,
    length_or_ind: SqlLen,
}

impl std::fmt::Debug for TsurugiOdbcBindParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsurugiOdbcBindParameter")
            .field("parameter_number", &self.parameter_number)
            .field("value_type", &self.value_type)
            .field("parameter_type", &self.parameter_type)
            .field("atom_type", &self.atom_type)
            .finish()
    }
}

impl TsurugiOdbcBindParameter {
    fn new(
        parameter_number: SqlUSmallInt,
        value_type: CDataType,
        parameter_type: SqlDataType,
        atom_type: AtomType,
        column_size: SqlULen,
        decimal_digits: SqlSmallInt,
        parameter_value_ptr: SqlPointer,
        buffer_length: SqlLen,
        length_or_ind: SqlLen,
    ) -> TsurugiOdbcBindParameter {
        TsurugiOdbcBindParameter {
            parameter_number,
            value_type,
            parameter_type,
            atom_type,
            _column_size: column_size,
            decimal_digits,
            parameter_value_ptr,
            _buffer_length: buffer_length,
            length_or_ind,
        }
    }

    pub(crate) fn parameter_number(&self) -> SqlUSmallInt {
        self.parameter_number
    }

    fn placeholder_name(&self) -> String {
        self.parameter_number.to_string()
    }
}

impl TsurugiOdbcBindParameter {
    pub(crate) fn tg_placeholder(&self) -> SqlPlaceholder {
        let name = self.placeholder_name();
        let atom_type = self.atom_type;
        SqlPlaceholder::of_atom_type(&name, atom_type)
    }

    pub(crate) fn tg_parameter(&self, stmt: &TsurugiOdbcStmt) -> Result<SqlParameter, SqlReturn> {
        const FUNCTION_NAME: &str = "tg_parameter()";

        let name = self.placeholder_name();

        if self.length_or_ind == SQL_NULL_DATA {
            return Ok(SqlParameter::null(&name));
        }

        let atom_type = self.atom_type;
        match atom_type {
            AtomType::Boolean => self.tg_parameter_boolean(name, stmt),
            AtomType::Int4 => self.tg_parameter_int4(name, stmt),
            AtomType::Int8 => self.tg_parameter_int8(name, stmt),
            AtomType::Float4 => self.tg_parameter_float4(name, stmt),
            AtomType::Float8 => self.tg_parameter_float8(name, stmt),
            AtomType::Decimal => self.tg_parameter_decimal(name, stmt),
            AtomType::Character => self.tg_parameter_character(name, stmt),
            AtomType::Octet => self.tg_parameter_octet(name, stmt),
            // AtomType::Bit => todo!(),
            AtomType::Date => self.tg_parameter_date(name, stmt),
            AtomType::TimeOfDay => self.tg_parameter_time(name, stmt),
            AtomType::TimePoint => self.tg_parameter_timestamp(name, stmt),
            // AtomType::DatetimeInterval => todo!(),
            // AtomType::TimeOfDayWithTimeZone => todo!(),
            // AtomType::TimePointWithTimeZone => todo!(),
            // AtomType::Clob => todo!(),
            // AtomType::Blob => todo!(),
            _ => {
                debug!(
                    "{stmt}.{FUNCTION_NAME} error. Unsupported AtomType. {:?}",
                    self
                );
                stmt.add_diag(
                    TsurugiOdbcError::UnsupportedSqlDataType,
                    format!("Unsupported AtomType {:?}", atom_type),
                );
                return Err(SqlReturn::SQL_ERROR);
            }
        }
    }
}

fn numeric_ptr_to_i128(value_ptr: SqlPointer) -> i128 {
    unsafe {
        let ptr = value_ptr as *const SqlNumericStruct;
        let s = &*ptr;
        i128::from(s)
    }
}

fn numeric_ptr_to_f64(value_ptr: SqlPointer) -> f64 {
    unsafe {
        let ptr = value_ptr as *const SqlNumericStruct;
        let s = &*ptr;
        f64::from(s)
    }
}

fn numeric_ptr_to_decimal(value_ptr: SqlPointer) -> TgDecimal {
    unsafe {
        let ptr = value_ptr as *const SqlNumericStruct;
        let s = &*ptr;
        TgDecimal::from(s)
    }
}

fn numeric_ptr_to_string(value_ptr: SqlPointer) -> String {
    unsafe {
        let ptr = value_ptr as *const SqlNumericStruct;
        let s = &*ptr;
        s.to_string()
    }
}

impl TsurugiOdbcBindParameter {
    fn char_ptr_to_string(
        &self,
        function_name: &str,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<String, SqlReturn> {
        let ptr = self.parameter_value_ptr as *mut SqlChar;
        let length = self.length_or_ind as SqlSmallInt;
        match char_to_string(ptr, length) {
            Ok(value) => Ok(value),
            Err(e) => {
                debug!("{stmt}.{function_name}: string error. {:?}", e);
                stmt.add_diag(
                    TsurugiOdbcError::StringError,
                    format!("string error. {}", e),
                );
                Err(SqlReturn::SQL_ERROR)
            }
        }
    }

    fn wchar_ptr_to_string(
        &self,
        function_name: &str,
        stmt: &TsurugiOdbcStmt,
    ) -> Result<String, SqlReturn> {
        let ptr = self.parameter_value_ptr as *mut SqlWChar;
        let length = self.length_or_ind as SqlSmallInt;
        match wchar_to_string(ptr, length) {
            Ok(value) => Ok(value),
            Err(e) => {
                debug!("{stmt}.{function_name}: string error. {:?}", e);
                stmt.add_diag(
                    TsurugiOdbcError::StringError,
                    format!("string error. {}", e),
                );
                Err(SqlReturn::SQL_ERROR)
            }
        }
    }
}
