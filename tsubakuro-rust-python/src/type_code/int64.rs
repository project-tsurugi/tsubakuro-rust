use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// BIGINT type.
///
/// Attributes:
///     value (Optional[int]): integer value. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python.type_code")]
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
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Int64>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<i64> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
