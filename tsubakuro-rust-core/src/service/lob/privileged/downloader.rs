use std::{path::Path, time::Duration};

use bytes::BytesMut;
use tonic::async_trait;

use crate::{error::TgError, io_error, service::lob::downloader::LobDownloader};

pub(crate) struct PrivilegedLobDownloader {
    file: tokio::fs::File,
}

impl PrivilegedLobDownloader {
    pub(crate) async fn new(path: &Path) -> Result<Self, TgError> {
        let file = tokio::fs::File::open(path)
            .await
            .map_err(|e| io_error!("Failed to open file", e))?;

        Ok(Self { file })
    }
}

#[async_trait]
impl LobDownloader for PrivilegedLobDownloader {
    async fn download_chunk(
        &mut self,
        buffer: &mut BytesMut,
        _timeout: Duration,
    ) -> Result<usize, TgError> {
        use tokio::io::AsyncReadExt;

        let len = self
            .file
            .read_buf(buffer)
            .await
            .map_err(|e| io_error!("Failed to read from file", e))?;
        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::Write;
    use std::time::Duration;
    use tempfile::NamedTempFile;

    use crate::service::lob::downloader::BlobDownloader;

    #[tokio::test]
    async fn download_empty_file() -> Result<(), TgError> {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"").unwrap();

        let timeout = Duration::ZERO;

        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            let r = downloader.download_chunk(16, timeout).await?;
            assert_eq!(None, r);
        }
        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            assert_eq!(true, downloader.is_eof(timeout).await?);
            let r = downloader.download_chunk(16, timeout).await?;
            assert_eq!(None, r);
            assert_eq!(true, downloader.is_eof(timeout).await?);
        }
        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            assert_eq!(true, downloader.is_eof(timeout).await?);
            let buf = &mut [0u8; 16];
            let len = downloader.download_chunk_into(buf, timeout).await?;
            assert_eq!(0, len);
            assert_eq!(true, downloader.is_eof(timeout).await?);
        }

        Ok(())
    }

    #[tokio::test]
    async fn download_file1() -> Result<(), TgError> {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"abc").unwrap();

        let timeout = Duration::ZERO;

        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            let r = downloader.download_chunk(16, timeout).await?;
            assert_eq!(Some(b"abc".to_vec()), r);
            let r = downloader.download_chunk(16, timeout).await?;
            assert_eq!(None, r);
        }
        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            assert_eq!(false, downloader.is_eof(timeout).await?);
            let r = downloader.download_chunk(16, timeout).await?;
            assert_eq!(Some(b"abc".to_vec()), r);
            assert_eq!(true, downloader.is_eof(timeout).await?);
            let r = downloader.download_chunk(16, timeout).await?;
            assert_eq!(None, r);
            assert_eq!(true, downloader.is_eof(timeout).await?);
        }
        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            assert_eq!(false, downloader.is_eof(timeout).await?);
            let buf = &mut [0u8; 16];
            let len = downloader.download_chunk_into(buf, timeout).await?;
            assert_eq!(3, len);
            assert_eq!(b"abc", &buf[..len]);
            assert_eq!(true, downloader.is_eof(timeout).await?);
            let len = downloader.download_chunk_into(buf, timeout).await?;
            assert_eq!(0, len);
            assert_eq!(true, downloader.is_eof(timeout).await?);
        }

        Ok(())
    }

    #[tokio::test]
    async fn download_file2() -> Result<(), TgError> {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"0123456789abcdef").unwrap();

        let timeout = Duration::ZERO;

        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            let r = downloader.download_chunk(8, timeout).await?;
            assert_eq!(Some(b"01234567".to_vec()), r);
            let r = downloader.download_chunk(8, timeout).await?;
            assert_eq!(Some(b"89abcdef".to_vec()), r);
            let r = downloader.download_chunk(8, timeout).await?;
            assert_eq!(None, r);
        }
        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            assert_eq!(false, downloader.is_eof(timeout).await?);
            let r = downloader.download_chunk(8, timeout).await?;
            assert_eq!(Some(b"01234567".to_vec()), r);
            assert_eq!(false, downloader.is_eof(timeout).await?);
            let r = downloader.download_chunk(8, timeout).await?;
            assert_eq!(Some(b"89abcdef".to_vec()), r);
            assert_eq!(true, downloader.is_eof(timeout).await?);
            let r = downloader.download_chunk(8, timeout).await?;
            assert_eq!(None, r);
            assert_eq!(true, downloader.is_eof(timeout).await?);
        }
        {
            let target = PrivilegedLobDownloader::new(file.path()).await?;
            let mut downloader = BlobDownloader::new(Box::new(target));

            assert_eq!(false, downloader.is_eof(timeout).await?);
            let buf = &mut [0u8; 8];
            let len = downloader.download_chunk_into(buf, timeout).await?;
            assert_eq!(8, len);
            assert_eq!(b"01234567", &buf[..len]);
            assert_eq!(false, downloader.is_eof(timeout).await?);
            let len = downloader.download_chunk_into(buf, timeout).await?;
            assert_eq!(8, len);
            assert_eq!(b"89abcdef", &buf[..len]);
            assert_eq!(true, downloader.is_eof(timeout).await?);
            let len = downloader.download_chunk_into(buf, timeout).await?;
            assert_eq!(0, len);
            assert_eq!(true, downloader.is_eof(timeout).await?);
        }

        Ok(())
    }
}
