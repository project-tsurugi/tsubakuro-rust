use std::{fmt::Debug, sync::Mutex};

use prost::bytes::BytesMut;

#[derive(Debug)]
pub(crate) struct LinkMessage {
    slot: i32,
    info: u8,
    payload: Mutex<Option<BytesMut>>,
    writer: u8,
}

impl LinkMessage {
    pub fn new(info: u8, payload: Option<BytesMut>, slot: i32, writer: u8) -> LinkMessage {
        LinkMessage {
            slot,
            info,
            payload: Mutex::new(payload),
            writer,
        }
    }

    pub fn slot(&self) -> i32 {
        self.slot
    }

    pub fn info(&self) -> u8 {
        self.info
    }

    pub fn take_payload(&self) -> Option<BytesMut> {
        let mut payload = self.payload.lock().unwrap();
        payload.take()
    }

    pub fn writer(&self) -> u8 {
        self.writer
    }
}
