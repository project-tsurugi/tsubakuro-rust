use crate::{client_error, error::TgError, util::Timeout};
use async_trait::async_trait;
use log::trace;
use std::time::Duration;

use super::{SqlQueryResult, SqlQueryResultFetch};

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
        date(value)
    }
}

fn date(value: i64) -> Result<time::Date, TgError> {
    let days = value + /* Date(1970-01-01).to_julian_day() */ 2440588;
    match time::Date::from_julian_day(days as i32) {
        Ok(value) => Ok(value),
        Err(e) => {
            trace!("time::Date::from_julian_day() error. days={}", days);
            Err(client_error!("time::Date generate error", e))
        }
    }
}

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
        time(value)
    }
}

fn time(value: u64) -> Result<time::Time, TgError> {
    let nanos = value % 1_000_000_000;
    let value = value / 1_000_000_000;
    time_nanos(value, nanos as u32)
}

fn time_nanos(seconds: u64, nanos: u32) -> Result<time::Time, TgError> {
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
        primitive_date_time(epoch_seconds, nanos)
    }
}

fn primitive_date_time(epoch_seconds: i64, nanos: u32) -> Result<time::PrimitiveDateTime, TgError> {
    const SECONDS_PER_DAY: i64 = 24 * 60 * 60;
    let mut days = epoch_seconds / SECONDS_PER_DAY;
    let mut value = epoch_seconds % SECONDS_PER_DAY;
    if value < 0 {
        value += SECONDS_PER_DAY;
        days -= 1;
    }

    let date = date(days)?;
    let time = time_nanos(value as u64, nanos)?;
    let value = time::PrimitiveDateTime::new(date, time);
    Ok(value)
}

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
        time_with_offset(time, offset_minutes)
    }
}

fn time_with_offset(
    time_arg: u64,
    offset_minutes: i32,
) -> Result<(time::Time, time::UtcOffset), TgError> {
    let value = time(time_arg)?;
    let offset = utc_offset(offset_minutes)?;

    Ok((value, offset))
}

fn utc_offset(offset_minutes: i32) -> Result<time::UtcOffset, TgError> {
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
        offset_date_time(epoch_seconds, nanos, offset_minutes)
    }
}

fn offset_date_time(
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

    let date = date(days)?;
    let time = time_nanos(value as u64, nanos)?;
    let offset = utc_offset(offset_minutes)?;
    let value = time::OffsetDateTime::new_in_offset(date, time, offset);
    Ok(value)
}

#[cfg(test)]
mod test {

    #[test]
    fn date() {
        date_test(0, "1970-01-01");
        date_test(-1, "1969-12-31");
        date_test(1, "1970-01-02");
        date_test(-719528, "0000-01-01");
        date_test(-719162, "0001-01-01");
        date_test(-719893, "-0001-01-01");
        date_test(2932896, "9999-12-31");
        date_test(20108, "2025-01-20");
    }

    fn date_test(value: i64, expected: &str) {
        let value = super::date(value).unwrap();

        let mut s = TimeString::new(expected);
        let year = s.next_year();
        let month = s.next_month();
        let day = s.next_day();
        let expected = time::Date::from_calendar_date(year, month, day).unwrap();
        assert_eq!(expected, value);
    }

    #[test]
    fn time() {
        time_test(0, "00:00:00");
        time_test(1, "00:00:00.000000001");
        time_test(123456789, "00:00:00.123456789");
        time_test(
            ((23 * 60 + 59) * 60 + 59) * 1_000_000_000 + 999_999_999,
            "23:59:59.999999999",
        );
        time_test(
            ((14 * 60 + 35) * 60 + 30) * 1_000_000_000 + 123_000_000,
            "14:35:30.123",
        );
    }

    fn time_test(value: u64, expected: &str) {
        let value = super::time(value).unwrap();

        let mut s = TimeString::new(expected);
        let hour = s.next_hour();
        let min = s.next_min();
        let sec = s.next_sec();
        let nanos = s.next_nanos();
        let expected = time::Time::from_hms_nano(hour, min, sec, nanos).unwrap();
        assert_eq!(expected, value);
    }

    #[test]
    fn primitive_date_time() {
        primitive_date_time_test(0, 0, "1970-01-01T00:00:00");
        primitive_date_time_test(0, 123456789, "1970-01-01T00:00:00.123456789");
        primitive_date_time_test(1, 123_000_000, "1970-01-01T00:00:01.123");
        primitive_date_time_test(-1, 123_000_000, "1969-12-31T23:59:59.123");
        primitive_date_time_test(-719528 * 24 * 60 * 60, 0, "0000-01-01T00:00:00");
        primitive_date_time_test(-719162 * 24 * 60 * 60, 0, "0001-01-01T00:00:00");
        primitive_date_time_test(-719893 * 24 * 60 * 60, 0, "-0001-01-01T00:00:00");
        primitive_date_time_test(
            2932896 * 24 * 60 * 60 + (23 * 60 + 59) * 60 + 59,
            999_999_999,
            "9999-12-31T23:59:59.999999999",
        );
        primitive_date_time_test(
            20108 * 24 * 60 * 60 + (14 * 60 + 57) * 60 + 30,
            123456789,
            "2025-01-20T14:57:30.123456789",
        );
    }

    fn primitive_date_time_test(epoch_seconds: i64, nanos: u32, expected: &str) {
        let value = super::primitive_date_time(epoch_seconds, nanos).unwrap();

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

    #[test]
    fn time_with_offset() {
        time_with_offset_test(0, 0, "00:00:00", "+00:00");
        time_with_offset_test(0, 9 * 60, "00:00:00", "+09:00");
        time_with_offset_test(0, -9 * 60, "00:00:00", "-09:00");
        time_with_offset_test(1, 0, "00:00:00.000000001", "+00:00");
        time_with_offset_test(123456789, 0, "00:00:00.123456789", "+00:00");
        time_with_offset_test(
            ((23 * 60 + 59) * 60 + 59) * 1_000_000_000 + 999_999_999,
            0,
            "23:59:59.999999999",
            "+00:00",
        );
        time_with_offset_test(
            ((14 * 60 + 35) * 60 + 30) * 1_000_000_000 + 123_000_000,
            9 * 60,
            "14:35:30.123",
            "+09:00",
        );
    }

    fn time_with_offset_test(
        value: u64,
        offset_minutes: i32,
        expected: &str,
        expected_offset: &str,
    ) {
        let (value, offset) = super::time_with_offset(value, offset_minutes).unwrap();

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

    #[test]
    fn offset_date_time() {
        offset_date_time_test(0, 0, 0, "1970-01-01T00:00:00+00:00");
        offset_date_time_test(0, 0, 9 * 60, "1970-01-01T00:00:00+09:00");
        offset_date_time_test(0, 0, -9 * 60, "1970-01-01T00:00:00-09:00");
        offset_date_time_test(0, 123456789, 0, "1970-01-01T00:00:00.123456789+00:00");
        offset_date_time_test(1, 123_000_000, 0, "1970-01-01T00:00:01.123+00:00");
        offset_date_time_test(1, 123_000_000, 9 * 60, "1970-01-01T00:00:01.123+09:00");
        offset_date_time_test(1, 123_000_000, -9 * 60, "1970-01-01T00:00:01.123-09:00");
        offset_date_time_test(-1, 123_000_000, 0, "1969-12-31T23:59:59.123+00:00");
        offset_date_time_test(-1, 123_000_000, 9 * 60, "1969-12-31T23:59:59.123+09:00");
        offset_date_time_test(-1, 123_000_000, -9 * 60, "1969-12-31T23:59:59.123-09:00");
        offset_date_time_test(-719528 * 24 * 60 * 60, 0, 0, "0000-01-01T00:00:00+00:00");
        offset_date_time_test(-719162 * 24 * 60 * 60, 0, 0, "0001-01-01T00:00:00+00:00");
        offset_date_time_test(-719893 * 24 * 60 * 60, 0, 0, "-0001-01-01T00:00:00+00:00");
        offset_date_time_test(
            2932896 * 24 * 60 * 60 + (23 * 60 + 59) * 60 + 59,
            999_999_999,
            0,
            "9999-12-31T23:59:59.999999999+00:00",
        );
        offset_date_time_test(
            20108 * 24 * 60 * 60 + (14 * 60 + 57) * 60 + 30,
            123456789,
            9 * 60,
            "2025-01-20T14:57:30.123456789+09:00",
        );
    }

    fn offset_date_time_test(epoch_seconds: i64, nanos: u32, offset_minutes: i32, expected: &str) {
        let value = super::offset_date_time(epoch_seconds, nanos, offset_minutes).unwrap();

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
