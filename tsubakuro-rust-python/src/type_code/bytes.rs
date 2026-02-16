use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// BINARY, VARBINARY type.
#[gen_stub_pyclass]
#[pyclass]
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
    pub const fn value(&self) -> &Option<Vec<u8>> {
        &self.value
    }
}
