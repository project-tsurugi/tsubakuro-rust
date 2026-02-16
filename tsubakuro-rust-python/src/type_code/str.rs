use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

/// CHAR, VARCHAR type.
#[gen_stub_pyclass]
#[pyclass]
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
    pub const fn value(&self) -> &Option<String> {
        &self.value
    }
}
