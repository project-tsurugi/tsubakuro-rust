use pyo3::{exceptions::PyException, *};
use pyo3_stub_gen::create_exception;
use tsubakuro_rust_core::prelude::{DiagnosticCode, TgError};

create_exception!(
    tsubakuro_rust_python,
    Warning,
    PyException,
    "important warning (PEP 249)"
);
create_exception!(
    tsubakuro_rust_python,
    Error,
    PyException,
    "base class of all other exceptions (PEP 249)"
);

// Subclass of Error
create_exception!(
    tsubakuro_rust_python,
    InterfaceError,
    Error,
    "interface error (PEP 249)"
);
create_exception!(
    tsubakuro_rust_python,
    DatabaseError,
    Error,
    "database error (PEP 249)"
);

// Subclasses of DatabaseError
create_exception!(
    tsubakuro_rust_python,
    DataError,
    DatabaseError,
    "data error (PEP 249)"
);
create_exception!(
    tsubakuro_rust_python,
    OperationalError,
    DatabaseError,
    "operation error (PEP 249)"
);
create_exception!(
    tsubakuro_rust_python,
    IntegrityError,
    DatabaseError,
    "integrity error (PEP 249)"
);
create_exception!(
    tsubakuro_rust_python,
    InternalError,
    DatabaseError,
    "internal error (PEP 249)"
);
create_exception!(
    tsubakuro_rust_python,
    ProgrammingError,
    DatabaseError,
    "programming error (PEP 249)"
);
create_exception!(
    tsubakuro_rust_python,
    NotSupportedError,
    DatabaseError,
    "not supported error (PEP 249)"
);

// ServerException
create_exception!(
    tsubakuro_rust_python.error,
    ServerException,
    OperationalError,
    "Tsurugi ServerException"
);

// SqlServiceException
create_exception!(
    tsubakuro_rust_python.error,
    SqlServiceException,
    ServerException,
    "Tsurugi SqlServiceException"
);
create_exception!(
    tsubakuro_rust_python.error,
    SqlExecutionException,
    SqlServiceException,
    "Tsurugi SqlExecutionException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ConstraintViolationException,
    IntegrityError,
    "Tsurugi ConstraintViolationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    UniqueConstraintViolationException,
    ConstraintViolationException,
    "Tsurugi UniqueConstraintViolationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    NotNullConstraintViolationException,
    ConstraintViolationException,
    "Tsurugi NotNullConstraintViolationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ReferentialIntegrityConstraintViolationException,
    ConstraintViolationException,
    "Tsurugi ReferentialIntegrityConstraintViolationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    CheckConstraintViolationException,
    ConstraintViolationException,
    "Tsurugi CheckConstraintViolationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    EvaluationException,
    DataError,
    "Tsurugi EvaluationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ValueEvaluationException,
    EvaluationException,
    "Tsurugi ValueEvaluationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ScalarSubqueryEvaluationException,
    EvaluationException,
    "Tsurugi ScalarSubqueryEvaluationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    TargetNotFoundException,
    SqlExecutionException,
    "Tsurugi TargetNotFoundException"
);
create_exception!(
    tsubakuro_rust_python.error,
    TargetAlreadyExistsException,
    SqlExecutionException,
    "Tsurugi TargetAlreadyExistsException"
);
create_exception!(
    tsubakuro_rust_python.error,
    InconsistentStatementException,
    SqlExecutionException,
    "Tsurugi InconsistentStatementException"
);
create_exception!(
    tsubakuro_rust_python.error,
    RestrictedOperationException,
    SqlExecutionException,
    "Tsurugi RestrictedOperationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    DependenciesViolationException,
    RestrictedOperationException,
    "Tsurugi DependenciesViolationException"
);
create_exception!(
    tsubakuro_rust_python.error,
    WriteOperationByRtxException,
    RestrictedOperationException,
    "Tsurugi WriteOperationByRtxException"
);
create_exception!(
    tsubakuro_rust_python.error,
    LtxWriteOperationWithoutWritePreserveException,
    RestrictedOperationException,
    "Tsurugi LtxWriteOperationWithoutWritePreserveException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ReadOperationOnRestrictedReadAreaException,
    RestrictedOperationException,
    "Tsurugi ReadOperationOnRestrictedReadAreaException"
);
create_exception!(
    tsubakuro_rust_python.error,
    InactiveTransactionException,
    RestrictedOperationException,
    "Tsurugi InactiveTransactionException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ParameterException,
    ProgrammingError,
    "Tsurugi ParameterException"
);
create_exception!(
    tsubakuro_rust_python.error,
    UnresolvedPlaceholderException,
    ParameterException,
    "Tsurugi UnresolvedPlaceholderException"
);
create_exception!(
    tsubakuro_rust_python.error,
    LoadFileException,
    SqlExecutionException,
    "Tsurugi LoadFileException"
);
create_exception!(
    tsubakuro_rust_python.error,
    LoadFileNotFoundException,
    LoadFileException,
    "Tsurugi LoadFileNotFoundException"
);
create_exception!(
    tsubakuro_rust_python.error,
    LoadFileFormatException,
    LoadFileException,
    "Tsurugi LoadFileFormatException"
);
create_exception!(
    tsubakuro_rust_python.error,
    DumpFileException,
    SqlExecutionException,
    "Tsurugi DumpFileException"
);
create_exception!(
    tsubakuro_rust_python.error,
    DumpDirectoryInaccessibleException,
    DumpFileException,
    "Tsurugi DumpDirectoryInaccessibleException"
);
create_exception!(
    tsubakuro_rust_python.error,
    SqlLimitReachedException,
    DataError,
    "Tsurugi SqlLimitReachedException"
);
create_exception!(
    tsubakuro_rust_python.error,
    TransactionExceededLimitException,
    SqlLimitReachedException,
    "Tsurugi TransactionExceededLimitException"
);
create_exception!(
    tsubakuro_rust_python.error,
    SqlRequestTimeoutException,
    SqlExecutionException,
    "Tsurugi SqlRequestTimeoutException"
);
create_exception!(
    tsubakuro_rust_python.error,
    DataCorruptionException,
    SqlExecutionException,
    "Tsurugi DataCorruptionException"
);
create_exception!(
    tsubakuro_rust_python.error,
    SecondaryIndexCorruptionException,
    DataCorruptionException,
    "Tsurugi SecondaryIndexCorruptionException"
);
create_exception!(
    tsubakuro_rust_python.error,
    RequestFailureException,
    SqlExecutionException,
    "Tsurugi RequestFailureException"
);
create_exception!(
    tsubakuro_rust_python.error,
    TransactionNotFoundException,
    RequestFailureException,
    "Tsurugi TransactionNotFoundException"
);
create_exception!(
    tsubakuro_rust_python.error,
    StatementNotFoundException,
    RequestFailureException,
    "Tsurugi StatementNotFoundException"
);
create_exception!(
    tsubakuro_rust_python.error,
    InternalException,
    InternalError,
    "Tsurugi InternalException"
);
create_exception!(
    tsubakuro_rust_python.error,
    UnsupportedRuntimeFeatureException,
    NotSupportedError,
    "Tsurugi UnsupportedRuntimeFeatureException"
);
create_exception!(
    tsubakuro_rust_python.error,
    BlockedByHighPriorityTransactionException,
    SqlExecutionException,
    "Tsurugi BlockedByHighPriorityTransactionException"
);
create_exception!(
    tsubakuro_rust_python.error,
    InvalidRuntimeValueException,
    SqlLimitReachedException,
    "Tsurugi InvalidRuntimeValueException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ValueOutOfRangeException,
    InvalidRuntimeValueException,
    "Tsurugi ValueOutOfRangeException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ValueTooLongException,
    InvalidRuntimeValueException,
    "Tsurugi ValueTooLongException"
);
create_exception!(
    tsubakuro_rust_python.error,
    InvalidDecimalValueException,
    InvalidRuntimeValueException,
    "Tsurugi InvalidDecimalValueException"
);
create_exception!(
    tsubakuro_rust_python.error,
    CompileException,
    ProgrammingError,
    "Tsurugi CompileException"
);
create_exception!(
    tsubakuro_rust_python.error,
    SyntaxException,
    CompileException,
    "Tsurugi SyntaxException"
);
create_exception!(
    tsubakuro_rust_python.error,
    AnalyzeException,
    CompileException,
    "Tsurugi AnalyzeException"
);
create_exception!(
    tsubakuro_rust_python.error,
    TypeAnalyzeException,
    AnalyzeException,
    "Tsurugi TypeAnalyzeException"
);
create_exception!(
    tsubakuro_rust_python.error,
    SymbolAnalyzeException,
    AnalyzeException,
    "Tsurugi SymbolAnalyzeException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ValueAnalyzeException,
    AnalyzeException,
    "Tsurugi ValueAnalyzeException"
);
create_exception!(
    tsubakuro_rust_python.error,
    UnsupportedCompilerFeatureException,
    NotSupportedError,
    "Tsurugi UnsupportedCompilerFeatureException"
);
create_exception!(
    tsubakuro_rust_python.error,
    CcException,
    SqlServiceException,
    "Tsurugi CcException"
);
create_exception!(
    tsubakuro_rust_python.error,
    OccException,
    CcException,
    "Tsurugi OccException"
);
create_exception!(
    tsubakuro_rust_python.error,
    OccReadException,
    OccException,
    "Tsurugi OccReadException"
);
create_exception!(
    tsubakuro_rust_python.error,
    ConflictOnWritePreserveException,
    OccReadException,
    "Tsurugi ConflictOnWritePreserveException"
);
create_exception!(
    tsubakuro_rust_python.error,
    OccWriteException,
    OccException,
    "Tsurugi OccWriteException"
);
create_exception!(
    tsubakuro_rust_python.error,
    LtxException,
    CcException,
    "Tsurugi LtxException"
);
create_exception!(
    tsubakuro_rust_python.error,
    LtxReadException,
    LtxException,
    "Tsurugi LtxReadException"
);
create_exception!(
    tsubakuro_rust_python.error,
    LtxWriteException,
    LtxException,
    "Tsurugi LtxWriteException"
);
create_exception!(
    tsubakuro_rust_python.error,
    RtxException,
    CcException,
    "Tsurugi RtxException"
);
create_exception!(
    tsubakuro_rust_python.error,
    BlockedByConcurrentOperationException,
    CcException,
    "Tsurugi BlockedByConcurrentOperationException"
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

#[pymodule]
pub(crate) mod error {
    // ServerException
    #[pymodule_export]
    use super::ServerException;

    // SqlServiceException
    #[pymodule_export]
    use super::{
        AnalyzeException, BlockedByConcurrentOperationException,
        BlockedByHighPriorityTransactionException, CcException, CheckConstraintViolationException,
        CompileException, ConflictOnWritePreserveException, ConstraintViolationException,
        DataCorruptionException, DependenciesViolationException,
        DumpDirectoryInaccessibleException, DumpFileException, EvaluationException,
        InactiveTransactionException, InconsistentStatementException, InternalException,
        InvalidDecimalValueException, InvalidRuntimeValueException, LoadFileException,
        LoadFileFormatException, LoadFileNotFoundException, LtxException, LtxReadException,
        LtxWriteException, LtxWriteOperationWithoutWritePreserveException,
        NotNullConstraintViolationException, OccException, OccReadException, OccWriteException,
        ParameterException, ReadOperationOnRestrictedReadAreaException,
        ReferentialIntegrityConstraintViolationException, RequestFailureException,
        RestrictedOperationException, RtxException, ScalarSubqueryEvaluationException,
        SecondaryIndexCorruptionException, SqlExecutionException, SqlLimitReachedException,
        SqlRequestTimeoutException, SqlServiceException, StatementNotFoundException,
        SymbolAnalyzeException, SyntaxException, TargetAlreadyExistsException,
        TargetNotFoundException, TransactionExceededLimitException, TransactionNotFoundException,
        TypeAnalyzeException, UniqueConstraintViolationException, UnresolvedPlaceholderException,
        UnsupportedCompilerFeatureException, UnsupportedRuntimeFeatureException,
        ValueAnalyzeException, ValueEvaluationException, ValueOutOfRangeException,
        ValueTooLongException, WriteOperationByRtxException,
    };
}
