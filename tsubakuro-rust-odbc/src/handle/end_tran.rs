use log::{debug, trace, warn};

use crate::{
    ctype::{SqlReturn, SqlSmallInt},
    handle::{
        hdbc::{end_tran_dbc, HDbc},
        henv::{end_tran_env, HEnv},
        Handle, HandleType,
    },
    handle_type,
};

#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub(crate) enum CompletionType {
    SQL_COMMIT = 0,
    SQL_ROLLBACK = 1,
}

impl TryFrom<i16> for CompletionType {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        use CompletionType::*;
        match value {
            0 => Ok(SQL_COMMIT),
            1 => Ok(SQL_ROLLBACK),
            _ => Err(()),
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLEndTran(
    handle_type: SqlSmallInt,
    handle: Handle,
    completion_type: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLEndTran()";
    trace!(
        "{FUNCTION_NAME} start. handle_type={:?}, handle={:?}, completion_type={:?}",
        handle_type,
        handle,
        completion_type
    );

    let rc = end_tran(handle_type, handle, completion_type);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn end_tran(handle_type: SqlSmallInt, handle: Handle, completion_type: SqlSmallInt) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLEndTran()";

    if handle.is_null() {
        debug!("{FUNCTION_NAME} error. handle is null");
        return SqlReturn::SQL_INVALID_HANDLE;
    }

    let handle_type = handle_type!(handle_type);

    let completion_type = match CompletionType::try_from(completion_type) {
        Ok(value) => value,
        Err(_) => {
            warn!(
                "{FUNCTION_NAME} error. Invalid completion_type {}",
                completion_type
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    match handle_type {
        HandleType::Env => end_tran_env(handle as HEnv, completion_type),
        HandleType::Dbc => end_tran_dbc(handle as HDbc, completion_type),
        _ => {
            warn!(
                "{FUNCTION_NAME} error. Unsupported handle_type {:?}",
                handle_type
            );
            SqlReturn::SQL_ERROR
        }
    }
}
