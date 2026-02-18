use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// DOUBLE type.
///
/// Attributes:
///     value (Optional[float]): float value. (read only)
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Float64 {
    /// Value.
    #[pyo3(get)]
    value: Option<f64>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Float64 {
    /// Create a new `Float64`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<f64>) -> PyResult<Self> {
        Ok(Float64 { value })
    }

    pub fn __float__(&self) -> Option<f64> {
        self.value
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Float64({})", v)
        } else {
            "Float64(None)".to_string()
        }
    }
}

impl Float64 {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Float64>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<f64> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
