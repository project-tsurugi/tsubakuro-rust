use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

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
    use pyo3_stub_gen::{derive::*, inventory::submit};

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
        SecondaryIndexCorruptionException, ServerException, SqlExecutionException,
        SqlLimitReachedException, SqlRequestTimeoutException, SqlServiceException,
        StatementNotFoundException, SymbolAnalyzeException, SyntaxException,
        TargetAlreadyExistsException, TargetNotFoundException, TransactionExceededLimitException,
        TransactionNotFoundException, TypeAnalyzeException, UniqueConstraintViolationException,
        UnresolvedPlaceholderException, UnsupportedCompilerFeatureException,
        UnsupportedRuntimeFeatureException, ValueAnalyzeException, ValueEvaluationException,
        ValueOutOfRangeException, ValueTooLongException, WriteOperationByRtxException,
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
    use crate::column::Column;
    #[pymodule_export]
    use crate::table_metadata::TableMetadata;

    #[pymodule_export]
    use crate::cursor::Cursor;

    #[pymodule_export]
    use crate::type_code::{
        bool::Bool, bytes::Bytes, date::Date, datetime::Datetime, decimal::Decimal,
        float32::Float32, float64::Float64, int32::Int32, int64::Int64,
        offset_datetime::OffsetDatetime, offset_time::OffsetTime, str::Str, time::Time,
    };

    /// Initialize env_logger.
    ///
    /// Args:
    ///     filters (str, optional): filter string. If ommitted, `"tsubakuro_rust_python=info"` is used.
    ///     file_path (str, optional): log file path. If None, logs to stderr.
    ///
    /// Examples:
    ///     ```python
    ///     import tsubakuro_rust_python as tsurugi
    ///
    ///     tsurugi.env_logger_init("tsubakuro_rust_python=trace")
    ///     ```
    ///
    /// Note:
    ///     Calls to `env_logger_init` other than the first one are ignored.
    #[gen_stub_pyfunction]
    #[pyfunction]
    #[pyo3(signature = (filters="tsubakuro_rust_python=info", file_path=None))]
    fn env_logger_init(filters: &str, file_path: Option<String>) {
        crate::logger::env_logger_init(filters, file_path);
    }

    /// Constructor for creating a connection to the Tsurugi.
    ///
    /// Args:
    ///     *args (Config, optional): configuration object.
    ///     **kwargs (dict, optional): e.g. `endpoint="tcp://localhost:12345"`, `user="tsurugi"`
    ///
    /// Returns:
    ///     Connection: Connection object.
    ///
    /// Examples:
    ///     ```python
    ///     import tsubakuro_rust_python as tsurugi
    ///
    ///     config = tsurugi.Config()
    ///     config.endpoint = "tcp://localhost:12345"
    ///     config.user = "tsurugi"
    ///     config.password = "password"
    ///     config.default_timeout = 30 // seconds
    ///     with tsurugi.connect(config) as connection:
    ///         pass
    ///     ```
    ///
    ///     ```python
    ///     import tsubakuro_rust_python as tsurugi
    ///
    ///     with tsurugi.connect(endpoint="tcp://localhost:12345", user="tsurugi", password="password", default_timeout=30) as connection:
    ///         pass
    ///     ```
    #[gen_stub_pyfunction]
    #[pyfunction]
    #[pyo3(signature = (*args, **kwargs))]
    fn connect(args: &Bound<PyTuple>, kwargs: Option<Bound<PyDict>>) -> PyResult<Connection> {
        let connection = Connection::connect(args, kwargs)?;
        Ok(connection)
    }

    submit! {
        gen_function_from_python!{r#"
        import tsubakuro_rust_python

        @overload
        def connect(*args: tsubakuro_rust_python.Config) -> tsubakuro_rust_python.Connection: ...
        "#}
    }

    submit! {
        gen_function_from_python!{r#"
        import tsubakuro_rust_python

        @overload
        def connect(*args: tsubakuro_rust_python.TransactionOption | tsubakuro_rust_python.CommitOption | tsubakuro_rust_python.ShutdownOption | str) -> tsubakuro_rust_python.Connection: ...
        "#}
    }

    submit! {
        gen_function_from_python!{r#"
        import tsubakuro_rust_python

        @overload
        def connect(**kwargs: str | int | tsubakuro_rust_python.TransactionOption | tsubakuro_rust_python.CommitOption | tsubakuro_rust_python.ShutdownOption) -> tsubakuro_rust_python.Connection: ...
        "#}
    }
}

define_stub_info_gatherer!(stub_info);
