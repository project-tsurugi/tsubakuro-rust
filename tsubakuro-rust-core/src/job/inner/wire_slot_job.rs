use std::{sync::Arc, time::Duration};

use log::{error, warn};
use tonic::async_trait;

use crate::{
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
    service::endpoint::endpoint_broker::EndpointBroker,
    session::wire::{response::WireResponse, response_box::SlotEntryHandle, Wire},
    util::Timeout,
};

pub(crate) struct WireSlotJob<T: Send> {
    wire: Arc<Wire>,
    slot_handle: Arc<SlotEntryHandle>,
    converter: Arc<dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send + Sync>,
}

impl<T: Send> WireSlotJob<T> {
    pub(crate) fn new(
        wire: Arc<Wire>,
        slot_handle: Arc<SlotEntryHandle>,
        converter: Arc<
            dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send + Sync,
        >,
    ) -> WireSlotJob<T> {
        WireSlotJob {
            wire,
            slot_handle,
            converter,
        }
    }
}

#[async_trait]
impl<T: Send> InnerJob<T> for WireSlotJob<T> {
    async fn wait(&self, timeout: Duration) -> Result<bool, TgError> {
        let slot_handle = self.slot_handle.clone();
        let timeout = Timeout::new(timeout);
        self.wire.wait_response(slot_handle, &timeout).await
    }

    async fn is_done(&self) -> Result<bool, TgError> {
        let slot_handle = self.slot_handle.clone();
        self.wire.check_response(slot_handle).await
    }

    async fn take_for(&self, timeout: Duration) -> Result<T, TgError> {
        let slot_handle = self.slot_handle.clone();
        let timeout = Timeout::new(timeout);
        let response = self.wire.pull_response(&slot_handle, &timeout).await?;
        (self.converter)(slot_handle, response)
    }

    async fn cancel_async(&self) -> Result<Option<CancelJob>, TgError> {
        self.send_cancel().await?;
        let job = CancelJob::new(self.wire.clone(), self.slot_handle.clone());
        Ok(Some(job))
    }

    async fn send_cancel(&self) -> Result<(), TgError> {
        let slot = self.slot_handle.slot();
        EndpointBroker::cancel(&self.wire, slot).await?;

        Ok(())
    }

    fn dispose(&self, name: &str, fail_on_error: bool) {
        if self.slot_handle.exists_wire_response() {
            return;
        }

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
