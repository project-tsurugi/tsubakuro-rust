use crate::{error::TgError, util::Timeout};
use async_trait::async_trait;
use prost::bytes::BytesMut;
use std::time::Duration;

use super::{SqlQueryResult, SqlQueryResultFetch};

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<rust_decimal::Decimal> for SqlQueryResult {
    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<rust_decimal::Decimal, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<rust_decimal::Decimal, TgError> {
        let timeout = Timeout::new(timeout);
        let (coefficient_bytes, coefficient, exponent) =
            self.value_stream.fetch_decimal_value(&timeout).await?;
        decimal(coefficient_bytes, coefficient, -exponent)
    }
}

fn decimal(
    coefficient_bytes: Option<BytesMut>,
    coefficient: i64,
    scale: i32,
) -> Result<rust_decimal::Decimal, TgError> {
    let value = match coefficient_bytes {
        Some(coefficient) => {
            let top = coefficient[0] as i8;
            let mut buf = if top >= 0 { [0u8; 16] } else { [0xffu8; 16] };
            buf[16 - coefficient.len()..].copy_from_slice(&coefficient);
            i128::from_be_bytes(buf)
        }
        None => coefficient as i128,
    };
    let value = if scale >= 0 {
        rust_decimal::Decimal::from_i128_with_scale(value, scale as u32)
    } else {
        let value = rust_decimal::Decimal::from_i128_with_scale(value, 0);
        let factor = rust_decimal::Decimal::from_i128_with_scale(10_i128.pow(-scale as u32), 0);
        value * factor
    };
    Ok(value)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use prost::bytes::BytesMut;

    #[test]
    fn decimal() {
        decimal_test(Some(&[0]), 0, 0, "0");
        decimal_test(Some(&[0]), 0, 1, "0");
        decimal_test(Some(&[0]), 0, -1, "0");
        decimal_test(Some(&[0x04, 0xd2]), 0, 0, "1234.0");
        decimal_test(Some(&[0x04, 0xd2]), 0, 1, "123.4");
        decimal_test(Some(&[0x04, 0xd2]), 0, -1, "12340");
        decimal_test(Some(&[0xfb, 0x2e]), 0, 0, "-1234.0");
        decimal_test(Some(&[0xfb, 0x2e]), 0, 1, "-123.4");
        decimal_test(Some(&[0xfb, 0x2e]), 0, -1, "-12340");
        decimal_test(None, 0, 0, "0");
        decimal_test(None, 0, 1, "0");
        decimal_test(None, 0, -1, "0");
        decimal_test(None, 1234, 0, "1234.0");
        decimal_test(None, 1234, 1, "123.4");
        decimal_test(None, 1234, -1, "12340");
        decimal_test(None, -1234, 0, "-1234.0");
        decimal_test(None, -1234, 1, "-123.4");
        decimal_test(None, -1234, -1, "-12340");
    }

    fn decimal_test(
        coefficient_bytes: Option<&[u8]>,
        coefficient: i64,
        scale: i32,
        expected: &str,
    ) {
        let value = super::decimal(
            coefficient_bytes.map(|slice| BytesMut::from(slice)),
            coefficient,
            scale,
        )
        .unwrap();
        let expected = rust_decimal::Decimal::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }
}
