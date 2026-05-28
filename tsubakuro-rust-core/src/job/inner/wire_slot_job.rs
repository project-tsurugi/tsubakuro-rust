use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use log::{error, warn};
use tonic::async_trait;

use crate::{
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
    service::endpoint::endpoint_broker::EndpointBroker,
    session::wire::{response::WireResponse, response_box::SlotEntryHandle, Wire},
    util::Timeout,
};

pub(crate) struct WireSlotInnerJob<T: Send> {
    name: String,
    wire: Arc<Wire>,
    slot_handle: Arc<SlotEntryHandle>,
    converter: Box<dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send + Sync>,
    done: AtomicBool,
    cancelled: AtomicBool,
    fail_on_drop_error: AtomicBool,
}

impl<T: Send> WireSlotInnerJob<T> {
    pub(crate) fn new(
        name: String,
        wire: Arc<Wire>,
        slot_handle: Arc<SlotEntryHandle>,
        converter: Box<
            dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send + Sync,
        >,
    ) -> WireSlotInnerJob<T> {
        WireSlotInnerJob {
            name,
            wire,
            slot_handle,
            converter,
            done: AtomicBool::new(false),
            cancelled: AtomicBool::new(false),
            fail_on_drop_error: AtomicBool::new(false),
        }
    }
}

#[async_trait(?Send)]
impl<T: Send> InnerJob<T> for WireSlotInnerJob<T> {
    async fn wait(&self, timeout: Duration) -> Result<bool, TgError> {
        let slot_handle = self.slot_handle.clone();
        let timeout = Timeout::new(timeout);
        let done = self.wire.wait_response(slot_handle, &timeout).await?;
        if done {
            self.done.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        Ok(done)
    }

    async fn is_done(&self) -> Result<bool, TgError> {
        if self.done.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(true);
        }

        let slot_handle = self.slot_handle.clone();
        let done = self.wire.check_response(slot_handle).await?;
        if done {
            self.done.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        Ok(done)
    }

    async fn take_for(&self, timeout: Duration) -> Result<T, TgError> {
        let slot_handle = self.slot_handle.clone();
        let timeout = Timeout::new(timeout);
        let response = self.wire.pull_response(&slot_handle, &timeout).await?;
        self.done.store(true, std::sync::atomic::Ordering::Relaxed);
        (self.converter)(slot_handle, response)
    }

    async fn cancel_async(&self) -> Result<Option<CancelJob>, TgError> {
        if self.done.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(None);
        }
        if self
            .cancelled
            .swap(true, std::sync::atomic::Ordering::Relaxed)
        {
            return Ok(None);
        }

        self.send_cancel().await?;
        let job = CancelJob::new(self.wire.clone(), self.slot_handle.clone());
        Ok(Some(job))
    }

    async fn send_cancel(&self) -> Result<(), TgError> {
        let slot = self.slot_handle.slot();
        EndpointBroker::cancel(&self.wire, slot).await?;

        Ok(())
    }

    fn set_fail_on_drop_error(&self, value: bool) {
        self.fail_on_drop_error
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }
}

impl<T: Send> Drop for WireSlotInnerJob<T> {
    fn drop(&mut self) {
        if self.done.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        if self
            .cancelled
            .swap(true, std::sync::atomic::Ordering::AcqRel)
        {
            return;
        }

        self.dispose();
    }
}

impl<T: Send> WireSlotInnerJob<T> {
    fn dispose(&self) {
        if self.slot_handle.exists_wire_response() {
            return;
        }

        let name = &self.name;
        let fail_on_error = self
            .fail_on_drop_error
            .load(std::sync::atomic::Ordering::Relaxed);
        std::thread::scope(|scope| {
            scope.spawn(move || {
                let runtime = {
                    match tokio::runtime::Runtime::new() {
                        Ok(runtime) => runtime,
                        Err(e) => {
                            error!("Job<{}>.drop() runtime::new error. {}", name, e);
                            if fail_on_error {
                                panic!("Job<{}>.drop() runtime::new error. {}", name, e);
                            }
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    let result = self.send_cancel().await; // send only (do not check response)
                    if let Err(e) = result {
                        warn!("Job<{}>.drop() close error. {}", name, e);
                        if fail_on_error {
                            panic!("Job<{}>.drop() close error. {}", name, e);
                        }
                    }
                })
            });
        });
    }
}
