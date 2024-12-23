use std::{
    collections::VecDeque,
    fmt::Debug,
    sync::{atomic::AtomicI32, Arc, Mutex},
};

use log::debug;

use super::response::WireResponse;

const SLOT_SIZE: i32 = 8;

#[derive(Debug)]
pub(crate) struct SlotEntry {
    slot: i32,
}

impl SlotEntry {
    pub fn slot(&self) -> i32 {
        self.slot
    }
}

pub(crate) struct SlotEntryHandle {
    slot: i32,
    response_box: Arc<ResponseBox>,
    slot_entry: Option<SlotEntry>,
    response: Mutex<VecDeque<WireResponse>>,
}

impl Debug for SlotEntryHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SlotEntryHandle")
            .field("slot", &self.slot)
            .field("slot_entry", &self.slot_entry)
            .finish()
    }
}

impl SlotEntryHandle {
    pub(crate) fn new(response_box: Arc<ResponseBox>, slot_entry: SlotEntry) -> SlotEntryHandle {
        SlotEntryHandle {
            slot: slot_entry.slot(),
            response_box,
            slot_entry: Some(slot_entry),
            response: Mutex::new(VecDeque::with_capacity(2)),
        }
    }

    pub(crate) fn slot(&self) -> i32 {
        self.slot
    }

    pub(crate) fn set_wire_response(&self, response: WireResponse) {
        let mut queue = self.response.lock().unwrap();
        queue.push_back(response);
    }

    pub(crate) fn take_wire_response(&self) -> Option<WireResponse> {
        let mut queue = self.response.lock().unwrap();
        queue.pop_front()
    }

    pub(crate) fn exists_wire_response(&self) -> bool {
        let queue = self.response.lock().unwrap();
        !queue.is_empty()
    }
}

impl Drop for SlotEntryHandle {
    fn drop(&mut self) {
        let slot_entry = self.slot_entry.take();
        if let Some(slot_entry) = slot_entry {
            self.response_box.release_slot_entry(slot_entry);
        }
    }
}

pub(crate) struct ResponseBox {
    slot_max: AtomicI32,
    slot_pool: Mutex<VecDeque<SlotEntry>>,
    recv_wait_pool: Mutex<Vec<Option<Arc<SlotEntryHandle>>>>,
}

impl Debug for ResponseBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseBox")
            .field("slot_max", &self.slot_max)
            .finish()
    }
}

impl ResponseBox {
    pub(crate) fn new() -> ResponseBox {
        Self::with_capacity(SLOT_SIZE)
    }

    pub(crate) fn with_capacity(slot_size: i32) -> ResponseBox {
        let response_box = ResponseBox {
            slot_max: AtomicI32::new(0),
            slot_pool: Mutex::new(VecDeque::with_capacity(slot_size as usize)),
            recv_wait_pool: Mutex::new(Vec::with_capacity(slot_size as usize)),
        };

        for _ in 0..slot_size {
            let slot_entry = response_box.new_slot_entry();
            let mut slot_pool = response_box.slot_pool.lock().unwrap();
            slot_pool.push_back(slot_entry);
        }

        response_box
    }

    fn get_available_slot_entry(&self) -> SlotEntry {
        let slot_entry = {
            let mut slot_pool = self.slot_pool.lock().unwrap();
            slot_pool.pop_front()
        };
        if let Some(slot_entry) = slot_entry {
            return slot_entry;
        }

        self.new_slot_entry()
    }

    fn new_slot_entry(&self) -> SlotEntry {
        let slot = self
            .slot_max
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        {
            let mut recv_wait_pool = self.recv_wait_pool.lock().unwrap();
            recv_wait_pool.push(None);
        }

        SlotEntry { slot }
    }

    pub(crate) fn create_slot_handle(self: Arc<ResponseBox>) -> Arc<SlotEntryHandle> {
        let slot_entry = self.get_available_slot_entry();

        let slot_handle = SlotEntryHandle::new(self.clone(), slot_entry);
        let slot_handle = Arc::new(slot_handle);

        self.set_slot_handle_to_wait_pool(slot_handle.clone());

        slot_handle
    }

    fn set_slot_handle_to_wait_pool(&self, slot_handle: Arc<SlotEntryHandle>) {
        let index = slot_handle.slot() as usize;

        let mut recv_wait_pool = self.recv_wait_pool.lock().unwrap();
        recv_wait_pool[index] = Some(slot_handle);
    }

    pub(crate) fn set_response_to_slot_handle(
        &self,
        slot: i32,
        response: WireResponse,
        is_end: bool,
    ) {
        const FUNCTION_NAME: &str = "set_response_to_slot_handle()";
        let index = slot as usize;

        if is_end {
            if let Some(slot_handle) = {
                let mut recv_wait_pool = self.recv_wait_pool.lock().unwrap();
                recv_wait_pool.get_mut(index).and_then(Option::take) // recv_wait_pool[index]をNoneに置き換える
            } {
                slot_handle.set_wire_response(response);
            } else {
                debug!(
                    "{FUNCTION_NAME} error. slot_handle {slot} not found. response={response:?}"
                );
            }
        } else {
            let mut recv_wait_pool = self.recv_wait_pool.lock().unwrap();
            if let Some(Some(slot_handle)) = recv_wait_pool.get_mut(index) {
                slot_handle.set_wire_response(response);
            } else {
                debug!(
                    "{FUNCTION_NAME} error. slot_handle {slot} not found. response={response:?}"
                );
            }
        }
    }

    fn release_slot_entry(&self, slot_entry: SlotEntry) {
        let mut slot_pool = self.slot_pool.lock().unwrap();
        slot_pool.push_front(slot_entry);
        // slot_pool.push_back(slot_entry);
    }
}
