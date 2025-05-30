use crate::{
    client_error, error::TgError, session::wire::data_channel::DataChannel, util::Timeout,
};

pub(crate) struct Base128Variant {}

impl Base128Variant {
    pub(crate) async fn read_unsigned(
        stream: &mut DataChannel,
        timeout: &Timeout,
    ) -> Result<u64, TgError> {
        let mut result = 0_u64;
        for i in 0..8 {
            let c = Self::read_u8(stream, timeout).await?;
            result |= ((c & 0x7f) as u64) << (i * 7);

            if (c & 0x80) == 0 {
                return Ok(result);
            }
        }

        let c = Self::read_u8(stream, timeout).await?;
        result |= (c as u64) << 56;
        Ok(result)
    }

    pub(crate) async fn read_signed(
        stream: &mut DataChannel,
        timeout: &Timeout,
    ) -> Result<i64, TgError> {
        let v = Self::read_unsigned(stream, timeout).await?;
        let r = if (v & 0x01) == 0 { v >> 1 } else { !(v >> 1) };
        Ok(r as i64)
    }

    async fn read_u8(stream: &mut DataChannel, timeout: &Timeout) -> Result<u8, TgError> {
        if let Some(c) = stream.read_u8(timeout).await? {
            Ok(c)
        } else {
            Err(client_error!("saw unexpected eof"))
        }
    }
}
