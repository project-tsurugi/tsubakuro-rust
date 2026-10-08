use std::{sync::Arc, time::Duration};

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

pub(crate) struct QueryResultContext<'a> {
    sql_client: &'a SqlClient,
    transaction: Option<Arc<Transaction>>,
    lob_download_timeout: Duration,
}

impl<'a> QueryResultContext<'a> {
    pub(crate) fn new(
        sql_client: &'a SqlClient,
        transaction: Option<Arc<Transaction>>,
        lob_download_timeout: Duration,
    ) -> QueryResultContext<'a> {
        QueryResultContext {
            sql_client,
            transaction,
            lob_download_timeout,
        }
    }

    fn sql_client(&self) -> &SqlClient {
        self.sql_client
    }

    fn transaction(&self) -> PyResult<&Arc<Transaction>> {
        self.transaction
            .as_ref()
            .ok_or_else(|| PyErr::new::<InternalError, _>("Transaction is not available"))
    }

    fn lob_download_timeout(&self) -> Duration {
        self.lob_download_timeout
    }
}

pub(crate) enum QueryValue {
    None,
    Boolean(Option<bool>),
    Int4(Option<i32>),
    Int8(Option<i64>),
    Float4(Option<f32>),
    Float8(Option<f64>),
    Decimal(Option<rust_decimal::Decimal>),
    Character(Option<String>),
    Binary(Option<Vec<u8>>),
    Date(Option<chrono::NaiveDate>),
    Time(Option<chrono::NaiveTime>),
    Timestamp(Option<chrono::NaiveDateTime>),
    TimeTz(Option<(chrono::NaiveTime, chrono::FixedOffset)>),
    TimestampTz(Option<chrono::DateTime<chrono::FixedOffset>>),
}

impl QueryValue {
    fn to_pyobject<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let value = match self {
            QueryValue::None => py.None().into_bound(py),
            QueryValue::Boolean(v) => v.into_pyobject(py)?,
            QueryValue::Int4(v) => v.into_pyobject(py)?,
            QueryValue::Int8(v) => v.into_pyobject(py)?,
            QueryValue::Float4(v) => v.into_pyobject(py)?,
            QueryValue::Float8(v) => v.into_pyobject(py)?,
            QueryValue::Decimal(v) => v.into_pyobject(py)?,
            QueryValue::Character(v) => v.into_pyobject(py)?,
            QueryValue::Binary(v) => v.into_pyobject(py)?,
            QueryValue::Date(v) => v.into_pyobject(py)?,
            QueryValue::Time(v) => v.into_pyobject(py)?,
            QueryValue::Timestamp(v) => v.into_pyobject(py)?,
            QueryValue::TimeTz(v) => to_py_time_tz(py, v)?,
            QueryValue::TimestampTz(v) => v.into_pyobject(py)?,
        };
        Ok(value)
    }
}

fn to_py_time_tz<'py>(
    py: Python<'py>,
    value: &Option<(chrono::NaiveTime, chrono::FixedOffset)>,
) -> PyResult<Bound<'py, PyAny>> {
    let (time, offset) = if let Some(v) = value {
        v
    } else {
        return Ok(py.None().into_bound(py));
    };

    let hour = time.hour() as u8;
    let minute = time.minute() as u8;
    let second = time.second() as u8;
    let microsecond = time.nanosecond() / 1000;
    let tzinfo = offset.into_pyobject(py)?;
    let time = PyTime::new(py, hour, minute, second, microsecond, Some(&tzinfo))?;
    Ok(time.into_any())
}

pub(crate) async fn next_row1(
    context: &QueryResultContext<'_>,
    qr: &mut SqlQueryResult,
    types: &Vec<AtomType>,
    row_number: &mut Option<RowNumber>,
) -> PyResult<Option<Vec<QueryValue>>> {
    if !qr.next_row().await.map_err(to_pyerr)? {
        return Ok(None);
    }

    let row = get_row1(context, qr, types, row_number).await?;

    Ok(Some(row))
}

async fn get_row1(
    context: &QueryResultContext<'_>,
    qr: &mut SqlQueryResult,
    types: &Vec<AtomType>,
    row_number: &mut Option<RowNumber>,
) -> PyResult<Vec<QueryValue>> {
    let mut vec: Vec<QueryValue> = Vec::with_capacity(types.len());
    for atom_type in types {
        if qr.next_column().await.map_err(to_pyerr)? {
            match atom_type {
                AtomType::Boolean => {
                    let value: Option<bool> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Boolean(value);
                    vec.push(value);
                }
                AtomType::Int4 => {
                    let value: Option<i32> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Int4(value);
                    vec.push(value);
                }
                AtomType::Int8 => {
                    let value: Option<i64> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Int8(value);
                    vec.push(value);
                }
                AtomType::Float4 => {
                    let value: Option<f32> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Float4(value);
                    vec.push(value);
                }
                AtomType::Float8 => {
                    let value: Option<f64> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Float8(value);
                    vec.push(value);
                }
                AtomType::Decimal => {
                    let value: Option<rust_decimal::Decimal> =
                        qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Decimal(value);
                    vec.push(value);
                }
                AtomType::Character => {
                    let value: Option<String> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Character(value);
                    vec.push(value);
                }
                AtomType::Octet => {
                    let value: Option<Vec<u8>> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Binary(value);
                    vec.push(value);
                }
                AtomType::Date => {
                    let value: Option<chrono::NaiveDate> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Date(value);
                    vec.push(value);
                }
                AtomType::TimeOfDay => {
                    let value: Option<chrono::NaiveTime> = qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Time(value);
                    vec.push(value);
                }
                AtomType::TimePoint => {
                    let value: Option<chrono::NaiveDateTime> =
                        qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::Timestamp(value);
                    vec.push(value);
                }
                AtomType::TimeOfDayWithTimeZone => {
                    let value: Option<(chrono::NaiveTime, chrono::FixedOffset)> =
                        qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::TimeTz(value);
                    vec.push(value);
                }
                AtomType::TimePointWithTimeZone => {
                    let value: Option<chrono::DateTime<chrono::FixedOffset>> =
                        qr.fetch().await.map_err(to_pyerr)?;
                    let value = QueryValue::TimestampTz(value);
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
                    let value = QueryValue::None;
                    vec.push(value);
                }
            }
        } else {
            warn!(
                "Cursor::next_row(): No column data for atom_type {:?}",
                atom_type
            );
            vec.push(QueryValue::None);
        }
    }

    if let Some(row_number) = row_number {
        row_number.increment();
    }

    Ok(vec)
}

async fn download_blob(
    context: &QueryResultContext<'_>,
    blob: Option<TgBlobReference>,
) -> PyResult<QueryValue> {
    let blob = match blob {
        Some(blob) => blob,
        None => return Ok(QueryValue::None),
    };

    let sql_client = context.sql_client();
    let tx = context.transaction()?;
    let timeout = context.lob_download_timeout();

    let value = sql_client
        .read_blob_for(&tx, &blob, timeout)
        .await
        .map_err(to_pyerr)?;
    Ok(QueryValue::Binary(Some(value)))
}

async fn download_clob(
    context: &QueryResultContext<'_>,
    clob: Option<TgClobReference>,
) -> PyResult<QueryValue> {
    let clob = match clob {
        Some(clob) => clob,
        None => return Ok(QueryValue::None),
    };

    let sql_client = context.sql_client();
    let tx = context.transaction()?;
    let timeout = context.lob_download_timeout();

    let value = sql_client
        .read_clob_for(&tx, &clob, timeout)
        .await
        .map_err(to_pyerr)?;
    Ok(QueryValue::Character(Some(value)))
}

pub(crate) fn convert_row_to_tuple<'py>(
    py: Python<'py>,
    row: Vec<QueryValue>,
) -> PyResult<Bound<'py, PyTuple>> {
    let mut vec: Vec<Bound<PyAny>> = Vec::with_capacity(row.len());
    for value in row {
        let value: Bound<PyAny> = value.to_pyobject(py)?;
        vec.push(value);
    }

    let tuple = PyTuple::new(py, vec)?;
    Ok(tuple)
}
