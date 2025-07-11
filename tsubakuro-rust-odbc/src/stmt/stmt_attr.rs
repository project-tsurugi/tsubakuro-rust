use log::{debug, trace, warn};

use crate::{
    check_stmt,
    ctype::{SqlInteger, SqlPointer, SqlReturn},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
};

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum StatementAttribute {
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
                log::debug!("{FUNCTION_NAME}: Unsupported attribute {:?}", $attribute);
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
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    let attribute = statement_attribute!(stmt, attribute);

    let rc = set_stmt_attr(&stmt, attribute, value_ptr, string_length, false);

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
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    let attribute = statement_attribute!(stmt, attribute);

    let rc = set_stmt_attr(&stmt, attribute, value_ptr, string_length, true);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn set_stmt_attr(
    stmt: &TsurugiOdbcStmt,
    attribute: StatementAttribute,
    _value_ptr: SqlPointer,
    _string_length: SqlInteger,
    _wide_char: bool,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "set_stmt_attr()";

    warn!(
        "{stmt}.{FUNCTION_NAME}: Unsupported attribute {:?}",
        attribute
    );
    stmt.add_diag(
        TsurugiOdbcError::InvalidAttribute,
        format!("Unsupported attribute {:?}", attribute),
    );
    SqlReturn::SQL_SUCCESS_WITH_INFO
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
    _wide_char: bool,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_stmt_attr()";

    if value_ptr.is_null() {
        warn!("{stmt}.{FUNCTION_NAME}: value_ptr is null");
        stmt.add_diag(TsurugiOdbcError::InvalidValuePtr, "value_ptr is null");
        return SqlReturn::SQL_ERROR;
    }

    match attribute {
        StatementAttribute::SQL_ATTR_APP_ROW_DESC => {
            let name = "SQL_ATTR_APP_ROW_DESC";
            let value = 0x11223344 as SqlPointer;
            debug!("{stmt}.{FUNCTION_NAME}: {}={:?}", name, value);
            write_pointer(value, value_ptr)
        }
        StatementAttribute::SQL_ATTR_APP_PARAM_DESC => {
            let name = "SQL_ATTR_APP_PARAM_DESC";
            let value = 0x11223355 as SqlPointer;
            debug!("{stmt}.{FUNCTION_NAME}: {}={:?}", name, value);
            write_pointer(value, value_ptr)
        }
        StatementAttribute::SQL_ATTR_IMP_ROW_DESC => {
            let name = "SQL_ATTR_IMP_ROW_DESC";
            let value = 0x11223366 as SqlPointer;
            debug!("{stmt}.{FUNCTION_NAME}: {}={:?}", name, value);
            write_pointer(value, value_ptr)
        }
        StatementAttribute::SQL_ATTR_IMP_PARAM_DESC => {
            let name = "SQL_ATTR_IMP_PARAM_DESC";
            let value = 0x11223377 as SqlPointer;
            debug!("{stmt}.{FUNCTION_NAME}: {}={:?}", name, value);
            write_pointer(value, value_ptr)
        }
    }
}

fn write_pointer(value: SqlPointer, value_ptr: SqlPointer) -> SqlReturn {
    unsafe {
        *(value_ptr as *mut SqlPointer) = value;
    }

    SqlReturn::SQL_SUCCESS
}
