use log::{debug, trace};
use tsubakuro_rust_core::prelude::{TgDecimalI128, TgDecimalResult};

use crate::{
    check_stmt,
    ctype::{
        sql_date_struct::SqlDateStruct, sql_numeric_struct::SqlNumericStruct,
        sql_time_struct::SqlTimeStruct, sql_timestamp_struct::SqlTimestampStruct, CDataType,
        SqlChar, SqlLen, SqlPointer, SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    util::{write_char_len, write_wchar_len},
};

mod from_bool;
mod from_bytes;
mod from_date;
mod from_decimal;
mod from_f32;
mod from_f64;
mod from_i32;
mod from_i64;
mod from_null;
mod from_string;
mod from_time;
mod from_time_tz;
mod from_timestamp;
mod from_timestamp_tz;

pub(crate) use from_bool::*;
pub(crate) use from_bytes::*;
pub(crate) use from_date::*;
pub(crate) use from_decimal::*;
pub(crate) use from_f32::*;
pub(crate) use from_f64::*;
pub(crate) use from_i32::*;
pub(crate) use from_i64::*;
pub(crate) use from_null::*;
pub(crate) use from_string::*;
pub(crate) use from_time::*;
pub(crate) use from_time_tz::*;
pub(crate) use from_timestamp::*;
pub(crate) use from_timestamp_tz::*;

const SQL_NULL_DATA: SqlLen = -1;

#[no_mangle]
pub extern "system" fn SQLGetData(
    hstmt: HStmt,
    col_or_param_num: SqlUSmallInt,
    target_type: SqlSmallInt,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetData()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, col_or_param_num={:?}, target_type={:?}, target_value_ptr={:?}, buffer_length={:?}, str_len_or_ind_ptr={:?}",
        hstmt,
        col_or_param_num,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = get_data(
        &stmt,
        col_or_param_num,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn get_data(
    stmt: &TsurugiOdbcStmt,
    col_or_param_num: SqlUSmallInt,
    target_type: SqlSmallInt,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetData()";

    let target_type = match CDataType::try_from(target_type) {
        Ok(value) => value,
        Err(e) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported target_type {}",
                target_type
            );
            stmt.add_diag(e, format!("Unsupported target_type {}", target_type));
            return SqlReturn::SQL_ERROR;
        }
    };

    let arg = GetDataArguments::new(
        col_or_param_num,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    );
    do_get_data(stmt, arg)
}

pub(crate) struct GetDataArguments {
    column_number: SqlUSmallInt,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
}

impl GetDataArguments {
    pub fn new(
        column_number: SqlUSmallInt,
        target_type: CDataType,
        target_value_ptr: SqlPointer,
        buffer_length: SqlLen,
        str_len_or_ind_ptr: *mut SqlLen,
    ) -> GetDataArguments {
        GetDataArguments {
            column_number,
            target_type,
            target_value_ptr,
            buffer_length,
            str_len_or_ind_ptr,
        }
    }

    pub(crate) fn column_index(&self) -> isize {
        self.column_number as isize - 1
    }
}

pub(crate) fn do_get_data(stmt: &TsurugiOdbcStmt, arg: GetDataArguments) -> SqlReturn {
    const FUNCTION_NAME: &str = "do_get_data()";

    let processor = match stmt.processor(FUNCTION_NAME) {
        Ok(processor) => processor,
        Err(rc) => return rc,
    };
    let mut processor = processor.borrow_mut();

    let column_number = arg.column_number;
    let number_of_columns = processor.number_of_columns();
    if column_number < 1 || column_number > number_of_columns {
        debug!(
            "{stmt}.{FUNCTION_NAME} error. index out of bounds. column_number={}, number_of_columns={}",
            column_number,
            number_of_columns
        );
        stmt.add_diag(
            TsurugiOdbcError::ColumnNumberOutOfBounds,
            format!(
                "column_number must be between 1 and {}, but got {}",
                number_of_columns, column_number
            ),
        );
        return SqlReturn::SQL_ERROR;
    }

    processor.get_data(stmt, arg)
}

fn check_target_value_ptr(
    function_name: &str,
    stmt: &TsurugiOdbcStmt,
    arg: &GetDataArguments,
) -> Result<(), SqlReturn> {
    if arg.target_value_ptr.is_null() {
        debug!("{stmt}.{function_name} error. target_value_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::GetDataInvalidTargetValuePtr,
            "SQLGetData.target_value_ptr is null",
        );
        Err(SqlReturn::SQL_ERROR)
    } else {
        Ok(())
    }
}

fn write_bool(arg: GetDataArguments, value: bool) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut u8) = if value { 1 } else { 0 };
    }
    write_str_len_or_ind(arg, 1);
    SqlReturn::SQL_SUCCESS
}

fn write_u8(arg: GetDataArguments, value: u8) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut u8) = value;
    }
    write_str_len_or_ind(arg, 1);
    SqlReturn::SQL_SUCCESS
}

fn write_i8(arg: GetDataArguments, value: i8) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut i8) = value;
    }
    write_str_len_or_ind(arg, 1);
    SqlReturn::SQL_SUCCESS
}

fn write_u16(arg: GetDataArguments, value: u16) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut u16) = value;
    }
    write_str_len_or_ind(arg, 2);
    SqlReturn::SQL_SUCCESS
}

fn write_i16(arg: GetDataArguments, value: i16) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut i16) = value;
    }
    write_str_len_or_ind(arg, 2);
    SqlReturn::SQL_SUCCESS
}

fn write_u32(arg: GetDataArguments, value: u32) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut u32) = value;
    }
    write_str_len_or_ind(arg, 4);
    SqlReturn::SQL_SUCCESS
}

fn write_i32(arg: GetDataArguments, value: i32) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut i32) = value;
    }
    write_str_len_or_ind(arg, 4);
    SqlReturn::SQL_SUCCESS
}

fn write_u64(arg: GetDataArguments, value: u64) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut u64) = value;
    }
    write_str_len_or_ind(arg, 8);
    SqlReturn::SQL_SUCCESS
}

fn write_i64(arg: GetDataArguments, value: i64) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut i64) = value;
    }
    write_str_len_or_ind(arg, 8);
    SqlReturn::SQL_SUCCESS
}

fn write_f32(arg: GetDataArguments, value: f32) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut f32) = value;
    }
    write_str_len_or_ind(arg, 4);
    SqlReturn::SQL_SUCCESS
}

fn write_f64(arg: GetDataArguments, value: f64) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut f64) = value;
    }
    write_str_len_or_ind(arg, 8);
    SqlReturn::SQL_SUCCESS
}

fn write_numeric_i128(arg: GetDataArguments, value: i128) -> SqlReturn {
    let value = SqlNumericStruct::from(value);
    write_numeric_struct(arg, value)
}

fn write_numeric_struct(arg: GetDataArguments, value: SqlNumericStruct) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut SqlNumericStruct) = value;
    }
    write_str_len_or_ind(arg, std::mem::size_of::<SqlNumericStruct>() as SqlLen);
    SqlReturn::SQL_SUCCESS
}

fn write_bytes(stmt: &TsurugiOdbcStmt, arg: GetDataArguments, value: &[u8]) -> SqlReturn {
    let value_len = value.len() as SqlLen;
    let buffer_length = arg.buffer_length;
    let copy_len = std::cmp::min(value_len, buffer_length);
    unsafe {
        std::ptr::copy_nonoverlapping(
            value.as_ptr(),
            arg.target_value_ptr as *mut u8,
            copy_len as usize,
        );
    }
    write_str_len_or_ind(arg, value_len);

    if value_len <= buffer_length {
        SqlReturn::SQL_SUCCESS
    } else {
        stmt.add_diag(TsurugiOdbcError::DataTruncated, "Data truncated");
        SqlReturn::SQL_SUCCESS_WITH_INFO
    }
}

fn write_date_struct(arg: GetDataArguments, value: SqlDateStruct) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut SqlDateStruct) = value;
    }
    write_str_len_or_ind(arg, std::mem::size_of::<SqlDateStruct>() as SqlLen);
    SqlReturn::SQL_SUCCESS
}

fn write_time_struct(arg: GetDataArguments, value: SqlTimeStruct) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut SqlTimeStruct) = value;
    }
    write_str_len_or_ind(arg, std::mem::size_of::<SqlTimeStruct>() as SqlLen);
    SqlReturn::SQL_SUCCESS
}

fn write_timestamp_struct(arg: GetDataArguments, value: SqlTimestampStruct) -> SqlReturn {
    unsafe {
        *(arg.target_value_ptr as *mut SqlTimestampStruct) = value;
    }
    write_str_len_or_ind(arg, std::mem::size_of::<SqlTimestampStruct>() as SqlLen);
    SqlReturn::SQL_SUCCESS
}

fn write_str_len_or_ind(arg: GetDataArguments, value: SqlLen) {
    let str_len_or_ind_ptr = arg.str_len_or_ind_ptr;
    if !str_len_or_ind_ptr.is_null() {
        unsafe {
            *str_len_or_ind_ptr = value;
        }
    }
}
