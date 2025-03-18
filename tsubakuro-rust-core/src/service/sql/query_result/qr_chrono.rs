use crate::{client_error, error::TgError, util::Timeout};
use async_trait::async_trait;
use log::trace;
use std::time::Duration;

use super::{SqlQueryResult, SqlQueryResultFetch};

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
        naive_time(value)
    }
}

fn naive_time(value: u64) -> Result<chrono::NaiveTime, TgError> {
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
        naive_date_time(epoch_seconds, nanos)
    }
}

fn naive_date_time(epoch_seconds: i64, nanos: u32) -> Result<chrono::NaiveDateTime, TgError> {
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
        naive_time_with_offset(time, offset_minutes)
    }
}

fn naive_time_with_offset(
    time: u64,
    offset_minutes: i32,
) -> Result<(chrono::NaiveTime, chrono::FixedOffset), TgError> {
    let value = naive_time(time)?;
    let offset = fixed_offset(offset_minutes)?;

    Ok((value, offset))
}

fn fixed_offset(offset_minutes: i32) -> Result<chrono::FixedOffset, TgError> {
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
        date_time(epoch_seconds, nanos, offset_minutes)
    }
}

fn date_time(
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

    let offset = fixed_offset(offset_minutes)?;

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

#[cfg(test)]
mod test {
    use std::str::FromStr;

    #[test]
    fn naive_date() {
        naive_date_test(0, "1970-01-01");
        naive_date_test(-1, "1969-12-31");
        naive_date_test(1, "1970-01-02");
        naive_date_test(-719528, "0000-01-01");
        naive_date_test(-719162, "0001-01-01");
        naive_date_test(-719893, "-0001-01-01");
        naive_date_test(2932896, "9999-12-31");
        naive_date_test(20108, "2025-01-20");
    }

    fn naive_date_test(value: i64, expected: &str) {
        let value = super::chrono_naive_date(value).unwrap();
        let expected = chrono::NaiveDate::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[test]
    fn naive_time() {
        naive_time_test(0, "00:00:00");
        naive_time_test(1, "00:00:00.000000001");
        naive_time_test(123456789, "00:00:00.123456789");
        naive_time_test(
            ((23 * 60 + 59) * 60 + 59) * 1_000_000_000 + 999_999_999,
            "23:59:59.999999999",
        );
        naive_time_test(
            ((14 * 60 + 35) * 60 + 30) * 1_000_000_000 + 123_000_000,
            "14:35:30.123",
        );
    }

    fn naive_time_test(value: u64, expected: &str) {
        let value = super::naive_time(value).unwrap();
        let expected = chrono::NaiveTime::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[test]
    fn naive_date_time() {
        naive_date_time_test(0, 0, "1970-01-01T00:00:00");
        naive_date_time_test(0, 123456789, "1970-01-01T00:00:00.123456789");
        naive_date_time_test(1, 123_000_000, "1970-01-01T00:00:01.123");
        naive_date_time_test(-1, 123_000_000, "1969-12-31T23:59:59.123");
        naive_date_time_test(-719528 * 24 * 60 * 60, 0, "0000-01-01T00:00:00");
        naive_date_time_test(-719162 * 24 * 60 * 60, 0, "0001-01-01T00:00:00");
        naive_date_time_test(-719893 * 24 * 60 * 60, 0, "-0001-01-01T00:00:00");
        naive_date_time_test(
            2932896 * 24 * 60 * 60 + (23 * 60 + 59) * 60 + 59,
            999_999_999,
            "9999-12-31T23:59:59.999999999",
        );
        naive_date_time_test(
            20108 * 24 * 60 * 60 + (14 * 60 + 57) * 60 + 30,
            123456789,
            "2025-01-20T14:57:30.123456789",
        );
    }

    fn naive_date_time_test(epoch_seconds: i64, nanos: u32, expected: &str) {
        let value = super::naive_date_time(epoch_seconds, nanos).unwrap();
        let expected = chrono::NaiveDateTime::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }

    #[test]
    fn naive_time_with_offset() {
        naive_time_with_offset_test(0, 0, "00:00:00", "+00:00");
        naive_time_with_offset_test(0, 9 * 60, "00:00:00", "+09:00");
        naive_time_with_offset_test(0, -9 * 60, "00:00:00", "-09:00");
        naive_time_with_offset_test(1, 0, "00:00:00.000000001", "+00:00");
        naive_time_with_offset_test(123456789, 0, "00:00:00.123456789", "+00:00");
        naive_time_with_offset_test(
            ((23 * 60 + 59) * 60 + 59) * 1_000_000_000 + 999_999_999,
            0,
            "23:59:59.999999999",
            "+00:00",
        );
        naive_time_with_offset_test(
            ((14 * 60 + 35) * 60 + 30) * 1_000_000_000 + 123_000_000,
            9 * 60,
            "14:35:30.123",
            "+09:00",
        );
    }

    fn naive_time_with_offset_test(
        value: u64,
        offset_minutes: i32,
        expected: &str,
        expected_offset: &str,
    ) {
        let (value, offset) = super::naive_time_with_offset(value, offset_minutes).unwrap();
        let expected = chrono::NaiveTime::from_str(expected).unwrap();
        assert_eq!(expected, value);
        let expected = chrono::FixedOffset::from_str(expected_offset).unwrap();
        assert_eq!(expected, offset);
    }

    #[test]
    fn date_time() {
        date_time_test(0, 0, 0, "1970-01-01T00:00:00+00:00");
        date_time_test(0, 0, 9 * 60, "1970-01-01T00:00:00+09:00");
        date_time_test(0, 0, -9 * 60, "1970-01-01T00:00:00-09:00");
        date_time_test(0, 123456789, 0, "1970-01-01T00:00:00.123456789+00:00");
        date_time_test(1, 123_000_000, 0, "1970-01-01T00:00:01.123+00:00");
        date_time_test(1, 123_000_000, 9 * 60, "1970-01-01T00:00:01.123+09:00");
        date_time_test(1, 123_000_000, -9 * 60, "1970-01-01T00:00:01.123-09:00");
        date_time_test(-1, 123_000_000, 0, "1969-12-31T23:59:59.123+00:00");
        date_time_test(-1, 123_000_000, 9 * 60, "1969-12-31T23:59:59.123+09:00");
        date_time_test(-1, 123_000_000, -9 * 60, "1969-12-31T23:59:59.123-09:00");
        date_time_test(-719528 * 24 * 60 * 60, 0, 0, "0000-01-01T00:00:00+00:00");
        date_time_test(-719162 * 24 * 60 * 60, 0, 0, "0001-01-01T00:00:00+00:00");
        date_time_test(-719893 * 24 * 60 * 60, 0, 0, "-0001-01-01T00:00:00+00:00");
        date_time_test(
            2932896 * 24 * 60 * 60 + (23 * 60 + 59) * 60 + 59,
            999_999_999,
            0,
            "9999-12-31T23:59:59.999999999+00:00",
        );
        date_time_test(
            20108 * 24 * 60 * 60 + (14 * 60 + 57) * 60 + 30,
            123456789,
            9 * 60,
            "2025-01-20T14:57:30.123456789+09:00",
        );
    }

    fn date_time_test(epoch_seconds: i64, nanos: u32, offset_minutes: i32, expected: &str) {
        let value = super::date_time(epoch_seconds, nanos, offset_minutes).unwrap();
        let expected = chrono::DateTime::<chrono::FixedOffset>::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }
}
