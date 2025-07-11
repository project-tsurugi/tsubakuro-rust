use log::{debug, trace};

use crate::{
    ctype::{SqlChar, SqlInteger, SqlReturn, SqlSmallInt, SqlWChar},
    handle::{diag::get_diag_collection, Handle, HandleType},
    handle_type,
    util::{write_char, write_wchar},
};

#[no_mangle]
pub extern "system" fn SQLGetDiagRec(
    handle_type: SqlSmallInt,
    handle: Handle,
    record_number: SqlSmallInt,
    state: *mut SqlChar,
    native_error_ptr: *mut SqlInteger,
    message_text: *mut SqlChar,
    buffer_length: SqlSmallInt,
    text_length_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetDiagRec()";
    trace!(
        "{FUNCTION_NAME} start. handle_type={:?}, handle={:?}, record_number={:?}, state={:?}, native_error_ptr={:?}, message_text={:?}, buffer_length={:?}, text_length_ptr={:?}",
        handle_type,
        handle,
        record_number,
        state,
        native_error_ptr,
        message_text,
        buffer_length,
        text_length_ptr
    );

    if handle.is_null() {
        debug!("{FUNCTION_NAME} error. handle is null");
        let rc = SqlReturn::SQL_INVALID_HANDLE;
        trace!("{FUNCTION_NAME} end. rc={:?}", rc);
        return rc;
    }

    if record_number <= 0 {
        debug!("{FUNCTION_NAME} error. record_number must be greater than 0");
        let rc = SqlReturn::SQL_ERROR;
        trace!("{FUNCTION_NAME} end. rc={:?}", rc);
        return rc;
    }

    let handle_type = handle_type!(handle_type);
    let diags = match get_diag_collection(handle_type, handle) {
        Ok(diags) => diags,
        Err(rc) => {
            trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
    };

    let diag = match diags.get(record_number as usize) {
        Some(diag) => diag,
        None => {
            let rc = SqlReturn::SQL_NO_DATA;
            trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
    };

    let state_code: &str = (&diag.error_code()).into();
    let rc1 = write_char(
        "SQLDiagRec.state",
        state_code,
        state,
        6,
        std::ptr::null_mut(),
        None,
    );

    let native_error = diag.error_code() as i32;
    write_integer(native_error, native_error_ptr);

    let rc2 = write_char(
        "SQLDiagRec.message_text",
        diag.message(),
        message_text,
        buffer_length,
        text_length_ptr,
        None,
    );

    let rc = rc1.or(rc2);
    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLGetDiagRecW(
    handle_type: SqlSmallInt,
    handle: Handle,
    record_number: SqlSmallInt,
    state: *mut SqlWChar,
    native_error_ptr: *mut SqlInteger,
    message_text: *mut SqlWChar,
    buffer_length: SqlSmallInt,
    text_length_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetDiagRecW()";
    trace!(
        "{FUNCTION_NAME} start. handle_type={:?}, handle={:?}, record_number={:?}, state={:?}, native_error_ptr={:?}, message_text={:?}, buffer_length={:?}, text_length_ptr={:?}",
        handle_type,
        handle,
        record_number,
        state,
        native_error_ptr,
        message_text,
        buffer_length,
        text_length_ptr
    );

    if handle.is_null() {
        debug!("{FUNCTION_NAME} error. handle is null");
        let rc = SqlReturn::SQL_INVALID_HANDLE;
        trace!("{FUNCTION_NAME} end. rc={:?}", rc);
        return rc;
    }

    if record_number <= 0 {
        debug!("{FUNCTION_NAME} error. record_number must be greater than 0");
        let rc = SqlReturn::SQL_ERROR;
        trace!("{FUNCTION_NAME} end. rc={:?}", rc);
        return rc;
    }

    let handle_type = handle_type!(handle_type);
    let diags = match get_diag_collection(handle_type, handle) {
        Ok(diags) => diags,
        Err(rc) => {
            trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
    };

    let diag = match diags.get(record_number as usize) {
        Some(diag) => diag,
        None => {
            let rc = SqlReturn::SQL_NO_DATA;
            trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return rc;
        }
    };

    let state_code: &str = (&diag.error_code()).into();
    let rc1 = write_wchar(
        "SQLDiagRecW.state",
        state_code,
        state,
        6,
        std::ptr::null_mut(),
        None,
    );

    let native_error = diag.error_code() as i32;
    write_integer(native_error, native_error_ptr);

    let rc2 = write_wchar(
        "SQLDiagRecW.message_text",
        diag.message(),
        message_text,
        buffer_length,
        text_length_ptr,
        None,
    );

    let rc = rc1.or(rc2);
    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn write_integer(value: i32, output: *mut SqlInteger) {
    if !output.is_null() {
        unsafe {
            *output = value;
        }
    }
}
