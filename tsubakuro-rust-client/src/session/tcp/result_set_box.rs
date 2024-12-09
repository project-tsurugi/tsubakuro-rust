use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{client_error, error::TgError};

use super::result_set_wire::TcpResultSetWire;

#[derive(Debug)]
pub(crate) struct TcpResultSetBox {
    name_map: Mutex<HashMap<String, i32>>,
    wait_pool: Mutex<Vec<Option<Arc<TcpResultSetWire>>>>,
}

impl TcpResultSetBox {
    pub(crate) fn new() -> TcpResultSetBox {
        TcpResultSetBox {
            name_map: Mutex::new(HashMap::new()),
            wait_pool: Mutex::new(Vec::new()),
        }
    }
    pub(crate) fn set_result_set_name(&self, result_set_name: String, rs_slot: i32) {
        let mut map = self.name_map.lock().unwrap();
        map.insert(result_set_name, rs_slot);
    }

    pub(crate) fn register_result_set_wire(
        &self,
        result_set_name: &String,
        rs_wire: Arc<TcpResultSetWire>,
    ) -> Result<(), TgError> {
        let rs_slot = {
            let mut map = self.name_map.lock().unwrap();
            if let Some(rs_slot) = map.remove(result_set_name) {
                rs_slot
            } else {
                return Err(client_error!(format!(
                    "result_set_name({}) not found in TcpResultSetBox.name_map",
                    result_set_name
                )));
            }
        };

        let index = rs_slot as usize;
        let mut rs_pool = self.wait_pool.lock().unwrap();
        while rs_pool.len() <= index {
            rs_pool.push(None);
        }
        rs_pool[index] = Some(rs_wire);

        Ok(())
    }

    pub(crate) fn get_result_set_wire(
        &self,
        rs_slot: i32,
    ) -> Result<Arc<TcpResultSetWire>, TgError> {
        let index = rs_slot as usize;
        let rs_pool = self.wait_pool.lock().unwrap();
        if let Some(Some(rs_wire)) = rs_pool.get(index) {
            Ok(rs_wire.clone())
        } else {
            Err(client_error!(format!(
                "rs_slot {rs_slot} not found in TcpResultSetBox.wait_pool"
            )))
        }
    }

    pub(crate) fn release_result_set_wire(
        &self,
        rs_slot: i32,
    ) -> Result<Arc<TcpResultSetWire>, TgError> {
        let index = rs_slot as usize;
        let mut rs_pool = self.wait_pool.lock().unwrap();
        if index < rs_pool.len() {
            let rs_wire = rs_pool[index].take().unwrap();
            Ok(rs_wire)
        } else {
            Err(client_error!(format!(
                "rs_slot {rs_slot} not found in TcpResultSetBox.wait_pool"
            )))
        }
    }
}
