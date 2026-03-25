use log::{debug, trace};
use pyo3::{exceptions::PyStopIteration, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use std::{collections::HashMap, sync::Arc, vec};
use tsubakuro_rust_core::prelude::{AtomType, SqlPreparedStatement, SqlQueryResult};

use crate::{
    column::columns_description,
    connection::{inner_connection::InnerConnection, Connection},
    cursor::query_result::next_row1,
    error::{to_pyerr, NotSupportedError, OperationalError, ProgrammingError},
};

mod execute;
mod query_result;

pub(crate) struct RowNumber {
    row_number: usize,
}

impl RowNumber {
    pub(crate) fn new() -> Self {
        RowNumber { row_number: 0 }
    }

    pub(crate) fn increment(&mut self) {
        self.row_number += 1;
    }

    pub(crate) fn get(&self) -> usize {
        self.row_number
    }
}

/// Cursor object for executing SQL statements and fetching results.
///
/// Attributes:
///     connection (Connection): Connection object associated with the cursor. (read only)
///     description (Optional[Sequence[Tuple[str, str, None, Optional[int], Optional[int], Optional[int], Optional[bool]]]]): Description of the query result set.
///         `(name, type_code, display_size, internal_size, precision, scale, null_ok)`.  (read only)
///     arraysize (int): Number of rows to fetch at a time with `Cursor.fetchmany()`. Default is 1.
///     rownumber (int): Current row number (0-based). (read only)
///     rowcount (int): Number of rows affected by the last `Cursor.execute*()` method. -1 if not applicable. (read only)
///     closed (bool): Whether the cursor is closed. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi")]
pub struct Cursor {
    py_connection: Py<Connection>,
    connection: Arc<InnerConnection>,
    /// Whether to execute `Cursor.executemany()` asynchronously. Default is `True`.
    #[pyo3(set, get)]
    executemany_async: bool, // internally used
    ps_map: HashMap<String, (SqlPreparedStatement, HashMap<String, AtomType>)>,
    query_result: Option<SqlQueryResult>,
    query_types: Vec<AtomType>,
    row_number: Option<RowNumber>,
    /// Number of rows to fetch at a time with `Cursor.fetchmany()`.
    #[pyo3(get)]
    arraysize: usize,
    /// Number of rows affected by the last `Cursor.execute*()` method.
    #[pyo3(get)]
    rowcount: isize,
    /// Whether the cursor is closed.
    #[pyo3(get)]
    closed: bool,
}

impl Cursor {
    pub(crate) fn new(py_connection: Py<Connection>, connection: Arc<InnerConnection>) -> Self {
        Self {
            py_connection,
            connection,
            executemany_async: true,
            ps_map: HashMap::new(),
            query_result: None,
            query_types: Vec::new(),
            row_number: None,
            arraysize: 1,
            rowcount: -1,
            closed: false,
        }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl Cursor {
    /// Connection object associated with the cursor. (read only)
    #[getter]
    pub fn connection(&self) -> &Py<Connection> {
        &self.py_connection
    }

    /// Execute a SQL statement.
    ///
    /// Args:
    ///     operation (str): SQL statement to be executed.
    ///     parameters (Tuple[Any, ...] | dict[str, Any], optional): Parameters for the SQL statement.
    ///
    /// Examples:
    ///     ```python
    ///     cursor.execute("insert into example values (1, 'Hello')")
    ///     connection.commit()
    ///     ```
    ///
    ///     ```python
    ///     cursor.execute("insert into example values (?, ?)", (1, "Hello"))
    ///     connection.commit()
    ///     ```
    ///
    ///     ```python
    ///     cursor.execute("insert into example values (:id, :name)", {"id": 1, "name": "Hello"})
    ///     connection.commit()
    ///     ```
    #[pyo3(signature = (operation, parameters=None))]
    pub fn execute(
        &mut self,
        py: Python,
        operation: &str,
        parameters: Option<Bound<PyAny>>,
    ) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Cursor.execute()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start. operation={}", operation);

        let result = if let Some(parameters) = parameters {
            let vec = vec![parameters];
            let seq_of_parameters = vec.into_pyobject(py)?;
            self.execute_with_parameters(operation, seq_of_parameters)
        } else {
            self.execute_direct(operation)
        };

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result
    }

    /// Prepare a SQL statement for execution.
    ///
    /// Args:
    ///     operation (str): SQL statement to be prepared.
    ///     parameters (Tuple[Any, ...] | dict[str, Any]): Parameters for the SQL statement.
    ///
    /// Examples:
    ///     ```python
    ///     import tsurugi_dbapi as tsurugi
    ///
    ///     sql = "insert into example values (?, ?)"
    ///     cursor.prepare(sql, (tsurugi.type_code.Int64, tsurugi.type_code.Str))
    ///     cursor.execute(sql, (1, "Hello"))
    ///     connection.commit()
    ///     ```
    ///
    ///     ```python
    ///     import tsurugi_dbapi as tsurugi
    ///
    ///     sql = "insert into example values (:id, :name)"
    ///     cursor.prepare(sql, {"id": tsurugi.type_code.Int64, "name": tsurugi.type_code.Str})
    ///     cursor.execute(sql, {"id": 1, "name": "Hello"})
    ///     connection.commit()
    ///     ```
    pub fn prepare(&mut self, operation: &str, parameters: Bound<PyAny>) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Cursor.prepare()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start. operation={}", operation);

        let result = self.prepare_placeholders(operation, parameters);

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result
    }

    /// Execute a prepared SQL statement multiple times.
    ///
    /// Args:
    ///     operation (str): SQL statement to be executed.
    ///     seq_of_parameters (Sequence[Tuple[Any, ...] | dict[str, Any]]): Sequence of parameters for the SQL statement.
    ///
    /// Examples:
    ///     ```python
    ///     cursor.executemany("insert into example values (?, ?)", [(1, "Hello"), (2, "World")])
    ///     connection.commit()
    ///     ```
    ///
    ///     ```python
    ///     cursor.executemany("insert into example values (:id, :name)", [{"id": 1, "name": "Hello"}, {"id": 2, "name": "World"}])
    ///     connection.commit()
    ///     ```
    pub fn executemany(
        &mut self,
        operation: &str,
        seq_of_parameters: Bound<PyAny>,
    ) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Cursor.executemany()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start. operation={}", operation);

        let result = self.execute_with_parameters(operation, seq_of_parameters);

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result
    }

    /// Description of the query result set.
    #[getter]
    #[gen_stub(override_return_type(type_repr = "Optional[Sequence[Tuple[
        str,           # name
        str,           # type_code
        None,          # display_size
        Optional[int], # internal_size
        Optional[int], # precision
        Optional[int], # scale
        Optional[bool] # null_ok
    ]]]"))]
    pub fn description<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyTuple>>> {
        const FUNCTION_NAME: &str = "Cursor.description()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start");

        let qr = if let Some(qr) = &self.query_result {
            qr
        } else {
            trace!("{FUNCTION_NAME} end. No query result");
            return Ok(None);
        };

        let metadata = qr
            .get_metadata()
            .ok_or_else(|| OperationalError::new_err("Failed to get query metadata"))?;
        let columns = metadata.columns();
        let result = columns_description(py, columns);

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result.map(Some)
    }

    /// Fetch the next row of a query result set.
    ///
    /// Returns:
    ///       Optional[Tuple[Any, ...]]: A single sequence representing the next row of the result set, or `None` if no more data is available.
    ///
    /// Examples:
    ///     ```python
    ///     cursor.execute("select * from example where id = 1")
    ///     row = cursor.fetchone()
    ///     connection.commit()
    ///     ```
    pub fn fetchone<'py>(&mut self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyTuple>>> {
        const FUNCTION_NAME: &str = "Cursor.fetchone()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start");

        let qr = if let Some(qr) = &mut self.query_result {
            qr
        } else {
            trace!("{FUNCTION_NAME} error: No query result available");
            return Err(ProgrammingError::new_err(
                "No query result available for fetchone",
            ));
        };

        let connection = &self.connection;
        let runtime = connection.runtime();
        let result = runtime.block_on(next_row1(py, qr, &self.query_types, &mut self.row_number));

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result
    }

    /// Fetch the next row of a query result set.
    ///
    /// Returns:
    ///      Tuple[Any, ...]: A single sequence representing the next row of the result set.
    ///
    /// Raises:
    ///     StopIteration: When no more data is available.
    pub fn next<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        const FUNCTION_NAME: &str = "Cursor.next()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start");

        let qr = if let Some(qr) = &mut self.query_result {
            qr
        } else {
            trace!("{FUNCTION_NAME} error: No query result available");
            return Err(ProgrammingError::new_err(
                "No query result available for next",
            ));
        };

        let connection = &self.connection;
        let runtime = connection.runtime();
        let result = runtime.block_on(next_row1(py, qr, &self.query_types, &mut self.row_number));

        match result {
            Ok(Some(row)) => {
                trace!("{FUNCTION_NAME} end");
                Ok(row)
            }
            Ok(None) => {
                trace!("{FUNCTION_NAME} end. Stop iteration");
                return Err(PyStopIteration::new_err(()));
            }
            Err(e) => {
                debug!("{FUNCTION_NAME} error: {:?}", e);
                Err(e)
            }
        }
    }

    /// Number of rows to fetch at a time with `Cursor.fetchmany()`.
    #[setter]
    pub fn set_arraysize(&mut self, size: isize) {
        const FUNCTION_NAME: &str = "Cursor.set_arraysize()";
        trace!("{FUNCTION_NAME} start. size={}", size);

        self.arraysize = size.max(1) as usize;

        trace!("{FUNCTION_NAME} end");
    }

    /// Fetch the next set of rows of a query result set.
    ///
    /// Args:
    ///     size (int, optional) - Number of rows to fetch. If not specified, use the cursor's `arraysize` attribute.
    ///
    /// Returns:
    ///      List[Tuple[Any, ...]]: A list of sequences, each representing a row of the result set.
    ///
    /// Examples:
    ///     ```python
    ///     cursor.execute("select * from example")
    ///     rows = cursor.fetchmany(10)
    ///     connection.commit()
    ///     ```
    ///
    /// Note:
    ///     See also `Cursor.arraysize` for setting the default number of rows to fetch with `fetchmany()`.
    #[pyo3(signature = (size=None))]
    fn fetchmany<'py>(
        &mut self,
        py: Python<'py>,
        size: Option<usize>,
    ) -> PyResult<Vec<Bound<'py, PyTuple>>> {
        const FUNCTION_NAME: &str = "Cursor.fetchmany()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start. size={:?}", size);

        let qr = if let Some(qr) = &mut self.query_result {
            qr
        } else {
            trace!("{FUNCTION_NAME} error: No query result available");
            return Err(ProgrammingError::new_err(
                "No query result available for fetchmany",
            ));
        };

        let size = size.unwrap_or(self.arraysize);

        let connection = &self.connection;
        let runtime = connection.runtime();
        let result = runtime.block_on(Self::next_rows(
            py,
            qr,
            &self.query_types,
            &mut self.row_number,
            size,
        ));

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result
    }

    /// Fetch all (remaining) rows of a query result set.
    ///
    /// Returns:
    ///      List[Tuple[Any, ...]]: A list of sequences, each representing a row of the result set.
    ///
    /// Examples:
    ///     ```python
    ///     cursor.execute("select * from example")
    ///     rows = cursor.fetchall()
    ///     connection.commit()
    ///     ```
    pub fn fetchall<'py>(&mut self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyTuple>>> {
        const FUNCTION_NAME: &str = "Cursor.fetchall()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start");

        let qr = if let Some(qr) = &mut self.query_result {
            qr
        } else {
            trace!("{FUNCTION_NAME} error: No query result available");
            return Err(ProgrammingError::new_err(
                "No query result available for fetchall",
            ));
        };

        let connection = &self.connection;
        let runtime = connection.runtime();
        let result = runtime.block_on(Self::all_rows(
            py,
            qr,
            &self.query_types,
            &mut self.row_number,
        ));

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result
    }

    /// Current row number (0-based).
    #[getter]
    pub fn rownumber(&self) -> Option<usize> {
        self.row_number.as_ref().map(RowNumber::get)
    }

    /// Not supported in this implementation.
    #[pyo3(signature = (_procname, _parameters=None))]
    pub fn callproc(&self, _procname: String, _parameters: Option<Bound<PyAny>>) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Cursor.callproc()";
        trace!("{FUNCTION_NAME} not supported");
        Err(NotSupportedError::new_err("callproc() is not supported"))
    }

    /// Not supported in this implementation.
    pub fn nextset(&self) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Cursor.nextset()";
        trace!("{FUNCTION_NAME} not supported");
        Err(NotSupportedError::new_err("nextset() is not supported"))
    }

    /// This method is a no-op in this implementation.
    pub fn setinputsizes(&self, _sizes: Bound<PyAny>) {
        const FUNCTION_NAME: &str = "Cursor.setinputsizes()";
        trace!("{FUNCTION_NAME} ignored");
    }

    /// This method is a no-op in this implementation.
    #[pyo3(signature = (_size, _column=None))]
    pub fn setoutputsize(&self, _size: isize, _column: Option<isize>) {
        const FUNCTION_NAME: &str = "Cursor.setoutputsize()";
        trace!("{FUNCTION_NAME} ignored");
    }

    // for
    /// Iterator protocol: return the iterator object itself.
    pub fn __iter__(slf: Bound<Self>) -> Bound<Self> {
        slf
    }

    /// Iterator protocol: return the next row in the result set.
    pub fn __next__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        const FUNCTION_NAME: &str = "Cursor.__next__()";
        self.check_closed(FUNCTION_NAME)?;
        trace!("{FUNCTION_NAME} start");

        let qr = if let Some(qr) = &mut self.query_result {
            qr
        } else {
            trace!("{FUNCTION_NAME} error: No query result available");
            return Err(ProgrammingError::new_err(
                "No query result available for iteration",
            ));
        };

        let connection = &self.connection;
        let runtime = connection.runtime();
        match runtime.block_on(next_row1(py, qr, &self.query_types, &mut self.row_number)) {
            Ok(Some(row)) => {
                trace!("{FUNCTION_NAME} end");
                Ok(row)
            }
            Ok(None) => {
                trace!("{FUNCTION_NAME} end. Stop iteration");
                Err(PyStopIteration::new_err(()))
            }
            Err(e) => {
                debug!("{FUNCTION_NAME} error: {:?}", e);
                Err(e)
            }
        }
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
        exc_value: Option<Bound<PyAny>>,
        _traceback: Option<Bound<PyAny>>,
    ) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Cursor.__exit__()";
        trace!("{FUNCTION_NAME} start");

        let result = self.close_internal();

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

    /// Closes the current result set and clears cached prepared statements.
    pub fn clear(&mut self) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Cursor.clear()";
        trace!("{FUNCTION_NAME} start");

        let result = self.clear_internal();

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result
    }

    /// Close the cursor.
    pub fn close(&mut self) -> PyResult<()> {
        const FUNCTION_NAME: &str = "Cursor.close()";
        trace!("{FUNCTION_NAME} start");

        let result = self.close_internal();

        match &result {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
        result
    }
}

impl Cursor {
    async fn next_rows<'py>(
        py: Python<'py>,
        qr: &mut SqlQueryResult,
        types: &Vec<AtomType>,
        row_number: &mut Option<RowNumber>,
        size: usize,
    ) -> PyResult<Vec<Bound<'py, PyTuple>>> {
        let mut rows = Vec::with_capacity(size);
        for _ in 0..size {
            if let Some(row) = next_row1(py, qr, types, row_number).await? {
                rows.push(row);
            } else {
                break;
            }
        }
        Ok(rows)
    }

    async fn all_rows<'py>(
        py: Python<'py>,
        qr: &mut SqlQueryResult,
        types: &Vec<AtomType>,
        row_number: &mut Option<RowNumber>,
    ) -> PyResult<Vec<Bound<'py, PyTuple>>> {
        let mut rows = Vec::new();
        loop {
            if let Some(row) = next_row1(py, qr, types, row_number).await? {
                rows.push(row);
            } else {
                break;
            }
        }
        Ok(rows)
    }

    fn clear_internal(&mut self) -> PyResult<()> {
        let err = if !self.ps_map.is_empty() || self.query_result.is_some() {
            let connection = &self.connection;
            let runtime = connection.runtime();
            runtime.block_on(async {
                let mut err = None;

                if let Some(qr) = self.query_result.as_mut() {
                    if let Err(e) = qr.close().await {
                        debug!("Cursor query_result close error: {:?}", e);
                        if connection.has_transaction() {
                            err = Some(e);
                        }
                    }
                }

                for (ps, _) in self.ps_map.values_mut() {
                    if let Err(e) = ps.close().await {
                        debug!("Cursor prepared_statement close error: {:?}", e);
                        if err.is_none() {
                            err = Some(e);
                        }
                    }
                }
                err
            })
        } else {
            None
        };

        self.ps_map.clear();
        self.query_result = None;
        self.query_types.clear();
        self.row_number = None;
        self.rowcount = -1;

        if let Some(e) = err {
            return Err(to_pyerr(e));
        }
        Ok(())
    }

    fn close_internal(&mut self) -> PyResult<()> {
        self.closed = true;
        self.clear_internal()
    }

    fn check_closed(&self, function_name: &str) -> PyResult<()> {
        if self.closed {
            trace!("{}: Cursor is already closed", function_name);
            return Err(ProgrammingError::new_err("Cursor is already closed"));
        }
        Ok(())
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        const FUNCTION_NAME: &str = "Cursor.drop()";
        trace!("{FUNCTION_NAME} start. closed={}", self.closed);

        match self.close_internal() {
            Ok(_) => trace!("{FUNCTION_NAME} end"),
            Err(e) => debug!("{FUNCTION_NAME} error: {:?}", e),
        }
    }
}
