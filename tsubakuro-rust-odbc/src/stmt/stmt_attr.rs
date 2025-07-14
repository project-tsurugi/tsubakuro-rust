use log::{debug, trace, warn};

use crate::{
    check_stmt,
    ctype::{SqlInteger, SqlPointer, SqlReturn, SqlULen},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
};

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum StatementAttribute {
    SQL_ATTR_ASYNC_ENABLE = 4, // SQL_ASYNC_ENABLE
    SQL_ATTR_CONCURRENCY = 7,  // SQL_CONCURRENCY
    SQL_ATTR_CURSOR_TYPE = 6,  // SQL_CURSOR_TYPE
    SQL_ATTR_ENABLE_AUTO_IPD = 15,
    SQL_ATTR_FETCH_BOOKMARK_PTR = 16,
    SQL_ATTR_KEYSET_SIZE = 8, // SQL_KEYSET_SIZE
    SQL_ATTR_MAX_LENGTH = 3,  // SQL_MAX_LENGTH
    SQL_ATTR_MAX_ROWS = 1,    // SQL_MAX_ROWS
    SQL_ATTR_NOSCAN = 2,      // SQL_NOSCAN
    SQL_ATTR_PARAM_BIND_OFFSET_PTR = 17,
    SQL_ATTR_PARAM_BIND_TYPE = 18,
    SQL_ATTR_PARAM_OPERATION_PTR = 19,
    SQL_ATTR_PARAM_STATUS_PTR = 20,
    SQL_ATTR_PARAMS_PROCESSED_PTR = 21,
    SQL_ATTR_PARAMSET_SIZE = 22,
    SQL_ATTR_QUERY_TIMEOUT = 0,  // SQL_QUERY_TIMEOUT
    SQL_ATTR_RETRIEVE_DATA = 11, // SQL_RETRIEVE_DATA
    SQL_ATTR_ROW_BIND_OFFSET_PTR = 23,
    SQL_ATTR_ROW_BIND_TYPE = 5, // SQL_BIND_TYPE
    SQL_ATTR_ROW_NUMBER = 14,   // SQL_ROW_NUMBER
    SQL_ATTR_ROW_OPERATION_PTR = 24,
    SQL_ATTR_ROW_STATUS_PTR = 25,
    SQL_ATTR_ROWS_FETCHED_PTR = 26,
    SQL_ATTR_ROW_ARRAY_SIZE = 27,
    SQL_ATTR_SIMULATE_CURSOR = 10, // SQL_SIMULATE_CURSOR
    SQL_ATTR_USE_BOOKMARKS = 12,   // SQL_USE_BOOKMARKS

    SQL_ATTR_APP_ROW_DESC = 10010,
    SQL_ATTR_APP_PARAM_DESC = 10011,
    SQL_ATTR_IMP_ROW_DESC = 10012,
    SQL_ATTR_IMP_PARAM_DESC = 10013,
}

impl TryFrom<i32> for StatementAttribute {
    type Error = TsurugiOdbcError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use StatementAttribute::*;
        match value {
            4 => Ok(SQL_ATTR_ASYNC_ENABLE),
            7 => Ok(SQL_ATTR_CONCURRENCY),
            6 => Ok(SQL_ATTR_CURSOR_TYPE),
            15 => Ok(SQL_ATTR_ENABLE_AUTO_IPD),
            16 => Ok(SQL_ATTR_FETCH_BOOKMARK_PTR),
            8 => Ok(SQL_ATTR_KEYSET_SIZE),
            3 => Ok(SQL_ATTR_MAX_LENGTH),
            1 => Ok(SQL_ATTR_MAX_ROWS),
            2 => Ok(SQL_ATTR_NOSCAN),
            17 => Ok(SQL_ATTR_PARAM_BIND_OFFSET_PTR),
            18 => Ok(SQL_ATTR_PARAM_BIND_TYPE),
            19 => Ok(SQL_ATTR_PARAM_OPERATION_PTR),
            20 => Ok(SQL_ATTR_PARAM_STATUS_PTR),
            21 => Ok(SQL_ATTR_PARAMS_PROCESSED_PTR),
            22 => Ok(SQL_ATTR_PARAMSET_SIZE),
            0 => Ok(SQL_ATTR_QUERY_TIMEOUT),
            11 => Ok(SQL_ATTR_RETRIEVE_DATA),
            23 => Ok(SQL_ATTR_ROW_BIND_OFFSET_PTR),
            5 => Ok(SQL_ATTR_ROW_BIND_TYPE),
            14 => Ok(SQL_ATTR_ROW_NUMBER),
            24 => Ok(SQL_ATTR_ROW_OPERATION_PTR),
            25 => Ok(SQL_ATTR_ROW_STATUS_PTR),
            26 => Ok(SQL_ATTR_ROWS_FETCHED_PTR),
            27 => Ok(SQL_ATTR_ROW_ARRAY_SIZE),
            10 => Ok(SQL_ATTR_SIMULATE_CURSOR),
            12 => Ok(SQL_ATTR_USE_BOOKMARKS),
            10010 => Ok(SQL_ATTR_APP_ROW_DESC),
            10011 => Ok(SQL_ATTR_APP_PARAM_DESC),
            10012 => Ok(SQL_ATTR_IMP_ROW_DESC),
            10013 => Ok(SQL_ATTR_IMP_PARAM_DESC),
            _ => Err(TsurugiOdbcError::InvalidAttribute),
        }
    }
}

macro_rules! statement_attribute {
    ($stmt:expr, $attribute:expr) => {
        match StatementAttribute::try_from($attribute) {
            Ok(value) => value,
            Err(e) => {
                log::debug!(
                    "{FUNCTION_NAME} error. Unsupported attribute {:?}",
                    $attribute
                );
                $stmt.add_diag(
                    e,
                    format!("{FUNCTION_NAME}: Unsupported attribute {:?}", $attribute),
                );
                let rc = SqlReturn::SQL_ERROR;
                log::trace!("{FUNCTION_NAME} end. rc={:?}", rc);
                return rc;
            }
        }
    };
}

#[no_mangle]
pub extern "C" fn SQLSetStmtAttr(
    hstmt: HStmt,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    string_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLSetStmtAttr()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, attribute={:?}, value_ptr={:?}, string_length={:?}",
        hstmt,
        attribute,
        value_ptr,
        string_length
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let attribute = statement_attribute!(stmt, attribute);

    let rc = match set_stmt_attr(&mut stmt, attribute, value_ptr, string_length, false) {
        Ok(_) => SqlReturn::SQL_SUCCESS,
        Err(rc) => rc,
    };

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn SQLSetStmtAttrW(
    hstmt: HStmt,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    string_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLSetStmtAttrW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, attribute={:?}, value_ptr={:?}, string_length={:?}",
        hstmt,
        attribute,
        value_ptr,
        string_length
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let attribute = statement_attribute!(stmt, attribute);

    let rc = match set_stmt_attr(&mut stmt, attribute, value_ptr, string_length, true) {
        Ok(_) => SqlReturn::SQL_SUCCESS,
        Err(rc) => rc,
    };

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn set_stmt_attr(
    stmt: &mut TsurugiOdbcStmt,
    attribute: StatementAttribute,
    value_ptr: SqlPointer,
    _string_length: SqlInteger,
    _wide_char: bool,
) -> Result<(), SqlReturn> {
    const FUNCTION_NAME: &str = "set_stmt_attr()";

    use StatementAttribute::*;
    match attribute {
        SQL_ATTR_QUERY_TIMEOUT => {
            let value = read_ulen(value_ptr) as u64;
            debug!("{stmt}.{FUNCTION_NAME}: {:?}={}", attribute, value);
            stmt.set_query_timeout(value);
        }
        _ => {
            warn!(
                "{stmt}.{FUNCTION_NAME}: Unsupported attribute {:?}",
                attribute
            );
            stmt.add_diag(
                TsurugiOdbcError::InvalidAttribute,
                format!("Unsupported attribute {:?}", attribute),
            );
            return Err(SqlReturn::SQL_SUCCESS_WITH_INFO);
        }
    }
    Ok(())
}

fn read_ulen(value_ptr: SqlPointer) -> SqlULen {
    value_ptr as SqlULen
}

#[no_mangle]
pub extern "system" fn SQLGetStmtAttr(
    hstmt: HStmt,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    buffer_length: SqlInteger,
    string_length_ptr: *mut SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetStmtAttr()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, attribute={:?}, value_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        hstmt, attribute, value_ptr, buffer_length, string_length_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    let attribute = statement_attribute!(stmt, attribute);

    let rc = get_stmt_attr(
        &stmt,
        attribute,
        value_ptr,
        buffer_length,
        string_length_ptr,
        false,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLGetStmtAttrW(
    hstmt: HStmt,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    buffer_length: SqlInteger,
    string_length_ptr: *mut SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetStmtAttrW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, attribute={:?}, value_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        hstmt, attribute, value_ptr, buffer_length, string_length_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    let attribute = statement_attribute!(stmt, attribute);

    let rc = get_stmt_attr(
        &stmt,
        attribute,
        value_ptr,
        buffer_length,
        string_length_ptr,
        true,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn get_stmt_attr(
    stmt: &TsurugiOdbcStmt,
    attribute: StatementAttribute,
    value_ptr: SqlPointer,
    _buffer_length: SqlInteger,
    _string_length_ptr: *mut SqlInteger,
    wide_char: bool,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_stmt_attr()";

    if value_ptr.is_null() {
        warn!("{stmt}.{FUNCTION_NAME} error. value_ptr is null");
        stmt.add_diag(TsurugiOdbcError::InvalidValuePtr, "value_ptr is null");
        return SqlReturn::SQL_ERROR;
    }

    use StatementAttribute::*;
    match attribute {
        SQL_ATTR_APP_ROW_DESC => {
            let value = 0x11223344 as SqlPointer;
            write_pointer(stmt, attribute, value, value_ptr)
        }
        SQL_ATTR_APP_PARAM_DESC => {
            let value = 0x11223355 as SqlPointer;
            write_pointer(stmt, attribute, value, value_ptr)
        }
        SQL_ATTR_IMP_ROW_DESC => {
            let value = 0x11223366 as SqlPointer;
            write_pointer(stmt, attribute, value, value_ptr)
        }
        SQL_ATTR_IMP_PARAM_DESC => {
            let value = 0x11223377 as SqlPointer;
            write_pointer(stmt, attribute, value, value_ptr)
        }
        SQL_ATTR_QUERY_TIMEOUT => {
            let value = stmt.query_timeout() as SqlULen;
            write_ulen(stmt, attribute, value, value_ptr)
        }
        _ => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported attribute {:?}",
                attribute
            );
            let odbc_function_name = if wide_char {
                "SQLGetStmtAttrW"
            } else {
                "SQLGetStmtAttrA"
            };
            stmt.add_diag(
                TsurugiOdbcError::InvalidAttribute,
                format!(
                    "{odbc_function_name}: Unsupported attribute {:?}",
                    attribute
                ),
            );
            SqlReturn::SQL_ERROR
        }
    }
}

fn write_pointer(
    stmt: &TsurugiOdbcStmt,
    attribute: StatementAttribute,
    value: SqlPointer,
    value_ptr: SqlPointer,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_stmt_attr().write_pointer()";

    debug!("{stmt}.{FUNCTION_NAME}: {:?}={:?}", attribute, value);

    unsafe {
        *(value_ptr as *mut SqlPointer) = value;
    }

    SqlReturn::SQL_SUCCESS
}

fn write_ulen(
    stmt: &TsurugiOdbcStmt,
    attribute: StatementAttribute,
    value: SqlULen,
    value_ptr: SqlPointer,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_stmt_attr().write_ulen()";

    debug!("{stmt}.{FUNCTION_NAME}: {:?}={}", attribute, value);

    unsafe {
        *(value_ptr as *mut SqlULen) = value;
    }

    SqlReturn::SQL_SUCCESS
}
