use chrono::Datelike;
use chrono::Offset;

use crate::jogasaki::proto::sql::common::Decimal as ProtoDecimal;
use crate::jogasaki::proto::sql::common::TimeOfDayWithTimeZone as ProtoTimeOfDayWithTimeZone;
use crate::jogasaki::proto::sql::common::TimePoint as ProtoTimePoint;
use crate::jogasaki::proto::sql::common::TimePointWithTimeZone as ProtoTimePointWithTimeZone;
use crate::jogasaki::proto::sql::request::parameter::{Placement, Value};
use crate::jogasaki::proto::sql::request::Parameter as SqlParameter;

impl SqlParameter {
    fn new(name: &str, value: Option<Value>) -> SqlParameter {
        let placement = Placement::Name(name.to_string());

        SqlParameter {
            placement: Some(placement),
            value,
        }
    }

    pub fn null(name: &str) -> SqlParameter {
        SqlParameter::new(name, None)
    }

    pub fn name(&self) -> Option<&String> {
        match self.placement {
            Some(Placement::Name(ref name)) => Some(name),
            _ => None,
        }
    }

    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }
}

pub trait SqlParameterOf<T> {
    fn of(name: &str, value: T) -> SqlParameter;
}

impl SqlParameterOf<bool> for SqlParameter {
    fn of(name: &str, value: bool) -> SqlParameter {
        let value = Value::BooleanValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<i32> for SqlParameter {
    fn of(name: &str, value: i32) -> SqlParameter {
        let value = Value::Int4Value(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<i64> for SqlParameter {
    fn of(name: &str, value: i64) -> SqlParameter {
        let value = Value::Int8Value(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<f32> for SqlParameter {
    fn of(name: &str, value: f32) -> SqlParameter {
        let value = Value::Float4Value(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<f64> for SqlParameter {
    fn of(name: &str, value: f64) -> SqlParameter {
        let value = Value::Float8Value(value);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_bigdecimal")]
impl SqlParameterOf<bigdecimal::BigDecimal> for SqlParameter {
    fn of(name: &str, value: bigdecimal::BigDecimal) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_bigdecimal")]
impl SqlParameterOf<&bigdecimal::BigDecimal> for SqlParameter {
    fn of(name: &str, value: &bigdecimal::BigDecimal) -> SqlParameter {
        let (value, scale) = value.as_bigint_and_exponent();
        let value = ProtoDecimal {
            unscaled_value: value.to_signed_bytes_be(),
            exponent: -scale as i32,
        };
        let value = Value::DecimalValue(value);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_rust_decimal")]
impl SqlParameterOf<rust_decimal::Decimal> for SqlParameter {
    fn of(name: &str, value: rust_decimal::Decimal) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_rust_decimal")]
impl SqlParameterOf<&rust_decimal::Decimal> for SqlParameter {
    fn of(name: &str, value: &rust_decimal::Decimal) -> SqlParameter {
        let value = ProtoDecimal {
            unscaled_value: value.mantissa().to_be_bytes().to_vec(),
            exponent: -(value.scale() as i32),
        };
        let value = Value::DecimalValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<&str> for SqlParameter {
    fn of(name: &str, value: &str) -> SqlParameter {
        let value = Value::CharacterValue(value.to_string());
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<String> for SqlParameter {
    fn of(name: &str, value: String) -> SqlParameter {
        let value = Value::CharacterValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<&String> for SqlParameter {
    fn of(name: &str, value: &String) -> SqlParameter {
        let value = Value::CharacterValue(value.clone());
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<&[u8]> for SqlParameter {
    fn of(name: &str, value: &[u8]) -> SqlParameter {
        let value = Value::OctetValue(value.to_vec());
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<Vec<u8>> for SqlParameter {
    fn of(name: &str, value: Vec<u8>) -> SqlParameter {
        let value = Value::OctetValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<&Vec<u8>> for SqlParameter {
    fn of(name: &str, value: &Vec<u8>) -> SqlParameter {
        let value = Value::OctetValue(value.clone());
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<chrono::NaiveDate> for SqlParameter {
    fn of(name: &str, value: chrono::NaiveDate) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<&chrono::NaiveDate> for SqlParameter {
    fn of(name: &str, value: &chrono::NaiveDate) -> SqlParameter {
        let days = value.num_days_from_ce() - 719_163;
        let value = Value::DateValue(days as i64);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<chrono::NaiveTime> for SqlParameter {
    fn of(name: &str, value: chrono::NaiveTime) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<&chrono::NaiveTime> for SqlParameter {
    fn of(name: &str, value: &chrono::NaiveTime) -> SqlParameter {
        use chrono::Timelike;

        let seconds = value.num_seconds_from_midnight() as u64;
        let nano = value.nanosecond() as u64;
        let value = Value::TimeOfDayValue(seconds * 1_000_000_000 + nano);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<chrono::NaiveDateTime> for SqlParameter {
    fn of(name: &str, value: chrono::NaiveDateTime) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<&chrono::NaiveDateTime> for SqlParameter {
    fn of(name: &str, value: &chrono::NaiveDateTime) -> SqlParameter {
        use chrono::Timelike;

        let days = (value.num_days_from_ce() - 719_163) as i64;
        let seconds = value.num_seconds_from_midnight() as i64;
        let seconds = days * 24 * 60 * 60 + seconds;
        let nanos = value.nanosecond();
        let value = ProtoTimePoint {
            offset_seconds: seconds,
            nano_adjustment: nanos,
        };
        let value = Value::TimePointValue(value);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<(chrono::NaiveTime, chrono::FixedOffset)> for SqlParameter {
    fn of(name: &str, value: (chrono::NaiveTime, chrono::FixedOffset)) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<&(chrono::NaiveTime, chrono::FixedOffset)> for SqlParameter {
    fn of(name: &str, value: &(chrono::NaiveTime, chrono::FixedOffset)) -> SqlParameter {
        use chrono::Timelike;

        let (time, offset) = value;

        let seconds = time.num_seconds_from_midnight() as u64;
        let nano = time.nanosecond() as u64;
        let time_zone_offset_minutes = offset.local_minus_utc() / 60;

        let value = ProtoTimeOfDayWithTimeZone {
            offset_nanoseconds: seconds * 1_000_000_000 + nano,
            time_zone_offset: time_zone_offset_minutes,
        };
        let value = Value::TimeOfDayWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_chrono")]
impl<Tz: chrono::TimeZone> SqlParameterOf<chrono::DateTime<Tz>> for SqlParameter {
    fn of(name: &str, value: chrono::DateTime<Tz>) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_chrono")]
impl<Tz: chrono::TimeZone> SqlParameterOf<&chrono::DateTime<Tz>> for SqlParameter {
    fn of(name: &str, value: &chrono::DateTime<Tz>) -> SqlParameter {
        use chrono::Timelike;

        let naive_date_time = value.naive_local();
        let days = (naive_date_time.num_days_from_ce() - 719_163) as i64;
        let seconds = naive_date_time.num_seconds_from_midnight() as i64;
        let seconds = days * 24 * 60 * 60 + seconds;
        let nanos = naive_date_time.nanosecond();
        let time_zone_offset_minutes = value.offset().fix().local_minus_utc() / 60;

        let value = ProtoTimePointWithTimeZone {
            offset_seconds: seconds,
            nano_adjustment: nanos,
            time_zone_offset: time_zone_offset_minutes,
        };
        let value = Value::TimePointWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<time::Date> for SqlParameter {
    fn of(name: &str, value: time::Date) -> SqlParameter {
        Self::of(name, &value)
    }
}

// #[cfg(feature = "with_time")]
// const TIME_EPOCH_START_DATE: Result<time::Date, time::error::ComponentRange> =
//     time::Date::from_ordinal_date(1970, 1);

#[cfg(feature = "with_time")]
impl SqlParameterOf<&time::Date> for SqlParameter {
    fn of(name: &str, value: &time::Date) -> SqlParameter {
        // let days = *value - TIME_EPOCH_START_DATE.unwrap();
        // let days = days.whole_days();
        let days = value.to_julian_day() - /* Date(1970-01-01).to_julian_day() */ 2440588;
        let value = Value::DateValue(days as i64);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<time::Time> for SqlParameter {
    fn of(name: &str, value: time::Time) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<&time::Time> for SqlParameter {
    fn of(name: &str, value: &time::Time) -> SqlParameter {
        let (hour, minute, second, nano) = value.as_hms_nano();
        let seconds = ((hour as u64) * 60 + minute as u64) * 60 + second as u64;
        let value = Value::TimeOfDayValue(seconds * 1_000_000_000 + nano as u64);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<time::PrimitiveDateTime> for SqlParameter {
    fn of(name: &str, value: time::PrimitiveDateTime) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<&time::PrimitiveDateTime> for SqlParameter {
    fn of(name: &str, value: &time::PrimitiveDateTime) -> SqlParameter {
        let days = value.to_julian_day() - /* Date(1970-01-01).to_julian_day() */ 2440588;
        let (hour, minute, second, nanos) = value.as_hms_nano();
        let seconds = ((hour as i64) * 60 + minute as i64) * 60 + second as i64;
        let seconds = days as i64 * 24 * 60 * 60 + seconds;
        let value = ProtoTimePoint {
            offset_seconds: seconds,
            nano_adjustment: nanos,
        };
        let value = Value::TimePointValue(value);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<(time::Time, time::UtcOffset)> for SqlParameter {
    fn of(name: &str, value: (time::Time, time::UtcOffset)) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<&(time::Time, time::UtcOffset)> for SqlParameter {
    fn of(name: &str, value: &(time::Time, time::UtcOffset)) -> SqlParameter {
        let (time, offset) = value;

        let (hour, minute, second, nano) = time.as_hms_nano();
        let seconds = ((hour as u64) * 60 + minute as u64) * 60 + second as u64;
        let (hour, minute, second) = offset.as_hms();
        let time_zone_offset_minutes = hour as i32 * 60 + minute as i32 + second as i32;

        let value = ProtoTimeOfDayWithTimeZone {
            offset_nanoseconds: seconds * 1_000_000_000 + nano as u64,
            time_zone_offset: time_zone_offset_minutes,
        };
        let value = Value::TimeOfDayWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<time::OffsetDateTime> for SqlParameter {
    fn of(name: &str, value: time::OffsetDateTime) -> SqlParameter {
        Self::of(name, &value)
    }
}

#[cfg(feature = "with_time")]
impl SqlParameterOf<&time::OffsetDateTime> for SqlParameter {
    fn of(name: &str, value: &time::OffsetDateTime) -> SqlParameter {
        let days = value.to_julian_day() - /* Date(1970-01-01).to_julian_day() */ 2440588;
        let (hour, minute, second, nanos) = value.to_hms_nano();
        let seconds = ((hour as i64) * 60 + minute as i64) * 60 + second as i64;
        let seconds = days as i64 * 24 * 60 * 60 + seconds;

        let (hour, minute, second) = value.offset().as_hms();
        let time_zone_offset_minutes = hour as i32 * 60 + minute as i32 + second as i32;

        let value = ProtoTimePointWithTimeZone {
            offset_seconds: seconds,
            nano_adjustment: nanos,
            time_zone_offset: time_zone_offset_minutes,
        };
        let value = Value::TimePointWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl<T> SqlParameterOf<Option<T>> for SqlParameter
where
    SqlParameter: SqlParameterOf<T>,
{
    fn of(name: &str, value: Option<T>) -> SqlParameter {
        match value {
            Some(value) => SqlParameter::of(name, value),
            _ => SqlParameter::null(name),
        }
    }
}

pub trait SqlParameterBind<T> {
    fn parameter(&self, value: T) -> SqlParameter;
}

impl<T> SqlParameterBind<T> for &str
where
    SqlParameter: SqlParameterOf<T>,
{
    fn parameter(&self, value: T) -> SqlParameter {
        SqlParameter::of(self, value)
    }
}

impl<T> SqlParameterBind<T> for String
where
    SqlParameter: SqlParameterOf<T>,
{
    fn parameter(&self, value: T) -> SqlParameter {
        SqlParameter::of(self, value)
    }
}

pub trait SqlParameterBindNull {
    fn parameter_null(&self) -> SqlParameter;
}

impl SqlParameterBindNull for &str {
    fn parameter_null(&self) -> SqlParameter {
        SqlParameter::null(self)
    }
}

impl SqlParameterBindNull for String {
    fn parameter_null(&self) -> SqlParameter {
        SqlParameter::null(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn null() {
        let target0 = SqlParameter::null("test");
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(None, target0.value);

        let target = SqlParameter::of("test", None::<i32>);
        assert_eq!(target0, target);

        let target = "test".parameter_null();
        assert_eq!(target0, target);

        let target = "test".to_string().parameter_null();
        assert_eq!(target0, target);
    }

    #[test]
    fn bool() {
        bool_test(true);
        bool_test(false);
    }

    fn bool_test(value: bool) {
        let target0 = SqlParameter::of("test", value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::BooleanValue(value), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(value));
        assert_eq!(target0, target);

        let target = "test".parameter(value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(value);
        assert_eq!(target0, target);
    }

    #[test]
    fn i32() {
        let target0 = SqlParameter::of("test", 123);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::Int4Value(123), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(123));
        assert_eq!(target0, target);

        let target = "test".parameter(123);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(123);
        assert_eq!(target0, target);
    }

    #[test]
    fn i64() {
        let target0 = SqlParameter::of("test", 123_i64);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::Int8Value(123), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(123_i64));
        assert_eq!(target0, target);

        let target = "test".parameter(123_i64);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(123_i64);
        assert_eq!(target0, target);
    }

    #[test]
    fn f32() {
        let target0 = SqlParameter::of("test", 123_f32);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::Float4Value(123.0), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(123_f32));
        assert_eq!(target0, target);

        let target = "test".parameter(123_f32);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(123_f32);
        assert_eq!(target0, target);
    }

    #[test]
    fn f64() {
        let target0 = SqlParameter::of("test", 123_f64);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::Float8Value(123.0), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(123_f64));
        assert_eq!(target0, target);

        let target = "test".parameter(123_f64);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(123_f64);
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_bigdecimal")]
    #[test]
    fn bigdecimal() {
        use std::str::FromStr;

        let value = bigdecimal::BigDecimal::from_str("123.4").unwrap();
        let expected = value.as_bigint_and_exponent();

        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::DecimalValue(ProtoDecimal {
                unscaled_value: expected.0.to_signed_bytes_be(),
                exponent: -expected.1 as i32
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

    #[cfg(feature = "with_bigdecimal")]
    #[test]
    fn bigdecimal_ref() {
        use std::str::FromStr;

        let value = bigdecimal::BigDecimal::from_str("123.4").unwrap();
        let expected = value.as_bigint_and_exponent();

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::DecimalValue(ProtoDecimal {
                unscaled_value: expected.0.to_signed_bytes_be(),
                exponent: -expected.1 as i32
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

    #[cfg(feature = "with_rust_decimal")]
    #[test]
    fn rust_decimal() {
        use std::str::FromStr;

        let value = rust_decimal::Decimal::from_str("123.4").unwrap();

        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::DecimalValue(ProtoDecimal {
                unscaled_value: value.mantissa().to_be_bytes().to_vec(),
                exponent: -(value.scale() as i32)
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

    #[cfg(feature = "with_rust_decimal")]
    #[test]
    fn rust_decimal_ref() {
        use std::str::FromStr;

        let value = rust_decimal::Decimal::from_str("123.4").unwrap();

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::DecimalValue(ProtoDecimal {
                unscaled_value: value.mantissa().to_be_bytes().to_vec(),
                exponent: -(value.scale() as i32)
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
    fn str() {
        let target0 = SqlParameter::of("test", "abc");
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::CharacterValue("abc".to_string()),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some("abc"));
        assert_eq!(target0, target);

        let target = "test".parameter("abc");
        assert_eq!(target0, target);

        let target = "test".to_string().parameter("abc");
        assert_eq!(target0, target);
    }

    #[test]
    fn string() {
        let target0 = SqlParameter::of("test", "abc".to_string());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::CharacterValue("abc".to_string()),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some("abc".to_string()));
        assert_eq!(target0, target);

        let target = "test".parameter("abc".to_string());
        assert_eq!(target0, target);

        let target = "test".to_string().parameter("abc".to_string());
        assert_eq!(target0, target);
    }

    #[test]
    fn string_ref() {
        let target0 = SqlParameter::of("test", &"abc".to_string());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::CharacterValue("abc".to_string()),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(&"abc".to_string()));
        assert_eq!(target0, target);

        let target = "test".parameter(&"abc".to_string());
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(&"abc".to_string());
        assert_eq!(target0, target);
    }

    #[test]
    fn array_u8() {
        let value = [0x12_u8, 0x34, 0xef].as_slice();
        let target0 = SqlParameter::of("test", value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::OctetValue(value.to_vec()), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(value));
        assert_eq!(target0, target);

        let target = "test".parameter(value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(value);
        assert_eq!(target0, target);
    }

    #[test]
    fn vec_u8() {
        let value = vec![0x12_u8, 0x34, 0xef];
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::OctetValue(value.clone()), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);

        let target = "test".parameter(value.clone());
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(value.clone());
        assert_eq!(target0, target);
    }

    #[test]
    fn vec_u8_ref() {
        let value = vec![0x12_u8, 0x34, 0xef];
        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::OctetValue(value.clone()), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(&value));
        assert_eq!(target0, target);

        let target = "test".parameter(&value);
        assert_eq!(target0, target);

        let target = "test".to_string().parameter(&value);
        assert_eq!(target0, target);
    }

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_date() {
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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_date_ref() {
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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_time() {
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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_time_ref() {
        chrono_naive_time_ref_test(chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(), 0);
        chrono_naive_time_ref_test(
            chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            86399000000000,
        );
        chrono_naive_time_ref_test(
            chrono::NaiveTime::from_hms_nano_opt(0, 0, 0, 123456789).unwrap(),
            123456789,
        );
        chrono_naive_time_ref_test(
            chrono::NaiveTime::from_hms_nano_opt(23, 59, 59, 999999999).unwrap(),
            86399999999999,
        );
    }

    #[cfg(feature = "with_chrono")]
    fn chrono_naive_time_ref_test(value: chrono::NaiveTime, expected: u64) {
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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_date_time() {
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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_date_time_ref() {
        chrono_naive_date_time_ref_test(2025, 1, 16, 17, 42, 30, 123456789, 1737049350);
        chrono_naive_date_time_ref_test(1970, 1, 1, 0, 0, 0, 0, 0);
        chrono_naive_date_time_ref_test(1969, 12, 31, 23, 59, 59, 999999999, -1);
    }

    #[cfg(feature = "with_chrono")]
    fn chrono_naive_date_time_ref_test(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
        expected_sec: i64,
    ) {
        let value = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap(),
            chrono::NaiveTime::from_hms_nano_opt(hour, min, sec, nano).unwrap(),
        );

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointValue(ProtoTimePoint {
                offset_seconds: expected_sec,
                nano_adjustment: nano
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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_time_with_offset() {
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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_naive_time_with_offset_ref() {
        chrono_naive_time_with_offset_ref_test(17, 42, 30, 123456789, 9);
        chrono_naive_time_with_offset_ref_test(0, 0, 0, 0, 9);
        chrono_naive_time_with_offset_ref_test(23, 59, 59, 999999999, 9);
        chrono_naive_time_with_offset_ref_test(17, 42, 30, 123456789, -9);
    }

    #[cfg(feature = "with_chrono")]
    fn chrono_naive_time_with_offset_ref_test(
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
        offset_hour: i32,
    ) {
        use std::str::FromStr;

        let time = chrono::NaiveTime::from_hms_nano_opt(hour, min, sec, nano).unwrap();
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
                    + nano as u64,
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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_date_time() {
        let value = date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

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

    #[cfg(feature = "with_chrono")]
    #[test]
    fn chrono_date_time_ref() {
        let value = date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

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

    #[cfg(feature = "with_chrono")]
    fn date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
        offset_hour: i32,
    ) -> chrono::DateTime<chrono::FixedOffset> {
        use std::str::FromStr;

        let s = format!("{year:04}-{month:02}-{day:02} {hour:02}:{min:02}:{sec:02}.{nano:09} +{offset_hour:02}:00");
        chrono::DateTime::from_str(&s).unwrap()
    }

    #[cfg(feature = "with_time")]
    #[test]
    fn time_date() {
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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_date_ref() {
        time_date_ref_test(2025, 1, 16, 20104);
        time_date_ref_test(1970, 1, 1, 0);
        time_date_ref_test(1969, 12, 31, -1);
        time_date_ref_test(0, 1, 1, -719528);
        time_date_ref_test(9999, 12, 31, 2932896);
        time_date_ref_test(-9999, 1, 1, -4371587);
    }

    #[cfg(feature = "with_time")]
    fn time_date_ref_test(year: i32, month: u8, day: u8, expected: i64) {
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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_time() {
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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_time_ref() {
        time_time_ref_test(time::Time::from_hms(0, 0, 0).unwrap(), 0);
        time_time_ref_test(time::Time::from_hms(23, 59, 59).unwrap(), 86399000000000);
        time_time_ref_test(
            time::Time::from_hms_nano(0, 0, 0, 123456789).unwrap(),
            123456789,
        );
        time_time_ref_test(
            time::Time::from_hms_nano(23, 59, 59, 999999999).unwrap(),
            86399999999999,
        );
    }

    #[cfg(feature = "with_time")]
    fn time_time_ref_test(value: time::Time, expected: u64) {
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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_primitive_date_time() {
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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_primitive_date_time_ref() {
        time_primitive_date_time_ref_test(2025, 1, 16, 17, 42, 30, 123456789, 1737049350);
        time_primitive_date_time_ref_test(1970, 1, 1, 0, 0, 0, 0, 0);
        time_primitive_date_time_ref_test(1969, 12, 31, 23, 59, 59, 999999999, -1);
    }

    #[cfg(feature = "with_time")]
    fn time_primitive_date_time_ref_test(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        min: u8,
        sec: u8,
        nano: u32,
        expected: i64,
    ) {
        let value = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day)
                .unwrap(),
            time::Time::from_hms_nano(hour, min, sec, nano).unwrap(),
        );
        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointValue(ProtoTimePoint {
                offset_seconds: expected,
                nano_adjustment: nano
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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_time_with_offset() {
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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_naive_time_with_offset_ref() {
        time_time_with_offset_ref_test(17, 42, 30, 123456789, 9);
        time_time_with_offset_ref_test(0, 0, 0, 0, 9);
        time_time_with_offset_ref_test(23, 59, 59, 999999999, 9);
        time_time_with_offset_ref_test(17, 42, 30, 123456789, -9);
    }

    #[cfg(feature = "with_time")]
    fn time_time_with_offset_ref_test(hour: u8, min: u8, sec: u8, nano: u32, offset_hour: i8) {
        let time = time::Time::from_hms_nano(hour, min, sec, nano).unwrap();
        let offset = time::UtcOffset::from_hms(offset_hour, 0, 0).unwrap();
        let value = (time, offset);

        let target0 = SqlParameter::of("test", &value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimeOfDayWithTimeZoneValue(ProtoTimeOfDayWithTimeZone {
                offset_nanoseconds: (((hour as u64 * 60) + min as u64) * 60 + sec as u64)
                    * 1_000_000_000
                    + nano as u64,
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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_offset_date_time() {
        let value = offset_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

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

    #[cfg(feature = "with_time")]
    #[test]
    fn time_offset_date_time_ref() {
        let value = offset_date_time(2025, 1, 16, 17, 42, 30, 123456789, 9);

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

    #[cfg(feature = "with_time")]
    fn offset_date_time(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        min: u8,
        sec: u8,
        nano: u32,
        offset_hour: i32,
    ) -> time::OffsetDateTime {
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day)
                .unwrap(),
            time::Time::from_hms_nano(hour, min, sec, nano).unwrap(),
            time::UtcOffset::from_whole_seconds(offset_hour * 60 * 60).unwrap(),
        )
    }
}
