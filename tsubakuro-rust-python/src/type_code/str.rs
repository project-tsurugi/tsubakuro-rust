use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// CHAR, VARCHAR type.
///
/// Attributes:
///     value (Optional[str]): string value. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python")]
#[derive(Debug)]
pub struct Str {
    /// Value.
    #[pyo3(get)]
    value: Option<String>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Str {
    /// Create a new `Str`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<String>) -> Self {
        Str { value }
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Str({})", v)
        } else {
            "Str(None)".to_string()
        }
    }
}

impl Str {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Str>>() {
            Ok(SqlParameter::of(name, v.value.clone()))
        } else {
            let v: Option<String> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
