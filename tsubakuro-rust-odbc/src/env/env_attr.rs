use std::sync::Arc;

use log::{debug, trace};

use crate::{
    check_env,
    ctype::{SqlInteger, SqlPointer, SqlReturn},
    handle::{
        diag::TsurugiOdbcError,
        henv::{HEnv, TsurugiOdbcEnv},
    },
};

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum EnvironmentAttribute {
    SQL_ATTR_ODBC_VERSION = 200,
}

impl TryFrom<i32> for EnvironmentAttribute {
    type Error = TsurugiOdbcError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use EnvironmentAttribute::*;
        match value {
            200 => Ok(SQL_ATTR_ODBC_VERSION),
            _ => Err(TsurugiOdbcError::InvalidAttribute),
        }
    }
}

macro_rules! environment_attribute {
    ($env:expr, $attribute:expr) => {
        match EnvironmentAttribute::try_from($attribute) {
            Ok(value) => value,
            Err(e) => {
                log::warn!("{FUNCTION_NAME}: Unsupported attribute {:?}", $attribute);
                $env.add_diag(
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
pub extern "system" fn SQLSetEnvAttr(
    henv: HEnv,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    string_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLSetEnvAttr()";
    trace!(
        "{FUNCTION_NAME} start. henv={:?}, attribute={:?}, value_ptr={:?}, string_length={:?}",
        henv,
        attribute,
        value_ptr,
        string_length
    );

    let env = check_env!(henv);
    let attribute = environment_attribute!(env, attribute);

    let rc = match attribute {
        EnvironmentAttribute::SQL_ATTR_ODBC_VERSION => env.set_odbc_version(value_ptr as u32),
    };

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLGetEnvAttr(
    henv: HEnv,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    buffer_length: SqlInteger,
    string_length: *mut SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetEnvAttr()";
    trace!(
        "{FUNCTION_NAME} start. henv={:?}, attribute={:?}, value_ptr={:?}, buffer_length={:?}, string_length={:?}",
        henv,
        attribute,
        value_ptr,
        buffer_length,
        string_length
    );

    let env = check_env!(henv);
    let attribute = environment_attribute!(env, attribute);

    let rc = match attribute {
        EnvironmentAttribute::SQL_ATTR_ODBC_VERSION => {
            let value = env.odbc_version();
            write_integer(&env, value as i32, value_ptr)
        }
    };

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn write_integer(env: &Arc<TsurugiOdbcEnv>, value: i32, value_ptr: SqlPointer) -> SqlReturn {
    const FUNCTION_NAME: &str = "write_integer()";

    if value_ptr.is_null() {
        debug!("env.{FUNCTION_NAME}: value_ptr is null");
        env.add_diag(
            TsurugiOdbcError::InvalidValuePtr,
            "SQLGetEnvAttr.value_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    unsafe {
        *(value_ptr as *mut i32) = value;
    }
    SqlReturn::SQL_SUCCESS
}
