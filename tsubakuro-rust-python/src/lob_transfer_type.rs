use std::hash::Hash;

use log::warn;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use serde::{Deserialize, Serialize};
use tsubakuro_rust_core::prelude::LobTransferType as CoreLobTransferType;

/// Large object transfer type.
///
/// Attributes:
///     NOT_USE: does not use transfer type.
///     RELAY: Blob Relay transfer type.
///
/// since 0.10.0
#[gen_stub_pyclass_enum]
#[pyclass(module = "tsurugi_dbapi")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum LobTransferType {
    /// Does not use transfer type.
    NOT_USE = 1,
    /// Blob Relay transfer type.
    RELAY = 2,
}

#[pymethods]
impl LobTransferType {
    pub fn __eq__(&self, other: &LobTransferType) -> bool {
        self == other
    }

    pub fn __hash__(&self) -> isize {
        *self as isize
    }

    pub fn __reduce__<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyAny>, (i32,))> {
        let callable = py.get_type::<LobTransferType>().getattr("_from_value")?;
        let args = (*self as i32,);
        Ok((callable, args))
    }

    #[staticmethod]
    pub fn _from_value(value: i32) -> Self {
        match value {
            1 => LobTransferType::NOT_USE,
            2 => LobTransferType::RELAY,
            _ => {
                warn!("LobTransferType._from_value(): unknown value {}", value);
                LobTransferType::NOT_USE
            }
        }
    }
}

impl LobTransferType {
    pub(crate) fn from_core_lob_transfer_type(
        value: CoreLobTransferType,
    ) -> Option<LobTransferType> {
        match value {
            CoreLobTransferType::NotUse => Some(LobTransferType::NOT_USE),
            CoreLobTransferType::Relay => Some(LobTransferType::RELAY),
            _ => None,
        }
    }
}
