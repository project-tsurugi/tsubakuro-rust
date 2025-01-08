use std::{sync::atomic::AtomicBool, time::Duration};

use log::trace;
use prost::bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::Mutex,
};

use crate::{
    error::TgError,
    illegal_argument_error, io_error,
    prelude::{ConnectionOption, Endpoint},
    session::{tcp::r#enum::TcpRequestInfo, wire::link::LinkMessage},
};

use super::r#enum::TcpResponseInfo;

pub(crate) struct TcpLink {
    endpoint: Endpoint,
    reader: Mutex<Option<ReadHalf<TcpStream>>>, // recv()とclose()の排他（recv()を呼ぶ側で排他するので、recv()同士の排他は不要）
    writer: Mutex<Option<WriteHalf<TcpStream>>>, // send()系同士およびclose()の排他
    send_timeout: Duration,
    recv_timeout: Duration,
    broken: AtomicBool,
    closed: AtomicBool,
}

impl std::fmt::Debug for TcpLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpLink")
            .field("endpoint", &self.endpoint)
            .field("send_timeout", &self.send_timeout)
            .field("recv_timeout", &self.recv_timeout)
            .field("broken", &self.broken)
            .field("closed", &self.closed)
            .finish()
    }
}

impl TcpLink {
    pub(crate) async fn connect(
        endpoint: &Endpoint,
        connection_option: &ConnectionOption,
    ) -> Result<TcpLink, TgError> {
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
            send_timeout: connection_option.send_timeout(),
            recv_timeout: connection_option.recv_timeout(),
            broken: AtomicBool::new(false),
            closed: AtomicBool::new(false),
        })
    }

    pub(crate) async fn send(
        &self,
        slot: i32,
        frame_header: &Vec<u8>,
        payload: &Vec<u8>,
    ) -> Result<(), TgError> {
        self.check_close()?;

        let mut tcp_header = [0u8; 7];
        let length = frame_header.len() + payload.len();
        tcp_header[0] = TcpRequestInfo::RequestSessionPayload.into();
        tcp_header[1] = (slot & 0xff) as u8;
        tcp_header[2] = ((slot >> 8) & 0xff) as u8;
        tcp_header[3] = (length & 0xff) as u8;
        tcp_header[4] = ((length >> 8) & 0xff) as u8;
        tcp_header[5] = ((length >> 16) & 0xff) as u8;
        tcp_header[6] = ((length >> 24) & 0xff) as u8;

        let mut writer = self.writer.lock().await;
        self.check_broken()?; // check after lock
        let writer = writer.as_mut().ok_or(io_error!("TcpLink already closed"))?;

        let result = {
            let timeout = self.send_timeout;
            if timeout.is_zero() {
                Self::send_body(writer, &tcp_header, frame_header, payload).await
            } else {
                let result = tokio::time::timeout(
                    timeout,
                    Self::send_body(writer, &tcp_header, frame_header, payload),
                )
                .await;
                match result {
                    Ok(result) => result,
                    _ => Err(io_error!("TcpLink.send() timeout")),
                }
            }
        };
        if result.is_err() {
            self.set_broken(); // set during lock
        }
        result
    }

    async fn send_body(
        writer: &mut WriteHalf<TcpStream>,
        tcp_header: &[u8],
        frame_header: &Vec<u8>,
        payload: &Vec<u8>,
    ) -> Result<(), TgError> {
        writer
            .write_all(tcp_header)
            .await
            .map_err(|e| io_error!("TcpLink.send(): sned[tcp_header] error", e))?;
        writer
            .write_all(frame_header)
            .await
            .map_err(|e| io_error!("TcpLink.send(): sned[frame_header] error", e))?;
        writer
            .write_all(payload)
            .await
            .map_err(|e| io_error!("TcpLink.send(): sned[payload] error", e))?;
        writer
            .flush()
            .await
            .map_err(|e| io_error!("TcpLink.send(): flush error", e))
    }

    pub(crate) async fn send_header_only(
        &self,
        info: TcpRequestInfo,
        slot: i32,
    ) -> Result<(), TgError> {
        self.check_close()?;

        let mut tcp_header = [0u8; 7];
        tcp_header[0] = info.into();
        tcp_header[1] = (slot & 0xff) as u8;
        tcp_header[2] = ((slot >> 8) & 0xff) as u8;

        let mut writer = self.writer.lock().await;
        self.check_broken()?; // check after lock
        let writer = writer.as_mut().ok_or(io_error!("TcpLink already closed"))?;

        let result = {
            let timeout = self.send_timeout;
            if timeout.is_zero() {
                Self::send_header_only_body(writer, &tcp_header).await
            } else {
                let result =
                    tokio::time::timeout(timeout, Self::send_header_only_body(writer, &tcp_header))
                        .await;
                match result {
                    Ok(result) => result,
                    _ => Err(io_error!("TcpLink.send_header_only() timeout")),
                }
            }
        };
        if result.is_err() {
            self.set_broken(); // set during lock
        }
        result
    }

    async fn send_header_only_body(
        writer: &mut WriteHalf<TcpStream>,
        tcp_header: &[u8],
    ) -> Result<(), TgError> {
        writer
            .write_all(tcp_header)
            .await
            .map_err(|e| io_error!("TcpLink.send_header_only(): send[tcp_header] error", e))?;
        writer
            .flush()
            .await
            .map_err(|e| io_error!("TcpLink.send_header_only(): flush error", e))
    }

    pub(crate) async fn recv(&self) -> Result<Option<LinkMessage>, TgError> {
        self.check_close()?;

        let mut reader = match self.reader.try_lock() {
            Ok(reader) => reader,
            Err(_) => return Ok(None),
        };
        self.check_broken()?; // check after lock
        let reader = reader.as_mut().ok_or(io_error!("TcpLink already closed"))?;

        let result = {
            let timeout = self.recv_timeout;
            if timeout.is_zero() {
                Self::recv_body(reader).await
            } else {
                let result = tokio::time::timeout(timeout, Self::recv_body(reader)).await;
                match result {
                    Ok(result) => result,
                    Err(_) => Err(io_error!("TcpLink.recv() timeout")),
                }
            }
        };
        if result.is_err() {
            self.set_broken(); // set during lock
        }
        result
    }

    async fn recv_body(reader: &mut ReadHalf<TcpStream>) -> Result<Option<LinkMessage>, TgError> {
        let info = {
            let mut buffer = [0u8; 1];
            let read_length = reader
                .read(&mut buffer)
                .await
                .map_err(|e| io_error!("TcpLink.recv(): read[info] error", e))?;
            if read_length == 0 {
                return Ok(None);
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
                .map_err(|e| io_error!("TcpLink.recv(): read[writerId] error", e))?;
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

            let mut length = 0_usize;
            for i in 0..4 {
                length |= (buffer[i] as usize) << (i * 8);
            }
            if (length as i32) < 0 {
                return Err(io_error!(format!(
                    "TcpLink.recv(): read[length] error (length={})",
                    length as i32
                )));
            }
            length
        };

        let payload = if length > 0 {
            let mut buffer = BytesMut::with_capacity(length);
            unsafe { buffer.set_len(length) };
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
    }

    fn set_broken(&self) {
        self.broken.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn is_broken(&self) -> bool {
        self.broken.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn check_broken(&self) -> Result<(), TgError> {
        if self.is_broken() {
            Err(io_error!("TcpLink is broken"))
        } else {
            Ok(())
        }
    }

    pub(crate) async fn close(&self) -> Result<(), TgError> {
        if let Ok(_) = self.closed.compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
        ) {
            // FIXME send()/recv()の終了を待たずにNoneにする方法
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

    fn check_close(&self) -> Result<(), TgError> {
        if self.is_closed() {
            Err(io_error!("TcpLink already closed"))
        } else {
            Ok(())
        }
    }
}
