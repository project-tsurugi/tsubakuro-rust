use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::Duration,
};

use log::warn;
use pyo3::{
    exceptions::PyRuntimeError,
    prelude::*,
    types::{PyTuple, PyType},
};
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};
use tsubakuro_rust_core::prelude::{
    TransactionOption as CoreTransactionOption, TransactionOptionSetter,
    TransactionType as CoreTransactionType,
};

/// Transaction type.
///
/// Attributes:
///     OCC: Optimistic concurrency control (OCC) transaction.
///     LTX: Long transaction (LTX).
///     RTX: Read-only transaction (RTX).
#[gen_stub_pyclass_enum]
#[pyclass(module = "tsurugi_dbapi")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum TransactionType {
    /// Optimistic concurrency control (OCC) transaction.
    OCC = 1,
    /// Long transaction (LTX).
    LTX = 2,
    /// Read-only transaction (RTX).
    RTX = 3,
}

#[pymethods]
impl TransactionType {
    pub fn __eq__(&self, other: &TransactionType) -> bool {
        self == other
    }

    pub fn __hash__(&self) -> isize {
        *self as isize
    }

    pub fn __reduce__<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyAny>, (i32,))> {
        let callable = py.get_type::<TransactionType>().getattr("_from_value")?;
        let args = (*self as i32,);
        Ok((callable, args))
    }

    #[staticmethod]
    pub fn _from_value(value: i32) -> Self {
        match value {
            1 => TransactionType::OCC,
            2 => TransactionType::LTX,
            3 => TransactionType::RTX,
            _ => {
                warn!("TransactionType._from_value(): unknown value {}", value);
                TransactionType::OCC
            }
        }
    }
}

impl TransactionType {
    pub(crate) fn to_core_transaction_type(&self) -> CoreTransactionType {
        match self {
            TransactionType::OCC => CoreTransactionType::Short,
            TransactionType::LTX => CoreTransactionType::Long,
            TransactionType::RTX => CoreTransactionType::ReadOnly,
        }
    }
}

/// Transaction option.
///
/// Attributes:
///     transaction_type (TransactionType): Transaction type. Default is `TransactionType.OCC`.
///     label (str): Transaction label.
///     include_ddl (bool): Whether the transaction modifies definitions (DDL). Default is `False`. Only applicable for `TransactionType.LTX`.
///     write_preserve (List[str]): List of table names to preserve for write operations. Only applicable for `TransactionType.LTX`.
///     inclusive_read_area (List[str]): List of table names to include in the read area. Only applicable for `TransactionType.LTX`.
///     exclusive_read_area (List[str]): List of table names to exclude from the read area. Only applicable for `TransactionType.LTX`.
///     scan_parallel (int): Degree of parallelism for scanning. Only applicable for `TransactionType.RTX`.
///     begin_timeout (int): Begin transaction timeout in seconds
///
/// Examples:
///     ```python
///     import tsurugi_dbapi as tsurugi
///
///     tx_option = tsurugi.TransactionOption(tsurugi.TransactionType.OCC)
///     tx_option.label = "tsurugi-dbapi OCC example"
///     ```
///
///     ```python
///     import tsurugi_dbapi as tsurugi
///
///     tx_option = tsurugi.TransactionOption(tsurugi.TransactionType.LTX)
///     tx_option.label = "tsurugi-dbapi LTX example"
///     tx_option.write_preserve = ["table1", "table2"]
///     ```
///
///     ```python
///     import tsurugi_dbapi as tsurugi
///
///     tx_option = tsurugi.TransactionOption(tsurugi.TransactionType.RTX)
///     tx_option.label = "tsurugi-dbapi RTX example"
///     tx_option.scan_parallel = 4
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionOption {
    /// Transaction type.
    #[pyo3(get, set)]
    transaction_type: TransactionType,
    /// Transaction label.
    #[pyo3(get, set)]
    label: Option<String>,
    /// Include DDL flag.
    #[pyo3(get, set)]
    include_ddl: bool,
    /// Write preserve.
    #[pyo3(get, set)]
    write_preserve: Option<Vec<String>>,
    /// Inclusive read area.
    #[pyo3(get, set)]
    inclusive_read_area: Option<Vec<String>>,
    /// Exclusive read area.
    #[pyo3(get, set)]
    exclusive_read_area: Option<Vec<String>>,
    /// Scan parallel.
    #[pyo3(get, set)]
    scan_parallel: Option<i32>,
    /// Begin transaction timeout in seconds.
    #[pyo3(get, set)]
    begin_timeout: Option<u64>,
}

impl Default for TransactionOption {
    fn default() -> Self {
        Self::new(TransactionType::OCC)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl TransactionOption {
    /// Create a new `TransactionOption`.
    ///
    /// Args:
    ///     type (TransactionType): Transaction type. Default is `TransactionType.OCC`.
    ///
    /// Returns:
    ///     TransactionOption: A new `TransactionOption` instance.
    #[new]
    #[pyo3(signature = (r#type=TransactionType::OCC))]
    pub fn new(r#type: TransactionType) -> Self {
        Self {
            transaction_type: r#type,
            label: None,
            include_ddl: false,
            write_preserve: None,
            inclusive_read_area: None,
            exclusive_read_area: None,
            scan_parallel: None,
            begin_timeout: None,
        }
    }

    /// Create a new `TransactionOption` for OCC transaction.
    ///
    /// Args:
    ///     label (str, optional): Transaction label.
    ///
    /// Returns:
    ///     TransactionOption: A new `TransactionOption` instance for OCC transaction.
    ///
    /// Examples:
    ///     ```python
    ///     import tsurugi_dbapi as tsurugi
    ///
    ///     tx_option = tsurugi.TransactionOption.occ(label="OCC transaction")
    ///     ```
    #[classmethod]
    #[pyo3(signature = (label=None))]
    pub fn occ(_cls: &Bound<PyType>, label: Option<String>) -> Self {
        let mut option = Self::new(TransactionType::OCC);
        option.label = label;
        option
    }

    /// Create a new `TransactionOption` for LTX transaction.
    ///
    /// Args:
    ///     label (str, optional): Transaction label.
    ///     write_preserve (List[str], optional): List of table names to preserve for write operations.
    ///     inclusive_read_area (List[str], optional): List of table names to include in the read area.
    ///     exclusive_read_area (List[str], optional): List of table names to exclude from the read area.
    ///
    /// Returns:
    ///     TransactionOption: A new `TransactionOption` instance for LTX transaction.
    ///
    /// Examples:
    ///     ```python
    ///     import tsurugi_dbapi as tsurugi
    ///
    ///     tx_option = tsurugi.TransactionOption.ltx(
    ///         label="LTX transaction",
    ///         write_preserve=["table1", "table2"],
    ///     )
    ///     ```
    #[classmethod]
    #[pyo3(signature = (label=None, write_preserve=None, inclusive_read_area=None, exclusive_read_area=None))]
    pub fn ltx(
        _cls: &Bound<PyType>,
        label: Option<String>,
        write_preserve: Option<Vec<String>>,
        inclusive_read_area: Option<Vec<String>>,
        exclusive_read_area: Option<Vec<String>>,
    ) -> Self {
        let mut option = Self::new(TransactionType::LTX);
        option.label = label;
        option.write_preserve = write_preserve;
        option.inclusive_read_area = inclusive_read_area;
        option.exclusive_read_area = exclusive_read_area;
        option.include_ddl = false;
        option
    }

    /// Create a new `TransactionOption` for LTX transaction for DDL.
    ///
    /// Args:
    ///     label (str, optional): Transaction label.
    ///
    /// Returns:
    ///     TransactionOption: A new `TransactionOption` instance for LTX transaction for DDL.
    ///
    /// Examples:
    ///     ```python
    ///     import tsurugi_dbapi as tsurugi
    ///
    ///     tx_option = tsurugi.TransactionOption.ddl(label="LTX transaction for DDL")
    ///     ```
    #[classmethod]
    #[pyo3(signature = (label=None))]
    pub fn ddl(_cls: &Bound<PyType>, label: Option<String>) -> Self {
        let mut option = Self::new(TransactionType::LTX);
        option.label = label;
        option.include_ddl = true;
        option
    }

    /// Create a new `TransactionOption` for RTX transaction.
    ///
    /// Args:
    ///     label (str, optional): Transaction label.
    ///     scan_parallel (int, optional): Degree of parallelism for scanning.
    ///
    /// Returns:
    ///     TransactionOption: A new `TransactionOption` instance for RTX transaction.
    ///
    /// Examples:
    ///     ```python
    ///     import tsurugi_dbapi as tsurugi
    ///
    ///     tx_option = tsurugi.TransactionOption.rtx(
    ///         label="RTX transaction",
    ///         scan_parallel=4,
    ///     )
    ///     ```
    #[classmethod]
    #[pyo3(signature = (label=None, scan_parallel=None))]
    pub fn rtx(_cls: &Bound<PyType>, label: Option<String>, scan_parallel: Option<i32>) -> Self {
        let mut option = Self::new(TransactionType::RTX);
        option.label = label;
        option.scan_parallel = scan_parallel;
        option
    }

    pub fn __eq__(&self, other: &TransactionOption) -> bool {
        self == other
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyType>, Bound<'py, PyTuple>, Vec<u8>)> {
        let cls = py.get_type::<TransactionOption>();
        let args = PyTuple::empty(py);
        let state = self.__getstate__()?;
        Ok((cls, args, state))
    }

    pub fn __getstate__(&self) -> PyResult<Vec<u8>> {
        serde_pickle::to_vec(self, Default::default())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    pub fn __setstate__(&mut self, state: Vec<u8>) -> PyResult<()> {
        let state: TransactionOption = serde_pickle::from_slice(&state, Default::default())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        *self = state;
        Ok(())
    }
}

impl TransactionOption {
    pub(crate) fn to_core_transaction_option(&self) -> CoreTransactionOption {
        let mut option = CoreTransactionOption::new();
        option.set_transaction_type(self.transaction_type.to_core_transaction_type());
        if let Some(label) = &self.label {
            option.set_transaction_label(label.clone());
        }

        match self.transaction_type {
            TransactionType::OCC => {}
            TransactionType::LTX => {
                option.set_modifies_definitions(self.include_ddl);
                if let Some(write_preserve) = &self.write_preserve {
                    option.set_write_preserve(write_preserve);
                }
                if let Some(read_area) = &self.inclusive_read_area {
                    option.set_inclusive_read_area(read_area);
                }
                if let Some(read_area) = &self.exclusive_read_area {
                    option.set_exclusive_read_area(read_area);
                }
            }
            TransactionType::RTX => {
                if let Some(scan_parallel) = self.scan_parallel {
                    option.set_scan_parallel(scan_parallel);
                }
            }
        }

        option
    }

    pub(crate) fn begin_timeout(&self) -> Option<Duration> {
        self.begin_timeout.map(Duration::from_secs)
    }
}
