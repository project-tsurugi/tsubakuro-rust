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
    session::wire::{response::WireResponse, response_box::SlotEntryHandle, Wire},
    util::Timeout,
};
use async_trait::async_trait;
use prost::{bytes::BytesMut, Message};
use std::{sync::Arc, time::Duration};

use super::value_stream::ResultSetValueStream;

/// Represents a server side SQL result set.
///
/// A `SqlQueryResult` instance can only be used while the transaction is alive.
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
///             let pk: i32 = query_result.fetch().await?; // not null
///             println!("pk={}", pk);
///         }
///         if query_result.next_column().await? {
///             let value: Option<String> = query_result.fetch().await?; // nullable
///             println!("value={:?}", value);
///         }
///     }
///
///     query_result.close().await?;
///
///     Ok(())
/// }
/// ```
pub struct SqlQueryResult {
    wire: Arc<Wire>,
    slot_handle: Option<Arc<SlotEntryHandle>>,
    name: String,
    metadata: Option<SqlQueryResultMetadata>,
    pub(crate) value_stream: ResultSetValueStream,
    pub(crate) default_timeout: Duration,
    close_timeout: Duration,
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
        wire: Arc<Wire>,
        slot_handle: Arc<SlotEntryHandle>,
        name: String,
        metadata: Option<SqlQueryResultMetadata>,
        value_stream: ResultSetValueStream,
        default_timeout: Duration,
    ) -> SqlQueryResult {
        SqlQueryResult {
            wire,
            slot_handle: Some(slot_handle),
            name,
            metadata,
            value_stream,
            default_timeout,
            close_timeout: default_timeout,
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
    ///             if query_result.is_null()? {
    ///                 continue;
    ///             }
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
    ///     query_result.close().await?;
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
    slot_handle: Arc<SlotEntryHandle>,
    response: WireResponse,
    default_timeout: Duration,
) -> Result<SqlQueryResult, TgError> {
    // const FUNCTION_NAME: &str = "query_result_processor()";

    let (dc_name, metadata) = read_result_set_metadata(response)?;

    let data_channel = wire.create_data_channel(&dc_name)?;
    let value_stream = ResultSetValueStream::new(data_channel);
    let query_result = SqlQueryResult::new(
        wire,
        slot_handle,
        dc_name,
        metadata,
        value_stream,
        default_timeout,
    );

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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<T, TgError>;

    /// Retrieves a value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<T, TgError>;
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<bool> for SqlQueryResult {
    /// Retrieves a `BOOLEAN` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<bool, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_boolean_value(&timeout).await
    }

    /// Retrieves a `BOOLEAN` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<i32, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_int4_value(&timeout).await
    }

    /// Retrieves a `INT4` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<i64, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_int8_value(&timeout).await
    }

    /// Retrieves a `INT8` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<f32, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_float4_value(&timeout).await
    }

    /// Retrieves a `FLOAT4` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<f64, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_float8_value(&timeout).await
    }

    /// Retrieves a `FLOAT8` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<TgDecimalResult, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<TgDecimalI128, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgDecimalI128, TgError> {
        let value: TgDecimalResult = self.fetch_for(timeout).await?;
        TgDecimalI128::try_from(value)
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<String> for SqlQueryResult {
    /// Retrieves a `CHARACTER` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<String, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_character_value(&timeout).await
    }

    /// Retrieves a `CHARACTER` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<BytesMut, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        self.value_stream.fetch_octet_value(&timeout).await
    }

    /// Retrieves a `OCTET` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<Vec<u8>, TgError> {
        let timeout = Timeout::new(self.default_timeout);
        let value = self.value_stream.fetch_octet_value(&timeout).await?;
        Ok(value.to_vec())
    }

    /// Retrieves a `OCTET` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<TgDate, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DATE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<TgTimeOfDay, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_OF_DAY` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<TgTimePoint, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<TgTimeOfDayWithTimeZone, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_OF_DAY_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
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
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch(&mut self) -> Result<TgTimePointWithTimeZone, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `TIME_POINT_WITH_TIME_ZONE` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<TgTimePointWithTimeZone, TgError> {
        let timeout = Timeout::new(timeout);
        let value = self
            .value_stream
            .fetch_time_point_with_time_zone_value(&timeout)
            .await?;
        Ok(TgTimePointWithTimeZone::new(value.0, value.1, value.2))
    }
}

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<TgBlobReference> for SqlQueryResult {
    /// Retrieves a `BLOB` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
    ///
    /// See [`SqlClient::open_blob`](crate::prelude::SqlClient::open_blob), [`copy_blob_to`](crate::prelude::SqlClient::copy_blob_to).
    async fn fetch(&mut self) -> Result<TgBlobReference, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `BLOB` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
    ///
    /// See [`SqlClient::open_blob`](crate::prelude::SqlClient::open_blob), [`copy_blob_to`](crate::prelude::SqlClient::copy_blob_to).
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
    ///
    /// This method can only be used while the transaction is alive.
    ///
    /// See [`SqlClient::open_clob`](crate::prelude::SqlClient::open_clob), [`copy_clob_to`](crate::prelude::SqlClient::copy_clob_to).
    async fn fetch(&mut self) -> Result<TgClobReference, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `CLOB` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    ///
    /// This method can only be used while the transaction is alive.
    ///
    /// See [`SqlClient::open_clob`](crate::prelude::SqlClient::open_clob), [`copy_clob_to`](crate::prelude::SqlClient::copy_clob_to).
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

impl SqlQueryResult {
    /// Set close timeout.
    ///
    /// since 0.3.0
    pub fn set_close_timeout(&mut self, timeout: Duration) {
        self.close_timeout = timeout;
    }

    /// Get close timeout.
    ///
    /// since 0.3.0
    pub fn close_timeout(&self) -> Duration {
        self.close_timeout
    }

    /// Disposes this resource.
    ///
    /// since 0.3.0
    pub async fn close(&mut self) -> Result<(), TgError> {
        self.close_for(self.close_timeout).await
    }

    /// Disposes this resource.
    ///
    /// since 0.3.0
    pub async fn close_for(&mut self, timeout: Duration) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "close()";

        let slot_handle = self.slot_handle.take();
        if let Some(slot_handle) = slot_handle {
            let timeout = Timeout::new(timeout);
            let response = self.wire.pull_response(&slot_handle, &timeout).await?;
            convert_sql_response(FUNCTION_NAME, &response)?;
        }
        Ok(())
    }

    /// Check if this resource is closed.
    ///
    /// since 0.3.0
    pub fn is_closed(&self) -> bool {
        self.slot_handle.is_none()
    }
}
