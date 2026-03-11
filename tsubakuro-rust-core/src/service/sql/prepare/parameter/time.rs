use crate::jogasaki::proto::sql::common::{
    TimeOfDayWithTimeZone as ProtoTimeOfDayWithTimeZone, TimePoint as ProtoTimePoint,
    TimePointWithTimeZone as ProtoTimePointWithTimeZone,
};
use crate::jogasaki::proto::sql::request::{parameter::Value, Parameter as SqlParameter};
use crate::prelude::r#type::feature::time::{date_to_epoch_days, time_to_seconds};
use crate::prelude::SqlParameterOf;

impl SqlParameterOf<time::Date> for SqlParameter {
    fn of(name: &str, value: time::Date) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&time::Date> for SqlParameter {
    fn of(name: &str, value: &time::Date) -> SqlParameter {
        let epoch_days = date_to_epoch_days(value);

        let value = Value::DateValue(epoch_days);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<time::Time> for SqlParameter {
    fn of(name: &str, value: time::Time) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&time::Time> for SqlParameter {
    fn of(name: &str, value: &time::Time) -> SqlParameter {
        let (seconds, nanos) = time_to_seconds(value);
        let value = seconds * 1_000_000_000 + nanos as u64;

        let value = Value::TimeOfDayValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<time::PrimitiveDateTime> for SqlParameter {
    fn of(name: &str, value: time::PrimitiveDateTime) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&time::PrimitiveDateTime> for SqlParameter {
    fn of(name: &str, value: &time::PrimitiveDateTime) -> SqlParameter {
        let value: ProtoTimePoint = value.into();
        let value = Value::TimePointValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<(time::Time, time::UtcOffset)> for SqlParameter {
    fn of(name: &str, value: (time::Time, time::UtcOffset)) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&(time::Time, time::UtcOffset)> for SqlParameter {
    fn of(name: &str, value: &(time::Time, time::UtcOffset)) -> SqlParameter {
        let value: ProtoTimeOfDayWithTimeZone = value.into();
        let value = Value::TimeOfDayWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<time::OffsetDateTime> for SqlParameter {
    fn of(name: &str, value: time::OffsetDateTime) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&time::OffsetDateTime> for SqlParameter {
    fn of(name: &str, value: &time::OffsetDateTime) -> SqlParameter {
        let value: ProtoTimePointWithTimeZone = value.into();
        let value = Value::TimePointWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::SqlParameterBind;

    #[test]
    fn date() {
        let value = time::Date::from_calendar_date(2025, time::Month::January, 16).unwrap();
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::DateValue(20104), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);

        let target = "test".parameter(value.clone());
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(value.clone());
        assert_eq!(target0, target);
    }

    #[test]
    fn date_ref() {
        date_ref_test(2025, 1, 16, 20104);
        date_ref_test(1970, 1, 1, 0);
        date_ref_test(1969, 12, 31, -1);
        date_ref_test(0, 1, 1, -719528);
        date_ref_test(9999, 12, 31, 2932896);
        date_ref_test(-9999, 1, 1, -4371587);
    }

    fn date_ref_test(year: i32, month: u8, day: u8, expected: i64) {
        let value =
            time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day)
                .unwrap();
        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::DateValue(expected), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(&value));
        assert_eq!(target0, target);

        let target = "test".parameter(&value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(&value);
        assert_eq!(target0, target);
    }

    #[test]
    fn time() {
        let value = time::Time::from_hms_milli(16, 24, 30, 456).unwrap();
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimeOfDayValue(59070456000000),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);

        let target = "test".parameter(value.clone());
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(value.clone());
        assert_eq!(target0, target);
    }

    #[test]
    fn time_ref() {
        time_ref_test(time::Time::from_hms(0, 0, 0).unwrap(), 0);
        time_ref_test(time::Time::from_hms(23, 59, 59).unwrap(), 86399000000000);
        time_ref_test(
            time::Time::from_hms_nano(0, 0, 0, 123456789).unwrap(),
            123456789,
        );
        time_ref_test(
            time::Time::from_hms_nano(23, 59, 59, 999999999).unwrap(),
            86399999999999,
        );
    }

    fn time_ref_test(value: time::Time, expected: u64) {
        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::TimeOfDayValue(expected), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(&value));
        assert_eq!(target0, target);

        let target = "test".parameter(&value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(&value);
        assert_eq!(target0, target);
    }

    #[test]
    fn primitive_date_time() {
        let value = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2025, time::Month::January, 16).unwrap(),
            time::Time::from_hms_nano(17, 42, 30, 123456789).unwrap(),
        );
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointValue(ProtoTimePoint {
                offset_seconds: 1737049350,
                nano_adjustment: 123456789
            }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);

        let target = "test".parameter(value.clone());
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(value.clone());
        assert_eq!(target0, target);
    }

    #[test]
    fn primitive_date_time_ref() {
        primitive_date_time_ref_test(2025, 1, 16, 17, 42, 30, 123456789, 1737049350);
        primitive_date_time_ref_test(1970, 1, 1, 0, 0, 0, 0, 0);
        primitive_date_time_ref_test(1969, 12, 31, 23, 59, 59, 999999999, -1);
    }

    fn primitive_date_time_ref_test(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        min: u8,
        sec: u8,
        nanos: u32,
        expected: i64,
    ) {
        let value = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day)
                .unwrap(),
            time::Time::from_hms_nano(hour, min, sec, nanos).unwrap(),
        );
        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointValue(ProtoTimePoint {
                offset_seconds: expected,
                nano_adjustment: nanos
            }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(&value));
        assert_eq!(target0, target);

        let target = "test".parameter(&value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(&value);
        assert_eq!(target0, target);
    }

    #[test]
    fn time_with_offset() {
        let time = time::Time::from_hms_nano(17, 42, 30, 123456789).unwrap();
        let offset = time::UtcOffset::from_hms(9, 0, 0).unwrap();
        let value = (time, offset);

        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimeOfDayWithTimeZoneValue(ProtoTimeOfDayWithTimeZone {
                offset_nanoseconds: (((17 * 60) + 42) * 60 + 30) * 1_000_000_000 + 123456789,
                time_zone_offset: 9 * 60
            }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);

        let target = "test".parameter(value.clone());
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(value.clone());
        assert_eq!(target0, target);
    }

    #[test]
    fn time_with_offset_ref() {
        time_with_offset_ref_test(17, 42, 30, 123456789, 9);
        time_with_offset_ref_test(0, 0, 0, 0, 9);
        time_with_offset_ref_test(23, 59, 59, 999999999, 9);
        time_with_offset_ref_test(17, 42, 30, 123456789, -9);
    }

    fn time_with_offset_ref_test(hour: u8, min: u8, sec: u8, nanos: u32, offset_hour: i8) {
        let time = time::Time::from_hms_nano(hour, min, sec, nanos).unwrap();
        let offset = time::UtcOffset::from_hms(offset_hour, 0, 0).unwrap();
        let value = (time, offset);

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimeOfDayWithTimeZoneValue(ProtoTimeOfDayWithTimeZone {
                offset_nanoseconds: (((hour as u64 * 60) + min as u64) * 60 + sec as u64)
                    * 1_000_000_000
                    + nanos as u64,
                time_zone_offset: offset_hour as i32 * 60
            }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(&value));
        assert_eq!(target0, target);

        let target = "test".parameter(&value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(&value);
        assert_eq!(target0, target);
    }

    #[test]
    fn offset_date_time() {
        let value = create_offset_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointWithTimeZoneValue(ProtoTimePointWithTimeZone {
                offset_seconds: 1737049350,
                nano_adjustment: 123456789,
                time_zone_offset: 9 * 60
            }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);

        let target = "test".parameter(value.clone());
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(value.clone());
        assert_eq!(target0, target);
    }

    #[test]
    fn offset_date_time_ref() {
        let value = create_offset_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointWithTimeZoneValue(ProtoTimePointWithTimeZone {
                offset_seconds: 1737049350,
                nano_adjustment: 123456789,
                time_zone_offset: 9 * 60
            }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(&value));
        assert_eq!(target0, target);

        let target = "test".parameter(&value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(&value);
        assert_eq!(target0, target);
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
