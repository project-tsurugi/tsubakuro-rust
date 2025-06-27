use std::str::FromStr;

pub use crate::jogasaki::proto::sql::common::Decimal as TgDecimal;
use crate::{client_error, error::TgError};

impl TgDecimal {
    /// Creates a new instance.
    pub fn new(unscaled_value: Vec<u8>, exponent: i32) -> TgDecimal {
        TgDecimal {
            unscaled_value,
            exponent,
        }
    }
}

/// Decimal for [SqlQueryResult](crate::prelude::SqlQueryResult).
#[derive(Debug, Clone)]
pub struct TgDecimalResult {
    /// unscaled value
    pub unscaled_value_bytes: Option<Vec<u8>>,
    /// unscaled value
    pub unscaled_value: i64,
    /// exponent
    pub exponent: i32,
}

impl TgDecimalResult {
    /// Creates a new instance.
    pub fn new(
        unscaled_value_bytes: Option<Vec<u8>>,
        unscaled_value: i64,
        exponent: i32,
    ) -> TgDecimalResult {
        TgDecimalResult {
            unscaled_value_bytes,
            unscaled_value,
            exponent,
        }
    }
}

/// Decimal of i128.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TgDecimalI128 {
    /// unscaled value
    pub unscaled_value: i128,
    /// exponent
    pub exponent: i32,
}

impl TgDecimalI128 {
    /// Creates a new instance.
    pub fn new(unscaled_value: i128, exponent: i32) -> TgDecimalI128 {
        TgDecimalI128 {
            unscaled_value,
            exponent,
        }
    }
}

impl std::fmt::Display for TgDecimalI128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let exponent = self.exponent;
        if exponent == 0 {
            return write!(f, "{}", self.unscaled_value);
        }
        if exponent > 0 {
            if self.unscaled_value == 0 {
                return write!(f, "{}", self.unscaled_value);
            }
            let zeros = "0".repeat(exponent as usize);
            return write!(f, "{}{}", self.unscaled_value, zeros);
        }

        let s = self.unscaled_value.abs().to_string();
        let positive = self.unscaled_value >= 0;

        let pos = s.len() as i32 + exponent;
        match pos {
            0 => {
                if positive {
                    write!(f, "0.{}", s)
                } else {
                    write!(f, "-0.{}", s)
                }
            }
            p if p < 0 => {
                let zeros = "0".repeat(pos.unsigned_abs() as usize);
                if positive {
                    write!(f, "0.{}{}", zeros, s)
                } else {
                    write!(f, "-0.{}{}", zeros, s)
                }
            }
            _ => {
                let pos = pos as usize;
                if positive {
                    write!(f, "{}.{}", &s[..pos], &s[pos..])
                } else {
                    write!(f, "-{}.{}", &s[..pos], &s[pos..])
                }
            }
        }
    }
}

impl FromStr for TgDecimalI128 {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, exponent) = if let Some((left, right)) = s.split_once('e') {
            let exponent = right.parse::<i32>()?;
            (left, exponent)
        } else if let Some((left, right)) = s.split_once('E') {
            let exponent = right.parse::<i32>()?;
            (left, exponent)
        } else {
            (s, 0)
        };
        let (unscaled_value, scale) = if let Some((left, right)) = value.split_once(".") {
            let value = format!("{}{}", left, right);
            let value = value.parse::<i128>()?;
            let scale = right.len() as i32;
            (value, scale)
        } else {
            let value = value.parse::<i128>()?;
            (value, 0)
        };
        let exponent = exponent - scale;

        Ok(TgDecimalI128::new(unscaled_value, exponent))
    }
}

impl From<TgDecimalI128> for TgDecimal {
    fn from(value: TgDecimalI128) -> Self {
        TgDecimal::new(value.unscaled_value.to_be_bytes().to_vec(), value.exponent)
    }
}

impl TryFrom<TgDecimalResult> for TgDecimalI128 {
    type Error = TgError;

    fn try_from(value: TgDecimalResult) -> Result<Self, Self::Error> {
        if let Some(bytes) = value.unscaled_value_bytes {
            let size = bytes.len();
            if size == 0 {
                return Ok(TgDecimalI128::new(0, value.exponent));
            }
            if 16 < size {
                return Err(client_error!("unsupported unscaled_value_bytes size"));
            }
            if (bytes[0] as i8) >= 0 {
                let mut array = [0u8; 16];
                array[16 - size..].copy_from_slice(&bytes);
                let unscaled_value = i128::from_be_bytes(array);
                return Ok(TgDecimalI128::new(unscaled_value, value.exponent));
            } else {
                let mut array = [0xffu8; 16];
                array[16 - size..].copy_from_slice(&bytes);
                let unscaled_value = i128::from_be_bytes(array);
                return Ok(TgDecimalI128::new(unscaled_value, value.exponent));
            }
        }

        Ok(TgDecimalI128::new(
            value.unscaled_value as i128,
            value.exponent,
        ))
    }
}

impl From<TgDecimalResult> for TgDecimal {
    fn from(value: TgDecimalResult) -> Self {
        if let Some(bytes) = value.unscaled_value_bytes {
            TgDecimal::new(bytes, value.exponent)
        } else {
            TgDecimal::new(value.unscaled_value.to_be_bytes().to_vec(), value.exponent)
        }
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn display_decimal_i128() {
        {
            let v = 0;
            let value = TgDecimalI128::new(v, 0);
            assert_eq!("0", value.to_string());
            let value = TgDecimalI128::new(v, 1);
            assert_eq!("0", value.to_string());
            let value = TgDecimalI128::new(v, 2);
            assert_eq!("0", value.to_string());
            let value = TgDecimalI128::new(v, 3);
            assert_eq!("0", value.to_string());

            let value = TgDecimalI128::new(v, -1);
            assert_eq!("0.0", value.to_string());
            let value = TgDecimalI128::new(v, -2);
            assert_eq!("0.00", value.to_string());
            let value = TgDecimalI128::new(v, -3);
            assert_eq!("0.000", value.to_string());
        }
        {
            let v = 1234;
            let value = TgDecimalI128::new(v, 0);
            assert_eq!("1234", value.to_string());
            let value = TgDecimalI128::new(v, 1);
            assert_eq!("12340", value.to_string());
            let value = TgDecimalI128::new(v, 2);
            assert_eq!("123400", value.to_string());
            let value = TgDecimalI128::new(v, 3);
            assert_eq!("1234000", value.to_string());

            let value = TgDecimalI128::new(v, -1);
            assert_eq!("123.4", value.to_string());
            let value = TgDecimalI128::new(v, -2);
            assert_eq!("12.34", value.to_string());
            let value = TgDecimalI128::new(v, -3);
            assert_eq!("1.234", value.to_string());
            let value = TgDecimalI128::new(v, -4);
            assert_eq!("0.1234", value.to_string());
            let value = TgDecimalI128::new(v, -5);
            assert_eq!("0.01234", value.to_string());
            let value = TgDecimalI128::new(v, -6);
            assert_eq!("0.001234", value.to_string());
        }
        {
            let v = -1234;
            let value = TgDecimalI128::new(v, 0);
            assert_eq!("-1234", value.to_string());
            let value = TgDecimalI128::new(v, 1);
            assert_eq!("-12340", value.to_string());
            let value = TgDecimalI128::new(v, 2);
            assert_eq!("-123400", value.to_string());
            let value = TgDecimalI128::new(v, 3);
            assert_eq!("-1234000", value.to_string());

            let value = TgDecimalI128::new(v, -1);
            assert_eq!("-123.4", value.to_string());
            let value = TgDecimalI128::new(v, -2);
            assert_eq!("-12.34", value.to_string());
            let value = TgDecimalI128::new(v, -3);
            assert_eq!("-1.234", value.to_string());
            let value = TgDecimalI128::new(v, -4);
            assert_eq!("-0.1234", value.to_string());
            let value = TgDecimalI128::new(v, -5);
            assert_eq!("-0.01234", value.to_string());
            let value = TgDecimalI128::new(v, -6);
            assert_eq!("-0.001234", value.to_string());
        }
    }

    #[test]
    fn decimal_i128_from_str() {
        {
            let value = TgDecimalI128::from_str("0").unwrap();
            assert_eq!(0, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("+0").unwrap();
            assert_eq!(0, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("-0").unwrap();
            assert_eq!(0, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("0.").unwrap();
            assert_eq!(0, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("0.00").unwrap();
            assert_eq!(0, value.unscaled_value);
            assert_eq!(-2, value.exponent);
        }

        {
            let value = TgDecimalI128::from_str("123e0").unwrap();
            assert_eq!(123, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("123e2").unwrap();
            assert_eq!(123, value.unscaled_value);
            assert_eq!(2, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("123e-2").unwrap();
            assert_eq!(123, value.unscaled_value);
            assert_eq!(-2, value.exponent);
        }

        {
            let value = TgDecimalI128::from_str("123E0").unwrap();
            assert_eq!(123, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("123E2").unwrap();
            assert_eq!(123, value.unscaled_value);
            assert_eq!(2, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("123E-2").unwrap();
            assert_eq!(123, value.unscaled_value);
            assert_eq!(-2, value.exponent);
        }

        {
            let value = TgDecimalI128::from_str("123").unwrap();
            assert_eq!(123, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("123.0").unwrap();
            assert_eq!(1230, value.unscaled_value);
            assert_eq!(-1, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("123.4").unwrap();
            assert_eq!(1234, value.unscaled_value);
            assert_eq!(-1, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("-123").unwrap();
            assert_eq!(-123, value.unscaled_value);
            assert_eq!(0, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("-123.0").unwrap();
            assert_eq!(-1230, value.unscaled_value);
            assert_eq!(-1, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("-123.4").unwrap();
            assert_eq!(-1234, value.unscaled_value);
            assert_eq!(-1, value.exponent);
        }

        {
            let value = TgDecimalI128::from_str("123.45e0").unwrap();
            assert_eq!(12345, value.unscaled_value);
            assert_eq!(-2, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("123.45e1").unwrap();
            assert_eq!(12345, value.unscaled_value);
            assert_eq!(-1, value.exponent);
        }
        {
            let value = TgDecimalI128::from_str("123.45e-1").unwrap();
            assert_eq!(12345, value.unscaled_value);
            assert_eq!(-3, value.exponent);
        }
    }

    #[test]
    fn from_decimal_i128_to_decimal() {
        let value = TgDecimalI128::new(1234, -1);
        let value = TgDecimal::from(value);
        let expected = TgDecimal::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0xd2], -1);
        assert_eq!(expected, value);
    }

    #[test]
    fn from_decimal_result_to_decimal_i128() {
        {
            let value = TgDecimalResult::new(Some(vec![]), 999, -1);
            let value = TgDecimalI128::try_from(value).unwrap();
            let expected = TgDecimalI128::new(0, -1);
            assert_eq!(expected, value);
        }
        {
            let value = TgDecimalResult::new(Some(vec![4, 0xd2]), 999, -1);
            let value = TgDecimalI128::try_from(value).unwrap();
            let expected = TgDecimalI128::new(1234, -1);
            assert_eq!(expected, value);
        }
        {
            let value = TgDecimalResult::new(None, 1234, -1);
            let value = TgDecimalI128::try_from(value).unwrap();
            let expected = TgDecimalI128::new(1234, -1);
            assert_eq!(expected, value);
        }
    }

    #[test]
    fn from_decimal_result_to_decimal() {
        {
            let value = TgDecimalResult::new(Some(vec![4, 0xd2]), 999, -1);
            let value = TgDecimal::from(value);
            let expected = TgDecimal::new(vec![4, 0xd2], -1);
            assert_eq!(expected, value);
        }
        {
            let value = TgDecimalResult::new(None, 1234, -1);
            let value = TgDecimal::from(value);
            let expected = TgDecimal::new(vec![0, 0, 0, 0, 0, 0, 4, 0xd2], -1);
            assert_eq!(expected, value);
        }
    }
}
