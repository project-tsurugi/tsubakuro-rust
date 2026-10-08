use log::{debug, trace};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use std::sync::Arc;

use crate::{
    commit_option::CommitOption, connection::inner_connection::InnerConnection, cursor::Cursor,
    lob_transfer_type::LobTransferType, shutdown_option::ShutdownOption,
    table_metadata::TableMetadata, transaction_option::TransactionOption,
};

pub(crate) mod inner_connection;
mod internal;

/// Connection to Tsurugi.
///
/// Attributes:
///     transaction_option (TransactionOption): Transaction option. (write only)
///     commit_option (CommitOption): Commit option. (write only)
///     shutdown_option (ShutdownOption): Shutdown option. (write only)
///     closed (bool): Whether the connection is closed. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi", unsendable)]
pub struct Connection {
    inner: Arc<InnerConnection>,
}

impl Connection {
    pub(crate) fn new(inner: Arc<InnerConnection>) -> Self {
        Connection { inner }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Connection {
    /// Get the large object transfer type.
    ///
    /// Returns:
    ///     LobTransferType: Large object transfer type.
    ///
    /// Examples:
    ///     ```python
    ///     lob_transfer_type = connection.lob_transfer_type()
    ///     ```
    /// since 0.10.0
    pub fn lob_transfer_type(&self) -> PyResult<Option<LobTransferType>> {
        const FUNCTION_NAME: &str = "Connection.lob_transfer_type()";
        trace!("{FUNCTION_NAME} start");

        let connection = &self.inner;
        let session = connection.session();
        let core_type = session.lob_transfer_type();
        let lob_transfer_type = LobTransferType::from_core_lob_transfer_type(core_type);

        trace!("{FUNCTION_NAME} end");
        Ok(lob_transfer_type)
    }

    /// List table names.
    ///
    /// Returns:
    ///     List[str]: List of table names.
    ///
    /// Examples:
    ///     ```python
    ///     table_names = connection.list_tables()
    ///     ```
    pub fn list_tables(&self, py: Python) -> PyResult<Vec<String>> {
        const FUNCTION_NAME: &str = "Connection.list_tables()";
        self.check_closed(FUNCTION_NAME)?;

        py.detach(|| self.list_tables_internal())
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
    ///     import tsurugi_dbapi as tsurugi
    ///
    ///     try:
    ///         metadata = connection.get_table_metadata("my_table")
    ///     except tsurugi.error.TargetNotFoundException:
    ///         pass
    ///     ```
    pub fn get_table_metadata(&self, py: Python, table_name: &str) -> PyResult<TableMetadata> {
        const FUNCTION_NAME: &str = "Connection.get_table_metadata()";
        self.check_closed(FUNCTION_NAME)?;

        py.detach(|| self.get_table_metadata_internal(table_name))
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
    pub fn find_table_metadata(
        &self,
        py: Python,
        table_name: &str,
    ) -> PyResult<Option<TableMetadata>> {
        const FUNCTION_NAME: &str = "Connection.find_table_metadata()";
        self.check_closed(FUNCTION_NAME)?;

        py.detach(|| self.find_table_metadata_internal(table_name))
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
    #[pyo3(signature = ())]
    pub fn cursor(py_self: Py<Self>, py: Python) -> PyResult<Cursor> {
        const FUNCTION_NAME: &str = "Connection.cursor()";
        trace!("{FUNCTION_NAME} start");

        let connection = {
            let slf = py_self.borrow(py);
            slf.check_closed(FUNCTION_NAME)?;

            slf.inner.clone()
        };

        let cursor = Cursor::new(py_self, connection);

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
    pub fn commit(&mut self, py: Python, option: Option<CommitOption>) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.commit()";
        self.check_closed(FUNCTION_NAME)?;

        py.detach(|| self.commit_internal(option))
    }

    /// Rollback the current transaction.
    ///
    /// Examples:
    ///     ```python
    ///     connection.rollback()
    ///     ```
    pub fn rollback(&mut self, py: Python) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.rollback()";
        self.check_closed(FUNCTION_NAME)?;

        py.detach(|| self.rollback_internal())
    }

    // with
    /// Enter the runtime context related to this object.
    pub fn __enter__(slf: Bound<Self>) -> Bound<Self> {
        slf
    }

    /// Exit the runtime context related to this object.
    pub fn __exit__(
        &mut self,
        py: Python,
        _exc_type: Option<Bound<PyAny>>,
        exc_value: Option<Bound<PyAny>>,
        _traceback: Option<Bound<PyAny>>,
    ) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.__exit__()";
        trace!("{FUNCTION_NAME} start");

        let result = py.detach(|| self.close_internal());

        match result {
            Ok(_) => {
                trace!("{FUNCTION_NAME} end");
                Ok(())
            }
            Err(e) => {
                debug!("{FUNCTION_NAME} error: {:?}", e);
                if exc_value.is_none() {
                    Err(e)
                } else {
                    Ok(())
                }
            }
        }
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
    pub fn close(&mut self, py: Python) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Connection.close()";
        trace!("{FUNCTION_NAME} start");

        let result = py.detach(|| self.close_internal());

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        };
        result
    }

    /// Whether the connection is closed.
    #[getter]
    pub fn closed(&self) -> bool {
        self.inner.is_closed()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        const FUNCTION_NAME: &str = "Connection.drop()";
        trace!("{FUNCTION_NAME} start. closed={}", self.closed());

        match self.close_internal() {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
    }
}
