use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// DATE type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Date {
    /// Value.
    #[pyo3(get)]
    value: Option<chrono::NaiveDate>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Date {
    /// Create a new `Date`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<chrono::NaiveDate>) -> PyResult<Self> {
        Ok(Date { value })
    }

    /// Create a `Date` from year, month, and day.
    #[classmethod]
    pub fn of(_cls: Bound<PyType>, year: i32, month: u32, day: u32) -> PyResult<Self> {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| PyValueError::new_err("invalid date value"))?;
        Ok(Date { value: Some(date) })
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Date({})", v)
        } else {
            "Date(None)".to_string()
        }
    }
}

impl Date {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Date>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<chrono::NaiveDate> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
