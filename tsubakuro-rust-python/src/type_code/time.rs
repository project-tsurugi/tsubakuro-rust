use chrono::Timelike;
use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgTimeOfDay};

/// TIME type.
///
/// Attributes:
///     value (Optional[datetime.time]): time value. (read only)
///     nanosecond (Optional[int]): nanosecond part of the time. (read only)
///
/// Examples:
///     ```python
///     import tsubakuro_rust_python as tsurugi
///     import datetime
///
///     value = tsurugi.type_code.Time(datetime.time(12, 34, 56, 123456)) # microsecond precision
///     value = tsurugi.type_code.Time(datetime.time(12, 34, 56), 123456879) # nanosecond precision
///     value = tsurugi.type_code.Time(None)
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python.type_code")]
#[derive(Debug)]
pub struct Time {
    value: Option<TgTimeOfDay>,
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
            let value = TgTimeOfDay::from(v);
            Ok(Time { value: Some(value) })
        } else {
            Ok(Time { value: None })
        }
    }

    /// Create a `Time` from hour, minute, second, and nanosecond.
    ///
    /// Args:
    ///     hour (int, optional): hour (0-23)
    ///     minute (int, optional): minute (0-59)
    ///     second (int, optional): second (0-59)
    ///     nanosecond (int, optional): nanosecond (0-999,999,999)
    ///
    /// Returns:
    ///     Time: created `Time` object
    #[classmethod]
    #[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0))]
    pub fn of(
        _cls: &Bound<PyType>,
        hour: u32,
        minute: u32,
        second: u32,
        nanosecond: u32,
    ) -> PyResult<Self> {
        let time = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)
            .ok_or_else(|| PyValueError::new_err("invalid time value"))?;
        let value = TgTimeOfDay::from(time);
        Ok(Time { value: Some(value) })
    }

    /// Create a `Time` from nanoseconds of day.
    ///
    /// Args:
    ///     nanoseconds_of_day (int): time of day (nanoseconds since 00:00:00)
    ///
    /// Returns:
    ///     Time: created `Time` object
    #[classmethod]
    pub fn raw(_cls: &Bound<PyType>, nanoseconds_of_day: u64) -> PyResult<Self> {
        let value = TgTimeOfDay::new(nanoseconds_of_day);
        Ok(Time { value: Some(value) })
    }

    /// Value.
    #[getter]
    pub fn value(&self) -> Option<chrono::NaiveTime> {
        self.value.as_ref().map(|v| v.into())
    }

    /// Nnanosecond.
    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        if let Some(v) = self.value {
            let nanos = v.offset_nanoseconds % 1_000_000_000;
            Some(nanos as u32)
        } else {
            None
        }
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
