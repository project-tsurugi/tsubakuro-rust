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

pub trait SqlParameterGenerator<T> {
    fn parameter(self, name: &str) -> SqlParameter;
}

impl<T> SqlParameterGenerator<T> for T
where
    SqlParameter: SqlParameterOf<T>,
{
    fn parameter(self, name: &str) -> SqlParameter {
        SqlParameter::of(name, self)
    }
}

pub trait SqlParameterBind<T> {
    fn bind(&self, value: T) -> SqlParameter;
}

impl<T> SqlParameterBind<T> for &str
where
    SqlParameter: SqlParameterOf<T>,
{
    fn bind(&self, value: T) -> SqlParameter {
        SqlParameter::of(self, value)
    }
}

impl<T> SqlParameterBind<T> for String
where
    SqlParameter: SqlParameterOf<T>,
{
    fn bind(&self, value: T) -> SqlParameter {
        SqlParameter::of(self, value)
    }
}

pub trait SqlParameterBindNull {
    fn bind_null(&self) -> SqlParameter;
}

impl SqlParameterBindNull for &str {
    fn bind_null(&self) -> SqlParameter {
        SqlParameter::null(self)
    }
}

impl SqlParameterBindNull for String {
    fn bind_null(&self) -> SqlParameter {
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

        let target = "test".bind_null();
        assert_eq!(target0, target);

        let target = "test".to_string().bind_null();
        assert_eq!(target0, target);
    }

    #[test]
    fn i32() {
        let target0 = SqlParameter::of("test", 123);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::Int4Value(123), target0.value().unwrap());

        let target = 123.parameter("test");
        assert_eq!(target0, target);

        let target = "test".bind(123);
        assert_eq!(target0, target);

        let target = "test".to_string().bind(123);
        assert_eq!(target0, target);
    }

    #[test]
    fn i64() {
        let target0 = SqlParameter::of("test", 123_i64);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::Int8Value(123), target0.value().unwrap());

        let target = 123_i64.parameter("test");
        assert_eq!(target0, target);

        let target = "test".bind(123_i64);
        assert_eq!(target0, target);

        let target = "test".to_string().bind(123_i64);
        assert_eq!(target0, target);
    }

    #[test]
    fn f32() {
        let target0 = SqlParameter::of("test", 123_f32);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::Float4Value(123.0), target0.value().unwrap());

        let target = 123_f32.parameter("test");
        assert_eq!(target0, target);

        let target = "test".bind(123_f32);
        assert_eq!(target0, target);

        let target = "test".to_string().bind(123_f32);
        assert_eq!(target0, target);
    }

    #[test]
    fn f64() {
        let target0 = SqlParameter::of("test", 123_f64);
        assert_eq!("test", target0.name().unwrap());
        assert_eq!(&Value::Float8Value(123.0), target0.value().unwrap());

        let target = 123_f64.parameter("test");
        assert_eq!(target0, target);

        let target = "test".bind(123_f64);
        assert_eq!(target0, target);

        let target = "test".to_string().bind(123_f64);
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

        let target = "abc".parameter("test");
        assert_eq!(target0, target);

        let target = "test".bind("abc");
        assert_eq!(target0, target);

        let target = "test".to_string().bind("abc");
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

        let target = "abc".to_string().parameter("test");
        assert_eq!(target0, target);

        let target = "test".bind("abc".to_string());
        assert_eq!(target0, target);

        let target = "test".to_string().bind("abc".to_string());
        assert_eq!(target0, target);
    }
}
