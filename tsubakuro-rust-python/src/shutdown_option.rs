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
use tsubakuro_rust_core::prelude::ShutdownType as CoreShutdownType;

/// Shutdown type for connection.
///
/// Attributes:
///     NOTHING: Do nothing special during shutdown.
///     GRACEFUL: Waits for the ongoing requests and safely shutdown the session.
///     FORCEFUL: Cancelling the ongoing requests and safely shutdown the session.
#[gen_stub_pyclass_enum]
#[pyclass(module = "tsurugi_dbapi")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    pub fn __reduce__<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyAny>, (i32,))> {
        let callable = py.get_type::<ShutdownType>().getattr("_from_value")?;
        let args = (*self as i32,);
        Ok((callable, args))
    }

    #[staticmethod]
    pub fn _from_value(value: i32) -> Self {
        match value {
            0 => ShutdownType::NOTHING,
            1 => ShutdownType::GRACEFUL,
            2 => ShutdownType::FORCEFUL,
            _ => {
                warn!("ShutdownType._from_value(): unknown value {}", value);
                ShutdownType::GRACEFUL
            }
        }
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
///     import tsurugi_dbapi as tsurugi
///
///     shutdown_option = tsurugi.ShutdownOption(tsurugi.ShutdownType.GRACEFUL, 30)
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsurugi_dbapi")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    ///
    /// Args:
    ///     shutdown_type (ShutdownType): Shutdown type. Default is `ShutdownType.GRACEFUL`.
    ///     timeout (int, optional): Shutdown timeout in seconds.
    ///
    /// Returns:
    ///     ShutdownOption: A new `ShutdownOption` instance.
    #[new]
    #[pyo3(signature = (shutdown_type=ShutdownType::GRACEFUL, timeout=None))]
    pub fn new(shutdown_type: ShutdownType, timeout: Option<u64>) -> Self {
        Self {
            shutdown_type,
            shutdown_timeout: timeout,
        }
    }

    pub fn __eq__(&self, other: &ShutdownOption) -> bool {
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
        let cls = py.get_type::<ShutdownOption>();
        let args = PyTuple::empty(py);
        let state = self.__getstate__()?;
        Ok((cls, args, state))
    }

    pub fn __getstate__(&self) -> PyResult<Vec<u8>> {
        serde_pickle::to_vec(self, Default::default())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    pub fn __setstate__(&mut self, state: Vec<u8>) -> PyResult<()> {
        let state: ShutdownOption = serde_pickle::from_slice(&state, Default::default())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        *self = state;
        Ok(())
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
