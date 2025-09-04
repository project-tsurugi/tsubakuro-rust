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
    StringError = 10001,
    DataTruncated = 10002,

    // Env
    // SQLSetEnvAttr/SQLGetEnvAttr
    EnvAttrUnsupportedAttribute = 22001,
    UnsupportedOdbcVersion = 22101,
    GetEnvAttrInvalidValuePtr = 22201,

    // Dbc
    // Connect
    ConnectEndpointNotFound = 30001,
    ConnectEndpointError = 30002,
    ConnectCredentialError = 30003,
    ConnectError = 30011,
    ConnectTimeout = 30012,
    ConnectAuthenticationError = 30013,
    // SQLSetConnectAttr/SQLGetConnectAttr
    ConnectAttrUnsupportedAttribute = 31001,
    ConnectAttrUnsupportedDriverCompletion = 31002,
    SetAutoCommitError = 31111,
    GetConnectAttrInvalidValuePtr = 31201,
    // SQLGetInfo
    GetInfoUnsupportedInfoType = 32001,
    // Transaction
    TransactionStartError = 33001,
    TransactionNotFound = 33002,
    TransactionCommitError = 33003,
    TransactionRollbackError = 33004,
    TransactionCloseError = 33009,
    // SQLEndTran
    EndTranError = 34001,
    // SQLDisconnect
    DisconnectShutdownError = 39008,
    DisconnectCloseError = 39009,

    // Stmt
    StatementProcessorNotFound = 40001,
    NotConnected = 40002,
    // SQLSetStmtAttr/SQLGetStmtAttr
    StmtAttrUnsupportedAttribute = 40101,
    GetStmtAttrInvalidValuePtr = 40121,
    // SQLGetTypeInfo
    GetTypeInfoUnsupportedDataType = 40201,
    // SQLTables
    ListTablesError = 40301,
    // SQLColumns
    GetTableMetadataError = 40401,

    // SQLPrepare
    PrepareError = 50101,
    PreparedStatementCloseError = 50102,
    // SQLBindParameter
    BindParameterError = 50201,
    BindParameterInvalidParameterNumber = 50202,
    BindParameterUnsupportedInputOutputType = 50203,
    BindParameterUnsupportedValueType = 50204,
    BindParameterUnsupportedParameterType = 50205,
    BindParameterInvalidValuePtr = 50208,
    BindParameterInvalidStrLenOrIndPtr = 50210,
    BindParameterConvertDecimalError = 50211,
    BindParameterConvertDateError = 50212,
    BindParameterConvertTimeError = 50213,
    BindParameterConvertTimestampError = 50214,

    // SQLExecute
    // SQLExecDirect
    PreparedQueryError = 50301,
    SqlQueryResultNextRowError = 50302,
    SqlQueryResultNextColumnError = 50303,
    SqlQueryResultIsNullError = 50304,
    SqlQueryResultFetchError = 50305,
    SqlQueryResultAtomTypeError = 50306,
    SqlQueryResultCloseError = 50307,
    PreparedExecuteError = 50401,

    // SQLBindCol
    BindColUnsupportedTargetType = 60103,
    BindColInvalidStrLenOrIndPtr = 60106,

    // SQLNumResultCols
    NumResultColsInvalidColumnCountPtr = 60202,

    // SQLDescribeCol
    DescribeColInvalidColumnNumber = 60302,

    // SQLColAttribute
    ColAttributeInvalidColumnNumber = 60402,
    ColAttributeUnsupportedFieldIdentifier = 60403,

    // SQLRowCount
    RowCountInvalidRowCountPtr = 60502,

    // SQLFetch

    // SQLGetData
    GetDataInvalidColumnNumber = 60702,
    GetDataUnsupportedTargetType = 60703,
    GetDataInvalidTargetValuePtr = 60704,
    GetDataInvalidStrLenOrIndPtr = 60706,
    GetDataConvertBoolError = 60711,
    GetDataConvertI128Error = 60712,
    GetDataConvertF32Error = 60713,
    GetDataConvertF64Error = 60714,
    GetDataConvertDecimalError = 60715,
    GetDataConvertTimeError = 60716,
    GetDataConvertTimestampError = 60717,

    // SQLFreeStmt
    FreeStmtUnsupportedOption = 49001,
}

// for state
impl From<&TsurugiOdbcError> for &str {
    fn from(value: &TsurugiOdbcError) -> Self {
        use TsurugiOdbcError::*;
        match value {
            StringError => "HY000",
            DataTruncated => "01004",

            // Env
            // SQLSetEnvAttr/SQLGetEnvAttr
            EnvAttrUnsupportedAttribute => "HY092",
            UnsupportedOdbcVersion => "HYC00",
            GetEnvAttrInvalidValuePtr => "HY009",

            // Dbc
            // Connect
            ConnectEndpointNotFound => "08001",
            ConnectEndpointError => "08001",
            ConnectCredentialError => "08001",
            ConnectError => "08S01",
            ConnectTimeout => "HYT01",
            ConnectAuthenticationError => "28000",
            // SQLSetConnectAttr/SQLGetConnectAttr
            ConnectAttrUnsupportedAttribute => "HY092",
            ConnectAttrUnsupportedDriverCompletion => "HY092",
            SetAutoCommitError => "HY000",
            GetConnectAttrInvalidValuePtr => "HY009",
            // SQLGetInfo
            GetInfoUnsupportedInfoType => "HY096",
            // Transaction
            TransactionStartError => "HY000",
            TransactionNotFound => "HY000",
            TransactionCommitError => "HY000",
            TransactionRollbackError => "HY000",
            TransactionCloseError => "HY000",
            // SQLEndTran
            EndTranError => "HY000",
            // SQLDisconnect
            DisconnectShutdownError => "HY000",
            DisconnectCloseError => "HY000",

            // Stmt
            StatementProcessorNotFound => "HY000",
            NotConnected => "HY000",
            // SQLSetStmtAttr/SQLGetStmtAttr
            StmtAttrUnsupportedAttribute => "HY092",
            GetStmtAttrInvalidValuePtr => "HY009",
            // SQLGetTypeInfo
            GetTypeInfoUnsupportedDataType => "HY004",
            // SQLTables
            ListTablesError => "HY000",
            // SQLColumns
            GetTableMetadataError => "HY000",

            // SQLPrepare
            PrepareError => "HY000",
            PreparedStatementCloseError => "HY000",
            // SQLBindParameter
            BindParameterError => "HY000",
            BindParameterInvalidParameterNumber => "07009",
            BindParameterUnsupportedInputOutputType => "HY105",
            BindParameterUnsupportedValueType => "HY003",
            BindParameterUnsupportedParameterType => "HY004",
            BindParameterInvalidValuePtr => "HY009",
            BindParameterInvalidStrLenOrIndPtr => "HY009",
            BindParameterConvertDecimalError => "HY000",
            BindParameterConvertDateError => "HY000",
            BindParameterConvertTimeError => "HY000",
            BindParameterConvertTimestampError => "HY000",
            // SQLExecute
            // SQLExecDirect
            PreparedQueryError => "HY000",
            SqlQueryResultNextRowError => "HY000",
            SqlQueryResultNextColumnError => "HY000",
            SqlQueryResultIsNullError => "HY000",
            SqlQueryResultFetchError => "HY000",
            SqlQueryResultAtomTypeError => "HY000",
            SqlQueryResultCloseError => "HY000",
            PreparedExecuteError => "HY000",

            // SQLBindCol
            BindColUnsupportedTargetType => "HY003",
            BindColInvalidStrLenOrIndPtr => "HY009",
            // SQLNumResultCols
            NumResultColsInvalidColumnCountPtr => "HY009",
            // SQLDescribeCol
            DescribeColInvalidColumnNumber => "07009",
            // SQLColAttribute
            ColAttributeInvalidColumnNumber => "07009",
            ColAttributeUnsupportedFieldIdentifier => "HY091",
            // SQLRowCount
            RowCountInvalidRowCountPtr => "HY009",

            // SQLGetData
            GetDataInvalidColumnNumber => "07009",
            GetDataUnsupportedTargetType => "07006",
            GetDataInvalidTargetValuePtr => "HY009",
            GetDataInvalidStrLenOrIndPtr => "22002",
            GetDataConvertBoolError => "HY000",
            GetDataConvertI128Error => "HY000",
            GetDataConvertF32Error => "HY000",
            GetDataConvertF64Error => "HY000",
            GetDataConvertDecimalError => "HY000",
            GetDataConvertTimeError => "22007",
            GetDataConvertTimestampError => "22007",

            // SQLFreeStmt
            FreeStmtUnsupportedOption => "HY092",
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
    const FUNCTION_NAME: &str = "get_diag_collection()";

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
            debug!("{FUNCTION_NAME}: Unsupported handle_type {:?}", handle_type);
            return Err(SqlReturn::SQL_ERROR);
        }
    };
    Ok(diags)
}
