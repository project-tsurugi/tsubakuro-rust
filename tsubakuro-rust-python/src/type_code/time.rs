use chrono::Timelike;
use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// TIME type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct Time {
    /// Value.
    #[pyo3(get)]
    value: Option<chrono::NaiveTime>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Time {
    /// Create a new `Time`.
    #[new]
    #[pyo3(signature = (value=None, nanosecond=None))]
    pub fn new(value: Option<chrono::NaiveTime>, nanosecond: Option<u32>) -> PyResult<Self> {
        if let Some(v) = value {
            let v = if let Some(ns) = nanosecond {
                v.with_nanosecond(ns)
                    .ok_or_else(|| PyValueError::new_err("invalid nanosecond value"))?
            } else {
                v
            };
            Ok(Time { value: Some(v) })
        } else {
            Ok(Time { value: None })
        }
    }

    /// Create a `Time` from hour, minute, second, and nanosecond.
    #[classmethod]
    #[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0))]
    pub fn of(
        _cls: Bound<PyType>,
        hour: u32,
        minute: u32,
        second: u32,
        nanosecond: u32,
    ) -> PyResult<Self> {
        let time = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)
            .ok_or_else(|| PyValueError::new_err("invalid time value"))?;
        Ok(Time { value: Some(time) })
    }

    /// Nnanosecond.
    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value.map(|v| v.nanosecond())
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("Time({})", v)
        } else {
            "Time(None)".to_string()
        }
    }
}

impl Time {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Time>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<chrono::NaiveTime> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
