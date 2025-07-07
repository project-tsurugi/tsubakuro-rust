use std::str::FromStr;

use rust_decimal::prelude::FromPrimitive;
use tsubakuro_rust_core::prelude::{TgDecimal, TgDecimalI128, TgDecimalResult};

use crate::ctype::{SqlChar, SqlSChar};

pub const SQL_MAX_NUMERIC_LEN: usize = 16;

#[repr(C)]
#[derive(Debug)]
pub struct SqlNumericStruct {
    precision: SqlChar,
    scale: SqlSChar,
    sign: SqlChar, // 1 if positive, 0 if negative
    val: [SqlChar; SQL_MAX_NUMERIC_LEN],
}

impl std::fmt::Display for SqlNumericStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = TgDecimalI128::from(self);
        write!(f, "{}", value)
    }
}

impl SqlNumericStruct {
    fn new(
        precision: SqlChar,
        scale: SqlSChar,
        sign: SqlChar,
        val: [SqlChar; SQL_MAX_NUMERIC_LEN],
    ) -> SqlNumericStruct {
        SqlNumericStruct {
            precision,
            scale,
            sign,
            val,
        }
    }

    fn exponent(&self) -> i32 {
        -self.scale as i32
    }

    fn positive(&self) -> bool {
        self.sign != 0
    }

    fn unscaled_value(&self) -> [u8; 16] {
        if self.positive() {
            let mut be = [0u8; 16];
            for i in 0..SQL_MAX_NUMERIC_LEN {
                be[15 - i] = self.val[i] as u8;
            }
            be
        } else {
            let n = -i128::from_le_bytes(self.val);
            n.to_be_bytes()
        }
    }

    fn unscaled_value_i128(&self) -> i128 {
        if self.positive() {
            i128::from_le_bytes(self.val)
        } else {
            -i128::from_le_bytes(self.val)
        }
    }
}

impl From<TgDecimalResult> for SqlNumericStruct {
    fn from(value: TgDecimalResult) -> Self {
        let precision = 38 as SqlChar; // TODO precision
        let scale = -value.exponent as SqlSChar;
        let bytes = match value.unscaled_value_bytes {
            Some(bytes) => bytes,
            None => value.unscaled_value.to_be_bytes().to_vec(),
        };

        let size = bytes.len();
        if size == 0 {
            let val = [0 as SqlChar; SQL_MAX_NUMERIC_LEN];
            return SqlNumericStruct::new(precision, scale, 1, val);
        }

        if (bytes[0] as i8) >= 0 {
            let mut val = [0 as SqlChar; SQL_MAX_NUMERIC_LEN];
            for (i, byte) in bytes.iter().rev().enumerate() {
                val[i] = *byte;
            }
            SqlNumericStruct::new(precision, scale, 1, val)
        } else {
            let mut buf = [0xffu8; 16];
            buf[16 - bytes.len()..].copy_from_slice(&bytes);
            let v = -i128::from_be_bytes(buf);
            let val = v.to_le_bytes();
            assert_eq!(SQL_MAX_NUMERIC_LEN, val.len());
            SqlNumericStruct::new(precision, scale, 0, val)
        }
    }
}

impl TryFrom<&str> for SqlNumericStruct {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = TgDecimalI128::from_str(&value)?;
        let unscaled_value = value.unscaled_value;
        let scale = -value.exponent as i8;
        let value = if unscaled_value >= 0 {
            let val = unscaled_value.to_le_bytes();
            SqlNumericStruct::new(38, scale, 1, val)
        } else {
            let val = (-unscaled_value).to_le_bytes();
            SqlNumericStruct::new(38, scale, 0, val)
        };
        Ok(value)
    }
}

impl From<i128> for SqlNumericStruct {
    fn from(value: i128) -> Self {
        let precision = 38;
        let (sign, value) = if value >= 0 { (1, value) } else { (0, -value) };
        let scale = 0;
        let val = value.to_le_bytes();
        SqlNumericStruct::new(precision, scale, sign, val)
    }
}

impl TryFrom<f32> for SqlNumericStruct {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let precision = 38;
        if value == 0f32 {
            return Ok(SqlNumericStruct::new(precision, 0, 1, [0u8; 16]));
        }
        let value = match rust_decimal::Decimal::from_f32(value) {
            Some(value) => value,
            None => return Err(format!("Decimal convert error. value={value}").into()),
        };

        Ok(value.into())
    }
}

impl TryFrom<f64> for SqlNumericStruct {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == 0f64 {
            return Ok(SqlNumericStruct::new(38, 0, 1, [0u8; 16]));
        }
        let value = match rust_decimal::Decimal::from_f64(value) {
            Some(value) => value,
            None => return Err(format!("Decimal convert error. value={value}").into()),
        };

        Ok(value.into())
    }
}

impl From<rust_decimal::Decimal> for SqlNumericStruct {
    fn from(value: rust_decimal::Decimal) -> Self {
        let precision = 38;
        let scale = value.scale() as i8;
        let sign = if value.is_sign_negative() { 0 } else { 1 };
        let val = value.mantissa().abs().to_le_bytes();
        SqlNumericStruct::new(precision, scale, sign, val)
    }
}

impl From<&SqlNumericStruct> for TgDecimal {
    fn from(value: &SqlNumericStruct) -> Self {
        let unscaled_value = value.unscaled_value().to_vec();
        let exponent = value.exponent();
        TgDecimal::new(unscaled_value, exponent)
    }
}

impl From<&SqlNumericStruct> for TgDecimalI128 {
    fn from(value: &SqlNumericStruct) -> Self {
        let unscaled_value = value.unscaled_value_i128();
        let exponent = value.exponent();
        TgDecimalI128::new(unscaled_value, exponent)
    }
}

impl From<&SqlNumericStruct> for i128 {
    fn from(value: &SqlNumericStruct) -> Self {
        let unscaled_value = value.unscaled_value_i128();
        let scale = value.scale;
        if scale == 0 {
            unscaled_value
        } else if scale > 0 {
            unscaled_value / 10_i128.pow(scale as u32)
        } else {
            let exponent = -scale;
            unscaled_value * 10_i128.pow(exponent as u32)
        }
    }
}

impl From<&SqlNumericStruct> for f32 {
    fn from(value: &SqlNumericStruct) -> Self {
        let unscaled_value = value.unscaled_value_i128() as f32;
        let exponent = value.exponent();
        if exponent == 0 {
            unscaled_value
        } else {
            unscaled_value * 10_f32.powi(exponent)
        }
    }
}

impl From<&SqlNumericStruct> for f64 {
    fn from(value: &SqlNumericStruct) -> Self {
        let unscaled_value = value.unscaled_value_i128() as f64;
        let exponent = value.exponent();
        if exponent == 0 {
            unscaled_value
        } else {
            unscaled_value * 10_f64.powi(exponent)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_decimal_result() {
        {
            let src = TgDecimalResult::new(None, 123456, 0);
            let value = SqlNumericStruct::from(src);
            assert_eq!(38, value.precision);
            assert_eq!(0, value.scale);
            assert_eq!(1, value.sign);
            assert_eq!(123456, i128::from_le_bytes(value.val));
        }
        {
            let src = TgDecimalResult::new(None, 123456, -3);
            let value = SqlNumericStruct::from(src);
            assert_eq!(38, value.precision);
            assert_eq!(3, value.scale);
            assert_eq!(1, value.sign);
            assert_eq!(123456, i128::from_le_bytes(value.val));
        }
        {
            let src = TgDecimalResult::new(None, -123456, -3);
            let value = SqlNumericStruct::from(src);
            assert_eq!(38, value.precision);
            assert_eq!(3, value.scale);
            assert_eq!(0, value.sign);
            assert_eq!(123456, i128::from_le_bytes(value.val));
        }

        {
            let src = TgDecimalResult::new(Some(vec![]), 0, -3);
            let value = SqlNumericStruct::from(src);
            assert_eq!(38, value.precision);
            assert_eq!(3, value.scale);
            assert_eq!(1, value.sign);
            assert_eq!(0, i128::from_le_bytes(value.val));
        }
        {
            let src = TgDecimalResult::new(Some(123456_i32.to_be_bytes().to_vec()), 0, 0);
            let value = SqlNumericStruct::from(src);
            assert_eq!(38, value.precision);
            assert_eq!(0, value.scale);
            assert_eq!(1, value.sign);
            assert_eq!(123456, i128::from_le_bytes(value.val));
        }
        {
            let src = TgDecimalResult::new(Some((-123456_i32).to_be_bytes().to_vec()), 0, 3);
            let value = SqlNumericStruct::from(src);
            assert_eq!(38, value.precision);
            assert_eq!(-3, value.scale);
            assert_eq!(0, value.sign);
            assert_eq!(123456, i128::from_le_bytes(value.val));
        }
    }

    #[test]
    fn for_decimal() {
        {
            let src = SqlNumericStruct::new(38, 0, 1, 0_i128.to_le_bytes());
            let value = TgDecimal::from(&src);

            assert_eq!(
                0_i128,
                i128::from_be_bytes(value.unscaled_value.try_into().unwrap())
            );
            assert_eq!(0, value.exponent);
        }
        {
            let src = SqlNumericStruct::new(38, 0, 1, 12345_i128.to_le_bytes());
            let value = TgDecimal::from(&src);

            assert_eq!(
                12345_i128,
                i128::from_be_bytes(value.unscaled_value.try_into().unwrap())
            );
            assert_eq!(0, value.exponent);
        }
        {
            let src = SqlNumericStruct::new(38, 2, 1, 12345_i128.to_le_bytes());
            let value = TgDecimal::from(&src);

            assert_eq!(
                12345_i128,
                i128::from_be_bytes(value.unscaled_value.try_into().unwrap())
            );
            assert_eq!(-2, value.exponent);
        }
        {
            let src = SqlNumericStruct::new(38, -2, 1, 12345_i128.to_le_bytes());
            let value = TgDecimal::from(&src);

            assert_eq!(
                12345_i128,
                i128::from_be_bytes(value.unscaled_value.try_into().unwrap())
            );
            assert_eq!(2, value.exponent);
        }
        {
            let src = SqlNumericStruct::new(38, 0, 0, 12345_i128.to_le_bytes());
            let value = TgDecimal::from(&src);

            assert_eq!(
                -12345_i128,
                i128::from_be_bytes(value.unscaled_value.try_into().unwrap())
            );
            assert_eq!(0, value.exponent);
        }
        {
            let src = SqlNumericStruct::new(38, 2, 0, 12345_i128.to_le_bytes());
            let value = TgDecimal::from(&src);

            assert_eq!(
                -12345_i128,
                i128::from_be_bytes(value.unscaled_value.try_into().unwrap())
            );
            assert_eq!(-2, value.exponent);
        }
        {
            let src = SqlNumericStruct::new(38, -2, 0, 12345_i128.to_le_bytes());
            let value = TgDecimal::from(&src);

            assert_eq!(
                -12345_i128,
                i128::from_be_bytes(value.unscaled_value.try_into().unwrap())
            );
            assert_eq!(2, value.exponent);
        }
    }

    #[test]
    fn for_i128() {
        {
            let src = SqlNumericStruct::new(38, 0, 1, 0_i128.to_le_bytes());
            let value = i128::from(&src);

            assert_eq!(0_i128, value);
        }
        {
            let src = SqlNumericStruct::new(38, 0, 1, 12345_i128.to_le_bytes());
            let value = i128::from(&src);

            assert_eq!(12345_i128, value);
        }
        {
            let src = SqlNumericStruct::new(38, 2, 1, 12345_i128.to_le_bytes());
            let value = i128::from(&src);

            assert_eq!(123_i128, value);
        }
        {
            let src = SqlNumericStruct::new(38, -2, 1, 12345_i128.to_le_bytes());
            let value = i128::from(&src);

            assert_eq!(1234500_i128, value);
        }
        {
            let src = SqlNumericStruct::new(38, 0, 0, 12345_i128.to_le_bytes());
            let value = i128::from(&src);

            assert_eq!(-12345_i128, value);
        }
        {
            let src = SqlNumericStruct::new(38, 2, 0, 12345_i128.to_le_bytes());
            let value = i128::from(&src);

            assert_eq!(-123_i128, value);
        }
        {
            let src = SqlNumericStruct::new(38, -2, 0, 12345_i128.to_le_bytes());
            let value = i128::from(&src);

            assert_eq!(-1234500_i128, value);
        }
    }

    #[test]
    fn for_f64() {
        {
            let src = SqlNumericStruct::new(38, 0, 1, 0_i128.to_le_bytes());
            let value = f64::from(&src);

            assert_eq!(0_f64, value);
        }
        {
            let src = SqlNumericStruct::new(38, 0, 1, 12345_i128.to_le_bytes());
            let value = f64::from(&src);

            assert_eq!(12345_f64, value);
        }
        {
            let src = SqlNumericStruct::new(38, 2, 1, 12345_i128.to_le_bytes());
            let value = f64::from(&src);

            assert_eq!(123.45_f64, value);
        }
        {
            let src = SqlNumericStruct::new(38, -2, 1, 12345_i128.to_le_bytes());
            let value = f64::from(&src);

            assert_eq!(1234500_f64, value);
        }
        {
            let src = SqlNumericStruct::new(38, 0, 0, 12345_i128.to_le_bytes());
            let value = f64::from(&src);

            assert_eq!(-12345_f64, value);
        }
        {
            let src = SqlNumericStruct::new(38, 2, 0, 12345_i128.to_le_bytes());
            let value = f64::from(&src);

            assert_eq!(-123.45_f64, value);
        }
        {
            let src = SqlNumericStruct::new(38, -2, 0, 12345_i128.to_le_bytes());
            let value = f64::from(&src);

            assert_eq!(-1234500_f64, value);
        }
    }
}
