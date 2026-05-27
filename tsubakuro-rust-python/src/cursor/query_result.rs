use std::sync::Arc;

use chrono::Timelike;
use log::{debug, warn};
use pyo3::{
    prelude::*,
    types::{PyTime, PyTuple},
};
use tsubakuro_rust_core::prelude::{
    AtomType, SqlClient, SqlQueryResult, SqlQueryResultFetch, TgBlobReference, TgClobReference,
    Transaction,
};

use crate::{
    cursor::RowNumber,
    error::{to_pyerr, InternalError},
};

pub(crate) struct QueryResultContext<'py, 'a> {
    py: Python<'py>,
    sql_client: &'a SqlClient,
    transaction: Option<Arc<Transaction>>,
}

impl<'py, 'a> QueryResultContext<'py, 'a> {
    pub(crate) fn new(
        py: Python<'py>,
        sql_client: &'a SqlClient,
        transaction: Option<Arc<Transaction>>,
    ) -> QueryResultContext<'py, 'a> {
        QueryResultContext {
            py,
            sql_client,
            transaction,
        }
    }

    fn py(&self) -> Python<'py> {
        self.py
    }

    fn sql_client(&self) -> &SqlClient {
        self.sql_client
    }

    fn transaction(&self) -> PyResult<&Arc<Transaction>> {
        self.transaction
            .as_ref()
            .ok_or_else(|| PyErr::new::<InternalError, _>("Transaction is not available"))
    }
}

pub(crate) async fn next_row1<'py>(
    context: &QueryResultContext<'py, '_>,
    qr: &mut SqlQueryResult,
    types: &Vec<AtomType>,
    row_number: &mut Option<RowNumber>,
) -> PyResult<Option<Bound<'py, PyTuple>>> {
    if !qr.next_row().await.map_err(to_pyerr)? {
        return Ok(None);
    }

    let py = context.py();

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
                AtomType::Blob => {
                    let value: Option<TgBlobReference> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = download_blob(context, value).await?;
                    vec.push(value);
                }
                AtomType::Clob => {
                    let value: Option<TgClobReference> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = download_clob(context, value).await?;
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

async fn download_blob<'py>(
    context: &QueryResultContext<'py, '_>,
    blob: Option<TgBlobReference>,
) -> PyResult<Bound<'py, PyAny>> {
    let py = context.py();
    let blob = match blob {
        Some(blob) => blob,
        None => return Ok(py.None().into_pyobject(py)?),
    };

    let sql_client = context.sql_client();
    let tx = context.transaction()?;

    let value = sql_client.read_blob(&tx, &blob).await.map_err(to_pyerr)?;
    Ok(value.into_pyobject(py)?)
}

async fn download_clob<'py>(
    context: &QueryResultContext<'py, '_>,
    clob: Option<TgClobReference>,
) -> PyResult<Bound<'py, PyAny>> {
    let py = context.py();
    let clob = match clob {
        Some(clob) => clob,
        None => return Ok(py.None().into_pyobject(py)?),
    };

    let sql_client = context.sql_client();
    let tx = context.transaction()?;

    let value = sql_client.read_clob(&tx, &clob).await.map_err(to_pyerr)?;
    Ok(value.into_pyobject(py)?.into_any())
}
