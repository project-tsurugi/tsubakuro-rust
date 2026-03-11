use crate::prelude::{bytes_to_i128, TgDecimal};

impl From<bigdecimal::BigDecimal> for TgDecimal {
    fn from(value: bigdecimal::BigDecimal) -> Self {
        Self::from(&value)
    }
}

impl From<&bigdecimal::BigDecimal> for TgDecimal {
    fn from(value: &bigdecimal::BigDecimal) -> Self {
        let (value, scale) = value.as_bigint_and_exponent();
        TgDecimal {
            unscaled_value: value.to_signed_bytes_be(),
            exponent: -scale as i32,
        }
    }
}

impl From<TgDecimal> for bigdecimal::BigDecimal {
    fn from(value: TgDecimal) -> Self {
        Self::from(&value)
    }
}

impl From<&TgDecimal> for bigdecimal::BigDecimal {
    fn from(value: &TgDecimal) -> Self {
        let unscaled_value = bytes_to_i128(&value.unscaled_value);
        let scale = (-value.exponent) as i64;
        Self::new(unscaled_value.into(), scale)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tg_decimal_from_bigdecimal() {
        use std::str::FromStr;

        let value = bigdecimal::BigDecimal::from_str("123.4").unwrap();
        let actual: TgDecimal = (&value).into();

        let expected = TgDecimal::new(
            value.as_bigint_and_exponent().0.to_signed_bytes_be(),
            -(value.as_bigint_and_exponent().1 as i32),
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn bigdecimal_from_tg_decimal() {
        use std::str::FromStr;

        let value = TgDecimal::new(vec![0x04, 0xd2], -1);

        let actual: bigdecimal::BigDecimal = (&value).into();
        let expected = bigdecimal::BigDecimal::from_str("123.4").unwrap();
        assert_eq!(expected, actual);
    }
}
