use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::ResultSetMetadata,
    prelude::convert_sql_response,
    session::wire::{response::WireResponse, Wire},
    util::Timeout,
};
use crate::{
    jogasaki::proto::sql::response::{
        response::Response as SqlResponseType, Response as SqlResponse,
    },
    prost_decode_error,
};
use async_trait::async_trait;
use bigdecimal::FromPrimitive;
use prost::{bytes::BytesMut, Message};
use std::{sync::Arc, time::Duration};
use value_stream::ResultSetValueStream;

pub mod metadata;
mod value_stream;
mod variant;

/// thread unsafe
pub struct SqlQueryResult {
    name: String,
    metadata: Option<ResultSetMetadata>,
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
        metadata: Option<ResultSetMetadata>,
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

    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    pub fn get_metadata(&self) -> Option<&ResultSetMetadata> {
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
) -> Result<(String, Option<ResultSetMetadata>), TgError> {
    const FUNCTION_NAME: &str = "read_result_set_metadata()";

    let _ = convert_sql_response(FUNCTION_NAME, &response)?;
    match response {
        WireResponse::ResponseSessionBodyhead(_slot, payload, error) => {
            if let Some(e) = error {
                return Err(e.to_tg_error());
            }
            let payload = payload.unwrap();
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
    /// After this operation, you need to invoke [`next_column`] to retrieve the first column data of the next row.
    pub async fn next_row(&mut self) -> Result<bool, TgError> {
        self.next_row_for(self.default_timeout).await
    }

    /// Advances the cursor to the head of the next row.
    ///
    /// If this operation was succeeded (returns `true`), this cursor points the head of the next row.
    /// After this operation, you need to invoke [`next_column`] to retrieve the first column data of the next row.
    pub async fn next_row_for(&mut self, timeout: Duration) -> Result<bool, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.next_row(&timeout).await
    }

    /// Advances the cursor to the next column in the current row.
    ///
    /// If this operation was succeeded (returns `true`), this cursor will point to the next column of the row.
    /// You can invoke [`fetch`] method to obtain the column value.
    pub async fn next_column(&mut self) -> Result<bool, TgError> {
        self.next_column_for(self.default_timeout).await
    }

    /// Advances the cursor to the next column in the current row.
    ///
    /// If this operation was succeeded (returns `true`), this cursor will point to the next column of the row.
    /// You can invoke [`fetch`] method to obtain the column value.
    pub async fn next_column_for(&mut self, timeout: Duration) -> Result<bool, TgError> {
        let timeout = Timeout::new(timeout);
        self.value_stream.next_column(&timeout).await
    }

    /// Returns whether or not the column on this cursor is `NULL`.
    pub fn is_null(&mut self) -> Result<bool, TgError> {
        self.value_stream.is_null()
    }
}

#[async_trait(?Send)] // thread unsafe
pub trait SqlQueryResultFetch<T> {
    async fn fetch(&mut self) -> Result<T, TgError>;
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
        let value = self.value_stream.fetch_decimal_value(&timeout).await?;
        let value = match value {
            (Some(coefficient), _, scale) => {
                let value = bigdecimal::num_bigint::BigInt::from_signed_bytes_be(&coefficient);
                bigdecimal::BigDecimal::new(value, scale as i64)
            }
            (None, value, scale) => {
                if scale == 0 {
                    bigdecimal::BigDecimal::from_i64(value).unwrap()
                } else {
                    let value = bigdecimal::num_bigint::BigInt::from_i64(value).unwrap();
                    bigdecimal::BigDecimal::from_bigint(value, scale as i64)
                }
            }
        };
        Ok(value)
    }
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
        let value = self.value_stream.fetch_decimal_value(&timeout).await?;
        let value = match value {
            (Some(coefficient), _, scale) => {
                let top = coefficient[0] as i8;
                let mut buf = if top >= 0 { [0u8; 16] } else { [0xffu8; 16] };
                buf[16 - coefficient.len()..].copy_from_slice(&coefficient);
                let value = i128::from_be_bytes(buf);
                rust_decimal::Decimal::from_i128_with_scale(value, scale as u32)
            }
            (None, value, scale) => {
                rust_decimal::Decimal::from_i128_with_scale(value as i128, scale as u32)
            }
        };
        Ok(value)
    }
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
        let days = value + 719_163;
        let value = chrono::NaiveDate::from_num_days_from_ce_opt(days as i32).unwrap();
        Ok(value)
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
        let seconds = (value / 1_000_000_000) as u32;
        let nanos = (value % 1_000_000_000) as u32;
        let value = chrono::NaiveTime::from_num_seconds_from_midnight_opt(seconds, nanos).unwrap();
        Ok(value)
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
        let value = chrono::DateTime::from_timestamp(epoch_seconds, nanos as u32).unwrap();
        let value = value.naive_utc();
        Ok(value)
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

        let seconds = (time / 1_000_000_000) as u32;
        let nanos = (time % 1_000_000_000) as u32;
        let value = chrono::NaiveTime::from_num_seconds_from_midnight_opt(seconds, nanos).unwrap();

        let offset_seconds = offset_minutes * 60;
        let offset = if offset_minutes >= 0 {
            chrono::FixedOffset::east_opt(offset_seconds)
        } else {
            chrono::FixedOffset::west_opt(-offset_seconds)
        }
        .unwrap();

        Ok((value, offset))
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
        let value = chrono::DateTime::from_timestamp(epoch_seconds, nanos as u32).unwrap();
        let offset_seconds = offset_minutes * 60;
        let offset = if offset_minutes >= 0 {
            chrono::FixedOffset::east_opt(offset_seconds)
        } else {
            chrono::FixedOffset::west_opt(-offset_seconds)
        }
        .unwrap();
        let value = value.with_timezone(&offset);
        Ok(value)
    }
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
        let days = value + /* Date(1970-01-01).to_julian_day() */ 2440588;
        let value = time::Date::from_julian_day(days as i32).unwrap();
        Ok(value)
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

        let nanosecond = value % 1000_000_000;
        let value = value / 1000_000_000;
        let second = value % 60;
        let value = value / 60;
        let minute = value % 60;
        let hour = value / 60;

        let value =
            time::Time::from_hms_nano(hour as u8, minute as u8, second as u8, nanosecond as u32)
                .unwrap();
        Ok(value)
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

        const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
        let mut days =
            epoch_seconds / SECONDS_PER_DAY + /* Date(1970-01-01).to_julian_day() */ 2440588;
        let mut value = epoch_seconds % SECONDS_PER_DAY;
        if value < 0 {
            value += SECONDS_PER_DAY;
            days -= 1;
        }
        let second = value % 60;
        let value = value / 60;
        let minute = value % 60;
        let value = value / 60;
        let hour = value % 24;

        let value = time::PrimitiveDateTime::new(
            time::Date::from_julian_day(days as i32).unwrap(),
            time::Time::from_hms_nano(hour as u8, minute as u8, second as u8, nanos as u32)
                .unwrap(),
        );
        Ok(value)
    }
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

        let nanosecond = time % 1000_000_000;
        let value = time / 1000_000_000;
        let second = value % 60;
        let value = value / 60;
        let minute = value % 60;
        let hour = value / 60;
        let value =
            time::Time::from_hms_nano(hour as u8, minute as u8, second as u8, nanosecond as u32)
                .unwrap();

        let offset_seconds = offset_minutes * 60;
        let offset = time::UtcOffset::from_whole_seconds(offset_seconds).unwrap();

        Ok((value, offset))
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

        const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
        let mut days =
            epoch_seconds / SECONDS_PER_DAY + /* Date(1970-01-01).to_julian_day() */ 2440588;
        let mut value = epoch_seconds % SECONDS_PER_DAY;
        if value < 0 {
            value += SECONDS_PER_DAY;
            days -= 1;
        }
        let second = value % 60;
        let value = value / 60;
        let minute = value % 60;
        let value = value / 60;
        let hour = value % 24;

        let offset_seconds = offset_minutes * 60;
        let offset = time::UtcOffset::from_whole_seconds(offset_seconds).unwrap();

        let value = time::OffsetDateTime::new_in_offset(
            time::Date::from_julian_day(days as i32).unwrap(),
            time::Time::from_hms_nano(hour as u8, minute as u8, second as u8, nanos as u32)
                .unwrap(),
            offset,
        );
        Ok(value)
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
