use crate::jogasaki::proto::sql::common::{
    TimeOfDayWithTimeZone as ProtoTimeOfDayWithTimeZone, TimePoint as ProtoTimePoint,
    TimePointWithTimeZone as ProtoTimePointWithTimeZone,
};
use crate::jogasaki::proto::sql::request::{parameter::Value, Parameter as SqlParameter};
use crate::prelude::r#type::feature::chrono::{naive_date_to_epoch_days, naive_time_to_seconds};
use crate::prelude::SqlParameterOf;

impl SqlParameterOf<chrono::NaiveDate> for SqlParameter {
    fn of(name: &str, value: chrono::NaiveDate) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&chrono::NaiveDate> for SqlParameter {
    fn of(name: &str, value: &chrono::NaiveDate) -> SqlParameter {
        let epoch_days = naive_date_to_epoch_days(value);

        let value = Value::DateValue(epoch_days);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<chrono::NaiveTime> for SqlParameter {
    fn of(name: &str, value: chrono::NaiveTime) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&chrono::NaiveTime> for SqlParameter {
    fn of(name: &str, value: &chrono::NaiveTime) -> SqlParameter {
        let (seconds, nanos) = naive_time_to_seconds(value);
        let value = seconds * 1_000_000_000 + nanos as u64;

        let value = Value::TimeOfDayValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<chrono::NaiveDateTime> for SqlParameter {
    fn of(name: &str, value: chrono::NaiveDateTime) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&chrono::NaiveDateTime> for SqlParameter {
    fn of(name: &str, value: &chrono::NaiveDateTime) -> SqlParameter {
        let value: ProtoTimePoint = value.into();
        let value = Value::TimePointValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<(chrono::NaiveTime, chrono::FixedOffset)> for SqlParameter {
    fn of(name: &str, value: (chrono::NaiveTime, chrono::FixedOffset)) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl SqlParameterOf<&(chrono::NaiveTime, chrono::FixedOffset)> for SqlParameter {
    fn of(name: &str, value: &(chrono::NaiveTime, chrono::FixedOffset)) -> SqlParameter {
        let value: ProtoTimeOfDayWithTimeZone = value.into();
        let value = Value::TimeOfDayWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl<Tz: chrono::TimeZone> SqlParameterOf<chrono::DateTime<Tz>> for SqlParameter {
    fn of(name: &str, value: chrono::DateTime<Tz>) -> SqlParameter {
        Self::of(name, &value)
    }
}

impl<Tz: chrono::TimeZone> SqlParameterOf<&chrono::DateTime<Tz>> for SqlParameter {
    fn of(name: &str, value: &chrono::DateTime<Tz>) -> SqlParameter {
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
    fn naive_date() {
        let value = chrono::NaiveDate::from_ymd_opt(2025, 1, 16).unwrap();
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
    fn naive_date_ref() {
        let value = chrono::NaiveDate::from_ymd_opt(2025, 1, 16).unwrap();

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::DateValue(20104), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(&value));
        assert_eq!(target0, target);

        let target = "test".parameter(&value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(&value);
        assert_eq!(target0, target);
    }

    #[test]
    fn naive_time() {
        let value = chrono::NaiveTime::from_hms_milli_opt(16, 24, 30, 456).unwrap();
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
    fn naive_time_ref() {
        naive_time_ref_test(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(), 0);
        naive_time_ref_test(
            chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            86399000000000,
        );
        naive_time_ref_test(
            chrono::NaiveTime::from_hms_nano_opt(0, 0, 0, 123456789).unwrap(),
            123456789,
        );
        naive_time_ref_test(
            chrono::NaiveTime::from_hms_nano_opt(23, 59, 59, 999999999).unwrap(),
            86399999999999,
        );
    }

    fn naive_time_ref_test(value: chrono::NaiveTime, expected: u64) {
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
    fn naive_date_time() {
        let value = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(2025, 1, 16).unwrap(),
            chrono::NaiveTime::from_hms_nano_opt(17, 42, 30, 123456789).unwrap(),
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
    fn naive_date_time_ref() {
        naive_date_time_ref_test(2025, 1, 16, 17, 42, 30, 123456789, 1737049350);
        naive_date_time_ref_test(1970, 1, 1, 0, 0, 0, 0, 0);
        naive_date_time_ref_test(1969, 12, 31, 23, 59, 59, 999999999, -1);
    }

    fn naive_date_time_ref_test(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        nanos: u32,
        expected_sec: i64,
    ) {
        let value = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap(),
            chrono::NaiveTime::from_hms_nano_opt(hour, min, sec, nanos).unwrap(),
        );

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointValue(ProtoTimePoint {
                offset_seconds: expected_sec,
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
    fn naive_time_with_offset() {
        use std::str::FromStr;

        let time = chrono::NaiveTime::from_hms_nano_opt(17, 42, 30, 123456789).unwrap();
        let offset = chrono::FixedOffset::from_str("+09:00").unwrap();
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
    fn naive_time_with_offset_ref() {
        naive_time_with_offset_ref_test(17, 42, 30, 123456789, 9);
        naive_time_with_offset_ref_test(0, 0, 0, 0, 9);
        naive_time_with_offset_ref_test(23, 59, 59, 999999999, 9);
        naive_time_with_offset_ref_test(17, 42, 30, 123456789, -9);
    }

    fn naive_time_with_offset_ref_test(
        hour: u32,
        min: u32,
        sec: u32,
        nanos: u32,
        offset_hour: i32,
    ) {
        use std::str::FromStr;

        let time = chrono::NaiveTime::from_hms_nano_opt(hour, min, sec, nanos).unwrap();
        let offset = if offset_hour >= 0 {
            format!("+{:02}:00", offset_hour)
        } else {
            format!("-{:02}:00", offset_hour.abs())
        };
        let offset = chrono::FixedOffset::from_str(&offset).unwrap();
        let value = (time, offset);

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimeOfDayWithTimeZoneValue(ProtoTimeOfDayWithTimeZone {
                offset_nanoseconds: (((hour as u64 * 60) + min as u64) * 60 + sec as u64)
                    * 1_000_000_000
                    + nanos as u64,
                time_zone_offset: offset_hour * 60
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
    fn date_time() {
        let value = create_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

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
    fn date_time_ref() {
        let value = create_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

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
