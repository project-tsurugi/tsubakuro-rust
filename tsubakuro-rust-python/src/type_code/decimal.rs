use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// DECIMAL type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Decimal {
    /// Value.
    #[pyo3(get)]
    value: Option<rust_decimal::Decimal>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Decimal {
    /// Create a new `Decimal`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<rust_decimal::Decimal>) -> Self {
        Decimal { value }
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Decimal({})", v)
        } else {
            "Decimal(None)".to_string()
        }
    }
}

impl Decimal {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Decimal>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<rust_decimal::Decimal> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
