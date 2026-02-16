use crate::prelude::{
    TgDate, TgTimeOfDay, TgTimeOfDayWithTimeZone, TgTimePoint, TgTimePointWithTimeZone,
};
use chrono::{Datelike, Offset, TimeZone};

impl From<chrono::NaiveDate> for TgDate {
    fn from(value: chrono::NaiveDate) -> Self {
        Self::from(&value)
    }
}

impl From<&chrono::NaiveDate> for TgDate {
    fn from(value: &chrono::NaiveDate) -> Self {
        let epoch_days = naive_date_to_epoch_days(value);
        TgDate { epoch_days }
    }
}

impl From<TgDate> for chrono::NaiveDate {
    fn from(value: TgDate) -> Self {
        Self::from(&value)
    }
}

impl From<&TgDate> for chrono::NaiveDate {
    fn from(value: &TgDate) -> Self {
        epoch_days_to_naive_date(value.epoch_days)
    }
}

pub(crate) fn naive_date_to_epoch_days(value: &chrono::NaiveDate) -> i64 {
    value.num_days_from_ce() as i64 - /* NaiveDate(1970-01-01).num_days_from_ce() */ 719_163
}

pub(crate) fn epoch_days_to_naive_date(epoch_days: i64) -> chrono::NaiveDate {
    let days = epoch_days + /* NaiveDate(1970-01-01).num_days_from_ce() */ 719_163;
    chrono::NaiveDate::from_num_days_from_ce_opt(days as i32).unwrap()
}

impl From<chrono::NaiveTime> for TgTimeOfDay {
    fn from(value: chrono::NaiveTime) -> Self {
        Self::from(&value)
    }
}

impl From<&chrono::NaiveTime> for TgTimeOfDay {
    fn from(value: &chrono::NaiveTime) -> Self {
        let (seconds, nanos) = naive_time_to_seconds(value);
        TgTimeOfDay {
            offset_nanoseconds: seconds * 1_000_000_000 + nanos as u64,
        }
    }
}

impl From<TgTimeOfDay> for chrono::NaiveTime {
    fn from(value: TgTimeOfDay) -> Self {
        Self::from(&value)
    }
}

impl From<&TgTimeOfDay> for chrono::NaiveTime {
    fn from(value: &TgTimeOfDay) -> Self {
        let total_nanos = value.offset_nanoseconds;
        let seconds = total_nanos / 1_000_000_000;
        let nanos = (total_nanos % 1_000_000_000) as u32;

        chrono::NaiveTime::from_num_seconds_from_midnight_opt(seconds as u32, nanos).unwrap()
    }
}

pub(crate) fn naive_time_to_seconds(value: &chrono::NaiveTime) -> (u64, u32) {
    use chrono::Timelike;

    let seconds = value.num_seconds_from_midnight() as u64;
    let nanos = value.nanosecond();

    (seconds, nanos)
}

impl From<chrono::NaiveDateTime> for TgTimePoint {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self::from(&value)
    }
}

impl From<&chrono::NaiveDateTime> for TgTimePoint {
    fn from(value: &chrono::NaiveDateTime) -> Self {
        let (seconds, nanos) = naive_date_time_to_seconds(value);
        TgTimePoint {
            offset_seconds: seconds,
            nano_adjustment: nanos,
        }
    }
}

impl From<TgTimePoint> for chrono::NaiveDateTime {
    fn from(value: TgTimePoint) -> Self {
        Self::from(&value)
    }
}

impl From<&TgTimePoint> for chrono::NaiveDateTime {
    fn from(value: &TgTimePoint) -> Self {
        let seconds = value.offset_seconds;
        let nanos = value.nano_adjustment;

        let days = seconds / (24 * 60 * 60);
        let secs_of_day = (seconds % (24 * 60 * 60)) as u32;

        let date = epoch_days_to_naive_date(days);
        let time =
            chrono::NaiveTime::from_num_seconds_from_midnight_opt(secs_of_day, nanos).unwrap();

        chrono::NaiveDateTime::new(date, time)
    }
}

pub(crate) fn naive_date_time_to_seconds(value: &chrono::NaiveDateTime) -> (i64, u32) {
    let days = naive_date_to_epoch_days(&value.date());
    let (seconds, nanos) = naive_time_to_seconds(&value.time());
    let seconds = days * 24 * 60 * 60 + seconds as i64;

    (seconds, nanos)
}

impl From<(chrono::NaiveTime, chrono::FixedOffset)> for TgTimeOfDayWithTimeZone {
    fn from(value: (chrono::NaiveTime, chrono::FixedOffset)) -> Self {
        Self::from(&value)
    }
}

impl From<&(chrono::NaiveTime, chrono::FixedOffset)> for TgTimeOfDayWithTimeZone {
    fn from(value: &(chrono::NaiveTime, chrono::FixedOffset)) -> Self {
        let (time, offset) = value;

        let (seconds, nanos) = naive_time_to_seconds(time);
        let offset_minutes = fixed_offset_to_minutes(offset);

        TgTimeOfDayWithTimeZone {
            offset_nanoseconds: seconds * 1_000_000_000 + nanos as u64,
            time_zone_offset: offset_minutes,
        }
    }
}

impl From<TgTimeOfDayWithTimeZone> for (chrono::NaiveTime, chrono::FixedOffset) {
    fn from(value: TgTimeOfDayWithTimeZone) -> Self {
        Self::from(&value)
    }
}

impl From<&TgTimeOfDayWithTimeZone> for (chrono::NaiveTime, chrono::FixedOffset) {
    fn from(value: &TgTimeOfDayWithTimeZone) -> Self {
        let total_nanos = value.offset_nanoseconds;
        let seconds = total_nanos / 1_000_000_000;
        let nanos = (total_nanos % 1_000_000_000) as u32;

        let time =
            chrono::NaiveTime::from_num_seconds_from_midnight_opt(seconds as u32, nanos).unwrap();
        let offset = chrono::FixedOffset::east_opt((value.time_zone_offset * 60) as i32).unwrap();

        (time, offset)
    }
}

pub(crate) fn fixed_offset_to_minutes(value: &chrono::FixedOffset) -> i32 {
    value.local_minus_utc() / 60
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for TgTimePointWithTimeZone {
    fn from(value: chrono::DateTime<Tz>) -> Self {
        Self::from(&value)
    }
}

impl<Tz: chrono::TimeZone> From<&chrono::DateTime<Tz>> for TgTimePointWithTimeZone {
    fn from(value: &chrono::DateTime<Tz>) -> Self {
        let (seconds, nanos) = naive_date_time_to_seconds(&value.naive_local());
        let offset_minutes = fixed_offset_to_minutes(&value.offset().fix());

        TgTimePointWithTimeZone {
            offset_seconds: seconds,
            nano_adjustment: nanos,
            time_zone_offset: offset_minutes,
        }
    }
}

impl From<TgTimePointWithTimeZone> for chrono::DateTime<chrono::FixedOffset> {
    fn from(value: TgTimePointWithTimeZone) -> Self {
        Self::from(&value)
    }
}

impl From<&TgTimePointWithTimeZone> for chrono::DateTime<chrono::FixedOffset> {
    fn from(value: &TgTimePointWithTimeZone) -> Self {
        let seconds = value.offset_seconds;
        let nanos = value.nano_adjustment;

        let days = seconds / (24 * 60 * 60);
        let secs_of_day = (seconds % (24 * 60 * 60)) as u32;

        let date = epoch_days_to_naive_date(days);
        let time =
            chrono::NaiveTime::from_num_seconds_from_midnight_opt(secs_of_day, nanos).unwrap();
        let datetime = chrono::NaiveDateTime::new(date, time);
        let offset = chrono::FixedOffset::east_opt((value.time_zone_offset * 60) as i32).unwrap();

        offset.from_local_datetime(&datetime).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tg_date_from_naive_date() {
        let value = chrono::NaiveDate::from_ymd_opt(2025, 1, 16).unwrap();
        let actual: TgDate = value.into();
        let expected = TgDate { epoch_days: 20104 };
        assert_eq!(expected, actual);
    }

    #[test]
    fn naive_date_from_tg_date() {
        let value = TgDate { epoch_days: 20104 };
        let actual: chrono::NaiveDate = value.into();
        let expected = chrono::NaiveDate::from_ymd_opt(2025, 1, 16).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tg_time_from_naive_time() {
        let value = chrono::NaiveTime::from_hms_nano_opt(16, 24, 30, 123456789).unwrap();
        let actual: TgTimeOfDay = value.into();
        let expected = TgTimeOfDay {
            offset_nanoseconds: 59070123456789,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn naive_time_from_tg_time() {
        let value = TgTimeOfDay {
            offset_nanoseconds: 59070123456789,
        };
        let actual: chrono::NaiveTime = value.into();
        let expected = chrono::NaiveTime::from_hms_nano_opt(16, 24, 30, 123456789).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tg_time_point_from_naive_date_time() {
        let value = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(2025, 1, 16).unwrap(),
            chrono::NaiveTime::from_hms_nano_opt(17, 42, 30, 123456789).unwrap(),
        );
        let actual: TgTimePoint = value.into();
        let expected = TgTimePoint {
            offset_seconds: 1737049350,
            nano_adjustment: 123456789,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn naive_date_time_from_tg_time_point() {
        let value = TgTimePoint {
            offset_seconds: 1737049350,
            nano_adjustment: 123456789,
        };
        let actual: chrono::NaiveDateTime = value.into();
        let expected = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(2025, 1, 16).unwrap(),
            chrono::NaiveTime::from_hms_nano_opt(17, 42, 30, 123456789).unwrap(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn tg_time_tz_from_naive_time_with_offset() {
        use std::str::FromStr;

        let time = chrono::NaiveTime::from_hms_nano_opt(17, 42, 30, 123456789).unwrap();
        let offset = chrono::FixedOffset::from_str("+09:00").unwrap();
        let value = (time, offset);

        let actual: TgTimeOfDayWithTimeZone = value.into();
        let expected = TgTimeOfDayWithTimeZone {
            offset_nanoseconds: (((17 * 60) + 42) * 60 + 30) * 1_000_000_000 + 123456789,
            time_zone_offset: 9 * 60,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn naive_time_with_offset_from_tg_time_tz() {
        use std::str::FromStr;

        let value = TgTimeOfDayWithTimeZone {
            offset_nanoseconds: (((17 * 60) + 42) * 60 + 30) * 1_000_000_000 + 123456789,
            time_zone_offset: 9 * 60,
        };
        let actual: (chrono::NaiveTime, chrono::FixedOffset) = value.into();
        let expected_time = chrono::NaiveTime::from_hms_nano_opt(17, 42, 30, 123456789).unwrap();
        let expected_offset = chrono::FixedOffset::from_str("+09:00").unwrap();
        assert_eq!(expected_time, actual.0);
        assert_eq!(expected_offset, actual.1);
    }

    #[test]
    fn tg_time_point_tz_from_date_time() {
        let value = create_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);
        let actual: TgTimePointWithTimeZone = value.into();
        let expected = TgTimePointWithTimeZone {
            offset_seconds: 1737049350,
            nano_adjustment: 123456789,
            time_zone_offset: 9 * 60,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn date_time_from_tg_time_point_tz() {
        let value = TgTimePointWithTimeZone {
            offset_seconds: 1737049350,
            nano_adjustment: 123456789,
            time_zone_offset: 9 * 60,
        };
        let actual: chrono::DateTime<chrono::FixedOffset> = value.into();
        let expected = create_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);
        assert_eq!(expected, actual);
    }

    fn create_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        nanos: u32,
        offset_hour: i32,
    ) -> chrono::DateTime<chrono::FixedOffset> {
        use std::str::FromStr;

        let s = format!("{year:04}-{month:02}-{day:02} {hour:02}:{min:02}:{sec:02}.{nanos:09} +{offset_hour:02}:00");
        chrono::DateTime::from_str(&s).unwrap()
    }
}
