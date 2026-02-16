use chrono::{TimeZone, Timelike};
use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;

/// TIMESTAMP WITH TIME ZONE type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct OffsetDatetime {
    /// Value.
    #[pyo3(get)]
    value: Option<chrono::DateTime<chrono::FixedOffset>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl OffsetDatetime {
    /// Create a new `OffsetDatetime`.
    #[new]
    #[pyo3(signature = (value=None, nanosecond=None))]
    pub fn new(
        value: Option<chrono::DateTime<chrono::FixedOffset>>,
        nanosecond: Option<u32>,
    ) -> PyResult<Self> {
        if let Some(v) = value {
            let v = if let Some(ns) = nanosecond {
                v.with_nanosecond(ns)
                    .ok_or_else(|| PyValueError::new_err("invalid nanosecond value"))?
            } else {
                v
            };
            Ok(OffsetDatetime { value: Some(v) })
        } else {
            Ok(OffsetDatetime { value: None })
        }
    }

    /// Create a `OffsetDatetime` from year, month, day, hour, minute, second, nanosecond, and tzinfo.
    #[classmethod]
    #[pyo3(signature = (year, month, day, hour=0, minute=0, second=0, nanosecond=0, tzinfo=None))]
    pub fn of(
        _cls: Bound<PyType>,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        nanosecond: u32,
        tzinfo: Option<chrono::FixedOffset>,
    ) -> PyResult<Self> {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| PyValueError::new_err("invalid date value"))?;
        let time = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)
            .ok_or_else(|| PyValueError::new_err("invalid time value"))?;
        let datetime = chrono::NaiveDateTime::new(date, time);
        let offset = if let Some(tz) = tzinfo {
            tz
        } else {
            chrono::FixedOffset::east_opt(0).unwrap()
        };
        let v = offset
            .from_local_datetime(&datetime)
            .single()
            .ok_or_else(|| PyValueError::new_err("ambiguous or invalid local datetime"))?;
        Ok(OffsetDatetime { value: Some(v) })
    }

    /// Nnanosecond.
    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value.map(|v| v.nanosecond())
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            format!("OffsetDatetime({})", v)
        } else {
            "OffsetDatetime(None)".to_string()
        }
    }
}

impl OffsetDatetime {
    pub const fn value(&self) -> &Option<chrono::DateTime<chrono::FixedOffset>> {
        &self.value
    }
}
