use crate::jogasaki::proto::sql::request::parameter::{Placement, Value};
use crate::jogasaki::proto::sql::request::Parameter as SqlParameter;
use crate::jogasaki::proto::sql::request::{
    client_only_large_object_info::Data as ClientOnlyLargeObjectInfoData, ClientOnlyLargeObjectInfo,
};
use crate::prelude::{
    TgBlob, TgClob, TgDate, TgDecimal, TgDecimalI128, TgTimeOfDay, TgTimeOfDayWithTimeZone,
    TgTimePoint, TgTimePointWithTimeZone,
};
use crate::service::lob::lob_client::RemoteLob;
use crate::service::sql::r#type::blob::InnerBlob;
use crate::service::sql::r#type::clob::InnerClob;

#[cfg(feature = "with_bigdecimal")]
mod bigdecimal;
#[cfg(feature = "with_chrono")]
mod chrono;
#[cfg(feature = "with_rust_decimal")]
mod rust_decimal;
#[cfg(feature = "with_time")]
mod time;

impl SqlParameter {
    pub(crate) fn new(name: &str, value: Option<Value>) -> SqlParameter {
        let placement = Placement::Name(name.to_string());

        SqlParameter {
            placement: Some(placement),
            value,
        }
    }

    /// Creates a null parameter.
    pub fn null(name: &str) -> SqlParameter {
        SqlParameter::new(name, None)
    }

    /// Get name.
    pub fn name(&self) -> Option<&String> {
        match self.placement {
            Some(Placement::Name(ref name)) => Some(name),
            _ => None,
        }
    }

    /// Get value.
    pub fn value(&self) -> Option<&Value> {
        self.value.as_ref()
    }
}

/// `of` method for [SqlParameter].
pub trait SqlParameterOf<T> {
    /// Creates a new instance.
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

impl SqlParameterOf<TgDecimal> for SqlParameter {
    fn of(name: &str, value: TgDecimal) -> SqlParameter {
        let value = Value::DecimalValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<TgDecimalI128> for SqlParameter {
    fn of(name: &str, value: TgDecimalI128) -> SqlParameter {
        let value = TgDecimal::from(value);
        Self::of(name, value)
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

impl SqlParameterOf<TgDate> for SqlParameter {
    fn of(name: &str, value: TgDate) -> SqlParameter {
        let value = Value::DateValue(value.epoch_days);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<TgTimeOfDay> for SqlParameter {
    fn of(name: &str, value: TgTimeOfDay) -> SqlParameter {
        let value = Value::TimeOfDayValue(value.offset_nanoseconds);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<TgTimePoint> for SqlParameter {
    fn of(name: &str, value: TgTimePoint) -> SqlParameter {
        let value = Value::TimePointValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<TgTimeOfDayWithTimeZone> for SqlParameter {
    fn of(name: &str, value: TgTimeOfDayWithTimeZone) -> SqlParameter {
        let value = Value::TimeOfDayWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<TgTimePointWithTimeZone> for SqlParameter {
    fn of(name: &str, value: TgTimePointWithTimeZone) -> SqlParameter {
        let value = Value::TimePointWithTimeZoneValue(value);
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<TgBlob> for SqlParameter {
    fn of(name: &str, value: TgBlob) -> SqlParameter {
        let value = match value.inner {
            InnerBlob::Path(path) => {
                let data = ClientOnlyLargeObjectInfoData::ClientPath(path);
                let info = ClientOnlyLargeObjectInfo { data: Some(data) };
                Value::LargeObjectInfoBlob(info)
            }
            InnerBlob::Contents(value) => {
                let data = crate::jogasaki::proto::sql::common::blob::Data::Contents(value);
                let value = crate::jogasaki::proto::sql::common::Blob { data: Some(data) };
                Value::Blob(value)
            }
            InnerBlob::RemoteLob(remote_lob) => match remote_lob {
                RemoteLob::ServerPath(path) => {
                    let data = ClientOnlyLargeObjectInfoData::ServerPath(path);
                    let info = ClientOnlyLargeObjectInfo { data: Some(data) };
                    Value::LargeObjectInfoBlob(info)
                }
                RemoteLob::LobReference(storage_id, object_id, tag) => {
                    let lob_ref = crate::jogasaki::proto::sql::request::BlobRelayReference {
                        storage_id,
                        object_id,
                        tag,
                    };
                    let data = ClientOnlyLargeObjectInfoData::BlobRelayReference(lob_ref);
                    let info = ClientOnlyLargeObjectInfo { data: Some(data) };
                    Value::LargeObjectInfoBlob(info)
                }
            },
        };
        SqlParameter::new(name, Some(value))
    }
}

impl SqlParameterOf<TgClob> for SqlParameter {
    fn of(name: &str, value: TgClob) -> SqlParameter {
        let value = match value.inner {
            InnerClob::Path(path) => {
                let data = ClientOnlyLargeObjectInfoData::ClientPath(path);
                let info = ClientOnlyLargeObjectInfo { data: Some(data) };
                Value::LargeObjectInfoClob(info)
            }
            InnerClob::Contents(value) => {
                let data = crate::jogasaki::proto::sql::common::clob::Data::Contents(value);
                let value = crate::jogasaki::proto::sql::common::Clob { data: Some(data) };
                Value::Clob(value)
            }
            InnerClob::RemoteLob(remote_lob) => match remote_lob {
                RemoteLob::ServerPath(path) => {
                    let data = ClientOnlyLargeObjectInfoData::ServerPath(path);
                    let info = ClientOnlyLargeObjectInfo { data: Some(data) };
                    Value::LargeObjectInfoClob(info)
                }
                RemoteLob::LobReference(storage_id, object_id, tag) => {
                    let lob_ref = crate::jogasaki::proto::sql::request::BlobRelayReference {
                        storage_id,
                        object_id,
                        tag,
                    };
                    let data = ClientOnlyLargeObjectInfoData::BlobRelayReference(lob_ref);
                    let info = ClientOnlyLargeObjectInfo { data: Some(data) };
                    Value::LargeObjectInfoClob(info)
                }
            },
        };
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

/// `parameter` method for [SqlParameter].
pub trait SqlParameterBind<T> {
    /// Creates a new instance.
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

/// `parameter_null` method for [SqlParameter].
pub trait SqlParameterBindNull {
    /// Creates a null parameter.
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
    use crate::jogasaki::proto::sql::common::Decimal as ProtoDecimal;

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

    #[test]
    fn decimal() {
        let value = TgDecimal::new(vec![4, 0xd2], -1);
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::DecimalValue(ProtoDecimal {
                unscaled_value: vec![4, 0xd2],
                exponent: -1
            }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);

        let value = TgDecimalI128::new(1234, -1);
        let target = SqlParameter::of("test", value.clone());
        assert_eq!(target0.name(), target.name());
        assert_eq!(
            &Value::DecimalValue(ProtoDecimal {
                unscaled_value: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0xd2],
                exponent: -1
            }),
            target.value().unwrap()
        );

        let target1 = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target1, target);
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

    #[test]
    fn date() {
        let value = TgDate::new(20126);
        let target0 = SqlParameter::of("test", value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::DateValue(value.epoch_days),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value));
        assert_eq!(target0, target);
    }

    #[test]
    fn time_of_day() {
        let value = TgTimeOfDay::new(30551971944200);
        let target0 = SqlParameter::of("test", value);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimeOfDayValue(value.offset_nanoseconds),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value));
        assert_eq!(target0, target);
    }

    #[test]
    fn time_point() {
        let value = TgTimePoint::new(1738917213, 123456789);
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointValue(value.clone()),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);
    }

    #[test]
    fn time_of_day_with_time_zone() {
        let value = TgTimeOfDayWithTimeZone::new(30551971944200, 9 * 60);
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimeOfDayWithTimeZoneValue(value.clone()),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);
    }

    #[test]
    fn time_point_with_time_zone() {
        let value = TgTimePointWithTimeZone::new(1738917213, 123456789, 9 * 60);
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(
            &Value::TimePointWithTimeZoneValue(value.clone()),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);
    }

    #[test]
    fn blob_path() {
        #[allow(deprecated)]
        let value = TgBlob::new("/path/to/file");
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());

        let data = ClientOnlyLargeObjectInfoData::ClientPath("/path/to/file".to_string());
        let info = ClientOnlyLargeObjectInfo { data: Some(data) };
        assert_eq!(&Value::LargeObjectInfoBlob(info), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);
    }

    #[test]
    fn blob_contents() {
        let contents = vec![1, 2, 3];
        let value = TgBlob::from(contents.clone());
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        let data = crate::jogasaki::proto::sql::common::blob::Data::Contents(contents);
        assert_eq!(
            &Value::Blob(crate::jogasaki::proto::sql::common::Blob { data: Some(data) }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);
    }

    #[test]
    fn clob_path() {
        #[allow(deprecated)]
        let value = TgClob::new("/path/to/file");
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        let data = ClientOnlyLargeObjectInfoData::ClientPath("/path/to/file".to_string());
        let info = ClientOnlyLargeObjectInfo { data: Some(data) };
        assert_eq!(&Value::LargeObjectInfoClob(info), target0.value().unwrap());

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);
    }

    #[test]
    fn clob_contents() {
        let contents = "abc";
        let value = TgClob::from(contents);
        let target0 = SqlParameter::of("test", value.clone());
        assert_eq!("test", target0.name().unwrap());
        let data =
            crate::jogasaki::proto::sql::common::clob::Data::Contents(contents.as_bytes().to_vec());
        assert_eq!(
            &Value::Clob(crate::jogasaki::proto::sql::common::Clob { data: Some(data) }),
            target0.value().unwrap()
        );

        let target = SqlParameter::of("test", Some(value.clone()));
        assert_eq!(target0, target);

        let value = TgClob::from(contents.to_string());
        let target = SqlParameter::of("test", value);
        assert_eq!(target0, target);
    }
}
