use chrono::Timelike;
use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf};

/// TIME WITH TIME ZONE type.
#[gen_stub_pyclass]
#[pyclass]
#[derive(Debug)]
pub struct OffsetTime {
    value: Option<(chrono::NaiveTime, chrono::FixedOffset)>,
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
            Ok(OffsetTime {
                value: Some((time, offset)),
            })
        } else {
            Ok(OffsetTime { value: None })
        }
    }

    /// Create a `OffsetTime` from hour, minute, second, nanosecond, and tzinfo.
    #[classmethod]
    #[pyo3(signature = (hour=0, minute=0, second=0, nanosecond=0, tzinfo=None))]
    pub fn of(
        _cls: Bound<PyType>,
        hour: u32,
        minute: u32,
        second: u32,
        nanosecond: u32,
        tzinfo: Option<chrono::FixedOffset>,
    ) -> PyResult<Self> {
        let time = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nanosecond)
            .ok_or_else(|| PyValueError::new_err("invalid time value"))?;
        let tzinfo = if let Some(tz) = tzinfo {
            tz
        } else {
            chrono::FixedOffset::east_opt(0).unwrap()
        };
        Ok(OffsetTime {
            value: Some((time, tzinfo)),
        })
    }

    /// Value.
    #[getter]
    pub fn get_value<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyTime>>> {
        if let Some((time, offset)) = &self.value {
            let hour = time.hour() as u8;
            let minute = time.minute() as u8;
            let second = time.second() as u8;
            let microsecond = time.nanosecond() / 1000;
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
        self.value.map(|(time, _)| time.nanosecond())
    }

    pub fn __repr__(&self) -> String {
        if let Some((time, offset)) = &self.value {
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
