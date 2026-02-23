use std::time::Duration;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
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
#[pyclass(module = "tsubakuro_rust_python")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
///     import tsubakuro_rust_python as tsurugi
///
///     commit_option = tsurugi.CommitOption(tsurugi.CommitType.DEFAULT, False, 60)
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python")]
#[derive(Debug, Clone)]
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
