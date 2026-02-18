use std::time::Duration;

use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::ShutdownType as CoreShutdownType;

/// Shutdown type for connection.
///
/// Attributes:
///     NOTHING: Do nothing special during shutdown.
///     GRACEFUL: Waits for the ongoing requests and safely shutdown the session.
///     FORCEFUL: Cancelling the ongoing requests and safely shutdown the session.
#[gen_stub_pyclass_enum]
#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum ShutdownType {
    /// Do nothing special during shutdown.
    NOTHING = 0,
    /// Waits for the ongoing requests and safely shutdown the session.
    GRACEFUL = 1,
    /// Cancelling the ongoing requests and safely shutdown the session.
    FORCEFUL = 2,
}

#[pymethods]
impl ShutdownType {
    pub fn __eq__(&self, other: &ShutdownType) -> bool {
        self == other
    }

    pub fn __hash__(&self) -> isize {
        *self as isize
    }
}

impl ShutdownType {
    pub(crate) fn to_core_shutdown_type(&self) -> CoreShutdownType {
        match self {
            ShutdownType::NOTHING => CoreShutdownType::NotSet,
            ShutdownType::GRACEFUL => CoreShutdownType::Graceful,
            ShutdownType::FORCEFUL => CoreShutdownType::Forceful,
        }
    }
}

/// Shutdown option for connection.
///
/// Attributes:
///     shutdown_type (ShutdownType): Shutdown type. Default is `ShutdownType.GRACEFUL`.
///     shutdown_timeout (int): Shutdown timeout in seconds.
///
/// Examples:
///     ```python
///     import tsubakuro_rust_python as tsurugi
///
///     shutdown_option = tsurugi.ShutdownOption(tsurugi.ShutdownType.GRACEFUL, 30)
///     ```
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug, Clone)]
pub struct ShutdownOption {
    /// Shutdown type.
    #[pyo3(get, set)]
    shutdown_type: ShutdownType,
    /// Shutdown timeout in seconds.
    #[pyo3(get, set)]
    shutdown_timeout: Option<u64>,
}

impl Default for ShutdownOption {
    fn default() -> Self {
        Self::new(ShutdownType::GRACEFUL, None)
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl ShutdownOption {
    /// Create a new `ShutdownOption`.
    #[new]
    #[pyo3(signature = (shutdown_type=ShutdownType::GRACEFUL, timeout=None))]
    pub fn new(shutdown_type: ShutdownType, timeout: Option<u64>) -> Self {
        Self {
            shutdown_type,
            shutdown_timeout: timeout,
        }
    }
}

impl ShutdownOption {
    pub(crate) fn core_shutdown_type(&self) -> CoreShutdownType {
        self.shutdown_type.to_core_shutdown_type()
    }

    pub(crate) fn shutdown_timeout(&self) -> Option<Duration> {
        self.shutdown_timeout.map(Duration::from_secs)
    }
}
