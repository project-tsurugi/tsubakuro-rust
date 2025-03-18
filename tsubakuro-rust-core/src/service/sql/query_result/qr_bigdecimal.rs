use crate::{client_error, error::TgError, util::Timeout};
use async_trait::async_trait;
use log::trace;
use prost::bytes::BytesMut;
use std::time::Duration;

use super::{SqlQueryResult, SqlQueryResultFetch};

#[async_trait(?Send)] // thread unsafe
impl SqlQueryResultFetch<bigdecimal::BigDecimal> for SqlQueryResult {
    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch(&mut self) -> Result<bigdecimal::BigDecimal, TgError> {
        self.fetch_for(self.default_timeout).await
    }

    /// Retrieves a `DECIMAL` value on the column of the cursor position.
    ///
    /// You can only take once to retrieve the value on the column.
    async fn fetch_for(&mut self, timeout: Duration) -> Result<bigdecimal::BigDecimal, TgError> {
        let timeout = Timeout::new(timeout);
        let (coefficient_bytes, coefficient, exponent) =
            self.value_stream.fetch_decimal_value(&timeout).await?;
        big_decimal(coefficient_bytes, coefficient, -exponent)
    }
}

fn big_decimal(
    coefficient_bytes: Option<BytesMut>,
    coefficient: i64,
    scale: i32,
) -> Result<bigdecimal::BigDecimal, TgError> {
    use bigdecimal::FromPrimitive;

    let value = match coefficient_bytes {
        Some(coefficient) => {
            let value = bigdecimal::num_bigint::BigInt::from_signed_bytes_be(&coefficient);
            bigdecimal::BigDecimal::new(value, scale as i64)
        }
        None => {
            if scale == 0 {
                match bigdecimal::BigDecimal::from_i64(coefficient) {
                    Some(value) => value,
                    None => {
                        trace!(
                            "bigdecimal::BigDecimal::from_i64() error. coefficient={}",
                            coefficient
                        );
                        return Err(client_error!("bigdecimal::BigDecimal generate error"));
                    }
                }
            } else {
                let value = match bigdecimal::num_bigint::BigInt::from_i64(coefficient) {
                    Some(value) => value,
                    None => {
                        trace!(
                            "bigdecimal::BigInt::from_i64() error. coefficient={}",
                            coefficient
                        );
                        return Err(client_error!("bigdecimal::BigDecimal generate error"));
                    }
                };
                bigdecimal::BigDecimal::from_bigint(value, scale as i64)
            }
        }
    };
    Ok(value)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use prost::bytes::BytesMut;

    #[test]
    fn big_decimal() {
        big_decimal_test(Some(&[0]), 0, 0, "0");
        big_decimal_test(Some(&[0]), 0, 1, "0");
        big_decimal_test(Some(&[0]), 0, -1, "0");
        big_decimal_test(Some(&[0x04, 0xd2]), 0, 0, "1234.0");
        big_decimal_test(Some(&[0x04, 0xd2]), 0, 1, "123.4");
        big_decimal_test(Some(&[0x04, 0xd2]), 0, -1, "12340");
        big_decimal_test(Some(&[0xfb, 0x2e]), 0, 0, "-1234.0");
        big_decimal_test(Some(&[0xfb, 0x2e]), 0, 1, "-123.4");
        big_decimal_test(Some(&[0xfb, 0x2e]), 0, -1, "-12340");
        big_decimal_test(None, 0, 0, "0");
        big_decimal_test(None, 0, 1, "0");
        big_decimal_test(None, 0, -1, "0");
        big_decimal_test(None, 1234, 0, "1234.0");
        big_decimal_test(None, 1234, 1, "123.4");
        big_decimal_test(None, 1234, -1, "12340");
        big_decimal_test(None, -1234, 0, "-1234.0");
        big_decimal_test(None, -1234, 1, "-123.4");
        big_decimal_test(None, -1234, -1, "-12340");
    }

    fn big_decimal_test(
        coefficient_bytes: Option<&[u8]>,
        coefficient: i64,
        scale: i32,
        expected: &str,
    ) {
        let value = super::big_decimal(
            coefficient_bytes.map(|slice| BytesMut::from(slice)),
            coefficient,
            scale,
        )
        .unwrap();
        let expected = bigdecimal::BigDecimal::from_str(expected).unwrap();
        assert_eq!(expected, value);
    }
}
