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

    do_get_data(
        stmt,
        col_or_param_num,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    )
}

pub(crate) fn do_get_data(
    stmt: &TsurugiOdbcStmt,
    column_number: SqlUSmallInt,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "execute_get_data()";

    let processor = match stmt.processor(FUNCTION_NAME) {
        Ok(processor) => processor,
        Err(rc) => return rc,
    };
    let mut processor = processor.borrow_mut();

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
    let column_index = column_number - 1;

    processor.get_data(
        &stmt,
        column_index,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    )
}

fn write_bool(
    value: bool,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut u8) = if value { 1 } else { 0 };
    }
    write_str_len_or_ind(1, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_u8(value: u8, target_value_ptr: SqlPointer, str_len_or_ind_ptr: *mut SqlLen) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut u8) = value;
    }
    write_str_len_or_ind(1, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_i8(value: i8, target_value_ptr: SqlPointer, str_len_or_ind_ptr: *mut SqlLen) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut i8) = value;
    }
    write_str_len_or_ind(1, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_u16(
    value: u16,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut u16) = value;
    }
    write_str_len_or_ind(2, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_i16(
    value: i16,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut i16) = value;
    }
    write_str_len_or_ind(2, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_u32(
    value: u32,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut u32) = value;
    }
    write_str_len_or_ind(4, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_i32(
    value: i32,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut i32) = value;
    }
    write_str_len_or_ind(4, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_u64(
    value: u64,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut u64) = value;
    }
    write_str_len_or_ind(8, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_i64(
    value: i64,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut i64) = value;
    }
    write_str_len_or_ind(8, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_f32(
    value: f32,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut f32) = value;
    }
    write_str_len_or_ind(4, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_f64(
    value: f64,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut f64) = value;
    }
    write_str_len_or_ind(8, str_len_or_ind_ptr);
    SqlReturn::SQL_SUCCESS
}

fn write_numeric_i128(
    value: i128,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    let value = SqlNumericStruct::from(value);
    write_numeric_struct(value, target_value_ptr, str_len_or_ind_ptr)
}

fn write_numeric_struct(
    value: SqlNumericStruct,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut SqlNumericStruct) = value;
    }
    write_str_len_or_ind(
        std::mem::size_of::<SqlNumericStruct>() as SqlLen,
        str_len_or_ind_ptr,
    );
    SqlReturn::SQL_SUCCESS
}

fn write_bytes(
    stmt: &TsurugiOdbcStmt,
    value: &Vec<u8>,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    let value_len = value.len() as SqlLen;
    let copy_len = std::cmp::min(value_len, buffer_length);
    unsafe {
        std::ptr::copy_nonoverlapping(
            value.as_ptr(),
            target_value_ptr as *mut u8,
            copy_len as usize,
        );
    }
    write_str_len_or_ind(value_len, str_len_or_ind_ptr);

    if value_len <= buffer_length {
        SqlReturn::SQL_SUCCESS
    } else {
        stmt.add_diag(TsurugiOdbcError::DataTruncated, "Data truncated");
        SqlReturn::SQL_SUCCESS_WITH_INFO
    }
}

fn write_date_struct(
    value: SqlDateStruct,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut SqlDateStruct) = value;
    }
    write_str_len_or_ind(
        std::mem::size_of::<SqlDateStruct>() as SqlLen,
        str_len_or_ind_ptr,
    );
    SqlReturn::SQL_SUCCESS
}

fn write_time_struct(
    value: SqlTimeStruct,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut SqlTimeStruct) = value;
    }
    write_str_len_or_ind(
        std::mem::size_of::<SqlTimeStruct>() as SqlLen,
        str_len_or_ind_ptr,
    );
    SqlReturn::SQL_SUCCESS
}

fn write_timestamp_struct(
    value: SqlTimestampStruct,
    target_value_ptr: SqlPointer,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    unsafe {
        *(target_value_ptr as *mut SqlTimestampStruct) = value;
    }
    write_str_len_or_ind(
        std::mem::size_of::<SqlTimestampStruct>() as SqlLen,
        str_len_or_ind_ptr,
    );
    SqlReturn::SQL_SUCCESS
}

fn write_str_len_or_ind(value: SqlLen, str_len_or_ind_ptr: *mut SqlLen) {
    if !str_len_or_ind_ptr.is_null() {
        unsafe {
            *str_len_or_ind_ptr = value;
        }
    }
}
