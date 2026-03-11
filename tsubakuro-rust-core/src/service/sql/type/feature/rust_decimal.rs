use crate::prelude::{bytes_to_i128, TgDecimal};

impl From<rust_decimal::Decimal> for TgDecimal {
    fn from(value: rust_decimal::Decimal) -> Self {
        Self::from(&value)
    }
}

impl From<&rust_decimal::Decimal> for TgDecimal {
    fn from(value: &rust_decimal::Decimal) -> Self {
        TgDecimal {
            unscaled_value: value.mantissa().to_be_bytes().to_vec(),
            exponent: -(value.scale() as i32),
        }
    }
}

impl From<TgDecimal> for rust_decimal::Decimal {
    fn from(value: TgDecimal) -> Self {
        Self::from(&value)
    }
}

impl From<&TgDecimal> for rust_decimal::Decimal {
    fn from(value: &TgDecimal) -> Self {
        let unscaled_value = bytes_to_i128(&value.unscaled_value);
        let scale = (-value.exponent) as u32;
        Self::from_i128_with_scale(unscaled_value, scale)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tg_decimal_from_rust_decimal() {
        use std::str::FromStr;

        let value = rust_decimal::Decimal::from_str("123.4").unwrap();
        let actual: TgDecimal = value.into();
        let expected = TgDecimal::new(
            value.mantissa().to_be_bytes().to_vec(),
            -(value.scale() as i32),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn rust_decimal_from_tg_decimal() {
        use std::str::FromStr;

        let value = TgDecimal::new(vec![0x04, 0xd2], -1);

        let actual: rust_decimal::Decimal = value.into();
        let expected = rust_decimal::Decimal::from_str("123.4").unwrap();
        assert_eq!(expected, actual);
    }
}
