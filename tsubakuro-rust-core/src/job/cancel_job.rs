use std::{sync::Arc, time::Duration};

use crate::{
    error::TgError,
    session::wire::{response_box::SlotEntryHandle, Wire},
    util::Timeout,
};

/// Cancel Job.
///
/// Wait for cancellation process to complete.
///
/// **thread unsafe**
pub struct CancelJob {
    wire: Arc<Wire>,
    slot_handle: Arc<SlotEntryHandle>,
    done: bool,
}

impl CancelJob {
    pub(crate) fn new(wire: Arc<Wire>, slot_handle: Arc<SlotEntryHandle>) -> CancelJob {
        CancelJob {
            wire,
            slot_handle,
            done: false,
        }
    }

    /// Wait for response.
    ///
    /// # Returns
    /// - `Ok(true)` - Response received.
    /// - `Ok(false)` - Timed out.
    pub async fn wait(&mut self, timeout: Duration) -> Result<bool, TgError> {
        if self.done {
            return Ok(true);
        }

        let slot_handle = self.slot_handle.clone();
        let timeout = Timeout::new(timeout);
        let result = self.wire.wait_response(slot_handle, &timeout).await;
        if let Ok(true) = result {
            self.done = true;
        }
        result
    }

    /// Whether a response has been received.
    ///
    /// # Returns
    /// - `Ok(true)` - Response received.
    /// - `Ok(false)` - No response received.
    pub async fn is_done(&mut self) -> Result<bool, TgError> {
        if self.done {
            return Ok(true);
        }

        let slot_handle = self.slot_handle.clone();
        let result = self.wire.check_response(slot_handle).await;
        if let Ok(true) = result {
            self.done = true;
        }
        result
    }
}
