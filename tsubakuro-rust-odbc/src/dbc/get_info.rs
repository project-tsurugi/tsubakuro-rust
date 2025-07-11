use std::sync::Arc;

use log::{debug, trace};

use crate::{
    check_dbc,
    ctype::{SqlChar, SqlPointer, SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar},
    handle::{
        diag::TsurugiOdbcError,
        hdbc::{HDbc, TsurugiOdbcDbc},
    },
    util::{write_char, write_wchar_bytes},
};

#[no_mangle]
pub extern "system" fn SQLGetInfo(
    hdbc: HDbc,
    info_type: SqlUSmallInt,
    info_value_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetInfo()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, info_type={:?}, info_value_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        hdbc, info_type, info_value_ptr, buffer_length, string_length_ptr
    );

    let dbc = check_dbc!(hdbc);

    let info = SqlGetInfo::new(dbc, info_value_ptr, buffer_length, string_length_ptr, false);
    let rc = info.get_info(info_type);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLGetInfoW(
    hdbc: HDbc,
    info_type: SqlUSmallInt,
    info_value_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetInfoW()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, info_type={:?}, info_value_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        hdbc, info_type, info_value_ptr, buffer_length, string_length_ptr
    );

    let dbc = check_dbc!(hdbc);

    let info = SqlGetInfo::new(dbc, info_value_ptr, buffer_length, string_length_ptr, true);
    let rc = info.get_info(info_type);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum SqlCursorBehavior {
    SQL_CB_DELETE = 0,
    SQL_CB_CLOSE = 1,
    SQL_CB_PRESERVE = 2,
}

struct SqlGetInfo {
    dbc: Arc<TsurugiOdbcDbc>,
    info_value_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
    wide_char: bool,
}

impl SqlGetInfo {
    fn new(
        dbc: Arc<TsurugiOdbcDbc>,
        info_value_ptr: SqlPointer,
        buffer_length: SqlSmallInt,
        string_length_ptr: *mut SqlSmallInt,
        wide_char: bool,
    ) -> SqlGetInfo {
        SqlGetInfo {
            dbc,
            info_value_ptr,
            buffer_length,
            string_length_ptr,
            wide_char,
        }
    }

    fn get_info(&self, info_type: SqlUSmallInt) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_info()";

        let dbc = &self.dbc;

        match info_type {
            23 => {
                let name = "SQL_CURSOR_COMMIT_BEHAVIOR";
                let value = SqlCursorBehavior::SQL_CB_DELETE;
                debug!("{dbc}.{FUNCTION_NAME}: {}={:?}", name, value);
                self.write_integer(value as i32, 2)
            }
            24 => {
                let name = "SQL_CURSOR_ROLLBACK_BEHAVIOR";
                let value = SqlCursorBehavior::SQL_CB_DELETE;
                debug!("{dbc}.{FUNCTION_NAME}: {}={:?}", name, value);
                self.write_integer(value as i32, 2)
            }
            77 => {
                let name = "SQL_DRIVER_ODBC_VER";
                let value = "03.51";
                debug!("{dbc}.{FUNCTION_NAME}: {}={}", name, value);
                self.write_string(value)
            }
            81 => {
                let name = "SQL_GETDATA_EXTENSIONS";
                let value = 0;
                debug!("{dbc}.{FUNCTION_NAME}: {}={}", name, value);
                self.write_integer(value, 4)
            }
            _ => {
                debug!(
                    "{dbc}.{FUNCTION_NAME} error. Unsupported info type: {}",
                    info_type
                );
                dbc.add_diag(
                    TsurugiOdbcError::UnsupportedInfoType,
                    format!("Unsupported info type: {}", info_type),
                );
                SqlReturn::SQL_ERROR
            }
        }
    }

    fn write_string(&self, value: &str) -> SqlReturn {
        if self.wide_char {
            write_wchar_bytes(
                "SQLGetInfoW.info_value_ptr",
                value,
                self.info_value_ptr as *mut SqlWChar,
                self.buffer_length,
                self.string_length_ptr,
                Some(&self.dbc.diag_collection()),
            )
        } else {
            write_char(
                "SQLGetInfo.info_value_ptr",
                value,
                self.info_value_ptr as *mut SqlChar,
                self.buffer_length,
                self.string_length_ptr,
                Some(&self.dbc.diag_collection()),
            )
        }
    }

    fn write_integer(&self, value: i32, buffer_length: SqlSmallInt) -> SqlReturn {
        const FUNCTION_NAME: &str = "write_integer()";
        match buffer_length {
            2 => {
                let int_ptr = self.info_value_ptr as *mut i16;
                unsafe {
                    *int_ptr = value as i16;
                }
                SqlReturn::SQL_SUCCESS
            }
            4 => {
                let int_ptr = self.info_value_ptr as *mut i32;
                unsafe {
                    *int_ptr = value;
                }
                SqlReturn::SQL_SUCCESS
            }
            _ => {
                debug!(
                    "{FUNCTION_NAME} error. Unsupported buffer_length: {}",
                    buffer_length
                );
                self.dbc.add_diag(
                    TsurugiOdbcError::UnsupportedBufferLength,
                    format!("Unsupported buffer_length: {}", buffer_length),
                );
                SqlReturn::SQL_ERROR
            }
        }
    }
}
