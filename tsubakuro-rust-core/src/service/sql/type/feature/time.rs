use crate::prelude::{
    TgDate, TgTimeOfDay, TgTimeOfDayWithTimeZone, TgTimePoint, TgTimePointWithTimeZone,
};

impl From<time::Date> for TgDate {
    fn from(value: time::Date) -> Self {
        Self::from(&value)
    }
}

impl From<&time::Date> for TgDate {
    fn from(value: &time::Date) -> Self {
        let epoch_days = date_to_epoch_days(value);
        TgDate { epoch_days }
    }
}

impl From<TgDate> for time::Date {
    fn from(value: TgDate) -> Self {
        Self::from(&value)
    }
}

impl From<&TgDate> for time::Date {
    fn from(value: &TgDate) -> Self {
        epoch_days_to_date(value.epoch_days)
    }
}

// const TIME_EPOCH_START_DATE: Result<time::Date, time::error::ComponentRange> =
//     time::Date::from_ordinal_date(1970, 1);

pub(crate) fn date_to_epoch_days(value: &time::Date) -> i64 {
    // let days = *value - TIME_EPOCH_START_DATE.unwrap();
    // let days = days.whole_days();
    value.to_julian_day() as i64 - /* Date(1970-01-01).to_julian_day() */ 2440588
}

fn epoch_days_to_date(epoch_days: i64) -> time::Date {
    let days = epoch_days + /* Date(1970-01-01).to_julian_day() */ 2440588;
    time::Date::from_julian_day(days as i32).unwrap()
}

impl From<time::Time> for TgTimeOfDay {
    fn from(value: time::Time) -> Self {
        Self::from(&value)
    }
}

impl From<&time::Time> for TgTimeOfDay {
    fn from(value: &time::Time) -> Self {
        let (seconds, nanos) = time_to_seconds(value);
        TgTimeOfDay {
            offset_nanoseconds: seconds * 1_000_000_000 + nanos as u64,
        }
    }
}

impl From<TgTimeOfDay> for time::Time {
    fn from(value: TgTimeOfDay) -> Self {
        Self::from(&value)
    }
}

impl From<&TgTimeOfDay> for time::Time {
    fn from(value: &TgTimeOfDay) -> Self {
        offset_nanos_to_time(value.offset_nanoseconds)
    }
}

pub(crate) fn time_to_seconds(value: &time::Time) -> (u64, u32) {
    let (hour, min, sec, nanos) = value.as_hms_nano();
    let seconds = ((hour as u64) * 60 + min as u64) * 60 + sec as u64;

    (seconds, nanos)
}

fn offset_nanos_to_time(offset_nanoseconds: u64) -> time::Time {
    let seconds = (offset_nanoseconds / 1_000_000_000) as u32;
    let nanos = (offset_nanoseconds % 1_000_000_000) as u32;

    offset_seconds_to_time(seconds, nanos)
}

impl From<time::PrimitiveDateTime> for TgTimePoint {
    fn from(value: time::PrimitiveDateTime) -> Self {
        Self::from(&value)
    }
}

impl From<&time::PrimitiveDateTime> for TgTimePoint {
    fn from(value: &time::PrimitiveDateTime) -> Self {
        let (seconds, nanos) = date_time_to_seconds(&value.date(), &value.time());
        TgTimePoint {
            offset_seconds: seconds,
            nano_adjustment: nanos,
        }
    }
}

impl From<TgTimePoint> for time::PrimitiveDateTime {
    fn from(value: TgTimePoint) -> Self {
        Self::from(&value)
    }
}

impl From<&TgTimePoint> for time::PrimitiveDateTime {
    fn from(value: &TgTimePoint) -> Self {
        let seconds = value.offset_seconds;
        let nanos = value.nano_adjustment;

        let (date, time) = epoch_seconds_to_date_time(seconds, nanos);

        time::PrimitiveDateTime::new(date, time)
    }
}

pub(crate) fn date_time_to_seconds(date: &time::Date, time: &time::Time) -> (i64, u32) {
    let days = date_to_epoch_days(date);
    let (seconds, nanos) = time_to_seconds(time);
    let seconds = days * 24 * 60 * 60 + seconds as i64;

    (seconds, nanos)
}

fn epoch_seconds_to_date_time(epoch_seconds: i64, nanos: u32) -> (time::Date, time::Time) {
    let days = epoch_seconds / (24 * 60 * 60);
    let secs_of_day = (epoch_seconds % (24 * 60 * 60)) as u32;

    let date = epoch_days_to_date(days);
    let time = offset_seconds_to_time(secs_of_day, nanos);
    (date, time)
}

fn offset_seconds_to_time(secs_of_day: u32, nanos: u32) -> time::Time {
    let hour = (secs_of_day / 3600) as u8;
    let min = ((secs_of_day % 3600) / 60) as u8;
    let sec = (secs_of_day % 60) as u8;

    time::Time::from_hms_nano(hour, min, sec, nanos).unwrap()
}

impl From<(time::Time, time::UtcOffset)> for TgTimeOfDayWithTimeZone {
    fn from(value: (time::Time, time::UtcOffset)) -> Self {
        Self::from(&value)
    }
}

impl From<&(time::Time, time::UtcOffset)> for TgTimeOfDayWithTimeZone {
    fn from(value: &(time::Time, time::UtcOffset)) -> Self {
        let (time, offset) = value;

        let (seconds, nanos) = time_to_seconds(time);
        let offset_minutes = utc_offset_to_minutes(offset);

        TgTimeOfDayWithTimeZone {
            offset_nanoseconds: seconds * 1_000_000_000 + nanos as u64,
            time_zone_offset: offset_minutes,
        }
    }
}

impl From<TgTimeOfDayWithTimeZone> for (time::Time, time::UtcOffset) {
    fn from(value: TgTimeOfDayWithTimeZone) -> Self {
        Self::from(&value)
    }
}

impl From<&TgTimeOfDayWithTimeZone> for (time::Time, time::UtcOffset) {
    fn from(value: &TgTimeOfDayWithTimeZone) -> Self {
        let time = offset_nanos_to_time(value.offset_nanoseconds);
        let offset =
            time::UtcOffset::from_whole_seconds((value.time_zone_offset * 60) as i32).unwrap();

        (time, offset)
    }
}

pub(crate) fn utc_offset_to_minutes(offset: &time::UtcOffset) -> i32 {
    let (hour, min, _sec) = offset.as_hms();
    hour as i32 * 60 + min as i32
}

impl From<time::OffsetDateTime> for TgTimePointWithTimeZone {
    fn from(value: time::OffsetDateTime) -> Self {
        Self::from(&value)
    }
}

impl From<&time::OffsetDateTime> for TgTimePointWithTimeZone {
    fn from(value: &time::OffsetDateTime) -> Self {
        let (seconds, nanos) = date_time_to_seconds(&value.date(), &value.time());
        let offset_minutes = utc_offset_to_minutes(&value.offset());

        TgTimePointWithTimeZone {
            offset_seconds: seconds,
            nano_adjustment: nanos,
            time_zone_offset: offset_minutes,
        }
    }
}

impl From<TgTimePointWithTimeZone> for time::OffsetDateTime {
    fn from(value: TgTimePointWithTimeZone) -> Self {
        Self::from(&value)
    }
}

impl From<&TgTimePointWithTimeZone> for time::OffsetDateTime {
    fn from(value: &TgTimePointWithTimeZone) -> Self {
        let seconds = value.offset_seconds;
        let nanos = value.nano_adjustment;
        let (date, time) = epoch_seconds_to_date_time(seconds, nanos);

        let offset =
            time::UtcOffset::from_whole_seconds((value.time_zone_offset * 60) as i32).unwrap();

        time::OffsetDateTime::new_in_offset(date, time, offset)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tg_date_from_date() {
        let value = time::Date::from_calendar_date(2025, time::Month::January, 16).unwrap();

        let actual: TgDate = value.into();
        let expected = TgDate { epoch_days: 20104 };
        assert_eq!(expected, actual);
    }

    #[test]
    fn date_from_tg_date() {
        let value = TgDate { epoch_days: 20104 };

        let actual: time::Date = value.into();
        let expected = time::Date::from_calendar_date(2025, time::Month::January, 16).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tg_time_from_time() {
        let value = time::Time::from_hms_nano(16, 24, 30, 123456789).unwrap();

        let actual: TgTimeOfDay = value.into();
        let expected = TgTimeOfDay {
            offset_nanoseconds: 59070123456789,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn time_from_tg_time() {
        let value = TgTimeOfDay {
            offset_nanoseconds: 59070123456789,
        };

        let actual: time::Time = value.into();
        let expected = time::Time::from_hms_nano(16, 24, 30, 123456789).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn tg_time_point_from_primitive_date_time() {
        let value = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2025, time::Month::January, 16).unwrap(),
            time::Time::from_hms_nano(17, 42, 30, 123456789).unwrap(),
        );

        let actual: TgTimePoint = value.into();
        let expected = TgTimePoint {
            offset_seconds: 1737049350,
            nano_adjustment: 123456789,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn primitive_date_time_from_tg_time_point() {
        let value = TgTimePoint {
            offset_seconds: 1737049350,
            nano_adjustment: 123456789,
        };

        let actual: time::PrimitiveDateTime = value.into();
        let expected = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2025, time::Month::January, 16).unwrap(),
            time::Time::from_hms_nano(17, 42, 30, 123456789).unwrap(),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn tg_time_tz_from_time_with_offset() {
        let time = time::Time::from_hms_nano(17, 42, 30, 123456789).unwrap();
        let offset = time::UtcOffset::from_hms(9, 0, 0).unwrap();
        let value = (time, offset);

        let actual: TgTimeOfDayWithTimeZone = value.into();
        let expected = TgTimeOfDayWithTimeZone {
            offset_nanoseconds: (((17 * 60) + 42) * 60 + 30) * 1_000_000_000 + 123456789,
            time_zone_offset: 9 * 60,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn time_with_offset_from_tg_time_tz() {
        let value = TgTimeOfDayWithTimeZone {
            offset_nanoseconds: (((17 * 60) + 42) * 60 + 30) * 1_000_000_000 + 123456789,
            time_zone_offset: 9 * 60,
        };

        let actual: (time::Time, time::UtcOffset) = value.into();
        let expected_time = time::Time::from_hms_nano(17, 42, 30, 123456789).unwrap();
        let expected_offset = time::UtcOffset::from_hms(9, 0, 0).unwrap();
        assert_eq!(expected_time, actual.0);
        assert_eq!(expected_offset, actual.1);
    }

    #[test]
    fn tg_time_point_tz_from_offset_date_time() {
        let value = create_offset_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

        let actual: TgTimePointWithTimeZone = value.into();
        let expected = TgTimePointWithTimeZone {
            offset_seconds: 1737049350,
            nano_adjustment: 123456789,
            time_zone_offset: 9 * 60,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn offset_date_time_from_tg_time_point_tz() {
        let value = TgTimePointWithTimeZone {
            offset_seconds: 1737049350,
            nano_adjustment: 123456789,
            time_zone_offset: 9 * 60,
        };

        let actual: time::OffsetDateTime = value.into();
        let expected = create_offset_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);
        assert_eq!(expected, actual);
    }

    fn create_offset_date_time(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        min: u8,
        sec: u8,
        nanos: u32,
        offset_hour: i32,
    ) -> time::OffsetDateTime {
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day)
                .unwrap(),
            time::Time::from_hms_nano(hour, min, sec, nanos).unwrap(),
            time::UtcOffset::from_whole_seconds(offset_hour * 60 * 60).unwrap(),
        )
    }
}
