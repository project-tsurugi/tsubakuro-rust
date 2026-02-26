use log::{debug, trace};
use pyo3::{prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use std::sync::Arc;
use tsubakuro_rust_core::prelude::{name::TName, Session, SqlClient, TgError};

use crate::{
    commit_option::CommitOption,
    config::Config,
    connection::inner_connection::InnerConnection,
    cursor::Cursor,
    error::{to_pyerr, ProgrammingError},
    shutdown_option::ShutdownOption,
    table_metadata::TableMetadata,
    transaction_option::TransactionOption,
};

pub(crate) mod inner_connection;

/// Connection to Tsurugi.
///
/// Attributes:
///     transaction_option (TransactionOption): Transaction option. (write only)
///     commit_option (CommitOption): Commit option. (write only)
///     shutdown_option (ShutdownOption): Shutdown option. (write only)
///     closed (bool): Whether the connection is closed. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python")]
pub struct Connection {
    inner: Arc<InnerConnection>,
    close_on_drop: bool,
}

impl Connection {
    pub(crate) fn new(inner: Arc<InnerConnection>, close_on_drop: bool) -> Self {
        Connection {
            inner,
            close_on_drop,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Connection {
    /// List table names.
    ///
    /// Returns:
    ///     List[str]: List of table names.
    ///
    /// Examples:
    ///     ```python
    ///     table_names = connection.list_tables()
    ///     ```
    pub fn list_tables(&self) -> PyResult<Vec<String>> {
        const FUNCTION_NAME: &str = "Connection.list_tables()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start");

        let connection = &self.inner;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

        match runtime.block_on(sql_client.list_tables_for(timeout)) {
            Ok(table_list) => {
                let table_names = table_list
                    .table_names()
                    .iter()
                    .map(TName::to_string)
                    .collect();
                trace!("{FUNCTION_NAME} end");
                Ok(table_names)
            }
            Err(e) => {
                trace!("{FUNCTION_NAME} error: {:?}", e);
                Err(to_pyerr(e))
            }
        }
    }

    /// Get table metadata.
    ///
    /// Args:
    ///     table_name (str): Table name.
    ///
    /// Returns:
    ///    TableMetadata: Table metadata.
    ///
    /// Raises:
    ///     TargetNotFoundException: If the table does not exist.
    ///
    /// Examples:
    ///     ```python
    ///     import tsubakuro_rust_python as tsurugi
    ///
    ///     try:
    ///         metadata = connection.get_table_metadata("my_table")
    ///     except tsurugi.error.TargetNotFoundException:
    ///         pass
    ///     ```
    pub fn get_table_metadata(&self, table_name: &str) -> PyResult<TableMetadata> {
        const FUNCTION_NAME: &str = "Connection.get_table_metadata()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start. table_name={}", table_name);

        let result = self
            .get_table_metadata_internal(table_name)
            .map_err(to_pyerr);

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => trace!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    /// Find table metadata.
    ///
    /// Args:
    ///     table_name (str): Table name.
    ///
    /// Returns:
    ///     Optional[TableMetadata]: Table metadata, or None if the table does not exist.
    ///
    /// Examples:
    ///     ```python
    ///     metadata = connection.find_table_metadata("my_table")
    ///     ```
    pub fn find_table_metadata(&self, table_name: &str) -> PyResult<Option<TableMetadata>> {
        const FUNCTION_NAME: &str = "Connection.find_table_metadata()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start. table_name={}", table_name);

        match self.get_table_metadata_internal(table_name) {
            Ok(metadata) => {
                trace!("{FUNCTION_NAME} end");
                Ok(Some(metadata))
            }
            Err(e) => {
                let code = e.diagnostic_code();
                if let Some(code) = code {
                    if code.name() == "TARGET_NOT_FOUND_EXCEPTION" {
                        trace!("{FUNCTION_NAME} end: table not found");
                        return Ok(None);
                    }
                }

                trace!("{FUNCTION_NAME} error: {:?}", e);
                Err(to_pyerr(e))
            }
        }
    }

    /// Create a new cursor object using the connection.
    ///
    /// Returns:
    ///     Cursor: Cursor object.
    ///
    /// Examples:
    ///     ```python
    ///     with connection.cursor() as cursor:
    ///        pass
    ///     ```
    pub fn cursor(&self) -> PyResult<Cursor> {
        const FUNCTION_NAME: &str = "Connection.cursor()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start");

        let cursor = Cursor::new(self.inner.clone());

        trace!("{FUNCTION_NAME} end");
        Ok(cursor)
    }

    /// Transaction option.
    #[setter]
    pub fn set_transaction_option(&mut self, option: TransactionOption) {
        const FUNCTION_NAME: &str = "Connection.set_transaction_option()";
        trace!("{FUNCTION_NAME} start. option={:?}", option);

        let connection = &mut self.inner;
        connection.set_transaction_option(option);

        trace!("{FUNCTION_NAME} end");
    }

    /// Commit option.
    #[setter]
    pub fn set_commit_option(&mut self, option: CommitOption) {
        const FUNCTION_NAME: &str = "Connection.set_commit_option()";
        trace!("{FUNCTION_NAME} start. option={:?}", option);

        let connection = &self.inner;
        connection.set_commit_option(option);

        trace!("{FUNCTION_NAME} end");
    }

    /// Commit the current transaction.
    ///
    /// Args:
    ///     option (CommitOption, optional): CommitOption object.
    ///
    /// Examples:
    ///     ```python
    ///     connection.commit()
    ///     ```
    #[pyo3(signature = (option=None))]
    pub fn commit(&mut self, option: Option<CommitOption>) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.commit()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start. option={:?}", option);

        let timeout = option.as_ref().and_then(CommitOption::commit_timeout);
        let connection = &self.inner;
        let result = if let Some(option) = option {
            let option = option.to_core_commit_option();
            connection.commit(Some(option), timeout)
        } else {
            connection.commit(None, timeout)
        };

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => trace!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    /// Rollback the current transaction.
    ///
    /// Examples:
    ///     ```python
    ///     connection.rollback()
    ///     ```
    pub fn rollback(&mut self) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.rollback()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start");

        let connection = &self.inner;
        let result = connection.rollback();

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => trace!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    // with
    /// Enter the runtime context related to this object.
    pub fn __enter__(slf: Bound<Self>) -> Bound<Self> {
        slf
    }

    /// Exit the runtime context related to this object.
    pub fn __exit__(
        &mut self,
        _exc_type: Option<Bound<PyAny>>,
        _exc_value: Option<Bound<PyAny>>,
        _traceback: Option<Bound<PyAny>>,
    ) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.__exit__()";
        trace!("{FUNCTION_NAME} start");

        let result = self.close_internal();

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => trace!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    // ==
    /// Compare if two Connection objects are the same connection.
    pub fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    /// Get the hash value of the connection object.
    pub fn __hash__(&self) -> usize {
        Arc::as_ptr(&self.inner) as usize
    }

    /// Shutdown option.
    #[setter]
    pub fn set_shutdown_option(&mut self, option: ShutdownOption) {
        const FUNCTION_NAME: &str = "Connection.set_shutdown_option()";
        trace!("{FUNCTION_NAME} start. option={:?}", option);

        let connection = &self.inner;
        connection.set_shutdown_option(option);

        trace!("{FUNCTION_NAME} end");
    }

    /// Close the connection.
    pub fn close(&mut self) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.close()";
        trace!("{FUNCTION_NAME} start");

        let result = self.close_internal();

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => trace!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    /// Whether the connection is closed.
    #[getter]
    pub fn closed(&self) -> bool {
        self.inner.is_closed()
    }
}

impl Connection {
    pub(crate) fn connect(
        args: &Bound<PyTuple>,
        kwargs: Option<Bound<PyDict>>,
    ) -> PyResult<Connection> {
        const FUNCTION_NAME: &str = "connect()";
        trace!("{FUNCTION_NAME} start");

        let config = Self::create_config(args, kwargs)?;

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let session = runtime.block_on(Self::session_connect(&config))?;
        let sql_client: SqlClient = session.make_client();

        let connection = InnerConnection::new(config, runtime, session, sql_client);

        trace!("{FUNCTION_NAME} end");
        Ok(Connection::new(Arc::new(connection), true))
    }

    fn create_config(args: &Bound<PyTuple>, kwargs: Option<Bound<PyDict>>) -> PyResult<Config> {
        let config = Config::new(args, kwargs)?;
        Ok(config)
    }

    async fn session_connect(config: &Config) -> PyResult<Arc<Session>> {
        let connection_option = config.connection_option()?;
        let timeout = config.connect_timeout();

        let session = Session::connect_for(&connection_option, timeout)
            .await
            .map_err(to_pyerr)?;
        Ok(session)
    }

    fn get_table_metadata_internal(&self, table_name: &str) -> Result<TableMetadata, TgError> {
        let connection = &self.inner;
        let runtime = connection.runtime();
        let sql_client = connection.sql_client();
        let timeout = connection.default_timeout();

        let table_metadata =
            runtime.block_on(sql_client.get_table_metadata_for(table_name, timeout))?;

        Ok(TableMetadata::new(table_metadata))
    }

    fn close_internal(&mut self) -> PyResult<()> {
        let connection = &self.inner;
        connection.close()
    }

    fn check_closed(&self, function_name: &str) -> PyResult<()> {
        if self.closed() {
            trace!("{}: Connection is already closed", function_name);
            return Err(ProgrammingError::new_err("Connection is already closed"));
        }
        Ok(())
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Connection {}

impl Drop for Connection {
    fn drop(&mut self) {
        const FUNCTION_NAME: &str = "Connection.drop()";

        if self.close_on_drop {
            trace!("{FUNCTION_NAME} start. closed={}", self.closed());

            match self.close_internal() {
                Ok(_) => trace!("{FUNCTION_NAME} end"),
                Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
            }
        } else {
            trace!("{FUNCTION_NAME} skip");
        }
    }
}
