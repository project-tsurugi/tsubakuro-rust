use pyo3::{exceptions::PyValueError, prelude::*, types::*};
use pyo3_stub_gen::derive::*;
use tsubakuro_rust_core::prelude::{SqlParameter, SqlParameterOf, TgDate};

/// DATE type.
///
/// Attributes:
///     value (Optional[datetime.date]): date value. (read only)
///
/// Examples:
///     ```python
///     import tsubakuro_rust_python as tsurugi
///     import datetime
///
///     value = tsurugi.type_code.Date(datetime.date(2026, 2, 24))
///     value = tsurugi.type_code.Date(None)
///     ```
#[gen_stub_pyclass]
#[pyclass(module = "tsubakuro_rust_python.type_code")]
#[derive(Debug)]
pub struct Date {
    value: Option<TgDate>,
}

#[gen_stub_pymethods]
#[pymethods]
impl Date {
    /// Create a new `Date`.
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new(value: Option<chrono::NaiveDate>) -> PyResult<Self> {
        let value = value.map(TgDate::from);
        Ok(Date { value })
    }

    /// Create a `Date` from year, month, and day.
    ///
    /// Args:
    ///     year (int): year
    ///     month (int): month (1-12)
    ///     day (int): day (1-31)
    ///
    /// Returns:
    ///     Date: created `Date` object
    #[classmethod]
    pub fn of(_cls: &Bound<PyType>, year: i32, month: u32, day: u32) -> PyResult<Self> {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| PyValueError::new_err("invalid date value"))?;
        let value = TgDate::from(date);
        Ok(Date { value: Some(value) })
    }

    /// Create a `Date` from epoch days.
    ///
    /// Args:
    ///     epoch_days (int): number of days offset of epoch 1970-01-01
    ///
    /// Returns:
    ///     Date: created `Date` object
    #[classmethod]
    pub fn raw(_cls: &Bound<PyType>, epoch_days: i64) -> PyResult<Self> {
        let value = TgDate::new(epoch_days);
        Ok(Date { value: Some(value) })
    }

    /// Value.
    #[getter]
    pub fn value(&self) -> Option<chrono::NaiveDate> {
        self.value.as_ref().map(|v| v.into())
    }

    pub fn __repr__(&self) -> String {
        if let Some(v) = self.value() {
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
