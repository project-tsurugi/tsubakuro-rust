use std::time::Duration;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{
    TransactionOption as CoreTransactionOption, TransactionOptionSetter,
    TransactionType as CoreTransactionType,
};

/// Transaction type.
#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum TransactionType {
    OCC = 1,
    LTX = 2,
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
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
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
