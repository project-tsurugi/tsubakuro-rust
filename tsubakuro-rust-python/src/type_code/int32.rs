use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// INT type.
///
/// Attributes:
///     value (Optional[int]): integer value. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python.type_code")]
#[derive(Debug)]
pub struct Int32 {
    /// Value.
    #[pyo3(get)]
    value: Option<i32>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Int32 {
    /// Create a new `Int32`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<i32>) -> PyResult<Self> {
        Ok(Int32 { value })
    }

    pub fn __int__(&self) -> Option<i32> {
        self.value
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value {
            format!("Int32({})", v)
        } else {
            "Int32(None)".to_string()
        }
    }
}

impl Int32 {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Int32>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<i32> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
