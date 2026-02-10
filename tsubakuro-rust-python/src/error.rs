use pyo3::{create_exception, exceptions::PyException, PyErr};
use tsubakuro_rust_core::prelude::{DiagnosticCode, TgError};

create_exception!(
    tsubakuro_rust_python,
    Warning,
    PyException,
    "important warning"
);
create_exception!(
    tsubakuro_rust_python,
    Error,
    PyException,
    "base class of all other exceptions"
);

// Subclass of Error
create_exception!(
    tsubakuro_rust_python,
    InterfaceError,
    Error,
    "interface error"
);
create_exception!(
    tsubakuro_rust_python,
    DatabaseError,
    Error,
    "database error"
);

// Subclasses of DatabaseError
create_exception!(
    tsubakuro_rust_python,
    DataError,
    DatabaseError,
    "data error"
);
create_exception!(
    tsubakuro_rust_python,
    OperationalError,
    DatabaseError,
    "operation error"
);
create_exception!(
    tsubakuro_rust_python,
    IntegrityError,
    DatabaseError,
    "integrity error"
);
create_exception!(
    tsubakuro_rust_python,
    InternalError,
    DatabaseError,
    "internal error"
);
create_exception!(
    tsubakuro_rust_python,
    ProgrammingError,
    DatabaseError,
    "programming error"
);
create_exception!(
    tsubakuro_rust_python,
    NotSupportedError,
    DatabaseError,
    "not supported error"
);

// ServerException
create_exception!(tsubakuro_rust_python, ServerException, OperationalError);

// SqlServiceException
create_exception!(tsubakuro_rust_python, SqlServiceException, ServerException);
create_exception!(
    tsubakuro_rust_python,
    SqlExecutionException,
    SqlServiceException
);
create_exception!(
    tsubakuro_rust_python,
    ConstraintViolationException,
    IntegrityError
);
create_exception!(
    tsubakuro_rust_python,
    UniqueConstraintViolationException,
    ConstraintViolationException
);
create_exception!(
    tsubakuro_rust_python,
    NotNullConstraintViolationException,
    ConstraintViolationException
);
create_exception!(
    tsubakuro_rust_python,
    ReferentialIntegrityConstraintViolationException,
    ConstraintViolationException
);
create_exception!(
    tsubakuro_rust_python,
    CheckConstraintViolationException,
    ConstraintViolationException
);
create_exception!(tsubakuro_rust_python, EvaluationException, DataError);
create_exception!(
    tsubakuro_rust_python,
    ValueEvaluationException,
    EvaluationException
);
create_exception!(
    tsubakuro_rust_python,
    ScalarSubqueryEvaluationException,
    EvaluationException
);
create_exception!(
    tsubakuro_rust_python,
    TargetNotFoundException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    TargetAlreadyExistsException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    InconsistentStatementException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    RestrictedOperationException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    DependenciesViolationException,
    RestrictedOperationException
);
create_exception!(
    tsubakuro_rust_python,
    WriteOperationByRtxException,
    RestrictedOperationException
);
create_exception!(
    tsubakuro_rust_python,
    LtxWriteOperationWithoutWritePreserveException,
    RestrictedOperationException
);
create_exception!(
    tsubakuro_rust_python,
    ReadOperationOnRestrictedReadAreaException,
    RestrictedOperationException
);
create_exception!(
    tsubakuro_rust_python,
    InactiveTransactionException,
    RestrictedOperationException
);
create_exception!(tsubakuro_rust_python, ParameterException, ProgrammingError);
create_exception!(
    tsubakuro_rust_python,
    UnresolvedPlaceholderException,
    ParameterException
);
create_exception!(
    tsubakuro_rust_python,
    LoadFileException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    LoadFileNotFoundException,
    LoadFileException
);
create_exception!(
    tsubakuro_rust_python,
    LoadFileFormatException,
    LoadFileException
);
create_exception!(
    tsubakuro_rust_python,
    DumpFileException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    DumpDirectoryInaccessibleException,
    DumpFileException
);
create_exception!(tsubakuro_rust_python, SqlLimitReachedException, DataError);
create_exception!(
    tsubakuro_rust_python,
    TransactionExceededLimitException,
    SqlLimitReachedException
);
create_exception!(
    tsubakuro_rust_python,
    SqlRequestTimeoutException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    DataCorruptionException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    SecondaryIndexCorruptionException,
    DataCorruptionException
);
create_exception!(
    tsubakuro_rust_python,
    RequestFailureException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    TransactionNotFoundException,
    RequestFailureException
);
create_exception!(
    tsubakuro_rust_python,
    StatementNotFoundException,
    RequestFailureException
);
create_exception!(tsubakuro_rust_python, InternalException, InternalError);
create_exception!(
    tsubakuro_rust_python,
    UnsupportedRuntimeFeatureException,
    NotSupportedError
);
create_exception!(
    tsubakuro_rust_python,
    BlockedByHighPriorityTransactionException,
    SqlExecutionException
);
create_exception!(
    tsubakuro_rust_python,
    InvalidRuntimeValueException,
    SqlLimitReachedException
);
create_exception!(
    tsubakuro_rust_python,
    ValueOutOfRangeException,
    InvalidRuntimeValueException
);
create_exception!(
    tsubakuro_rust_python,
    ValueTooLongException,
    InvalidRuntimeValueException
);
create_exception!(
    tsubakuro_rust_python,
    InvalidDecimalValueException,
    InvalidRuntimeValueException
);
create_exception!(tsubakuro_rust_python, CompileException, ProgrammingError);
create_exception!(tsubakuro_rust_python, SyntaxException, CompileException);
create_exception!(tsubakuro_rust_python, AnalyzeException, CompileException);
create_exception!(
    tsubakuro_rust_python,
    TypeAnalyzeException,
    AnalyzeException
);
create_exception!(
    tsubakuro_rust_python,
    SymbolAnalyzeException,
    AnalyzeException
);
create_exception!(
    tsubakuro_rust_python,
    ValueAnalyzeException,
    AnalyzeException
);
create_exception!(
    tsubakuro_rust_python,
    UnsupportedCompilerFeatureException,
    NotSupportedError
);
create_exception!(tsubakuro_rust_python, CcException, SqlServiceException);
create_exception!(tsubakuro_rust_python, OccException, CcException);
create_exception!(tsubakuro_rust_python, OccReadException, OccException);
create_exception!(
    tsubakuro_rust_python,
    ConflictOnWritePreserveException,
    OccReadException
);
create_exception!(tsubakuro_rust_python, OccWriteException, OccException);
create_exception!(tsubakuro_rust_python, LtxException, CcException);
create_exception!(tsubakuro_rust_python, LtxReadException, LtxException);
create_exception!(tsubakuro_rust_python, LtxWriteException, LtxException);
create_exception!(tsubakuro_rust_python, RtxException, CcException);
create_exception!(
    tsubakuro_rust_python,
    BlockedByConcurrentOperationException,
    CcException
);

pub(crate) fn to_pyerr(err: TgError) -> PyErr {
    match err {
        TgError::ClientError(message, cause) => {
            if let Some(cause) = cause {
                InterfaceError::new_err(format!("{}: {}", message, cause))
            } else {
                InterfaceError::new_err(format!("{}", message))
            }
        }
        TgError::TimeoutError(message) => {
            OperationalError::new_err(format!("TimeoutError: {}", message))
        }
        TgError::IoError(message, cause) => {
            if let Some(cause) = cause {
                OperationalError::new_err(format!("IoError: {}: {}", message, cause))
            } else {
                OperationalError::new_err(format!("IoError: {}", message))
            }
        }
        TgError::ServerError(_, _, diagnostic_code, server_message) => {
            server_error_to_pyerr(server_message, diagnostic_code)
        }
    }
}

macro_rules! server_error {
    ($error_type:ty, $diagnostic_code:expr, $message:expr) => {
        <$error_type>::new_err(format!(
            "{}({}): {}",
            $diagnostic_code.structured_code(),
            $diagnostic_code.name(),
            $message,
        ))
    };
}

fn server_error_to_pyerr(message: String, code: DiagnosticCode) -> PyErr {
    match code.structured_code().as_str() {
        // SqlServiceException
        "SQL-01000" => return server_error!(SqlServiceException, code, message),
        "SQL-02000" => return server_error!(SqlExecutionException, code, message),
        "SQL-02001" => return server_error!(ConstraintViolationException, code, message),
        "SQL-02002" => return server_error!(UniqueConstraintViolationException, code, message),
        "SQL-02003" => return server_error!(NotNullConstraintViolationException, code, message),
        "SQL-02004" => {
            return server_error!(
                ReferentialIntegrityConstraintViolationException,
                code,
                message
            )
        }
        "SQL-02005" => return server_error!(CheckConstraintViolationException, code, message),
        "SQL-02010" => return server_error!(EvaluationException, code, message),
        "SQL-02011" => return server_error!(ValueEvaluationException, code, message),
        "SQL-02012" => return server_error!(ScalarSubqueryEvaluationException, code, message),
        "SQL-02014" => return server_error!(TargetNotFoundException, code, message),
        "SQL-00100" => return server_error!(TargetAlreadyExistsException, code, message),
        "SQL-02018" => return server_error!(InconsistentStatementException, code, message),
        "SQL-02020" => return server_error!(RestrictedOperationException, code, message),
        "SQL-02021" => return server_error!(DependenciesViolationException, code, message),
        "SQL-02022" => return server_error!(WriteOperationByRtxException, code, message),
        "SQL-02023" => {
            return server_error!(
                LtxWriteOperationWithoutWritePreserveException,
                code,
                message
            )
        }
        "SQL-02024" => {
            return server_error!(ReadOperationOnRestrictedReadAreaException, code, message)
        }
        "SQL-02025" => return server_error!(InactiveTransactionException, code, message),
        "SQL-02027" => return server_error!(ParameterException, code, message),
        "SQL-02028" => return server_error!(UnresolvedPlaceholderException, code, message),
        "SQL-02030" => return server_error!(LoadFileException, code, message),
        "SQL-02031" => return server_error!(LoadFileNotFoundException, code, message),
        "SQL-02032" => return server_error!(LoadFileFormatException, code, message),
        "SQL-02033" => return server_error!(DumpFileException, code, message),
        "SQL-02034" => return server_error!(DumpDirectoryInaccessibleException, code, message),
        "SQL-02036" => return server_error!(SqlLimitReachedException, code, message),
        "SQL-02037" => return server_error!(TransactionExceededLimitException, code, message),
        "SQL-02039" => return server_error!(SqlRequestTimeoutException, code, message),
        "SQL-02041" => return server_error!(DataCorruptionException, code, message),
        "SQL-02042" => return server_error!(SecondaryIndexCorruptionException, code, message),
        "SQL-02044" => return server_error!(RequestFailureException, code, message),
        "SQL-02045" => return server_error!(TransactionNotFoundException, code, message),
        "SQL-02046" => return server_error!(StatementNotFoundException, code, message),
        "SQL-02048" => return server_error!(InternalException, code, message),
        "SQL-02050" => return server_error!(UnsupportedRuntimeFeatureException, code, message),
        "SQL-02052" => {
            return server_error!(BlockedByHighPriorityTransactionException, code, message)
        }
        "SQL-02054" => return server_error!(InvalidRuntimeValueException, code, message),
        "SQL-02056" => return server_error!(ValueOutOfRangeException, code, message),
        "SQL-02058" => return server_error!(ValueTooLongException, code, message),
        "SQL-02060" => return server_error!(InvalidDecimalValueException, code, message),
        "SQL-03000" => return server_error!(CompileException, code, message),
        "SQL-03001" => return server_error!(SyntaxException, code, message),
        "SQL-03002" => return server_error!(AnalyzeException, code, message),
        "SQL-03003" => return server_error!(TypeAnalyzeException, code, message),
        "SQL-03004" => return server_error!(SymbolAnalyzeException, code, message),
        "SQL-03005" => return server_error!(ValueAnalyzeException, code, message),
        "SQL-03010" => return server_error!(UnsupportedCompilerFeatureException, code, message),
        "SQL-04000" => return server_error!(CcException, code, message),
        "SQL-04001" => return server_error!(OccException, code, message),
        "SQL-04010" => return server_error!(OccReadException, code, message),
        "SQL-04015" => return server_error!(ConflictOnWritePreserveException, code, message),
        "SQL-04011" => return server_error!(OccWriteException, code, message),
        "SQL-04003" => return server_error!(LtxException, code, message),
        "SQL-04013" => return server_error!(LtxReadException, code, message),
        "SQL-04014" => return server_error!(LtxWriteException, code, message),
        "SQL-04005" => return server_error!(RtxException, code, message),
        "SQL-04007" => return server_error!(BlockedByConcurrentOperationException, code, message),
        _ => {}
    }
    match code.category_str().as_str() {
        "SQL" => return server_error!(SqlServiceException, code, message),
        _ => {}
    }
    server_error!(ServerException, code, message)
}
