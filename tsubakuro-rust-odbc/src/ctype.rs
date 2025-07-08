use std::ffi::c_void;

mod c_data_type;
mod sql_data_type;
pub mod sql_date_struct;
pub mod sql_numeric_struct;
mod sql_return;
pub mod sql_time_struct;
pub mod sql_timestamp_struct;

pub(crate) use c_data_type::*;
pub(crate) use sql_data_type::*;
pub use sql_return::*;

pub type SqlChar = u8;
pub type SqlSChar = i8;
pub type SqlWChar = u16;
pub type SqlSmallInt = i16;
pub type SqlUSmallInt = u16;
pub type SqlInteger = i32;
pub type SqlUInteger = u32;
pub type SqlPointer = *mut c_void;

// 8 byte
pub type SqlLen = isize;
pub type SqlULen = usize;
pub type HWnd = SqlPointer;

pub const SQL_FALSE: i32 = 0;
pub const SQL_TRUE: i32 = 1;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SqlNullable {
    SQL_NO_NULLS = 0,
    SQL_NULLABLE = 1,
    SQL_NULLABLE_UNKNOWN = 2,
}

#[allow(dead_code)]
pub const SQL_PRED_NONE: i32 = 0;
pub const SQL_PRED_CHAR: i32 = 1;
pub const SQL_PRED_BASIC: i32 = 2;

pub const SQL_NULL_DATA: SqlLen = -1;
