use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// DOUBLE type.
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
    pub const fn value(&self) -> Option<f64> {
        self.value
    }
}
