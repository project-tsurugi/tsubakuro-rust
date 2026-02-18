use chrono::{TimeZone, Timelike};
use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgTimePointWithTimeZone};

/// TIMESTAMP WITH TIME ZONE type.
///
/// Attributes:
///     value (Optional[datetime.datetime]): datetime value with time zone. (read only)
///     nanosecond (Optional[int]): nanosecond part of the time. (read only)
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct OffsetDatetime {
    value: Option<TgTimePointWithTimeZone>,
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
            let value = TgTimePointWithTimeZone::from(v);
            Ok(OffsetDatetime { value: Some(value) })
        } else {
            Ok(OffsetDatetime { value: None })
        }
    }

    /// Create a `OffsetDatetime` from year, month, day, hour, minute, second, nanosecond, and tzinfo.
    ///
    /// Args:
    ///     year (int): year
    ///     month (int): month (1-12)
    ///     day (int): day (1-31)
    ///     hour (int, optional): hour (0-23)
    ///     minute (int, optional): minute (0-59)
    ///     second (int, optional): second (0-59)
    ///     nanosecond (int, optional): nanosecond (0-999,999,999)
    ///     tzinfo (datetime.tzinfo, optional): time zone info (default: UTC)
    ///
    /// Returns:
    ///     OffsetDatetime: created `OffsetDatetime` instance
    #[classmethod]
    #[pyo3(signature = (year, month, day, hour=0, minute=0, second=0, nanosecond=0, tzinfo=None))]
    pub fn of(
        _cls: &Bound<PyType>,
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
        let value = TgTimePointWithTimeZone::from(v);
        Ok(OffsetDatetime { value: Some(value) })
    }

    /// Create a `OffsetDatetime` from epoch seconds, nanoseconds, and time zone offset.
    ///
    /// Args:
    ///     epoch_seconds (int): offset seconds from epoch (1970-01-01 00:00:00)
    ///     nanos (int): nanosecond part of the time (0-999,999,999)
    ///     time_zone_offset (int): time zone offset in minutes
    ///
    /// Returns:
    ///     OffsetDatetime: created `OffsetDatetime` object
    #[classmethod]
    pub fn raw(
        _cls: &Bound<PyType>,
        epoch_seconds: i64,
        nanos: u32,
        time_zone_offset: i32,
    ) -> PyResult<Self> {
        let value = TgTimePointWithTimeZone::new(epoch_seconds, nanos, time_zone_offset);
        Ok(OffsetDatetime { value: Some(value) })
    }

    /// Value.
    #[getter]
    pub fn value(&self) -> Option<chrono::DateTime<chrono::FixedOffset>> {
        self.value.map(|v| v.into())
    }

    /// Nnanosecond.
    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value.map(|v| v.nano_adjustment)
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value() {
            format!("OffsetDatetime({})", v)
        } else {
            "OffsetDatetime(None)".to_string()
        }
    }
}

impl OffsetDatetime {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<OffsetDatetime>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            let v: Option<chrono::DateTime<chrono::FixedOffset>> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
