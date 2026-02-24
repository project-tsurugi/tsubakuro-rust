use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// REAL type.
///
/// Attributes:
///     value (Optional[float]): float value. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python.type_code")]
#[derive(Debug)]
pub struct Float32 {
    /// Value.
    #[pyo3(get)]
    value: Option<f32>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Float32 {
    /// Create a new `Float32`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<f32>) -> PyResult<Self> {
        Ok(Float32 { value })
    }

    pub fn __float__(&self) -> Option<f32> {
        self.value
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Float32({})", v)
        } else {
            "Float32(None)".to_string()
        }
    }
}

impl Float32 {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Float32>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<f32> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
