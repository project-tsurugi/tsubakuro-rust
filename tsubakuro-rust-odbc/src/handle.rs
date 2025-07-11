use std::ffi::c_void;

use hdbc::{alloc_handle_dbc, free_handle_dbc};
use henv::{alloc_handle_env, free_handle_env};
use log::{debug, trace, warn};

use crate::{
    ctype::{SqlReturn, SqlSmallInt},
    handle::{
        hdbc::HDbc,
        henv::HEnv,
        hstmt::{alloc_handle_stmt, free_handle_stmt, HStmt},
    },
    logger::env_logger_init,
};

pub(crate) mod diag;
pub mod end_tran;
pub mod get_diag_field;
pub mod get_diag_rec;
pub(crate) mod hdbc;
pub(crate) mod henv;
pub(crate) mod hstmt;

pub type Handle = *mut c_void;

#[repr(i16)]
#[derive(Debug, Clone, Copy)]
pub enum HandleType {
    Env = 1,
    Dbc = 2,
    Stmt = 3,
    Desc = 4,
}

#[macro_export]
macro_rules! handle_type {
    ($handle_type:expr) => {
        match $handle_type {
            1 => HandleType::Env,
            2 => HandleType::Dbc,
            3 => HandleType::Stmt,
            4 => HandleType::Desc,
            _ => {
                log::debug!("{FUNCTION_NAME}: unsupported handle_type {}", $handle_type);
                return SqlReturn::SQL_ERROR;
            }
        }
    };
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn SQLAllocHandle(
    handle_type: SqlSmallInt,
    input_handle: Handle,
    output_handle: *mut Handle,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLAllocHandle()";
    env_logger_init();
    trace!(
        "{FUNCTION_NAME} start. handle_type={:?}, input_handle={:?}, output_handle={:?}",
        handle_type,
        input_handle,
        output_handle
    );

    if output_handle.is_null() {
        debug!("{FUNCTION_NAME} error. output_handle is null");
        let rc = SqlReturn::SQL_ERROR;
        trace!("{FUNCTION_NAME} end. rc={:?}", rc);
        return rc;
    }

    let handle_type = handle_type!(handle_type);
    let handle = match alloc_handle(handle_type, input_handle) {
        Ok(handle) => handle,
        Err(rc) => {
            trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
    };

    unsafe {
        *output_handle = handle;
    }

    let rc = SqlReturn::SQL_SUCCESS;
    trace!(
        "{FUNCTION_NAME} end. rc={:?}, *output_handle={:?}",
        rc,
        handle
    );
    rc
}

fn alloc_handle(handle_type: HandleType, input_handle: Handle) -> Result<Handle, SqlReturn> {
    match handle_type {
        HandleType::Env => Ok(alloc_handle_env()? as Handle),
        HandleType::Dbc => Ok(alloc_handle_dbc(input_handle as HEnv)? as Handle),
        HandleType::Stmt => Ok(alloc_handle_stmt(input_handle as HDbc)? as Handle),
        _ => {
            warn!(
                "SQLAllocHandle(): unsupported handle_type {:?}",
                handle_type
            );
            Err(SqlReturn::SQL_ERROR)
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLFreeHandle(handle_type: SqlSmallInt, handle: Handle) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLFreeHandle()";
    trace!(
        "{FUNCTION_NAME} start. handle_type={:?}, handle={:?}",
        handle_type,
        handle
    );

    if handle.is_null() {
        let rc = SqlReturn::SQL_SUCCESS;
        trace!("{FUNCTION_NAME} end (handle is null). rc={:?}", rc);
        return rc;
    }

    let handle_type = handle_type!(handle_type);
    let rc = free_handle(handle_type, handle);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn free_handle(handle_type: HandleType, handle: Handle) -> SqlReturn {
    match handle_type {
        HandleType::Env => free_handle_env(handle as HEnv),
        HandleType::Dbc => free_handle_dbc(handle as HDbc),
        HandleType::Stmt => free_handle_stmt(handle as HStmt),
        _ => {
            debug!("SQLFreeHandle(): unsupported handle_type {:?}", handle_type);
            SqlReturn::SQL_SUCCESS
        }
    }
}
