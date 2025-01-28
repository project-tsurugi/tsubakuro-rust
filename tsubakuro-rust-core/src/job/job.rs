use std::{sync::Arc, time::Duration};

use super::cancel_job::CancelJob;
use log::debug;

use crate::{
    client_error,
    error::TgError,
    service::endpoint::endpoint_broker::EndpointBroker,
    session::wire::{response::WireResponse, response_box::SlotEntryHandle, Wire},
    util::Timeout,
};

/// thread unsafe
pub struct Job<T> {
    name: String,
    wire: Arc<Wire>,
    slot_handle: Arc<SlotEntryHandle>,
    converter: Box<dyn Fn(WireResponse) -> Result<T, TgError> + Send>,
    default_timeout: Duration,
    done: bool,
    taked: bool,
    canceled: bool,
    closed: bool,
    fail_on_drop_error: bool,
}

impl<T> std::fmt::Debug for Job<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("name", &self.name)
            .field("wire", &self.wire)
            .field("slot_handle", &self.slot_handle)
            .field("default_timeout", &self.default_timeout)
            .field("done", &self.done)
            .field("taked", &self.taked)
            .field("canceled", &self.canceled)
            .field("closed", &self.closed)
            .finish()
    }
}

impl<T> Job<T> {
    pub(crate) fn new<F>(
        name: &str,
        wire: Arc<Wire>,
        slot_handle: Arc<SlotEntryHandle>,
        converter: F,
        default_timeout: Duration,
        fail_on_drop_error: bool,
    ) -> Job<T>
    where
        F: Fn(WireResponse) -> Result<T, TgError> + Send + 'static,
    {
        Job {
            name: name.to_string(),
            wire,
            slot_handle,
            converter: Box::new(converter),
            default_timeout,
            done: false,
            taked: false,
            canceled: false,
            closed: false,
            fail_on_drop_error,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    pub async fn wait(&mut self, timeout: Duration) -> Result<bool, TgError> {
        // self.check_cancel()?;
        // self.check_close()?;
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

    pub async fn is_done(&mut self) -> Result<bool, TgError> {
        if self.done {
            return Ok(true);
        }
        if self.canceled {
            return Ok(false);
        }
        if self.closed {
            return Ok(false);
        }

        let slot_handle = self.slot_handle.clone();
        let result = self.wire.check_response(slot_handle).await;
        if let Ok(true) = result {
            self.done = true;
        }
        result
    }

    pub async fn take(&mut self) -> Result<T, TgError> {
        let timeout = self.default_timeout;
        self.take_for(timeout).await
    }

    pub async fn take_for(&mut self, timeout: Duration) -> Result<T, TgError> {
        if self.taked {
            return Err(client_error!(format!("Job<{}> already taked", self.name)));
        }
        // self.check_cancel()?;
        // self.check_close()?;

        let slot_handle = &self.slot_handle;
        let timeout = Timeout::new(timeout);
        let response = self.wire.pull_response(slot_handle, &timeout).await?;
        self.done = true;
        self.taked = true;
        (self.converter)(response)
    }

    pub async fn take_if_ready(&mut self) -> Result<Option<T>, TgError> {
        if self.is_done().await? {
            let t = self.take_for(Duration::ZERO).await?;
            Ok(Some(t))
        } else {
            Ok(None)
        }
    }

    pub async fn cancel(self) -> Result<bool, TgError> {
        let timeout = self.default_timeout;
        self.cancel_for(timeout).await
    }

    // 戻り値は、タイムアウトしたときfalse, レスポンスが来たらtrue（レスポンスはOPERATION_CANCELEDとは限らない。タイミングによっては正常な処理結果であることもある）
    pub async fn cancel_for(self, timeout: Duration) -> Result<bool, TgError> {
        let job = self.cancel_async().await?;
        match job {
            Some(mut job) => {
                let done = job.wait(timeout).await?;
                Ok(done)
            }
            _ => Ok(true),
        }
    }

    pub async fn cancel_async(mut self) -> Result<Option<CancelJob>, TgError> {
        // self.check_close()?;
        if self.done {
            return Ok(None);
        }
        if self.canceled {
            return Ok(None);
        }
        self.canceled = true;

        self.send_cancel().await?;
        let job = CancelJob::new(self.wire.clone(), self.slot_handle.clone());
        Ok(Some(job))
    }

    // fn check_cancel(&self) -> Result<(), TgError> {
    //     if self.canceled {
    //         return Err(client_error!(format!("Job<{}> canceled", self.name)));
    //     }
    //     Ok(())
    // }

    pub async fn close(mut self) -> Result<(), TgError> {
        if self.closed {
            return Ok(());
        }
        self.closed = true;

        if self.done || self.canceled {
            return Ok(());
        }

        self.send_cancel().await // send only (do not check response)
    }

    // fn check_close(&self) -> Result<(), TgError> {
    //     if self.closed {
    //         return Err(client_error!(format!("Job<{}> already closed", self.name)));
    //     }
    //     Ok(())
    // }

    async fn send_cancel(&self) -> Result<(), TgError> {
        let slot = self.slot_handle.slot();
        EndpointBroker::cancel(&self.wire, slot).await?;

        Ok(())
    }

    // for debug
    pub fn set_fail_on_drop_error(&mut self, value: bool) {
        self.fail_on_drop_error = value;
    }

    pub(crate) fn fail_on_drop_error(&self) -> bool {
        self.fail_on_drop_error
    }
}

impl<T> Drop for Job<T> {
    fn drop(&mut self) {
        if self.done || self.canceled || self.closed {
            return;
        }
        if self.slot_handle.exists_wire_response() {
            return;
        }

        std::thread::scope(|scope| {
            scope.spawn(move || {
                let runtime = {
                    match tokio::runtime::Runtime::new() {
                        Ok(runtime) => runtime,
                        Err(e) => {
                            debug!("Job<{}>.drop() runtime::new error. {}", self.name, e);
                            if self.fail_on_drop_error() {
                                panic!("Job<{}>.drop() runtime::new error. {}", self.name, e);
                            }
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    let result = self.send_cancel().await; // send only (do not check response)
                    if let Err(e) = result {
                        debug!("Job<{}>.drop() close error. {}", self.name, e);
                        if self.fail_on_drop_error() {
                            panic!("Job<{}>.drop() close error. {}", self.name, e);
                        }
                    }
                })
            });
        });
    }
}
