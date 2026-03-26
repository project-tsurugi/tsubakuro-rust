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
    CommitOption as CoreCommitOption, CommitType as CoreCommitType,
};

/// Commit type for transaction.
///
/// Attributes:
///     DEFAULT: the default commit type (rely on the database settings).
///     ACCEPTED: commit operation has accepted, and the transaction will never abort except system errors.
///     AVAILABLE: commit data has been visible for others.
///     STORED: commit data has been saved on the local disk.
///     PROPAGATED: commit data has been propagated to the all suitable nodes.
#[gen_stub_pyclass_enum]
#[pyclass(module = "tsurugi_dbapi")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum CommitType {
    /// the default commit type (rely on the database settings).
    DEFAULT = 0,
    /// commit operation has accepted, and the transaction will never abort except system errors.
    ACCEPTED = 10,
    /// commit data has been visible for others.
    AVAILABLE = 20,
    /// commit data has been saved on the local disk.
    STORED = 30,
    /// commit data has been propagated to the all suitable nodes.
    PROPAGATED = 40,
}

#[pymethods]
impl CommitType {
    pub fn __eq__(&self, other: &CommitType) -> bool {
        self == other
    }

    pub fn __hash__(&self) -> isize {
        *self as isize
    }

    pub fn __reduce__<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyAny>, (i32,))> {
        let callable = py.get_type::<CommitType>().getattr("_from_value")?;
        let args = (*self as i32,);
        Ok((callable, args))
    }

    #[staticmethod]
    pub fn _from_value(value: i32) -> Self {
        match value {
            0 => CommitType::DEFAULT,
            10 => CommitType::ACCEPTED,
            20 => CommitType::AVAILABLE,
            30 => CommitType::STORED,
            40 => CommitType::PROPAGATED,
            _ => {
                warn!("CommitType._from_value(): unknown value {}", value);
                CommitType::DEFAULT
            }
        }
    }
}

impl CommitType {
    pub(crate) fn to_core_commit_type(&self) -> CoreCommitType {
        match self {
            CommitType::DEFAULT => CoreCommitType::Unspecified,
            CommitType::ACCEPTED => CoreCommitType::Accepted,
            CommitType::AVAILABLE => CoreCommitType::Available,
            CommitType::STORED => CoreCommitType::Stored,
            CommitType::PROPAGATED => CoreCommitType::Propagated,
        }
    }
}

/// Commit option for transaction.
///
/// Attributes:
///     commit_type (CommitType): Commit type. Default is `CommitType.DEFAULT`.
///     auto_dispose (bool): Auto dispose flag. Default is `False`.
///     commit_timeout (int): Commit timeout in seconds.
///
/// Examples:
///     ```python
///     import tsurugi_dbapi as tsurugi
///
///     commit_option = tsurugi.CommitOption(tsurugi.CommitType.DEFAULT, False, 60)
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommitOption {
    /// Commit type.
    #[pyo3(get, set)]
    commit_type: CommitType,
    /// Auto dispose flag.
    #[pyo3(get, set)]
    auto_dispose: bool,
    /// Commit timeout in seconds.
    #[pyo3(get, set)]
    commit_timeout: Option<u64>,
}

impl Default for CommitOption {
    fn default() -> Self {
        Self::new(CommitType::DEFAULT, false, None)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl CommitOption {
    /// Create a new `CommitOption`.
    ///
    /// Args:
    ///     commit_type (CommitType): Commit type. Default is `CommitType.DEFAULT`.
    ///     auto_dispose (bool, optional): Auto dispose flag. Default is `False`.
    ///     timeout (int, optional): Commit timeout in seconds.
    ///
    /// Returns:
    ///     CommitOption: A new `CommitOption` instance.
    #[new]
    #[pyo3(signature = (commit_type=CommitType::DEFAULT, auto_dispose=false, timeout=None))]
    pub fn new(commit_type: CommitType, auto_dispose: bool, timeout: Option<u64>) -> Self {
        Self {
            commit_type,
            auto_dispose,
            commit_timeout: timeout,
        }
    }

    pub fn __eq__(&self, other: &CommitOption) -> bool {
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
        let cls = py.get_type::<CommitOption>();
        let args = PyTuple::empty(py);
        let state = self.__getstate__()?;
        Ok((cls, args, state))
    }

    pub fn __getstate__(&self) -> PyResult<Vec<u8>> {
        serde_pickle::to_vec(self, Default::default())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    pub fn __setstate__(&mut self, state: Vec<u8>) -> PyResult<()> {
        let state: CommitOption = serde_pickle::from_slice(&state, Default::default())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        *self = state;
        Ok(())
    }
}

impl CommitOption {
    pub(crate) fn to_core_commit_option(&self) -> CoreCommitOption {
        let mut option = CoreCommitOption::new();
        option.set_commit_type(self.commit_type.to_core_commit_type());
        option.set_auto_dispose(self.auto_dispose);
        option
    }

    pub(crate) fn commit_timeout(&self) -> Option<Duration> {
        self.commit_timeout.map(Duration::from_secs)
    }
}
