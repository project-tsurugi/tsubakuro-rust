use log::{debug, trace, warn};

use crate::{
    ctype::{SqlChar, SqlInteger, SqlPointer, SqlReturn, SqlSmallInt, SqlWChar},
    handle::{
        diag::{get_diag_collection, TsurugiOdbcError},
        Handle, HandleType,
    },
    handle_type,
    util::{write_char, write_wchar_bytes},
    DRIVER_NAME,
};

#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum DiagIdentifier {
    SQL_DIAG_CURSOR_ROW_COUNT = -1249,
    SQL_DIAG_ROW_NUMBER = -1248,
    SQL_DIAG_COLUMN_NUMBER = -1247,
    SQL_DIAG_RETURNCODE = 1,
    SQL_DIAG_NUMBER = 2,
    SQL_DIAG_ROW_COUNT = 3,
    SQL_DIAG_SQLSTATE = 4,
    SQL_DIAG_NATIVE = 5,
    SQL_DIAG_MESSAGE_TEXT = 6,
    SQL_DIAG_DYNAMIC_FUNCTION = 7,
    SQL_DIAG_CLASS_ORIGIN = 8,
    SQL_DIAG_SUBCLASS_ORIGIN = 9,
    SQL_DIAG_CONNECTION_NAME = 10,
    SQL_DIAG_SERVER_NAME = 11,
    SQL_DIAG_DYNAMIC_FUNCTION_CODE = 12,
}

impl TryFrom<i16> for DiagIdentifier {
    type Error = TsurugiOdbcError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        use DiagIdentifier::*;
        match value {
            -1249 => Ok(SQL_DIAG_CURSOR_ROW_COUNT),
            -1248 => Ok(SQL_DIAG_ROW_NUMBER),
            -1247 => Ok(SQL_DIAG_COLUMN_NUMBER),
            1 => Ok(SQL_DIAG_RETURNCODE),
            2 => Ok(SQL_DIAG_NUMBER),
            3 => Ok(SQL_DIAG_ROW_COUNT),
            4 => Ok(SQL_DIAG_SQLSTATE),
            5 => Ok(SQL_DIAG_NATIVE),
            6 => Ok(SQL_DIAG_MESSAGE_TEXT),
            7 => Ok(SQL_DIAG_DYNAMIC_FUNCTION),
            8 => Ok(SQL_DIAG_CLASS_ORIGIN),
            9 => Ok(SQL_DIAG_SUBCLASS_ORIGIN),
            10 => Ok(SQL_DIAG_CONNECTION_NAME),
            11 => Ok(SQL_DIAG_SERVER_NAME),
            12 => Ok(SQL_DIAG_DYNAMIC_FUNCTION_CODE),
            _ => Err(TsurugiOdbcError::UnsupportedDiagIdentifier),
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLGetDiagField(
    handle_type: SqlSmallInt,
    handle: Handle,
    record_number: SqlSmallInt,
    diag_identifier: SqlSmallInt,
    diag_info_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetDiagField()";
    trace!(
        "{FUNCTION_NAME} start. handle_type={:?}, handle={:?}, record_number={:?}, diag_identifier={:?}, diag_info_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        handle_type,
        handle,
        record_number,
        diag_identifier,
        diag_info_ptr,
        buffer_length,
        string_length_ptr
    );

    let arg =
        TsurugiOdbcDiagFieldArguments::new(diag_info_ptr, buffer_length, string_length_ptr, false);
    let rc = get_diag_field(handle_type, handle, record_number, diag_identifier, arg);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLGetDiagFieldW(
    handle_type: SqlSmallInt,
    handle: Handle,
    record_number: SqlSmallInt,
    diag_identifier: SqlSmallInt,
    diag_info_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetDiagFieldW()";
    trace!(
        "{FUNCTION_NAME} start. handle_type={:?}, handle={:?}, record_number={:?}, diag_identifier={:?}, diag_info_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        handle_type,
        handle,
        record_number,
        diag_identifier,
        diag_info_ptr,
        buffer_length,
        string_length_ptr
    );

    let arg =
        TsurugiOdbcDiagFieldArguments::new(diag_info_ptr, buffer_length, string_length_ptr, true);
    let rc = get_diag_field(handle_type, handle, record_number, diag_identifier, arg);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

struct TsurugiOdbcDiagFieldArguments {
    diag_info_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
    wide_char: bool,
}

impl TsurugiOdbcDiagFieldArguments {
    fn new(
        diag_info_ptr: SqlPointer,
        buffer_length: SqlSmallInt,
        string_length_ptr: *mut SqlSmallInt,
        wide_char: bool,
    ) -> TsurugiOdbcDiagFieldArguments {
        TsurugiOdbcDiagFieldArguments {
            diag_info_ptr,
            buffer_length,
            string_length_ptr,
            wide_char,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn get_diag_field(
    handle_type: SqlSmallInt,
    handle: Handle,
    record_number: SqlSmallInt,
    diag_identifier: SqlSmallInt,
    arg: TsurugiOdbcDiagFieldArguments,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_diag_field()";

    if handle.is_null() {
        debug!("{FUNCTION_NAME} error. handle is null");
        return SqlReturn::SQL_INVALID_HANDLE;
    }

    let handle_type = handle_type!(handle_type);
    let diags = match get_diag_collection(handle_type, handle) {
        Ok(diags) => diags,
        Err(rc) => return rc,
    };

    let diag_identifier = match DiagIdentifier::try_from(diag_identifier) {
        Ok(value) => value,
        Err(_) => {
            warn!(
                "{FUNCTION_NAME} error. Unsupported diag_identifier: {}",
                diag_identifier
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    use DiagIdentifier::*;
    match diag_identifier {
        SQL_DIAG_NUMBER => return write_integer(diags.len() as SqlInteger, arg),
        SQL_DIAG_CURSOR_ROW_COUNT
        | SQL_DIAG_DYNAMIC_FUNCTION
        | SQL_DIAG_DYNAMIC_FUNCTION_CODE
        | SQL_DIAG_RETURNCODE
        | SQL_DIAG_ROW_COUNT => {
            warn!(
                "{FUNCTION_NAME} error. Unsupported diag_identifier: {:?}",
                diag_identifier
            );
            return SqlReturn::SQL_ERROR;
        }
        _ => {}
    }

    if record_number <= 0 {
        debug!("{FUNCTION_NAME} error. record_number must be greater than 0");
        return SqlReturn::SQL_ERROR;
    }

    let diag = match diags.get(record_number as usize) {
        Some(diag) => diag,
        None => return SqlReturn::SQL_NO_DATA,
    };

    match diag_identifier {
        SQL_DIAG_CLASS_ORIGIN => write_string(DRIVER_NAME, arg),
        SQL_DIAG_CONNECTION_NAME => write_string("TODO SQL_DIAG_CONNECTION_NAME", arg), // TODO SQL_DIAG_CONNECTION_NAME
        SQL_DIAG_MESSAGE_TEXT => write_string(diag.message(), arg),
        SQL_DIAG_NATIVE => write_integer(diag.error_code() as SqlInteger, arg),
        SQL_DIAG_SERVER_NAME => write_string("TODO SQL_DIAG_SERVER_NAME", arg), // TODO SQL_DIAG_SERVER_NAME
        SQL_DIAG_SQLSTATE => write_string((&diag.error_code()).into(), arg),
        SQL_DIAG_SUBCLASS_ORIGIN => write_string(DRIVER_NAME, arg),
        _ => {
            warn!(
                "{FUNCTION_NAME} error. Unsupported diag_identifier: {:?}",
                diag_identifier
            );
            SqlReturn::SQL_ERROR
        }
    }
}

fn write_string(value: &str, arg: TsurugiOdbcDiagFieldArguments) -> SqlReturn {
    if arg.wide_char {
        write_wchar_bytes(
            "",
            value,
            arg.diag_info_ptr as *mut SqlWChar,
            arg.buffer_length,
            arg.string_length_ptr,
            None,
        )
    } else {
        write_char(
            "name",
            value,
            arg.diag_info_ptr as *mut SqlChar,
            arg.buffer_length,
            arg.string_length_ptr,
            None,
        )
    }
}

fn write_integer(value: SqlInteger, arg: TsurugiOdbcDiagFieldArguments) -> SqlReturn {
    let diag_info_ptr = arg.diag_info_ptr as *mut SqlInteger;
    if !diag_info_ptr.is_null() {
        unsafe {
            *diag_info_ptr = value;
        }
    }

    SqlReturn::SQL_SUCCESS
}
