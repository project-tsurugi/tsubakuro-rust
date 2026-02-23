use chrono::Timelike;
use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgTimePoint};

/// TIMESTAMP type.
///
/// Attributes:
///     value (Optional[datetime.datetime]): datetime value. (read only)
///     nanosecond (Optional[int]): nanosecond part of the time. (read only)
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python")]
#[derive(Debug)]
pub struct Datetime {
    value: Option<TgTimePoint>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Datetime {
    /// Create a new `Datetime`.
    #[new]
    #[pyo3(signature = (value=None, nanosecond=None))]
    pub fn new(value: Option<chrono::NaiveDateTime>, nanosecond: Option<u32>) -> PyResult<Self> {
        if let Some(v) = value {
            let v = if let Some(ns) = nanosecond {
                v.with_nanosecond(ns)
                    .ok_or_else(|| PyValueError::new_err("invalid nanosecond value"))?
            } else {
                v
            };
            let value = TgTimePoint::from(v);
            Ok(Datetime { value: Some(value) })
        } else {
            Ok(Datetime { value: None })
        }
    }

    /// Create a `Datetime` from year, month, day, hour, minute, second, and nanosecond.
    ///
    /// Args:
    ///     year (int): year
    ///     month (int): month (1-12)
    ///     day (int): day (1-31)
    ///     hour (int, optional): hour (0-23)
    ///     minute (int, optional): minute (0-59)
    ///     second (int, optional): second (0-59)
    ///     nanosecond (int, optional): nanosecond (0-999,999,999)
    ///
    /// Returns:
    ///     Datetime: created `Datetime` object
    #[classmethod]
    #[pyo3(signature = (year, month, day, hour=0, minute=0, second=0, nanosecond=0))]
    pub fn of(
        _cls: &Bound<PyType>,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        nanosecond: u32,
    ) -> PyResult<Self> {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| PyValueError::new_err("invalid date value"))?;
        let time = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)
            .ok_or_else(|| PyValueError::new_err("invalid time value"))?;
        let v = chrono::NaiveDateTime::new(date, time);
        let value = TgTimePoint::from(v);
        Ok(Datetime { value: Some(value) })
    }

    /// Create a `Datetime` from epoch seconds and nanoseconds.
    ///
    /// Args:
    ///     epoch_seconds (int): offset seconds from epoch (1970-01-01 00:00:00)
    ///     nanos (int): nanosecond part of the time (0-999,999,999)
    ///
    /// Returns:
    ///     Datetime: created `Datetime` object
    #[classmethod]
    pub fn raw(_cls: &Bound<PyType>, epoch_seconds: i64, nanos: u32) -> PyResult<Self> {
        let value = TgTimePoint::new(epoch_seconds, nanos);
        Ok(Datetime { value: Some(value) })
    }

    /// Value.
    #[getter]
    pub fn value(&self) -> Option<chrono::NaiveDateTime> {
        self.value.as_ref().map(|v| v.into())
    }

    /// Nnanosecond.
    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value.map(|v| v.nano_adjustment)
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value() {
            format!("Datetime({})", v)
        } else {
            "Datetime(None)".to_string()
        }
    }
}

impl Datetime {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<Datetime>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            if value.is_none() {
                return Ok(SqlParameter::null(name));
            }

            if let Ok(v) = value.call_method1("astype", ("datetime64[ns]",)) {
                let v = v.call_method1("astype", ("int64",))?;
                let epoch: i64 = v.extract()?;
                let dt = chrono::DateTime::<chrono::Utc>::from_timestamp_nanos(epoch);
                return Ok(SqlParameter::of(name, Some(dt.naive_utc())));
            }

            let v: Option<chrono::NaiveDateTime> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
