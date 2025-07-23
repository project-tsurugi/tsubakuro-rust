use std::sync::Arc;

use log::{debug, trace, warn};

use crate::{
    check_dbc,
    ctype::{SqlInteger, SqlPointer, SqlReturn, SqlUInteger},
    handle::{
        diag::TsurugiOdbcError,
        hdbc::{HDbc, TsurugiOdbcDbc},
    },
};

/// connection attribute
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum ConnectionAttribute {
    SQL_ATTR_AUTOCOMMIT = 102,
    SQL_ATTR_LOGIN_TIMEOUT = 103,
    SQL_ATTR_CONNECTION_TIMEOUT = 113,
    SQL_ATTR_ANSI_APP = 115,
    SQL_ATTR_CONNECTION_DEAD = 1209,
}

// SQL_ATTR_AUTOCOMMIT
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum AutoCommit {
    SQL_AUTOCOMMIT_OFF = 0,
    SQL_AUTOCOMMIT_ON = 1,
}

// SQL_ATTR_CONNECTION_DEAD
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum ConnectionDead {
    SQL_CD_TRUE = 1,
    SQL_CD_FALSE = 0,
}

impl TryFrom<i32> for ConnectionAttribute {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use ConnectionAttribute::*;
        match value {
            102 => Ok(SQL_ATTR_AUTOCOMMIT),
            103 => Ok(SQL_ATTR_LOGIN_TIMEOUT),
            113 => Ok(SQL_ATTR_CONNECTION_TIMEOUT),
            115 => Ok(SQL_ATTR_ANSI_APP),
            1209 => Ok(SQL_ATTR_CONNECTION_DEAD),
            e => Err(e),
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLSetConnectAttr(
    hdbc: HDbc,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    string_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLSetConnectAttr()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, attr={:?}, value_ptr={:?}, string_length={:?}",
        hdbc,
        attribute,
        value_ptr,
        string_length
    );

    let dbc = check_dbc!(hdbc);
    let delegator = SetConnectAttr::new(dbc, attribute, value_ptr, string_length, false);
    let rc = delegator.set_connect_attr();

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLSetConnectAttrW(
    hdbc: HDbc,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    string_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLSetConnectAttrW()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, attribute={:?}, value_ptr={:?}, string_length={:?}",
        hdbc,
        attribute,
        value_ptr,
        string_length
    );

    let dbc = check_dbc!(hdbc);
    let delegator = SetConnectAttr::new(dbc, attribute, value_ptr, string_length, true);
    let rc = delegator.set_connect_attr();

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

struct SetConnectAttr {
    dbc: Arc<TsurugiOdbcDbc>,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    _string_length: SqlInteger,
    wide_char: bool,
}

impl SetConnectAttr {
    fn new(
        dbc: Arc<TsurugiOdbcDbc>,
        attribute: SqlInteger,
        value_ptr: SqlPointer,
        string_length: SqlInteger,
        wide_char: bool,
    ) -> SetConnectAttr {
        SetConnectAttr {
            dbc,
            attribute,
            value_ptr,
            _string_length: string_length,
            wide_char,
        }
    }

    fn odbc_function_name(&self) -> &'static str {
        if self.wide_char {
            "SetConnectAttrW()"
        } else {
            "SetConnectAttr()"
        }
    }

    fn set_connect_attr(&self) -> SqlReturn {
        const FUNCTION_NAME: &str = "set_connect_attr()";

        let dbc = &self.dbc;

        let attribute = match ConnectionAttribute::try_from(self.attribute) {
            Ok(value) => value,
            Err(attribute) => {
                warn!(
                    "{FUNCTION_NAME} error. Unsupported attribute {:?}",
                    attribute
                );
                let odbc_function_name = self.odbc_function_name();
                dbc.add_diag(
                    TsurugiOdbcError::ConnectAttrUnsupportedAttribute,
                    format!(
                        "{odbc_function_name}: Unsupported attribute {:?}",
                        attribute
                    ),
                );
                return SqlReturn::SQL_ERROR;
            }
        };

        use ConnectionAttribute::*;
        match attribute {
            SQL_ATTR_AUTOCOMMIT => {
                let value = self.value_ptr as SqlUInteger;
                let value = value != AutoCommit::SQL_AUTOCOMMIT_OFF as SqlUInteger;
                debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", attribute, value);
                dbc.set_auto_commit(value)
            }
            SQL_ATTR_LOGIN_TIMEOUT | SQL_ATTR_CONNECTION_TIMEOUT => {
                let value = self.value_ptr as SqlUInteger as u64;
                debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", attribute, value);
                dbc.set_connection_timeout(value)
            }
            SQL_ATTR_ANSI_APP => {
                let value = self.value_ptr as SqlInteger;
                let value = match value {
                    1 => "SQL_AA_TRUE",
                    0 => "SQL_AA_FALSE",
                    _ => &value.to_string(),
                };
                debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", attribute, value);
                SqlReturn::SQL_SUCCESS
            }
            SQL_ATTR_CONNECTION_DEAD => {
                warn!(
                    "{dbc}.{FUNCTION_NAME} error. Unsupported attribute {:?}",
                    attribute
                );
                let odbc_function_name = self.odbc_function_name();
                dbc.add_diag(
                    TsurugiOdbcError::ConnectAttrUnsupportedAttribute,
                    format!(
                        "{odbc_function_name}: Unsupported attribute {:?}",
                        attribute
                    ),
                );
                SqlReturn::SQL_ERROR
            }
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLGetConnectAttr(
    hdbc: HDbc,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    buffer_length: SqlInteger,
    string_length_ptr: *mut SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetConnectAttr()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, attribute={:?}, value_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        hdbc,
        attribute,
        value_ptr,
        buffer_length,
        string_length_ptr
    );

    let dbc = check_dbc!(hdbc);
    let delegator = GetConnectAttr::new(
        dbc,
        attribute,
        value_ptr,
        buffer_length,
        string_length_ptr,
        false,
    );
    let rc = delegator.get_connect_attr();

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLGetConnectAttrW(
    hdbc: HDbc,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    buffer_length: SqlInteger,
    string_length_ptr: *mut SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetConnectAttrW()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, attribute={:?}, value_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        hdbc,
        attribute,
        value_ptr,
        buffer_length,
        string_length_ptr
    );

    let dbc = check_dbc!(hdbc);
    let delegator = GetConnectAttr::new(
        dbc,
        attribute,
        value_ptr,
        buffer_length,
        string_length_ptr,
        true,
    );
    let rc = delegator.get_connect_attr();

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

struct GetConnectAttr {
    dbc: Arc<TsurugiOdbcDbc>,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    _buffer_length: SqlInteger,
    _string_length_ptr: *mut SqlInteger,
    wide_char: bool,
}

impl GetConnectAttr {
    fn new(
        dbc: Arc<TsurugiOdbcDbc>,
        attribute: SqlInteger,
        value_ptr: SqlPointer,
        buffer_length: SqlInteger,
        string_length_ptr: *mut SqlInteger,
        wide_char: bool,
    ) -> GetConnectAttr {
        GetConnectAttr {
            dbc,
            attribute,
            value_ptr,
            _buffer_length: buffer_length,
            _string_length_ptr: string_length_ptr,
            wide_char,
        }
    }

    fn odbc_function_name(&self) -> &'static str {
        if self.wide_char {
            "GetConnectAttrW()"
        } else {
            "GetConnectAttr()"
        }
    }

    fn get_connect_attr(&self) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_connect_attr()";

        let dbc = &self.dbc;

        let attribute = match ConnectionAttribute::try_from(self.attribute) {
            Ok(value) => value,
            Err(attribute) => {
                warn!(
                    "{FUNCTION_NAME} error. Unsupported attribute {:?}",
                    attribute
                );
                let odbc_function_name = self.odbc_function_name();
                dbc.add_diag(
                    TsurugiOdbcError::ConnectAttrUnsupportedAttribute,
                    format!(
                        "{odbc_function_name}: Unsupported attribute {:?}",
                        attribute
                    ),
                );
                return SqlReturn::SQL_ERROR;
            }
        };

        if self.value_ptr.is_null() {
            debug!("{dbc}.{FUNCTION_NAME} error. value_ptr is null");
            let odbc_function_name = self.odbc_function_name();
            dbc.add_diag(
                TsurugiOdbcError::GetConnectAttrInvalidValuePtr,
                format!("{odbc_function_name}: value_ptr is null"),
            );
            return SqlReturn::SQL_ERROR;
        }

        use ConnectionAttribute::*;
        match attribute {
            SQL_ATTR_AUTOCOMMIT => {
                let value = get_auto_commit(dbc);
                debug!("{dbc}.{FUNCTION_NAME}: {:?}={:?}", attribute, value);
                self.write_uinteger(value as SqlUInteger)
            }
            SQL_ATTR_LOGIN_TIMEOUT | SQL_ATTR_CONNECTION_TIMEOUT => {
                let value = dbc.connection_timeout();
                debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", attribute, value);
                self.write_uinteger(value as SqlUInteger)
            }
            SQL_ATTR_CONNECTION_DEAD => {
                let value = get_connection_dead(dbc);
                debug!("{dbc}.{FUNCTION_NAME}: {:?}={:?}", attribute, value);
                self.write_uinteger(value as SqlUInteger)
            }
            SQL_ATTR_ANSI_APP => {
                warn!(
                    "{dbc}.{FUNCTION_NAME} error. Unsupported attribute {:?}",
                    attribute
                );
                let odbc_function_name = self.odbc_function_name();
                dbc.add_diag(
                    TsurugiOdbcError::ConnectAttrUnsupportedAttribute,
                    format!(
                        "{odbc_function_name}: Unsupported attribute {:?}",
                        attribute
                    ),
                );
                SqlReturn::SQL_ERROR
            }
        }
    }

    fn write_uinteger(&self, value: SqlUInteger) -> SqlReturn {
        let ptr = self.value_ptr as *mut SqlUInteger;
        unsafe {
            *ptr = value;
        }
        SqlReturn::SQL_SUCCESS
    }
}

fn get_auto_commit(dbc: &Arc<TsurugiOdbcDbc>) -> AutoCommit {
    let value = dbc.auto_commit();
    if value {
        AutoCommit::SQL_AUTOCOMMIT_ON
    } else {
        AutoCommit::SQL_AUTOCOMMIT_OFF
    }
}

fn get_connection_dead(dbc: &Arc<TsurugiOdbcDbc>) -> ConnectionDead {
    if let Some(session) = dbc.session() {
        // FIXME session.is_alive()
        if session.is_closed() || session.is_shutdowned() {
            // fall through
        } else {
            return ConnectionDead::SQL_CD_FALSE;
        }
    }
    ConnectionDead::SQL_CD_TRUE
}
