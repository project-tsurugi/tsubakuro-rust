use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// BOOLEAN type.
///
/// Attributes:
///     value (Optional[bool]): boolean value. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python")]
#[derive(Debug)]
pub struct Bool {
    /// Value.
    #[pyo3(get)]
    value: Option<bool>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Bool {
    /// Create a new `Bool`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<bool>) -> Self {
        Bool { value }
    }

    pub fn __bool__(&self) -> bool {
        self.value.unwrap_or(false)
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Bool({})", v)
        } else {
            "Bool(None)".to_string()
        }
    }
}

impl Bool {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Bool>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<bool> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
