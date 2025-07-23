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
    type Error = i32;

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
            e => Err(e),
        }
    }
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

    let delegator = SetStmtAttr::new(attribute, value_ptr, string_length, false);
    let rc = delegator.set_stmt_attr(&mut stmt);

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

    let delegator = SetStmtAttr::new(attribute, value_ptr, string_length, true);
    let rc = delegator.set_stmt_attr(&mut stmt);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

struct SetStmtAttr {
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    _string_length: SqlInteger,
    wide_char: bool,
}

impl SetStmtAttr {
    fn new(
        attribute: SqlInteger,
        value_ptr: SqlPointer,
        string_length: SqlInteger,
        wide_char: bool,
    ) -> SetStmtAttr {
        SetStmtAttr {
            attribute,
            value_ptr,
            _string_length: string_length,
            wide_char,
        }
    }

    fn odbc_function_name(&self) -> &str {
        if self.wide_char {
            "SQLSetStmtAttrW()"
        } else {
            "SQLSetStmtAttr()"
        }
    }

    fn set_stmt_attr(&self, stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        const FUNCTION_NAME: &str = "set_stmt_attr()";

        let attribute = match StatementAttribute::try_from(self.attribute) {
            Ok(value) => value,
            Err(attribute) => {
                debug!(
                    "{FUNCTION_NAME} error. Unsupported attribute {:?}",
                    attribute
                );
                let odbc_function_name = self.odbc_function_name();
                stmt.add_diag(
                    TsurugiOdbcError::StmtAttrUnsupportedAttribute,
                    format!(
                        "{odbc_function_name}: Unsupported attribute {:?}",
                        attribute
                    ),
                );
                return SqlReturn::SQL_ERROR;
            }
        };

        use StatementAttribute::*;
        match attribute {
            SQL_ATTR_QUERY_TIMEOUT => {
                let value = self.read_ulen() as u64;
                debug!("{stmt}.{FUNCTION_NAME}: {:?}={}", attribute, value);
                stmt.set_query_timeout(value);
            }
            _ => {
                warn!(
                    "{stmt}.{FUNCTION_NAME}: Unsupported attribute {:?}",
                    attribute
                );
                stmt.add_diag(
                    TsurugiOdbcError::StmtAttrUnsupportedAttribute,
                    format!("Unsupported attribute {:?}", attribute),
                );
                return SqlReturn::SQL_SUCCESS_WITH_INFO;
            }
        }

        SqlReturn::SQL_SUCCESS
    }

    fn read_ulen(&self) -> SqlULen {
        self.value_ptr as SqlULen
    }
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

    let delegator = GetStmtAttr::new(
        attribute,
        value_ptr,
        buffer_length,
        string_length_ptr,
        false,
    );
    let rc = delegator.get_stmt_attr(&stmt);

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

    let delegator = GetStmtAttr::new(attribute, value_ptr, buffer_length, string_length_ptr, true);
    let rc = delegator.get_stmt_attr(&stmt);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

struct GetStmtAttr {
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    _buffer_length: SqlInteger,
    _string_length_ptr: *mut SqlInteger,
    wide_char: bool,
}

impl GetStmtAttr {
    fn new(
        attribute: SqlInteger,
        value_ptr: SqlPointer,
        buffer_length: SqlInteger,
        string_length_ptr: *mut SqlInteger,
        wide_char: bool,
    ) -> GetStmtAttr {
        GetStmtAttr {
            attribute,
            value_ptr,
            _buffer_length: buffer_length,
            _string_length_ptr: string_length_ptr,
            wide_char,
        }
    }

    fn odbc_function_name(&self) -> &str {
        if self.wide_char {
            "SQLGetStmtAttrW()"
        } else {
            "SQLGetStmtAttr()"
        }
    }

    fn get_stmt_attr(&self, stmt: &TsurugiOdbcStmt) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_stmt_attr()";

        let attribute = match StatementAttribute::try_from(self.attribute) {
            Ok(value) => value,
            Err(attribute) => {
                debug!(
                    "{FUNCTION_NAME} error. Unsupported attribute {:?}",
                    attribute
                );
                let odbc_function_name = self.odbc_function_name();
                stmt.add_diag(
                    TsurugiOdbcError::StmtAttrUnsupportedAttribute,
                    format!(
                        "{odbc_function_name}: Unsupported attribute {:?}",
                        attribute
                    ),
                );
                return SqlReturn::SQL_ERROR;
            }
        };

        if self.value_ptr.is_null() {
            warn!("{stmt}.{FUNCTION_NAME} error. value_ptr is null");
            let odbc_function_name = self.odbc_function_name();
            stmt.add_diag(
                TsurugiOdbcError::GetStmtAttrInvalidValuePtr,
                format!("{odbc_function_name}: value_ptr is null"),
            );
            return SqlReturn::SQL_ERROR;
        }

        use StatementAttribute::*;
        match attribute {
            SQL_ATTR_APP_ROW_DESC => {
                let value = 0x11223344 as SqlPointer;
                self.write_pointer(stmt, attribute, value)
            }
            SQL_ATTR_APP_PARAM_DESC => {
                let value = 0x11223355 as SqlPointer;
                self.write_pointer(stmt, attribute, value)
            }
            SQL_ATTR_IMP_ROW_DESC => {
                let value = 0x11223366 as SqlPointer;
                self.write_pointer(stmt, attribute, value)
            }
            SQL_ATTR_IMP_PARAM_DESC => {
                let value = 0x11223377 as SqlPointer;
                self.write_pointer(stmt, attribute, value)
            }
            SQL_ATTR_QUERY_TIMEOUT => {
                let value = stmt.query_timeout() as SqlULen;
                self.write_ulen(stmt, attribute, value)
            }
            _ => {
                debug!(
                    "{stmt}.{FUNCTION_NAME} error. Unsupported attribute {:?}",
                    attribute
                );
                let odbc_function_name = self.odbc_function_name();
                stmt.add_diag(
                    TsurugiOdbcError::StmtAttrUnsupportedAttribute,
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
        &self,
        stmt: &TsurugiOdbcStmt,
        attribute: StatementAttribute,
        value: SqlPointer,
    ) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_stmt_attr().write_pointer()";

        debug!("{stmt}.{FUNCTION_NAME}: {:?}={:?}", attribute, value);

        unsafe {
            *(self.value_ptr as *mut SqlPointer) = value;
        }

        SqlReturn::SQL_SUCCESS
    }

    fn write_ulen(
        &self,
        stmt: &TsurugiOdbcStmt,
        attribute: StatementAttribute,
        value: SqlULen,
    ) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_stmt_attr().write_ulen()";

        debug!("{stmt}.{FUNCTION_NAME}: {:?}={}", attribute, value);

        unsafe {
            *(self.value_ptr as *mut SqlULen) = value;
        }

        SqlReturn::SQL_SUCCESS
    }
}
