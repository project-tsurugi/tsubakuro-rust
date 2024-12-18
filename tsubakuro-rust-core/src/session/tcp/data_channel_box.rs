use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{client_error, error::TgError};

use super::data_channel_wire::TcpDataChannelWire;

#[derive(Debug)]
pub(crate) struct TcpDataChannelBox {
    name_map: Mutex<HashMap<String, i32>>,
    wait_pool: Mutex<Vec<Option<Arc<TcpDataChannelWire>>>>,
}

impl TcpDataChannelBox {
    pub(crate) fn new() -> TcpDataChannelBox {
        TcpDataChannelBox {
            name_map: Mutex::new(HashMap::new()),
            wait_pool: Mutex::new(Vec::new()),
        }
    }
    pub(crate) fn set_data_channel_name(&self, dc_name: String, rs_slot: i32) {
        let mut map = self.name_map.lock().unwrap();
        map.insert(dc_name, rs_slot);
    }

    pub(crate) fn register_data_channel_wire(
        &self,
        dc_name: &String,
        dc_wire: Arc<TcpDataChannelWire>,
    ) -> Result<(), TgError> {
        let rs_slot = {
            let mut map = self.name_map.lock().unwrap();
            if let Some(rs_slot) = map.remove(dc_name) {
                rs_slot
            } else {
                return Err(client_error!(format!(
                    "data_channel_name({}) not found in TcpDataChannelBox.name_map",
                    dc_name
                )));
            }
        };

        let index = rs_slot as usize;
        let mut wait_pool = self.wait_pool.lock().unwrap();
        while wait_pool.len() <= index {
            wait_pool.push(None);
        }
        wait_pool[index] = Some(dc_wire);

        Ok(())
    }

    pub(crate) fn get_data_channel_wire(
        &self,
        rs_slot: i32,
    ) -> Result<Arc<TcpDataChannelWire>, TgError> {
        let index = rs_slot as usize;
        let wait_pool = self.wait_pool.lock().unwrap();
        if let Some(Some(dc_wire)) = wait_pool.get(index) {
            Ok(dc_wire.clone())
        } else {
            Err(client_error!(format!(
                "rs_slot {rs_slot} not found in TcpDataChannelBox.wait_pool"
            )))
        }
    }

    pub(crate) fn release_data_channel_wire(
        &self,
        rs_slot: i32,
    ) -> Result<Arc<TcpDataChannelWire>, TgError> {
        let index = rs_slot as usize;
        let mut wait_pool = self.wait_pool.lock().unwrap();
        if index < wait_pool.len() {
            let dc_wire = wait_pool[index].take().unwrap();
            Ok(dc_wire)
        } else {
            Err(client_error!(format!(
                "rs_slot {rs_slot} not found in TcpDataChannelBox.wait_pool"
            )))
        }
    }
}
