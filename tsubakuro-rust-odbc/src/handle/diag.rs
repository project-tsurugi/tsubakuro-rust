use std::sync::{Arc, Mutex};

use log::{debug, trace};

use crate::{
    ctype::{SqlChar, SqlInteger, SqlReturn, SqlSmallInt, SqlWChar},
    handle::{
        hdbc::{HDbc, TsurugiOdbcDbc},
        henv::{HEnv, TsurugiOdbcEnv},
        hstmt::{HStmt, TsurugiOdbcStmt},
        Handle, HandleType,
    },
    handle_type,
    util::{write_char, write_wchar},
};

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum TsurugiOdbcError {
    InvalidAttribute = 1,
    StringError = 2,
    DecimalError = 10003,
    InvalidValuePtr = 3,
    UnsupportedBufferLength = 4,
    UnsupportedCDataType = 5,
    UnsupportedSqlDataType = 6,
    InvalidArgumentPtr = 9,
    ConvertError = 10,
    DataTruncated = 10008,

    InvalidConnectionString = 201,
    EndpointError = 202,
    ConnectError = 203,
    ConnectTimeout = 204,
    UnsupportedInfoType = 205,
    StartTransactionError = 210,
    TransactionNotFound = 211,
    TransactionCommitError = 212,
    TransactionRollbackError = 213,
    TransactionCloseError = 219,
    DisconnectError = 291,

    NotConnected = 301,
    SetAutoCommitError = 30001,
    StatementProcessorNotFound = 302,
    ListTablesError = 31001,
    ColumnNumberOutOfBounds = 32001,

    // SQLColAttribute
    UnsupportedFieldIdentifier = 32101,

    // SQLGetData
    GetDataUnsupportedTargetType = 32002,
    GetDataInvalidTargetValuePtr = 32003,
    GetDataInvalidStrLenOrIndPtr = 32004,

    GetTableMetadataError = 33001,
    BindColError = 33101,

    PrepareError = 34001,
    PreparedStatementCloseError = 34009,
    BindParameterError = 34101,
    PreparedQueryError = 34201,
    SqlQueryResultNextRowError = 34202,
    SqlQueryResultNextColumnError = 34203,
    SqlQueryResultIsNullError = 34204,
    SqlQueryResultFetchError = 34205,
    SqlQueryResultAtomTypeError = 34206,
    SqlQueryResultCloseError = 34209,
    PreparedExecuteError = 35001,
    EndTranError = 39001,
}

// for state
impl From<&TsurugiOdbcError> for &str {
    fn from(value: &TsurugiOdbcError) -> Self {
        use TsurugiOdbcError::*;
        match value {
            InvalidAttribute => "HY092",
            StringError => "HY090",
            DecimalError => "HY090",
            InvalidValuePtr => "HY009",
            UnsupportedBufferLength => "HY000",
            UnsupportedCDataType => "HY092",
            UnsupportedSqlDataType => "HY092",
            InvalidArgumentPtr => "HY009",
            ConvertError => "22018",
            DataTruncated => "01004",
            //
            InvalidConnectionString => "HY000",
            EndpointError => "HY000",
            ConnectError => "HY000",
            ConnectTimeout => "HYT01",
            UnsupportedInfoType => "HY000",
            //
            StartTransactionError => "HY000",
            TransactionNotFound => "HY000",
            TransactionCommitError => "HY000",
            TransactionRollbackError => "HY000",
            TransactionCloseError => "HY000",
            //
            DisconnectError => "HY000",
            NotConnected => "HY000",
            SetAutoCommitError => "HY000",
            StatementProcessorNotFound => "HY000",
            //
            ListTablesError => "HY000",
            ColumnNumberOutOfBounds => "HY000",
            // SQLColAttribute
            UnsupportedFieldIdentifier => "HY000",
            // SQLGetData
            GetDataUnsupportedTargetType => "HY000",
            GetDataInvalidTargetValuePtr => "HY009",
            GetDataInvalidStrLenOrIndPtr => "22002",
            //
            GetTableMetadataError => "HY000",
            //
            BindColError => "HY000",
            //
            PrepareError => "HY000",
            PreparedStatementCloseError => "HY000",
            //
            BindParameterError => "HY000",
            //
            PreparedQueryError => "HY000",
            //
            SqlQueryResultNextRowError => "HY000",
            SqlQueryResultNextColumnError => "HY000",
            SqlQueryResultIsNullError => "HY000",
            SqlQueryResultFetchError => "HY000",
            SqlQueryResultAtomTypeError => "HY000",
            SqlQueryResultCloseError => "HY000",
            //
            PreparedExecuteError => "HY000",
            //
            EndTranError => "HY000",
        }
    }
}

#[derive(Debug)]
pub(crate) struct TsurugiOdbcDiagRec {
    error_code: TsurugiOdbcError,
    message: String,
}

impl TsurugiOdbcDiagRec {
    fn new(error_code: TsurugiOdbcError, message: String) -> TsurugiOdbcDiagRec {
        TsurugiOdbcDiagRec {
            error_code,
            message,
        }
    }
}

#[derive(Debug)]
pub(crate) struct TsurugiOdbcDiagCollection {
    diag_list: Mutex<Vec<Arc<TsurugiOdbcDiagRec>>>,
}

impl TsurugiOdbcDiagCollection {
    pub(crate) fn new() -> TsurugiOdbcDiagCollection {
        TsurugiOdbcDiagCollection {
            diag_list: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn clear(&self) {
        let mut list = self.diag_list.lock().unwrap();
        list.clear();
    }

    pub(crate) fn add_diag(&self, error_code: TsurugiOdbcError, message: impl Into<String>) {
        let diag_rec = TsurugiOdbcDiagRec::new(error_code, message.into());

        let mut list = self.diag_list.lock().unwrap();
        list.push(Arc::new(diag_rec));
    }

    fn get(&self, record_number: usize) -> Option<Arc<TsurugiOdbcDiagRec>> {
        let index = record_number - 1;

        let list = self.diag_list.lock().unwrap();
        list.get(index).cloned()
    }
}

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

    let state_code: &str = (&diag.error_code).into();
    let rc1 = write_char(
        "SQLDiagRec.state",
        state_code,
        state,
        6,
        std::ptr::null_mut(),
        None,
    );

    let native_error = diag.error_code as i32;
    write_integer(native_error, native_error_ptr);

    let rc2 = write_char(
        "SQLDiagRec.message_text",
        &diag.message,
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
    handle_type: HandleType,
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

    let state_code: &str = (&diag.error_code).into();
    let rc1 = write_wchar(
        "SQLDiagRecW.state",
        state_code,
        state,
        6,
        std::ptr::null_mut(),
        None,
    );

    let native_error = diag.error_code as i32;
    write_integer(native_error, native_error_ptr);

    let rc2 = write_wchar(
        "SQLDiagRecW.message_text",
        &diag.message,
        message_text,
        buffer_length,
        text_length_ptr,
        None,
    );

    let rc = rc1.or(rc2);
    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn get_diag_collection(
    handle_type: HandleType,
    handle: Handle,
) -> Result<Arc<TsurugiOdbcDiagCollection>, SqlReturn> {
    let diags = match handle_type {
        HandleType::Env => {
            let env = TsurugiOdbcEnv::from(handle as HEnv);
            env.get_diag_collection()
        }
        HandleType::Dbc => {
            let dbc = TsurugiOdbcDbc::from(handle as HDbc);
            dbc.diag_collection()
        }
        HandleType::Stmt => {
            let stmt = TsurugiOdbcStmt::from(handle as HStmt);
            let stmt = stmt.lock().unwrap();
            stmt.diag_collection()
        }
        _ => {
            debug!(
                "get_diag_collection(): unsupported handle_type {:?}",
                handle_type
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };
    Ok(diags)
}

fn write_integer(value: i32, output: *mut SqlInteger) {
    if !output.is_null() {
        unsafe {
            *output = value;
        }
    }
}
