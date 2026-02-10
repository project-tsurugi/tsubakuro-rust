use chrono::Timelike;
use log::{debug, warn};
use pyo3::{
    prelude::*,
    types::{PyTime, PyTuple},
};
use tsubakuro_rust_core::prelude::{AtomType, SqlQueryResult, SqlQueryResultFetch};

use crate::{cursor::RowNumber, error::to_pyerr};

pub(crate) async fn next_row1<'py>(
    py: Python<'py>,
    qr: &mut SqlQueryResult,
    types: &Vec<AtomType>,
    row_number: &mut Option<RowNumber>,
) -> PyResult<Option<Bound<'py, PyTuple>>> {
    if !qr.next_row().await.map_err(to_pyerr)? {
        return Ok(None);
    }

    let mut vec: Vec<Bound<PyAny>> = Vec::with_capacity(types.len());
    for atom_type in types {
        if qr.next_column().await.map_err(to_pyerr)? {
            match atom_type {
                AtomType::Boolean => {
                    let value: Option<bool> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::Int4 => {
                    let value: Option<i32> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::Int8 => {
                    let value: Option<i64> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::Float4 => {
                    let value: Option<f32> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::Float8 => {
                    let value: Option<f64> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::Decimal => {
                    let value: Option<rust_decimal::Decimal> =
                        qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::Character => {
                    let value: Option<String> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::Octet => {
                    let value: Option<Vec<u8>> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::Date => {
                    let value: Option<chrono::NaiveDate> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::TimeOfDay => {
                    let value: Option<chrono::NaiveTime> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::TimePoint => {
                    let value: Option<chrono::NaiveDateTime> =
                        qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                AtomType::TimeOfDayWithTimeZone => {
                    let value: Option<(chrono::NaiveTime, chrono::FixedOffset)> =
                        qr.fetch().await.map_err(to_pyerr)?;
                    let value = to_py_time_tz(py, value)?;
                    vec.push(value);
                }
                AtomType::TimePointWithTimeZone => {
                    let value: Option<chrono::DateTime<chrono::FixedOffset>> =
                        qr.fetch().await.map_err(to_pyerr)?;
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
                _ => {
                    debug!("Cursor::next_row(): Unsupported atom_type {:?}", atom_type);
                    let value = py.None();
                    let value = value.into_pyobject(py)?;
                    vec.push(value);
                }
            }
        } else {
            warn!(
                "Cursor::next_row(): No column data for atom_type {:?}",
                atom_type
            );
            let value = py.None();
            let value = value.into_pyobject(py)?;
            vec.push(value);
        }
    }

    if let Some(row_number) = row_number {
        row_number.increment();
    }

    let tuple = PyTuple::new(py, vec)?;
    Ok(Some(tuple))
}

fn to_py_time_tz<'py>(
    py: Python<'py>,
    value: Option<(chrono::NaiveTime, chrono::FixedOffset)>,
) -> PyResult<Bound<'py, PyAny>> {
    let (time, offset) = if let Some(v) = value {
        v
    } else {
        return Ok(py.None().into_pyobject(py)?);
    };

    let hour = time.hour() as u8;
    let minute = time.minute() as u8;
    let second = time.second() as u8;
    let microsecond = time.nanosecond() / 1000;
    let tzinfo = offset.into_pyobject(py)?;
    let time = PyTime::new(py, hour, minute, second, microsecond, Some(&tzinfo))?;
    Ok(time.into_any())
}
