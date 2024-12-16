use std::{sync::atomic::AtomicBool, time::Duration};

use log::trace;
use prost::bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::Mutex,
};

use crate::{
    client_error,
    error::TgError,
    illegal_argument_error, io_error,
    prelude::Endpoint,
    session::{tcp::r#enum::TcpRequestInfo, wire::link::LinkMessage},
    timeout_error,
};

use super::r#enum::TcpResponseInfo;

pub(crate) struct TcpLink {
    endpoint: Endpoint,
    reader: Mutex<Option<ReadHalf<TcpStream>>>,
    writer: Mutex<Option<WriteHalf<TcpStream>>>,
    closed: AtomicBool,
}

impl std::fmt::Debug for TcpLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpLink")
            .field("endpoint", &self.endpoint)
            .finish()
    }
}

impl TcpLink {
    pub(crate) async fn connect(endpoint: &Endpoint) -> Result<TcpLink, TgError> {
        let addr = if let Endpoint::Tcp(host, port) = endpoint {
            format!("{host}:{port}")
        } else {
            return Err(illegal_argument_error!("endpoint is not TCP"));
        };

        trace!("TcpLink connect start");
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|e| io_error!("TcpLink connect error", e))?;
        trace!("TcpLink connect end");

        let (reader, writer) = tokio::io::split(stream);
        Ok(TcpLink {
            endpoint: endpoint.clone(),
            reader: Mutex::new(Some(reader)),
            writer: Mutex::new(Some(writer)),
            closed: AtomicBool::new(false),
        })
    }

    pub(crate) async fn send(
        &self,
        slot: i32,
        frame_header: &Vec<u8>,
        payload: &Vec<u8>,
        timeout: Duration,
    ) -> Result<(), TgError> {
        let mut tcp_header = [0u8; 7];
        let length = frame_header.len() + payload.len();
        tcp_header[0] = TcpRequestInfo::RequestSessionPayload.into();
        tcp_header[1] = (slot & 0xff) as u8;
        tcp_header[2] = ((slot >> 8) & 0xff) as u8;
        tcp_header[3] = (length & 0xff) as u8;
        tcp_header[4] = ((length >> 8) & 0xff) as u8;
        tcp_header[5] = ((length >> 16) & 0xff) as u8;
        tcp_header[6] = ((length >> 24) & 0xff) as u8;

        let result = tokio::time::timeout(timeout, async {
            let mut writer = self.writer.lock().await;
            let writer = writer
                .as_mut()
                .ok_or(client_error!("TcpLink already closed"))?;
            writer
                .write_all(&tcp_header)
                .await
                .map_err(|e| io_error!("TcpLink.send(): sned[tcp_header] error", e))?;
            writer
                .write_all(&frame_header)
                .await
                .map_err(|e| io_error!("TcpLink.send(): sned[frame_header] error", e))?;
            writer
                .write_all(&payload)
                .await
                .map_err(|e| io_error!("TcpLink.send(): sned[payload] error", e))?;
            writer
                .flush()
                .await
                .map_err(|e| io_error!("TcpLink.send(): flush error", e))
        })
        .await;
        match result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(timeout_error!("TcpLink.send(): timeout")),
        }
    }

    pub(crate) async fn send_header_only(
        &self,
        info: TcpRequestInfo,
        slot: i32,
        timeout: Duration,
    ) -> Result<(), TgError> {
        let mut tcp_header = [0u8; 7];
        tcp_header[0] = info.into();
        tcp_header[1] = (slot & 0xff) as u8;
        tcp_header[2] = ((slot >> 8) & 0xff) as u8;

        let result = tokio::time::timeout(timeout, async {
            let mut writer = self.writer.lock().await;
            let writer = writer
                .as_mut()
                .ok_or(client_error!("TcpLink already closed"))?;
            writer
                .write_all(&tcp_header)
                .await
                .map_err(|e| io_error!("TcpLink.send_header_only(): send[tcp_header] error", e))?;
            writer
                .flush()
                .await
                .map_err(|e| io_error!("TcpLink.send_header_only(): flush error", e))
        })
        .await;
        match result {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(timeout_error!("TcpLink.send_header_only(): timeout")),
        }
    }

    pub(crate) async fn recv(&self, timeout: Duration) -> Result<Option<LinkMessage>, TgError> {
        let mut reader = {
            match self.reader.try_lock() {
                Ok(reader) => reader,
                Err(_) => return Ok(None),
            }
        };
        let reader = reader
            .as_mut()
            .ok_or(client_error!("TcpLink already closed"))?;
        let result = tokio::time::timeout(timeout, async {
            let info = {
                let mut buffer = [0u8; 1];
                let read_length = reader
                    .read_exact(&mut buffer)
                    .await
                    .map_err(|e| io_error!("TcpLink.recv(): read[info] error", e))?;
                if read_length == 0 {
                    return Ok::<Option<LinkMessage>, TgError>(None);
                }
                buffer[0]
            };

            let slot = {
                let mut buffer = [0u8; 2];
                reader
                    .read_exact(&mut buffer)
                    .await
                    .map_err(|e| io_error!("TcpLink.recv(): read[slot] error", e))?;
                (buffer[0] as i32) | ((buffer[1] as i32) << 8)
            };

            let writer = if info == TcpResponseInfo::ResponseResultSetPayload.value() {
                let mut buffer = [0u8; 1];
                reader
                    .read_exact(&mut buffer)
                    .await
                    .map_err(|e| io_error!("TcpLink.recv(): read[writer] error", e))?;
                buffer[0]
            } else {
                0u8
            };

            let length = {
                let mut buffer = [0u8; 4];
                reader
                    .read_exact(&mut buffer)
                    .await
                    .map_err(|e| io_error!("TcpLink.recv(): read[length] error", e))?;

                let mut length = 0;
                for i in 0..3 {
                    length |= (buffer[i] as i32) << (i * 8);
                }
                length
            };

            let payload = if length > 0 {
                // TODO 指定サイズ読む効率の良い方法
                let mut buffer = BytesMut::with_capacity(length as usize);
                buffer.resize(length as usize, 0);
                reader
                    .read_exact(&mut buffer)
                    .await
                    .map_err(|e| io_error!("TcpLink.recv(): read[payload] error", e))?;
                Some(buffer)
            } else {
                None
            };

            let link_message = LinkMessage::new(info, payload, slot, writer);
            Ok(Some(link_message))
        })
        .await;
        match result {
            Ok(Ok(message)) => Ok(message),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(timeout_error!("TcpLink.recv(): timeout")),
        }
    }

    pub(crate) async fn close(&self) -> Result<(), TgError> {
        if let Ok(_) = self.closed.compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
        ) {
            {
                let mut writer = self.writer.lock().await;
                writer.take(); // upadte to None
            }
            {
                let mut reader = self.reader.lock().await;
                reader.take(); // update to None
            }
        }
        Ok(())
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::SeqCst)
    }
}
