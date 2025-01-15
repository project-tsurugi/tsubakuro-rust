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
use prost::Message;
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
    /// Retrieves a `BigDecimal` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<bigdecimal::BigDecimal, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `BigDecimal` value on the column of the cursor position.
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
    /// Retrieves a `Decimal` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<rust_decimal::Decimal, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `Decimal` value on the column of the cursor position.
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
