use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// BINARY, VARBINARY type.
///
/// Attributes:
///     value (Optional[bytes]): binary data. (read only)
///
/// Examples:
///     ```python
///     import tsubakuro_rust_python as tsurugi
///
///     value = tsurugi.type_code.Bytes(b"0x01\x02\x03")
///     value = tsurugi.type_code.Bytes(None)
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python.type_code")]
#[derive(Debug)]
pub struct Bytes {
    /// Value.
    #[pyo3(get)]
    value: Option<Vec<u8>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Bytes {
    /// Create a new `Bytes`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<Vec<u8>>) -> Self {
        Bytes { value }
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Bytes({:?})", v)
        } else {
            "Bytes(None)".to_string()
        }
    }
}

impl Bytes {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Bytes>>() {
            Ok(SqlParameter::of(name, v.value.clone()))
        } else {
            let v: Option<Vec<u8>> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
