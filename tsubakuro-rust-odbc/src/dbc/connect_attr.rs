use std::sync::Arc;

use log::{debug, trace};

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
}

// SQL_ATTR_AUTOCOMMIT
const SQL_AUTOCOMMIT_OFF: SqlUInteger = 0;
const SQL_AUTOCOMMIT_ON: SqlUInteger = 1;

impl TryFrom<i32> for ConnectionAttribute {
    type Error = TsurugiOdbcError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use ConnectionAttribute::*;
        match value {
            102 => Ok(SQL_ATTR_AUTOCOMMIT),
            _ => Err(TsurugiOdbcError::InvalidAttribute),
        }
    }
}

macro_rules! connection_attribute {
    ($dbc:expr, $attribute:expr) => {
        match ConnectionAttribute::try_from($attribute) {
            Ok(value) => value,
            Err(e) => {
                log::debug!("{FUNCTION_NAME}: unsupported attribute {:?}", $attribute);
                $dbc.add_diag(e, format!("unsupported attribute {:?}", $attribute));
                let rc = SqlReturn::SQL_ERROR;
                log::trace!("{FUNCTION_NAME} end. rc={:?}", rc);
                return rc;
            }
        }
    };
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
    let attribute = connection_attribute!(dbc, attribute);

    let rc = set_connect_attr(dbc, attribute, value_ptr, string_length, false);

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
    let attribute = connection_attribute!(dbc, attribute);

    let rc = set_connect_attr(dbc, attribute, value_ptr, string_length, true);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn set_connect_attr(
    dbc: Arc<TsurugiOdbcDbc>,
    attribute: ConnectionAttribute,
    value_ptr: SqlPointer,
    _string_length: SqlInteger,
    _wide_char: bool,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "set_connect_attr()";

    use ConnectionAttribute::*;
    match attribute {
        SQL_ATTR_AUTOCOMMIT => {
            let auto_commit = value_ptr as SqlUInteger;
            let auto_commit = auto_commit != SQL_AUTOCOMMIT_OFF;
            debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", attribute, auto_commit);
            dbc.set_auto_commit(auto_commit)
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
    let attribute = connection_attribute!(dbc, attribute);

    let rc = get_connect_attr(
        dbc,
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
    let attribute = connection_attribute!(dbc, attribute);

    let rc = get_connect_attr(
        dbc,
        attribute,
        value_ptr,
        buffer_length,
        string_length_ptr,
        true,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn get_connect_attr(
    dbc: Arc<TsurugiOdbcDbc>,
    attribute: ConnectionAttribute,
    value_ptr: SqlPointer,
    _buffer_length: SqlInteger,
    _string_length_ptr: *mut SqlInteger,
    _wide_char: bool,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_connect_attr()";

    if value_ptr.is_null() {
        debug!("{dbc}.{FUNCTION_NAME}: value_ptr is null");
        dbc.add_diag(TsurugiOdbcError::InvalidValuePtr, "value_ptr is null");
        return SqlReturn::SQL_ERROR;
    }

    use ConnectionAttribute::*;
    match attribute {
        SQL_ATTR_AUTOCOMMIT => {
            let auto_commit = dbc.auto_commit();
            debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", attribute, auto_commit);
            let auto_commit = if auto_commit {
                SQL_AUTOCOMMIT_ON
            } else {
                SQL_AUTOCOMMIT_OFF
            };
            let ptr = value_ptr as *mut SqlUInteger;
            unsafe {
                *ptr = auto_commit;
            }
            SqlReturn::SQL_SUCCESS
        }
    }
}
