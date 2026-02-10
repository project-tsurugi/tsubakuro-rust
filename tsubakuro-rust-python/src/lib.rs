use pyo3::prelude::*;

mod column;
mod commit_option;
mod config;
mod connection;
mod cursor;
mod error;
mod logger;
mod shutdown_option;
mod table_metadata;
mod transaction_option;
mod type_code;

#[pymodule]
mod tsubakuro_rust_python {
    use pyo3::{prelude::*, types::*};

    #[pymodule_export]
    #[allow(non_upper_case_globals)]
    const apilevel: &str = "2.0";

    #[pymodule_export]
    #[allow(non_upper_case_globals)]
    const threadsafety: u8 = 1;

    #[pymodule_export]
    #[allow(non_upper_case_globals)]
    const paramstyle: &str = "qmark";

    #[pymodule_export]
    use crate::error::{
        DataError, DatabaseError, Error, IntegrityError, InterfaceError, InternalError,
        NotSupportedError, OperationalError, ProgrammingError, Warning,
    };

    // SqlServiceException
    #[pymodule_export]
    use crate::error::{
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

    #[pymodule_export]
    use crate::config::Config;

    #[pymodule_export]
    use crate::transaction_option::{TransactionOption, TransactionType};

    #[pymodule_export]
    use crate::commit_option::{CommitOption, CommitType};

    #[pymodule_export]
    use crate::shutdown_option::{ShutdownOption, ShutdownType};

    #[pymodule_export]
    use crate::connection::Connection;

    #[pymodule_export]
    use crate::cursor::Cursor;

    #[pymodule_export]
    use crate::type_code::{
        Bool, Bytes, Date, Datetime, Decimal, Float32, Float64, Int32, Int64, OffsetDatetime,
        OffsetTime, Str, Time,
    };

    /// Initialize env_logger.
    ///
    /// # Parameters
    /// - `filters` - filter string. (e.g. "tsubakuro_rust_python=trace")
    ///               If ommitted, "tsubakuro_rust_python=info" is used.
    /// - `file_path` - log file path. If None, logs to stderr.
    ///
    /// Calls to `env_logger_init` other than the first one are ignored.
    #[pyfunction]
    #[pyo3(signature = (filters="tsubakuro_rust_python=info", file_path=None))]
    fn env_logger_init(filters: &str, file_path: Option<String>) {
        crate::logger::env_logger_init(filters, file_path);
    }

    /// Constructor for creating a connection to the Tsurugi.
    ///
    /// # Parameters
    /// - `args` - see [`Config`].
    /// - `kwargs` - e.g. `endpoint="tcp://localhost:12345"`, `user="tsurugi"``
    ///
    /// # Returns
    /// [`Connection`] object.
    #[pyfunction]
    #[pyo3(signature = (*args, **kwargs))]
    fn connect(args: &Bound<PyTuple>, kwargs: Option<Bound<PyDict>>) -> PyResult<Connection> {
        let connection = Connection::connect(args, kwargs)?;
        Ok(connection)
    }
}
