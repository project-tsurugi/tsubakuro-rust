use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::{
        response::Response as SqlResponseType, Response as SqlResponse,
        ResultSetMetadata as SqlQueryResultMetadata,
    },
    prelude::{
        convert_sql_response, TgBlobReference, TgClobReference, TgDate, TgDecimalI128,
        TgDecimalResult, TgTimeOfDay, TgTimeOfDayWithTimeZone, TgTimePoint,
        TgTimePointWithTimeZone,
    },
    prost_decode_error,
    session::wire::{response::WireResponse, Wire},
    util::Timeout,
};
use async_trait::async_trait;
use prost::{bytes::BytesMut, Message};
use std::{sync::Arc, time::Duration};

#[cfg(any(
    feature = "with_bigdecimal",
    feature = "with_chrono",
    feature = "with_time"
))]
use {crate::client_error, log::trace};

use super::value_stream::ResultSetValueStream;

/// Represents a server side SQL result set.
///
/// **thread unsafe**
///
/// # Examples
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
///     let sql = "select pk, value from tb order by pk";
///     let mut query_result = client.query(transaction, sql).await?;
///
///     while query_result.next_row().await? {
///         if query_result.next_column().await? {
///             let pk: i32 = query_result.fetch().await?;
///             println!("pk={}", pk);
///         }
///         if query_result.next_column().await? {
///             let value: String = query_result.fetch().await?;
///             println!("value={}", value);
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub struct SqlQueryResult {
    name: String,
    metadata: Option<SqlQueryResultMetadata>,
    value_stream: ResultSetValueStream,
    default_timeout: Duration,
}

impl std::fmt::Debug for SqlQueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResultSet")
            .field("name", &self.name)
            .field("metadata", &self.metadata)
            .finish()
    }
}

impl SqlQueryResult {
    fn new(
        name: String,
        metadata: Option<SqlQueryResultMetadata>,
        value_stream: ResultSetValueStream,
        default_timeout: Duration,
    ) -> SqlQueryResult {
        SqlQueryResult {
            name,
            metadata,
            value_stream,
            default_timeout,
        }
    }

    /// Set default timeout.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Get default timeout.
    pub fn default_timeout(&mut self) -> &Duration {
        &self.default_timeout
    }

    /// Returns the metadata of this query result.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(mut query_result: SqlQueryResult) -> Result<(), TgError> {
    ///     let metadata = query_result.get_metadata().unwrap().clone();
    ///     let columns = metadata.columns();
    ///
    ///     while query_result.next_row().await? {
    ///         for column in columns {
    ///             let column_name = column.name();
    ///
    ///             assert!(query_result.next_column().await?);
    ///             match column.atom_type().unwrap() {
    ///                 AtomType::Int4 => { // int
    ///                     let value: i32 = query_result.fetch().await?;
    ///                 }
    ///                 AtomType::Int8 => { // bigint
    ///                     let value: i64 = query_result.fetch().await?;
    ///                 }
    ///                 AtomType::Character => { // char, varchar
    ///                     let value: String = query_result.fetch().await?;
    ///                 }
    ///                 _ => panic!(),
    ///             };
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_metadata(&self) -> Option<&SqlQueryResultMetadata> {
        self.metadata.as_ref()
    }
}

pub(crate) fn query_result_processor(
    wire: Arc<Wire>,
    response: WireResponse,
    default_timeout: Duration,
) -> Result<SqlQueryResult, TgError> {
    // const FUNCTION_NAME: &str = "query_result_processor()";

    let (dc_name, metadata) = read_result_set_metadata(response)?;

    let data_channel = wire.create_data_channel(&dc_name)?;
    let value_stream = ResultSetValueStream::new(data_channel);
    let query_result = SqlQueryResult::new(dc_name, metadata, value_stream, default_timeout);

    Ok(query_result)
}

fn read_result_set_metadata(
    response: WireResponse,
) -> Result<(String, Option<SqlQueryResultMetadata>), TgError> {
    const FUNCTION_NAME: &str = "read_result_set_metadata()";

    let _ = convert_sql_response(FUNCTION_NAME, &response)?;
    match response {
        WireResponse::ResponseSessionBodyhead(_slot, payload, error) => {
            if let Some(e) = error {
                return Err(e.to_tg_error());
            }
            let payload = if let Some(payload) = payload {
                payload
            } else {
                return Err(invalid_response_error!(FUNCTION_NAME, "payload is None"));
            };
            let message = SqlResponse::decode_length_delimited(payload)
                .map_err(|e| prost_decode_error!(FUNCTION_NAME, "SqlResponse", e))?;
            match message.response {
                Some(SqlResponseType::ExecuteQuery(execute_query)) => {
                    Ok((execute_query.name, execute_query.record_meta))
                }
                _ => Err(invalid_response_error!(
                    FUNCTION_NAME,
                    format!("response {:?} is not ExecuteQuery", message.response),
                )),
            }
        }
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response({:?}) is not ResponseSessionBodyhead", response),
        )),
    }
}

impl SqlQueryResult {
    /// Advances the cursor to the head of the next row.
    ///
    /// If this operation was succeeded (returns `true`), this cursor points the head of the next row.
    /// After this operation, you need to invoke [`Self::next_column`] to retrieve the first column data of the next row.
    pub async fn next_row(&mut self) -> Result<bool, TgError> {
        self.next_row_for(self.default_timeout).await
    }

    /// Advances the cursor to the head of the next row.
    ///
    /// If this operation was succeeded (returns `true`), this cursor points the head of the next row.
    /// After this operation, you need to invoke [`Self::next_column`] to retrieve the first column data of the next row.
    pub async fn next_row_for(&mut self, timeout: Duration) -> Result<bool, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.next_row(&timeout).await
    }

    /// Advances the cursor to the next column in the current row.
    ///
    /// If this operation was succeeded (returns `true`), this cursor will point to the next column of the row.
    /// You can invoke [`Self::fetch`] method to obtain the column value.
    pub async fn next_column(&mut self) -> Result<bool, TgError> {
        self.next_column_for(self.default_timeout).await
    }

    /// Advances the cursor to the next column in the current row.
    ///
    /// If this operation was succeeded (returns `true`), this cursor will point to the next column of the row.
    /// You can invoke [`Self::fetch`] method to obtain the column value.
    pub async fn next_column_for(&mut self, timeout: Duration) -> Result<bool, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.next_column(&timeout).await
    }

    /// Returns whether or not the column on this cursor is `NULL`.
    pub fn is_null(&mut self) -> Result<bool, TgError> {
        self.value_stream.is_null()
    }
}

/// `fetch` method for [SqlQueryResult].
#[async_trait(?Send)] // thread unsafe
pub trait SqlQueryResultFetch<T> {
    /// Retrieves a value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<T, TgError>;

    /// Retrieves a value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<T, TgError>;
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<bool> for SqlQueryResult {
    /// Retrieves a `BOOLEAN` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<bool, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_boolean_value(&timeout).await
    }

    /// Retrieves a `BOOLEAN` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<bool, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.fetch_boolean_value(&timeout).await
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<i32> for SqlQueryResult {
    /// Retrieves a `INT4` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<i32, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_int4_value(&timeout).await
    }

    /// Retrieves a `INT4` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<i32, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.fetch_int4_value(&timeout).await
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<i64> for SqlQueryResult {
    /// Retrieves a `INT8` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<i64, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_int8_value(&timeout).await
    }

    /// Retrieves a `INT8` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<i64, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.fetch_int8_value(&timeout).await
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<f32> for SqlQueryResult {
    /// Retrieves a `FLOAT4` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<f32, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_float4_value(&timeout).await
    }

    /// Retrieves a `FLOAT4` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<f32, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.fetch_float4_value(&timeout).await
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<f64> for SqlQueryResult {
    /// Retrieves a `FLOAT8` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<f64, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_float8_value(&timeout).await
    }

    /// Retrieves a `FLOAT8` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<f64, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.fetch_float8_value(&timeout).await
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgDecimalResult> for SqlQueryResult {
    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgDecimalResult, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgDecimalResult, TgError> {
        let timeout = Timeout::new(timeout);
        let (coefficient_bytes, coefficient, exponent) =
            self.value_stream.fetch_decimal_value(&timeout).await?;

        Ok(TgDecimalResult::new(
            coefficient_bytes.as_deref().map(<[u8]>::to_vec),
            coefficient,
            exponent,
        ))
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgDecimalI128> for SqlQueryResult {
    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgDecimalI128, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgDecimalI128, TgError> {
        let value: TgDecimalResult = self.fetch_for(timeout).await?;
        TgDecimalI128::try_from(value)
    }
}

#[cfg(feature = "with_bigdecimal")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<bigdecimal::BigDecimal> for SqlQueryResult {
    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<bigdecimal::BigDecimal, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<bigdecimal::BigDecimal, TgError> {
        let timeout = Timeout::new(timeout);
        let (coefficient_bytes, coefficient, exponent) =
            self.value_stream.fetch_decimal_value(&timeout).await?;
        bigdecimal_big_decimal(coefficient_bytes, coefficient, -exponent)
    }
}

#[cfg(feature = "with_bigdecimal")]
fn bigdecimal_big_decimal(
    coefficient_bytes: Option<BytesMut>,
    coefficient: i64,
    scale: i32,
) -> Result<bigdecimal::BigDecimal, TgError> {
    use bigdecimal::FromPrimitive;

    let value = match coefficient_bytes {
        Some(coefficient) => {
            let value = bigdecimal::num_bigint::BigInt::from_signed_bytes_be(&coefficient);
            bigdecimal::BigDecimal::new(value, scale as i64)
        }
        None => {
            if scale == 0 {
                match bigdecimal::BigDecimal::from_i64(coefficient) {
                    Some(value) => value,
                    None => {
                        trace!(
                            "bigdecimal::BigDecimal::from_i64() error. coefficient={}",
                            coefficient
                        );
                        return Err(client_error!("bigdecimal::BigDecimal generate error"));
                    }
                }
            } else {
                let value = match bigdecimal::num_bigint::BigInt::from_i64(coefficient) {
                    Some(value) => value,
                    None => {
                        trace!(
                            "bigdecimal::BigInt::from_i64() error. coefficient={}",
                            coefficient
                        );
                        return Err(client_error!("bigdecimal::BigDecimal generate error"));
                    }
                };
                bigdecimal::BigDecimal::from_bigint(value, scale as i64)
            }
        }
    };
    Ok(value)
}

#[cfg(feature = "with_rust_decimal")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<rust_decimal::Decimal> for SqlQueryResult {
    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<rust_decimal::Decimal, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<rust_decimal::Decimal, TgError> {
        let timeout = Timeout::new(timeout);
        let (coefficient_bytes, coefficient, exponent) =
            self.value_stream.fetch_decimal_value(&timeout).await?;
        rust_decimal_decimal(coefficient_bytes, coefficient, -exponent)
    }
}

#[cfg(feature = "with_rust_decimal")]
fn rust_decimal_decimal(
    coefficient_bytes: Option<BytesMut>,
    coefficient: i64,
    scale: i32,
) -> Result<rust_decimal::Decimal, TgError> {
    let value = match coefficient_bytes {
        Some(coefficient) => {
            let top = coefficient[0] as i8;
            let mut buf = if top >= 0 { [0u8; 16] } else { [0xffu8; 16] };
            buf[16 - coefficient.len()..].copy_from_slice(&coefficient);
            i128::from_be_bytes(buf)
        }
        None => coefficient as i128,
    };
    let value = if scale >= 0 {
        rust_decimal::Decimal::from_i128_with_scale(value, scale as u32)
    } else {
        let value = rust_decimal::Decimal::from_i128_with_scale(value, 0);
        let factor = rust_decimal::Decimal::from_i128_with_scale(10_i128.pow(-scale as u32), 0);
        value * factor
    };
    Ok(value)
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<String> for SqlQueryResult {
    /// Retrieves a `CHARACTER` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<String, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_character_value(&timeout).await
    }

    /// Retrieves a `CHARACTER` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<String, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.fetch_character_value(&timeout).await
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<BytesMut> for SqlQueryResult {
    /// Retrieves a `OCTET` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<BytesMut, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_octet_value(&timeout).await
    }

    /// Retrieves a `OCTET` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<BytesMut, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.fetch_octet_value(&timeout).await
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<Vec<u8>> for SqlQueryResult {
    /// Retrieves a `OCTET` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<Vec<u8>, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        let value = self.value_stream.fetch_octet_value(&timeout).await?;
        Ok(value.to_vec())
    }

    /// Retrieves a `OCTET` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<Vec<u8>, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self.value_stream.fetch_octet_value(&timeout).await?;
        Ok(value.to_vec())
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgDate> for SqlQueryResult {
    /// Retrieves a `DATE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgDate, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DATE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgDate, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self.value_stream.fetch_date_value(&timeout).await?;
        Ok(TgDate::new(value))
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgTimeOfDay> for SqlQueryResult {
    /// Retrieves a `TIME_OF_DAY` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgTimeOfDay, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_OF_DAY` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgTimeOfDay, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self.value_stream.fetch_time_of_day_value(&timeout).await?;
        Ok(TgTimeOfDay::new(value))
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgTimePoint> for SqlQueryResult {
    /// Retrieves a `TIME_POINT` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgTimePoint, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgTimePoint, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self.value_stream.fetch_time_point_value(&timeout).await?;
        Ok(TgTimePoint::new(value.0, value.1))
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgTimeOfDayWithTimeZone> for SqlQueryResult {
    /// Retrieves a `TIME_OF_DAY_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgTimeOfDayWithTimeZone, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_OF_DAY_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgTimeOfDayWithTimeZone, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self
            .value_stream
            .fetch_time_of_day_with_time_zone_value(&timeout)
            .await?;
        Ok(TgTimeOfDayWithTimeZone::new(value.0, value.1))
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgTimePointWithTimeZone> for SqlQueryResult {
    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgTimePointWithTimeZone, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgTimePointWithTimeZone, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self
            .value_stream
            .fetch_time_point_with_time_zone_value(&timeout)
            .await?;
        Ok(TgTimePointWithTimeZone::new(value.0, value.1, value.2))
    }
}

#[cfg(feature = "with_chrono")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<chrono::NaiveDate> for SqlQueryResult {
    /// Retrieves a `DATE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<chrono::NaiveDate, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DATE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<chrono::NaiveDate, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self.value_stream.fetch_date_value(&timeout).await?;
        chrono_naive_date(value)
    }
}

#[cfg(feature = "with_chrono")]
fn chrono_naive_date(value: i64) -> Result<chrono::NaiveDate, TgError> {
    let days = value + /* NaiveDate(1970-01-01).num_days_from_ce() */ 719_163;
    match chrono::NaiveDate::from_num_days_from_ce_opt(days as i32) {
        Some(value) => Ok(value),
        None => {
            trace!(
                "chrono::NaiveDate::from_num_days_from_ce_opt() error. days={}",
                days
            );
            Err(client_error!("chrono::NaiveDate generate error"))
        }
    }
}

#[cfg(feature = "with_chrono")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<chrono::NaiveTime> for SqlQueryResult {
    /// Retrieves a `TIME_OF_DAY` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<chrono::NaiveTime, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_OF_DAY` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<chrono::NaiveTime, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self.value_stream.fetch_time_of_day_value(&timeout).await?;
        chrono_naive_time(value)
    }
}

#[cfg(feature = "with_chrono")]
fn chrono_naive_time(value: u64) -> Result<chrono::NaiveTime, TgError> {
    let seconds = (value / 1_000_000_000) as u32;
    let nanos = (value % 1_000_000_000) as u32;
    match chrono::NaiveTime::from_num_seconds_from_midnight_opt(seconds, nanos) {
        Some(value) => Ok(value),
        None => {
            trace!(
                "chrono::NaiveTime::from_num_seconds_from_midnight_opt() error. seconds={}, nanos={}",
                seconds,
                nanos
            );
            Err(client_error!("chrono::NaiveTime generate error"))
        }
    }
}

#[cfg(feature = "with_chrono")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<chrono::NaiveDateTime> for SqlQueryResult {
    /// Retrieves a `TIME_POINT` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<chrono::NaiveDateTime, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<chrono::NaiveDateTime, TgError> {
        let timeout = Timeout::new(timeout);
        let (epoch_seconds, nanos) = self.value_stream.fetch_time_point_value(&timeout).await?;
        chrono_naive_date_time(epoch_seconds, nanos)
    }
}

#[cfg(feature = "with_chrono")]
fn chrono_naive_date_time(
    epoch_seconds: i64,
    nanos: u32,
) -> Result<chrono::NaiveDateTime, TgError> {
    match chrono::DateTime::from_timestamp(epoch_seconds, nanos) {
        Some(value) => Ok(value.naive_utc()),
        None => {
            trace!(
                "chrono::DateTime::from_timestamp() error. epoch_seconds={}, nanos={}",
                epoch_seconds,
                nanos
            );
            Err(client_error!("chrono::DateTime generate error"))
        }
    }
}

#[cfg(feature = "with_chrono")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<(chrono::NaiveTime, chrono::FixedOffset)> for SqlQueryResult {
    /// Retrieves a `TIME_OF_DAY_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<(chrono::NaiveTime, chrono::FixedOffset), TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_OF_DAY_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(
        &mut self,
        timeout: Duration,
    ) -> Result<(chrono::NaiveTime, chrono::FixedOffset), TgError> {
        let timeout = Timeout::new(timeout);
        let (time, offset_minutes) = self
            .value_stream
            .fetch_time_of_day_with_time_zone_value(&timeout)
            .await?;
        chrono_naive_time_with_offset(time, offset_minutes)
    }
}

#[cfg(feature = "with_chrono")]
fn chrono_naive_time_with_offset(
    time: u64,
    offset_minutes: i32,
) -> Result<(chrono::NaiveTime, chrono::FixedOffset), TgError> {
    let value = chrono_naive_time(time)?;
    let offset = chrono_fixed_offset(offset_minutes)?;

    Ok((value, offset))
}

#[cfg(feature = "with_chrono")]
fn chrono_fixed_offset(offset_minutes: i32) -> Result<chrono::FixedOffset, TgError> {
    let offset_seconds = offset_minutes * 60;
    match chrono::FixedOffset::east_opt(offset_seconds) {
        Some(value) => Ok(value),
        None => {
            trace!(
                "chrono::FixedOffset::east_opt() error. offset_seconds={}",
                offset_seconds
            );
            Err(client_error!("chrono::FixedOffset generate error"))
        }
    }
}

#[cfg(feature = "with_chrono")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<chrono::DateTime<chrono::FixedOffset>> for SqlQueryResult {
    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<chrono::DateTime<chrono::FixedOffset>, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(
        &mut self,
        timeout: Duration,
    ) -> Result<chrono::DateTime<chrono::FixedOffset>, TgError> {
        let timeout = Timeout::new(timeout);
        let (epoch_seconds, nanos, offset_minutes) = self
            .value_stream
            .fetch_time_point_with_time_zone_value(&timeout)
            .await?;
        chrono_date_time(epoch_seconds, nanos, offset_minutes)
    }
}

#[cfg(feature = "with_chrono")]
fn chrono_date_time(
    epoch_seconds: i64,
    nanos: u32,
    offset_minutes: i32,
) -> Result<chrono::DateTime<chrono::FixedOffset>, TgError> {
    let mut value = match chrono::DateTime::from_timestamp(epoch_seconds, nanos) {
        Some(value) => value.naive_utc(),
        None => {
            trace!(
                "chrono::DateTime::from_timestamp() error. epoch_seconds={}, nanos={}",
                epoch_seconds,
                nanos
            );
            return Err(client_error!("chrono::DateTime generate error"));
        }
    };

    let offset = chrono_fixed_offset(offset_minutes)?;

    if offset.local_minus_utc() != 0 {
        value = match value.checked_sub_offset(offset) {
            Some(value) => value,
            None => {
                trace!(
                    "chrono::NaiveDateTime::checked_sub_offset() error. value={}, offset={}",
                    value,
                    offset
                );
                return Err(client_error!(
                    "chrono::NaiveDateTime checked_sub_offset() error"
                ));
            }
        };
    }
    let value = chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(value, offset);
    Ok(value)
}

#[cfg(feature = "with_chrono")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<chrono::DateTime<chrono::Utc>> for SqlQueryResult {
    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<chrono::DateTime<chrono::Utc>, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(
        &mut self,
        timeout: Duration,
    ) -> Result<chrono::DateTime<chrono::Utc>, TgError> {
        let value: chrono::DateTime<chrono::FixedOffset> = self.fetch_for(timeout).await?;
        Ok(value.to_utc())
    }
}

#[cfg(feature = "with_time")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<time::Date> for SqlQueryResult {
    /// Retrieves a `DATE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<time::Date, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DATE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<time::Date, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self.value_stream.fetch_date_value(&timeout).await?;
        time_date(value)
    }
}

#[cfg(feature = "with_time")]
fn time_date(value: i64) -> Result<time::Date, TgError> {
    let days = value + /* Date(1970-01-01).to_julian_day() */ 2440588;
    match time::Date::from_julian_day(days as i32) {
        Ok(value) => Ok(value),
        Err(e) => {
            trace!("time::Date::from_julian_day() error. days={}", days);
            Err(client_error!("time::Date generate error", e))
        }
    }
}

#[cfg(feature = "with_time")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<time::Time> for SqlQueryResult {
    /// Retrieves a `TIME` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<time::Time, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<time::Time, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self.value_stream.fetch_time_of_day_value(&timeout).await?;
        time_time(value)
    }
}

#[cfg(feature = "with_time")]
fn time_time(value: u64) -> Result<time::Time, TgError> {
    let nanos = value % 1_000_000_000;
    let value = value / 1_000_000_000;
    time_time_nanos(value, nanos as u32)
}

#[cfg(feature = "with_time")]
fn time_time_nanos(seconds: u64, nanos: u32) -> Result<time::Time, TgError> {
    let sec = seconds % 60;
    let value = seconds / 60;
    let min = value % 60;
    let hour = value / 60;

    match time::Time::from_hms_nano(hour as u8, min as u8, sec as u8, nanos) {
        Ok(value) => Ok(value),
        Err(e) => {
            trace!(
                "time::Time::from_hms_nano() error. seconds={}, nanos={}",
                seconds,
                nanos
            );
            Err(client_error!("time::Time generate error", e))
        }
    }
}

#[cfg(feature = "with_time")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<time::PrimitiveDateTime> for SqlQueryResult {
    /// Retrieves a `TIME_POINT` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<time::PrimitiveDateTime, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<time::PrimitiveDateTime, TgError> {
        let timeout = Timeout::new(timeout);
        let (epoch_seconds, nanos) = self.value_stream.fetch_time_point_value(&timeout).await?;
        time_primitive_date_time(epoch_seconds, nanos)
    }
}

#[cfg(feature = "with_time")]
fn time_primitive_date_time(
    epoch_seconds: i64,
    nanos: u32,
) -> Result<time::PrimitiveDateTime, TgError> {
    const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
    let mut days = epoch_seconds / SECONDS_PER_DAY;
    let mut value = epoch_seconds % SECONDS_PER_DAY;
    if value < 0 {
        value += SECONDS_PER_DAY;
        days -= 1;
    }

    let date = time_date(days)?;
    let time = time_time_nanos(value as u64, nanos)?;
    let value = time::PrimitiveDateTime::new(date, time);
    Ok(value)
}

#[cfg(feature = "with_time")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<(time::Time, time::UtcOffset)> for SqlQueryResult {
    /// Retrieves a `TIME_OF_DAY_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<(time::Time, time::UtcOffset), TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_OF_DAY_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(
        &mut self,
        timeout: Duration,
    ) -> Result<(time::Time, time::UtcOffset), TgError> {
        let timeout = Timeout::new(timeout);
        let (time, offset_minutes) = self
            .value_stream
            .fetch_time_of_day_with_time_zone_value(&timeout)
            .await?;
        time_time_with_offset(time, offset_minutes)
    }
}

#[cfg(feature = "with_time")]
fn time_time_with_offset(
    time: u64,
    offset_minutes: i32,
) -> Result<(time::Time, time::UtcOffset), TgError> {
    let value = time_time(time)?;
    let offset = time_utc_offset(offset_minutes)?;

    Ok((value, offset))
}

#[cfg(feature = "with_time")]
fn time_utc_offset(offset_minutes: i32) -> Result<time::UtcOffset, TgError> {
    let offset_seconds = offset_minutes * 60;
    match time::UtcOffset::from_whole_seconds(offset_seconds) {
        Ok(value) => Ok(value),
        Err(e) => {
            trace!(
                "time::UtcOffset::from_whole_seconds() error. offset_seconds={}",
                offset_seconds
            );
            Err(client_error!("time::UtcOffset generate error", e))
        }
    }
}

#[cfg(feature = "with_time")]
#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<time::OffsetDateTime> for SqlQueryResult {
    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<time::OffsetDateTime, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<time::OffsetDateTime, TgError> {
        let timeout = Timeout::new(timeout);
        let (epoch_seconds, nanos, offset_minutes) = self
            .value_stream
            .fetch_time_point_with_time_zone_value(&timeout)
            .await?;
        time_offset_date_time(epoch_seconds, nanos, offset_minutes)
    }
}

#[cfg(feature = "with_time")]
fn time_offset_date_time(
    epoch_seconds: i64,
    nanos: u32,
    offset_minutes: i32,
) -> Result<time::OffsetDateTime, TgError> {
    const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
    let mut days = epoch_seconds / SECONDS_PER_DAY;
    let mut value = epoch_seconds % SECONDS_PER_DAY;
    if value < 0 {
        value += SECONDS_PER_DAY;
        days -= 1;
    }

    let date = time_date(days)?;
    let time = time_time_nanos(value as u64, nanos)?;
    let offset = time_utc_offset(offset_minutes)?;
    let value = time::OffsetDateTime::new_in_offset(date, time, offset);
    Ok(value)
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgBlobReference> for SqlQueryResult {
    /// Retrieves a `BLOB` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgBlobReference, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `BLOB` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgBlobReference, TgError> {
        let timeout = Timeout::new(timeout);
        let (provider, object_id) = self.value_stream.fetch_blob(&timeout).await?;
        Ok(TgBlobReference::new(provider, object_id))
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgClobReference> for SqlQueryResult {
    /// Retrieves a `CLOB` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<TgClobReference, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `CLOB` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgClobReference, TgError> {
        let timeout = Timeout::new(timeout);
        let (provider, object_id) = self.value_stream.fetch_clob(&timeout).await?;
        Ok(TgClobReference::new(provider, object_id))
    }
}

#[async_trait(?Send)] // thread unsafe
impl<T> SqlQueryResultFetch<Option<T>> for SqlQueryResult
where
    SqlQueryResult: SqlQueryResultFetch<T>,
{
    /// Retrieves a `Option<T>` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<Option<T>, TgError> {
        if self.is_null()? {
            Ok(None)
        } else {
            let value: T = self.fetch().await?;
            Ok(Some(value))
        }
    }

    /// Retrieves a `Option<T>` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<Option<T>, TgError> {
        if self.is_null()? {
            Ok(None)
        } else {
            let value: T = self.fetch_for(timeout).await?;
            Ok(Some(value))
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use prost::bytes::BytesMut;

    #[cfg(feature = "with_bigdecimal")]
    #[test]
    fn bigdecimal_big_decimal() {
        bigdecimal_big_decimal_test(Some(&[0]), 0, 0, "0");
        bigdecimal_big_decimal_test(Some(&[0]), 0, 1, "0");
        bigdecimal_big_decimal_test(Some(&[0]), 0, -1, "0");
        bigdecimal_big_decimal_test(Some(&[0x04, 0xd2]), 0, 0, "1234.0");
        bigdecimal_big_decimal_test(Some(&[0x04, 0xd2]), 0, 1, "123.4");
        bigdecimal_big_decimal_test(Some(&[0x04, 0xd2]), 0, -1, "12340");
        bigdecimal_big_decimal_test(Some(&[0xfb, 0x2e]), 0, 0, "-1234.0");
        bigdecimal_big_decimal_test(Some(&[0xfb, 0x2e]), 0, 1, "-123.4");
        bigdecimal_big_decimal_test(Some(&[0xfb, 0x2e]), 0, -1, "-12340");
        bigdecimal_big_decimal_test(None, 0, 0, "0");
        bigdecimal_big_decimal_test(None, 0, 1, "0");
        bigdecimal_big_decimal_test(None, 0, -1, "0");
        bigdecimal_big_decimal_test(None, 1234, 0, "1234.0");
        bigdecimal_big_decimal_test(None, 1234, 1, "123.4");
        bigdecimal_big_decimal_test(None, 1234, -1, "12340");
        bigdecimal_big_decimal_test(None, -1234, 0, "-1234.0");
        bigdecimal_big_decimal_test(None, -1234, 1, "-123.4");
        bigdecimal_big_decimal_test(None, -1234, -1, "-12340");
    }

    #[cfg(feature = "with_bigdecimal")]
    fn bigdecimal_big_decimal_test(
        coefficient_bytes: Option<&[u8]>,
        coefficient: i64,
        scale: i32,
        expected: &str,
    ) {
        let value = super::bigdecimal_big_decimal(
            coefficient_bytes.map(|slice| BytesMut::from(slice)),
            coefficient,
            scale,
        )
        .unwrap();
        let expected = bigdecimal::BigDecimal::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_rust_decimal")]
    #[test]
    fn rust_decimal_decimal() {
        rust_decimal_decimal_test(Some(&[0]), 0, 0, "0");
        rust_decimal_decimal_test(Some(&[0]), 0, 1, "0");
        rust_decimal_decimal_test(Some(&[0]), 0, -1, "0");
        rust_decimal_decimal_test(Some(&[0x04, 0xd2]), 0, 0, "1234.0");
        rust_decimal_decimal_test(Some(&[0x04, 0xd2]), 0, 1, "123.4");
        rust_decimal_decimal_test(Some(&[0x04, 0xd2]), 0, -1, "12340");
        rust_decimal_decimal_test(Some(&[0xfb, 0x2e]), 0, 0, "-1234.0");
        rust_decimal_decimal_test(Some(&[0xfb, 0x2e]), 0, 1, "-123.4");
        rust_decimal_decimal_test(Some(&[0xfb, 0x2e]), 0, -1, "-12340");
        rust_decimal_decimal_test(None, 0, 0, "0");
        rust_decimal_decimal_test(None, 0, 1, "0");
        rust_decimal_decimal_test(None, 0, -1, "0");
        rust_decimal_decimal_test(None, 1234, 0, "1234.0");
        rust_decimal_decimal_test(None, 1234, 1, "123.4");
        rust_decimal_decimal_test(None, 1234, -1, "12340");
        rust_decimal_decimal_test(None, -1234, 0, "-1234.0");
        rust_decimal_decimal_test(None, -1234, 1, "-123.4");
        rust_decimal_decimal_test(None, -1234, -1, "-12340");
    }

    #[cfg(feature = "with_rust_decimal")]
    fn rust_decimal_decimal_test(
        coefficient_bytes: Option<&[u8]>,
        coefficient: i64,
        scale: i32,
        expected: &str,
    ) {
        let value = super::rust_decimal_decimal(
            coefficient_bytes.map(|slice| BytesMut::from(slice)),
            coefficient,
            scale,
        )
        .unwrap();
        let expected = rust_decimal::Decimal::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_date() {
        chrono_naive_date_test(0, "1970-01-01");
        chrono_naive_date_test(-1, "1969-12-31");
        chrono_naive_date_test(1, "1970-01-02");
        chrono_naive_date_test(-719528, "0000-01-01");
        chrono_naive_date_test(-719162, "0001-01-01");
        chrono_naive_date_test(-719893, "-0001-01-01");
        chrono_naive_date_test(2932896, "9999-12-31");
        chrono_naive_date_test(20108, "2025-01-20");
    }

    #[cfg(feature = "with_chrono")]
    fn chrono_naive_date_test(value: i64, expected: &str) {
        let value = super::chrono_naive_date(value).unwrap();
        let expected = chrono::NaiveDate::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_time() {
        chrono_naive_time_test(0, "00:00:00");
        chrono_naive_time_test(1, "00:00:00.000000001");
        chrono_naive_time_test(123456789, "00:00:00.123456789");
        chrono_naive_time_test(
            ((23 * 60 + 59) * 60 + 59) * 1_000_000_000 + 999_999_999,
            "23:59:59.999999999",
        );
        chrono_naive_time_test(
            ((14 * 60 + 35) * 60 + 30) * 1_000_000_000 + 123_000_000,
            "14:35:30.123",
        );
    }

    #[cfg(feature = "with_chrono")]
    fn chrono_naive_time_test(value: u64, expected: &str) {
        let value = super::chrono_naive_time(value).unwrap();
        let expected = chrono::NaiveTime::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_date_time() {
        chrono_naive_date_time_test(0, 0, "1970-01-01T00:00:00");
        chrono_naive_date_time_test(0, 123456789, "1970-01-01T00:00:00.123456789");
        chrono_naive_date_time_test(1, 123_000_000, "1970-01-01T00:00:01.123");
        chrono_naive_date_time_test(-1, 123_000_000, "1969-12-31T23:59:59.123");
        chrono_naive_date_time_test(-719528 * 24 * 60 * 60, 0, "0000-01-01T00:00:00");
        chrono_naive_date_time_test(-719162 * 24 * 60 * 60, 0, "0001-01-01T00:00:00");
        chrono_naive_date_time_test(-719893 * 24 * 60 * 60, 0, "-0001-01-01T00:00:00");
        chrono_naive_date_time_test(
            2932896 * 24 * 60 * 60 + (23 * 60 + 59) * 60 + 59,
            999_999_999,
            "9999-12-31T23:59:59.999999999",
        );
        chrono_naive_date_time_test(
            20108 * 24 * 60 * 60 + (14 * 60 + 57) * 60 + 30,
            123456789,
            "2025-01-20T14:57:30.123456789",
        );
    }

    #[cfg(feature = "with_chrono")]
    fn chrono_naive_date_time_test(epoch_seconds: i64, nanos: u32, expected: &str) {
        let value = super::chrono_naive_date_time(epoch_seconds, nanos).unwrap();
        let expected = chrono::NaiveDateTime::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_time_with_offset() {
        chrono_naive_time_with_offset_test(0, 0, "00:00:00", "+00:00");
        chrono_naive_time_with_offset_test(0, 9 * 60, "00:00:00", "+09:00");
        chrono_naive_time_with_offset_test(0, -9 * 60, "00:00:00", "-09:00");
        chrono_naive_time_with_offset_test(1, 0, "00:00:00.000000001", "+00:00");
        chrono_naive_time_with_offset_test(123456789, 0, "00:00:00.123456789", "+00:00");
        chrono_naive_time_with_offset_test(
            ((23 * 60 + 59) * 60 + 59) * 1_000_000_000 + 999_999_999,
            0,
            "23:59:59.999999999",
            "+00:00",
        );
        chrono_naive_time_with_offset_test(
            ((14 * 60 + 35) * 60 + 30) * 1_000_000_000 + 123_000_000,
            9 * 60,
            "14:35:30.123",
            "+09:00",
        );
    }

    #[cfg(feature = "with_chrono")]
    fn chrono_naive_time_with_offset_test(
        value: u64,
        offset_minutes: i32,
        expected: &str,
        expected_offset: &str,
    ) {
        let (value, offset) = super::chrono_naive_time_with_offset(value, offset_minutes).unwrap();
        let expected = chrono::NaiveTime::from_str(expected).unwrap();
        assert_eq!(expected, value);
        let expected = chrono::FixedOffset::from_str(expected_offset).unwrap();
        assert_eq!(expected, offset);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_date_time() {
        chrono_date_time_test(0, 0, 0, "1970-01-01T00:00:00+00:00");
        chrono_date_time_test(0, 0, 9 * 60, "1970-01-01T00:00:00+09:00");
        chrono_date_time_test(0, 0, -9 * 60, "1970-01-01T00:00:00-09:00");
        chrono_date_time_test(0, 123456789, 0, "1970-01-01T00:00:00.123456789+00:00");
        chrono_date_time_test(1, 123_000_000, 0, "1970-01-01T00:00:01.123+00:00");
        chrono_date_time_test(1, 123_000_000, 9 * 60, "1970-01-01T00:00:01.123+09:00");
        chrono_date_time_test(1, 123_000_000, -9 * 60, "1970-01-01T00:00:01.123-09:00");
        chrono_date_time_test(-1, 123_000_000, 0, "1969-12-31T23:59:59.123+00:00");
        chrono_date_time_test(-1, 123_000_000, 9 * 60, "1969-12-31T23:59:59.123+09:00");
        chrono_date_time_test(-1, 123_000_000, -9 * 60, "1969-12-31T23:59:59.123-09:00");
        chrono_date_time_test(-719528 * 24 * 60 * 60, 0, 0, "0000-01-01T00:00:00+00:00");
        chrono_date_time_test(-719162 * 24 * 60 * 60, 0, 0, "0001-01-01T00:00:00+00:00");
        chrono_date_time_test(-719893 * 24 * 60 * 60, 0, 0, "-0001-01-01T00:00:00+00:00");
        chrono_date_time_test(
            2932896 * 24 * 60 * 60 + (23 * 60 + 59) * 60 + 59,
            999_999_999,
            0,
            "9999-12-31T23:59:59.999999999+00:00",
        );
        chrono_date_time_test(
            20108 * 24 * 60 * 60 + (14 * 60 + 57) * 60 + 30,
            123456789,
            9 * 60,
            "2025-01-20T14:57:30.123456789+09:00",
        );
    }

    #[cfg(feature = "with_chrono")]
    fn chrono_date_time_test(epoch_seconds: i64, nanos: u32, offset_minutes: i32, expected: &str) {
        let value = super::chrono_date_time(epoch_seconds, nanos, offset_minutes).unwrap();
        let expected = chrono::DateTime::<chrono::FixedOffset>::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_date() {
        time_date_test(0, "1970-01-01");
        time_date_test(-1, "1969-12-31");
        time_date_test(1, "1970-01-02");
        time_date_test(-719528, "0000-01-01");
        time_date_test(-719162, "0001-01-01");
        time_date_test(-719893, "-0001-01-01");
        time_date_test(2932896, "9999-12-31");
        time_date_test(20108, "2025-01-20");
    }

    #[cfg(feature = "with_time")]
    fn time_date_test(value: i64, expected: &str) {
        let value = super::time_date(value).unwrap();

        let mut s = TimeString::new(expected);
        let year = s.next_year();
        let month = s.next_month();
        let day = s.next_day();
        let expected = time::Date::from_calendar_date(year, month, day).unwrap();
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_time() {
        time_time_test(0, "00:00:00");
        time_time_test(1, "00:00:00.000000001");
        time_time_test(123456789, "00:00:00.123456789");
        time_time_test(
            ((23 * 60 + 59) * 60 + 59) * 1_000_000_000 + 999_999_999,
            "23:59:59.999999999",
        );
        time_time_test(
            ((14 * 60 + 35) * 60 + 30) * 1_000_000_000 + 123_000_000,
            "14:35:30.123",
        );
    }

    #[cfg(feature = "with_time")]
    fn time_time_test(value: u64, expected: &str) {
        let value = super::time_time(value).unwrap();

        let mut s = TimeString::new(expected);
        let hour = s.next_hour();
        let min = s.next_min();
        let sec = s.next_sec();
        let nanos = s.next_nanos();
        let expected = time::Time::from_hms_nano(hour, min, sec, nanos).unwrap();
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_primitive_date_time() {
        time_primitive_date_time_test(0, 0, "1970-01-01T00:00:00");
        time_primitive_date_time_test(0, 123456789, "1970-01-01T00:00:00.123456789");
        time_primitive_date_time_test(1, 123_000_000, "1970-01-01T00:00:01.123");
        time_primitive_date_time_test(-1, 123_000_000, "1969-12-31T23:59:59.123");
        time_primitive_date_time_test(-719528 * 24 * 60 * 60, 0, "0000-01-01T00:00:00");
        time_primitive_date_time_test(-719162 * 24 * 60 * 60, 0, "0001-01-01T00:00:00");
        time_primitive_date_time_test(-719893 * 24 * 60 * 60, 0, "-0001-01-01T00:00:00");
        time_primitive_date_time_test(
            2932896 * 24 * 60 * 60 + (23 * 60 + 59) * 60 + 59,
            999_999_999,
            "9999-12-31T23:59:59.999999999",
        );
        time_primitive_date_time_test(
            20108 * 24 * 60 * 60 + (14 * 60 + 57) * 60 + 30,
            123456789,
            "2025-01-20T14:57:30.123456789",
        );
    }

    #[cfg(feature = "with_time")]
    fn time_primitive_date_time_test(epoch_seconds: i64, nanos: u32, expected: &str) {
        let value = super::time_primitive_date_time(epoch_seconds, nanos).unwrap();

        let mut s = TimeString::new(expected);
        let year = s.next_year();
        let month = s.next_month();
        let day = s.next_day();
        s.skip(1); // T
        let hour = s.next_hour();
        let min = s.next_min();
        let sec = s.next_sec();
        let nanos = s.next_nanos();
        let expected = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(year, month, day).unwrap(),
            time::Time::from_hms_nano(hour, min, sec, nanos).unwrap(),
        );
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_time_with_offset() {
        time_time_with_offset_test(0, 0, "00:00:00", "+00:00");
        time_time_with_offset_test(0, 9 * 60, "00:00:00", "+09:00");
        time_time_with_offset_test(0, -9 * 60, "00:00:00", "-09:00");
        time_time_with_offset_test(1, 0, "00:00:00.000000001", "+00:00");
        time_time_with_offset_test(123456789, 0, "00:00:00.123456789", "+00:00");
        time_time_with_offset_test(
            ((23 * 60 + 59) * 60 + 59) * 1_000_000_000 + 999_999_999,
            0,
            "23:59:59.999999999",
            "+00:00",
        );
        time_time_with_offset_test(
            ((14 * 60 + 35) * 60 + 30) * 1_000_000_000 + 123_000_000,
            9 * 60,
            "14:35:30.123",
            "+09:00",
        );
    }

    #[cfg(feature = "with_time")]
    fn time_time_with_offset_test(
        value: u64,
        offset_minutes: i32,
        expected: &str,
        expected_offset: &str,
    ) {
        let (value, offset) = super::time_time_with_offset(value, offset_minutes).unwrap();

        let mut s = TimeString::new(expected);
        let hour = s.next_hour();
        let min = s.next_min();
        let sec = s.next_sec();
        let nanos = s.next_nanos();
        let expected = time::Time::from_hms_nano(hour, min, sec, nanos).unwrap();
        assert_eq!(expected, value);
        let mut s = TimeString::new(expected_offset);
        let offset_minutes = s.next_offset();
        let expected = time::UtcOffset::from_whole_seconds(offset_minutes * 60).unwrap();
        assert_eq!(expected, offset);
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_offset_date_time() {
        time_offset_date_time_test(0, 0, 0, "1970-01-01T00:00:00+00:00");
        time_offset_date_time_test(0, 0, 9 * 60, "1970-01-01T00:00:00+09:00");
        time_offset_date_time_test(0, 0, -9 * 60, "1970-01-01T00:00:00-09:00");
        time_offset_date_time_test(0, 123456789, 0, "1970-01-01T00:00:00.123456789+00:00");
        time_offset_date_time_test(1, 123_000_000, 0, "1970-01-01T00:00:01.123+00:00");
        time_offset_date_time_test(1, 123_000_000, 9 * 60, "1970-01-01T00:00:01.123+09:00");
        time_offset_date_time_test(1, 123_000_000, -9 * 60, "1970-01-01T00:00:01.123-09:00");
        time_offset_date_time_test(-1, 123_000_000, 0, "1969-12-31T23:59:59.123+00:00");
        time_offset_date_time_test(-1, 123_000_000, 9 * 60, "1969-12-31T23:59:59.123+09:00");
        time_offset_date_time_test(-1, 123_000_000, -9 * 60, "1969-12-31T23:59:59.123-09:00");
        time_offset_date_time_test(-719528 * 24 * 60 * 60, 0, 0, "0000-01-01T00:00:00+00:00");
        time_offset_date_time_test(-719162 * 24 * 60 * 60, 0, 0, "0001-01-01T00:00:00+00:00");
        time_offset_date_time_test(-719893 * 24 * 60 * 60, 0, 0, "-0001-01-01T00:00:00+00:00");
        time_offset_date_time_test(
            2932896 * 24 * 60 * 60 + (23 * 60 + 59) * 60 + 59,
            999_999_999,
            0,
            "9999-12-31T23:59:59.999999999+00:00",
        );
        time_offset_date_time_test(
            20108 * 24 * 60 * 60 + (14 * 60 + 57) * 60 + 30,
            123456789,
            9 * 60,
            "2025-01-20T14:57:30.123456789+09:00",
        );
    }

    #[cfg(feature = "with_time")]
    fn time_offset_date_time_test(
        epoch_seconds: i64,
        nanos: u32,
        offset_minutes: i32,
        expected: &str,
    ) {
        let value = super::time_offset_date_time(epoch_seconds, nanos, offset_minutes).unwrap();

        let mut s = TimeString::new(expected);
        let year = s.next_year();
        let month = s.next_month();
        let day = s.next_day();
        s.skip(1); // T
        let hour = s.next_hour();
        let min = s.next_min();
        let sec = s.next_sec();
        let nanos = s.next_nanos();
        let offset_minutes = s.next_offset();
        let expected = time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(year, month, day).unwrap(),
            time::Time::from_hms_nano(hour, min, sec, nanos).unwrap(),
            time::UtcOffset::from_whole_seconds(offset_minutes * 60).unwrap(),
        );
        assert_eq!(expected, value);
    }

    #[cfg(feature = "with_time")]
    struct TimeString {
        text: String,
        position: usize,
    }

    impl TimeString {
        fn new(s: &str) -> TimeString {
            TimeString {
                text: s.to_string(),
                position: 0,
            }
        }

        fn next_year(&mut self) -> i32 {
            if self.is_minus() {
                self.pop(5)
            } else {
                self.pop(4)
            }
        }

        fn next_month(&mut self) -> time::Month {
            self.skip(1); // -
            let month = self.pop(2) as u8;
            time::Month::try_from(month).unwrap()
        }

        fn next_day(&mut self) -> u8 {
            self.skip(1); // -
            self.pop(2) as u8
        }

        fn next_hour(&mut self) -> u8 {
            self.pop(2) as u8
        }

        fn next_min(&mut self) -> u8 {
            self.skip(1); // :
            self.pop(2) as u8
        }

        fn next_sec(&mut self) -> u8 {
            self.skip(1); // :
            self.pop(2) as u8
        }

        fn next_nanos(&mut self) -> u32 {
            if self.position >= self.text.len() {
                return 0;
            }
            if !self.is_period() {
                return 0;
            }
            self.skip(1); // .

            let mut value: u32 = 0;
            let mut factor = 100_000_000;
            while factor >= 1 {
                if self.position >= self.text.len() {
                    break;
                }

                let c = &self.text[self.position..self.position + 1];
                let result: Result<u32, _> = c.parse();
                let v = if let Ok(value) = result {
                    value
                } else {
                    break;
                };

                value += v * factor;
                self.position += 1;
                factor /= 10;
            }

            return value;
        }

        fn next_offset(&mut self) -> i32 {
            let hour = self.pop(3);
            self.skip(1); // :
            let min = self.pop(2);
            hour * 60 + min
        }

        fn is_minus(&self) -> bool {
            self.text.chars().nth(self.position) == Some('-')
        }

        fn is_period(&self) -> bool {
            self.text.chars().nth(self.position) == Some('.')
        }

        fn skip(&mut self, skip: usize) {
            self.position += skip;
        }

        fn pop(&mut self, len: usize) -> i32 {
            let start = self.position;
            self.position += len;
            let slice = &self.text[start..self.position];
            slice.parse().unwrap()
        }
    }
}
