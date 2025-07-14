use std::sync::{Arc, Mutex};

use log::debug;

use crate::{
    ctype::SqlReturn,
    handle::{
        hdbc::{HDbc, TsurugiOdbcDbc},
        henv::{HEnv, TsurugiOdbcEnv},
        hstmt::{HStmt, TsurugiOdbcStmt},
        Handle, HandleType,
    },
};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    // GetDiagField
    UnsupportedDiagIdentifier = 32201,

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
    EndTranError = 38001,

    // SQLFreeStmt
    UnsupportedFreeStmtOption = 39001,
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
            // GetDiagField
            UnsupportedDiagIdentifier => "HY000",
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

            // SQLFreeStmt
            UnsupportedFreeStmtOption => "HY092",
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

    pub(crate) fn error_code(&self) -> TsurugiOdbcError {
        self.error_code
    }

    pub(crate) fn message(&self) -> &String {
        &self.message
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

    pub(crate) fn len(&self) -> usize {
        let list = self.diag_list.lock().unwrap();
        list.len()
    }

    pub(crate) fn get(&self, record_number: usize) -> Option<Arc<TsurugiOdbcDiagRec>> {
        let index = record_number - 1;

        let list = self.diag_list.lock().unwrap();
        list.get(index).cloned()
    }
}

pub(crate) fn get_diag_collection(
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
