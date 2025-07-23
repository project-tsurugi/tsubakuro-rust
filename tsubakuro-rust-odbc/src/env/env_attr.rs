use std::sync::Arc;

use log::{debug, trace, warn};

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
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use EnvironmentAttribute::*;
        match value {
            200 => Ok(SQL_ATTR_ODBC_VERSION),
            e => Err(e),
        }
    }
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
    let rc = set_env_attr(&env, attribute, value_ptr, string_length);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn set_env_attr(
    env: &Arc<TsurugiOdbcEnv>,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    _string_length: SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "set_env_attr()";

    let attribute = match EnvironmentAttribute::try_from(attribute) {
        Ok(value) => value,
        Err(attribute) => {
            warn!(
                "{FUNCTION_NAME} error. Unsupported attribute {:?}",
                attribute
            );
            let odbc_function_name = "SQLSetEnvAttr()";
            env.add_diag(
                TsurugiOdbcError::EnvAttrUnsupportedAttribute,
                format!(
                    "{odbc_function_name}: Unsupported attribute {:?}",
                    attribute
                ),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    match attribute {
        EnvironmentAttribute::SQL_ATTR_ODBC_VERSION => env.set_odbc_version(value_ptr as u32),
    }
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
    let rc = get_env_attr(&env, attribute, value_ptr, buffer_length, string_length);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn get_env_attr(
    env: &Arc<TsurugiOdbcEnv>,
    attribute: SqlInteger,
    value_ptr: SqlPointer,
    _buffer_length: SqlInteger,
    _string_length: *mut SqlInteger,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_env_attr()";

    let attribute = match EnvironmentAttribute::try_from(attribute) {
        Ok(value) => value,
        Err(attribute) => {
            warn!(
                "{FUNCTION_NAME} error. Unsupported attribute {:?}",
                attribute
            );
            let odbc_function_name = "SQLGetEnvAttr()";
            env.add_diag(
                TsurugiOdbcError::EnvAttrUnsupportedAttribute,
                format!(
                    "{odbc_function_name}: Unsupported attribute {:?}",
                    attribute
                ),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    match attribute {
        EnvironmentAttribute::SQL_ATTR_ODBC_VERSION => {
            let value = env.odbc_version();
            write_integer(env, value as i32, value_ptr)
        }
    }
}

fn write_integer(env: &Arc<TsurugiOdbcEnv>, value: i32, value_ptr: SqlPointer) -> SqlReturn {
    const FUNCTION_NAME: &str = "write_integer()";

    if value_ptr.is_null() {
        debug!("env.{FUNCTION_NAME} error. value_ptr is null");
        let odbc_function_name = "SQLGetEnvAttr()";
        env.add_diag(
            TsurugiOdbcError::GetEnvAttrInvalidValuePtr,
            format!("{odbc_function_name}: value_ptr is null"),
        );
        return SqlReturn::SQL_ERROR;
    }

    unsafe {
        *(value_ptr as *mut i32) = value;
    }
    SqlReturn::SQL_SUCCESS
}
