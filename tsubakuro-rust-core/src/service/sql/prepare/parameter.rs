use crate::jogasaki::proto::sql::request::parameter::{Placement, Value};
use crate::jogasaki::proto::sql::request::Parameter as SqlParameter;

fn new_parameter(name: &str, value: Option<Value>) -> SqlParameter {
    let placement = Placement::Name(name.to_string());

    SqlParameter {
        placement: Some(placement),
        value,
    }
}

pub trait SqlParameterNull {
    fn null(name: &str) -> SqlParameter;
}

impl SqlParameterNull for SqlParameter {
    fn null(name: &str) -> SqlParameter {
        new_parameter(name, None)
    }
}

pub trait SqlParameterOf<T> {
    fn of(name: &str, value: T) -> SqlParameter;
}

impl SqlParameterOf<i32> for SqlParameter {
    fn of(name: &str, value: i32) -> SqlParameter {
        let value = Value::Int4Value(value);
        new_parameter(name, Some(value))
    }
}

impl SqlParameterOf<i64> for SqlParameter {
    fn of(name: &str, value: i64) -> SqlParameter {
        let value = Value::Int8Value(value);
        new_parameter(name, Some(value))
    }
}

impl SqlParameterOf<f32> for SqlParameter {
    fn of(name: &str, value: f32) -> SqlParameter {
        let value = Value::Float4Value(value);
        new_parameter(name, Some(value))
    }
}

impl SqlParameterOf<f64> for SqlParameter {
    fn of(name: &str, value: f64) -> SqlParameter {
        let value = Value::Float8Value(value);
        new_parameter(name, Some(value))
    }
}

impl SqlParameterOf<&str> for SqlParameter {
    fn of(name: &str, value: &str) -> SqlParameter {
        let value = Value::CharacterValue(value.to_string());
        new_parameter(name, Some(value))
    }
}

impl SqlParameterOf<String> for SqlParameter {
    fn of(name: &str, value: String) -> SqlParameter {
        let value = Value::CharacterValue(value);
        new_parameter(name, Some(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn name(target: &SqlParameter) -> Option<String> {
        match target.placement {
            Some(Placement::Name(ref name)) => Some(name.clone()),
            None => None,
        }
    }

    #[test]
    fn null() {
        let target = SqlParameter::null("test");
        assert_eq!("test", name(&target).unwrap());
        assert_eq!(None, target.value);
    }

    #[test]
    fn i32() {
        let target = SqlParameter::of("test", 123);
        assert_eq!("test", name(&target).unwrap());
        assert_eq!(Value::Int4Value(123), target.value.unwrap());
    }

    #[test]
    fn i64() {
        let target = SqlParameter::of("test", 123_i64);
        assert_eq!("test", name(&target).unwrap());
        assert_eq!(Value::Int8Value(123), target.value.unwrap());
    }

    #[test]
    fn f32() {
        let target = SqlParameter::of("test", 123_f32);
        assert_eq!("test", name(&target).unwrap());
        assert_eq!(Value::Float4Value(123.0), target.value.unwrap());
    }

    #[test]
    fn f64() {
        let target = SqlParameter::of("test", 123_f64);
        assert_eq!("test", name(&target).unwrap());
        assert_eq!(Value::Float8Value(123.0), target.value.unwrap());
    }

    #[test]
    fn str() {
        let target = SqlParameter::of("test", "abc");
        assert_eq!("test", name(&target).unwrap());
        assert_eq!(
            Value::CharacterValue("abc".to_string()),
            target.value.unwrap()
        );
    }

    #[test]
    fn string() {
        let target = SqlParameter::of("test", "abc".to_string());
        assert_eq!("test", name(&target).unwrap());
        assert_eq!(
            Value::CharacterValue("abc".to_string()),
            target.value.unwrap()
        );
    }
}
