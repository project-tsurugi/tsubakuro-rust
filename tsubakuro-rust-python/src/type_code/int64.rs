use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// BIGINT type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Int64 {
    /// Value.
    #[pyo3(get)]
    value: Option<i64>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Int64 {
    /// Create a new `Int64`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<i64>) -> PyResult<Self> {
        Ok(Int64 { value })
    }

    pub fn __int__(&self) -> Option<i64> {
        self.value
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Int64({})", v)
        } else {
            "Int64(None)".to_string()
        }
    }
}

impl Int64 {
    pub const fn value(&self) -> Option<i64> {
        self.value
    }
}
