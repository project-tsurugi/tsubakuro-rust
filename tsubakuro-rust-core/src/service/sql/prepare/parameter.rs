use crate::jogasaki::proto::sql::common::Decimal as ProtoDecimal;
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
pub(crate) const NATIVE_DATE_EPOCH_START: Option<chrono::NaiveDate> =
    chrono::NaiveDate::from_ymd_opt(1970, 1, 1);

#[cfg(feature = "with_chrono")]
impl SqlParameterOf<&chrono::NaiveDate> for SqlParameter {
    fn of(name: &str, value: &chrono::NaiveDate) -> SqlParameter {
        let days = value
            .signed_duration_since(NATIVE_DATE_EPOCH_START.unwrap())
            .num_days();
        let value = Value::DateValue(days);
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
    fn chrono_native_date() {
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
    fn chrono_native_date_ref() {
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
}
