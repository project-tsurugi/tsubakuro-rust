use std::time::Duration;

use bytes::{Buf, BytesMut};
use tonic::async_trait;

use crate::error::TgError;

#[async_trait]
pub(crate) trait LobDownloader {
    /// ## Returns
    /// - `Ok(0)` if the end of the stream is reached.
    async fn download_chunk(
        &mut self,
        buffer: &mut BytesMut,
        timeout: Duration,
    ) -> Result<usize, TgError>;
}

pub struct BlobDownloader {
    inner: Box<dyn LobDownloader>,
    buffer: BytesMut,
    eof: bool,
}

impl BlobDownloader {
    pub(crate) fn new(inner: Box<dyn LobDownloader>) -> BlobDownloader {
        BlobDownloader {
            inner,
            buffer: BytesMut::new(),
            eof: false,
        }
    }

    pub async fn download_chunk(
        &mut self,
        length: usize,
        timeout: Duration,
    ) -> Result<Option<Vec<u8>>, TgError> {
        if length == 0 {
            return Ok(Some(Vec::new()));
        }

        self.fill_buffer(length, timeout).await?;

        let buffer = &mut self.buffer;
        if buffer.is_empty() {
            return Ok(None);
        }

        let at = length.min(buffer.len());
        let chunk = buffer.split_to(at).to_vec();
        Ok(Some(chunk))
    }

    pub async fn download_chunk_into(
        &mut self,
        chunk: &mut [u8],
        timeout: Duration,
    ) -> Result<usize, TgError> {
        let length = chunk.len();
        if length == 0 {
            return Ok(0);
        }

        self.fill_buffer(length, timeout).await?;

        let buffer = &mut self.buffer;
        if buffer.is_empty() {
            return Ok(0);
        }

        let n = length.min(buffer.len());
        chunk[..n].copy_from_slice(&buffer[..n]);
        buffer.advance(n);
        Ok(n)
    }

    async fn fill_buffer(&mut self, length: usize, timeout: Duration) -> Result<(), TgError> {
        if self.eof {
            return Ok(());
        }

        if self.buffer.capacity() < length {
            self.buffer.reserve(length - self.buffer.len());
        }

        if self.buffer.len() < length {
            let len = self.inner.download_chunk(&mut self.buffer, timeout).await?;
            if len == 0 {
                self.eof = true;
            }
        }

        Ok(())
    }

    pub async fn is_eof(&mut self, timeout: Duration) -> Result<bool, TgError> {
        if !self.buffer.is_empty() {
            return Ok(false);
        }
        if self.eof {
            return Ok(true);
        }

        let len = self.inner.download_chunk(&mut self.buffer, timeout).await?;
        if len == 0 {
            self.eof = true;
        }

        Ok(self.eof && self.buffer.is_empty())
    }
}

pub struct ClobDownloader {
    inner: BlobDownloader,
}

impl ClobDownloader {
    pub(crate) fn new(inner: Box<dyn LobDownloader>) -> ClobDownloader {
        ClobDownloader {
            inner: BlobDownloader::new(inner),
        }
    }

    pub async fn download_chunk_utf8(
        &mut self,
        length: usize,
        timeout: Duration,
    ) -> Result<Option<Vec<u8>>, TgError> {
        self.inner.download_chunk(length, timeout).await
    }

    pub async fn download_chunk_into_utf8(
        &mut self,
        chunk: &mut [u8],
        timeout: Duration,
    ) -> Result<usize, TgError> {
        self.inner.download_chunk_into(chunk, timeout).await
    }

    pub async fn is_eof(&mut self, timeout: Duration) -> Result<bool, TgError> {
        self.inner.is_eof(timeout).await
    }
}
