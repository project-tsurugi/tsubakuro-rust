use chrono::Timelike;
use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgTimeOfDayWithTimeZone};

/// TIME WITH TIME ZONE type.
///
/// Attributes:
///     value (Optional[datetime.time]): time value with time zone. (read only)
///     nanosecond (Optional[int]): nanosecond part of the time. (read only)
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct OffsetTime {
    value: Option<TgTimeOfDayWithTimeZone>,
}

#[gen_stub_pymethods]
#[pymethods]
impl OffsetTime {
    /// Create a new `OffsetTime`.
    #[new]
    #[pyo3(signature = (value=None, nanosecond=None))]
    pub fn new(value: Option<Bound<PyTime>>, nanosecond: Option<u32>) -> PyResult<Self> {
        if let Some(v) = value {
            let time: chrono::NaiveTime = v.extract()?;
            let time = if let Some(ns) = nanosecond {
                time.with_nanosecond(ns)
                    .ok_or_else(|| PyValueError::new_err("invalid nanosecond value"))?
            } else {
                time
            };
            let tzinfo = v.get_tzinfo();
            let offset: chrono::FixedOffset = if let Some(tzinfo) = tzinfo {
                tzinfo.extract()?
            } else {
                chrono::FixedOffset::east_opt(0).unwrap()
            };
            let value = TgTimeOfDayWithTimeZone::from((time, offset));
            Ok(OffsetTime { value: Some(value) })
        } else {
            Ok(OffsetTime { value: None })
        }
    }

    /// Create a `OffsetTime` from hour, minute, second, nanosecond, and tzinfo.
    ///
    /// Args:
    ///     hour (int, optional): hour (0-23)
    ///     minute (int, optional): minute (0-59)
    ///     second (int, optional): second (0-59)
    ///     nanosecond (int, optional): nanosecond (0-999,999,999)
    ///     tzinfo (datetime.tzinfo, optional): time zone info (Default: UTC)
    ///
    /// Returns:
    ///     OffsetTime: created `OffsetTime` object
    #[classmethod]
    #[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0, tzinfo=None))]
    pub fn of(
        _cls: &Bound<PyType>,
        hour: u32,
        minute: u32,
        second: u32,
        nanosecond: u32,
        tzinfo: Option<chrono::FixedOffset>,
    ) -> PyResult<Self> {
        let time = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)
            .ok_or_else(|| PyValueError::new_err("invalid time value"))?;
        let offset = if let Some(tz) = tzinfo {
            tz
        } else {
            chrono::FixedOffset::east_opt(0).unwrap()
        };
        let value = TgTimeOfDayWithTimeZone::from((time, offset));
        Ok(OffsetTime { value: Some(value) })
    }

    /// Create a `OffsetTime` from epoch nanoseconds of day and time zone offset.
    ///
    /// Args:
    ///     nanoseconds_of_day (int): offset nano-seconds from epoch (00:00:00) in the time zone
    ///     time_zone_offset (int): timezone offset in minute
    ///
    /// Returns:
    ///     OffsetTime: created `OffsetTime` object
    #[classmethod]
    pub fn raw(
        _cls: &Bound<PyType>,
        nanoseconds_of_day: u64,
        time_zone_offset: i32,
    ) -> PyResult<Self> {
        let value = TgTimeOfDayWithTimeZone::new(nanoseconds_of_day, time_zone_offset);
        Ok(OffsetTime { value: Some(value) })
    }

    /// Value.
    #[getter]
    pub fn value<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyTime>>> {
        if let Some(v) = &self.value {
            let epoch_nanos = v.offset_nanoseconds;
            let hour = (epoch_nanos / 3_600_000_000_000) as u8;
            let minute = ((epoch_nanos % 3_600_000_000_000) / 60_000_000_000) as u8;
            let second = ((epoch_nanos % 60_000_000_000) / 1_000_000_000) as u8;
            let microsecond = ((epoch_nanos % 1_000_000_000) / 1000) as u32;

            let offset = chrono::FixedOffset::east_opt((v.time_zone_offset * 60) as i32).unwrap();
            let tzinfo = offset.into_pyobject(py)?;

            let time = PyTime::new(py, hour, minute, second, microsecond, Some(&tzinfo))?;
            Ok(Some(time))
        } else {
            Ok(None)
        }
    }

    /// Nnanosecond.
    #[getter]
    pub fn nanosecond(&self) -> Option<u32> {
        self.value
            .map(|v| (v.offset_nanoseconds % 1_000_000_000) as u32)
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = &self.value {
            let (time, offset): (chrono::NaiveTime, chrono::FixedOffset) = v.into();
            format!("OffsetTime({} {})", time, offset)
        } else {
            "OffsetTime(None)".to_string()
        }
    }
}

impl OffsetTime {
    pub(crate) fn create_parameter(name: &str, value: &Bound<PyAny>) -> PyResult<SqlParameter> {
        if let Ok(v) = value.extract::<PyRef<OffsetTime>>() {
            Ok(SqlParameter::of(name, v.value))
        } else {
            if value.is_none() {
                return Ok(SqlParameter::null(name));
            }

            if value.is_instance_of::<PyTime>() {
                let time: chrono::NaiveTime = value.extract()?;
                let tzinfo = value.getattr("tzinfo")?;
                let offset: chrono::FixedOffset = if tzinfo.is_none() {
                    chrono::FixedOffset::east_opt(0).unwrap()
                } else {
                    tzinfo.extract()?
                };
                let v = (time, offset);
                return Ok(SqlParameter::of(name, v));
            }

            let v: Option<(chrono::NaiveTime, chrono::FixedOffset)> = value.extract()?;
            Ok(SqlParameter::of(name, v))
        }
    }
}
