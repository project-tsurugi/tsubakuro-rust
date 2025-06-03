use std::{sync::Arc, time::Duration};

use super::cancel_job::CancelJob;
use log::{error, warn};

use crate::{
    client_error,
    error::TgError,
    service::endpoint::endpoint_broker::EndpointBroker,
    session::wire::{response::WireResponse, response_box::SlotEntryHandle, Wire},
    util::Timeout,
};

/// Job.
///
/// An object that provides asynchronous response data,
/// and it may be canceled before the underlying task was done.
///
/// **thread unsafe**
///
/// # Examples
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
///     let sql = "insert into tb values(1, 'abc')";
///     let mut job = client.execute_async(transaction, sql).await?;
///
///     let execute_result = job.take_for(std::time::Duration::from_secs(10)).await?;
///
///     Ok(())
/// }
/// ```
pub struct Job<T> {
    name: String,
    wire: Arc<Wire>,
    slot_handle: Arc<SlotEntryHandle>,
    converter: Box<dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send>,
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
        F: Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send + 'static,
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

    /// Get job name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Set default timeout.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Wait for response.
    ///
    /// # Returns
    /// - `Ok(true)` - Response received.
    /// - `Ok(false)` - Timed out.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(mut job: Job<SqlExecuteResult>) -> Result<(), TgError> {
    ///     let done = job.wait(std::time::Duration::from_secs(10)).await?;
    ///     if done {
    ///         let execute_result = job.take().await?;
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Whether a response has been received.
    ///
    /// # Returns
    /// - `Ok(true)` - Response received.
    /// - `Ok(false)` - No response received.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(mut job: Job<SqlExecuteResult>) -> Result<(), TgError> {
    ///     loop {
    ///         let done = job.is_done().await?;
    ///         if done {
    ///             let execute_result = job.take().await?;
    ///             break;
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Retrieves the result value, or wait until response has been received.
    ///
    /// You can only take once to retrieve the value.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(mut job: Job<SqlExecuteResult>) -> Result<(), TgError> {
    ///     let execute_result = job.take().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn take(&mut self) -> Result<T, TgError> {
        let timeout = self.default_timeout;
        self.take_for(timeout).await
    }

    /// Retrieves the result value, or wait until response has been received.
    ///
    /// You can only take once to retrieve the value.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(mut job: Job<SqlExecuteResult>) -> Result<(), TgError> {
    ///     let execute_result = job.take_for(std::time::Duration::from_secs(10)).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
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
        (self.converter)(slot_handle.clone(), response)
    }

    /// Retrieves the result value if a response has been received.
    ///
    /// You can only take once to retrieve the value.
    ///
    /// # Returns
    /// - `Ok(Some(value))` - result value
    /// - `Ok(None)` - No response received.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(mut job: Job<SqlExecuteResult>) -> Result<(), TgError> {
    ///     loop {
    ///         if let Some(execute_result) = job.take_if_ready().await? {
    ///             break;
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn take_if_ready(&mut self) -> Result<Option<T>, TgError> {
        if self.is_done().await? {
            let value = self.take_for(Duration::ZERO).await?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Cancel job.
    ///
    /// # Returns
    /// - `Ok(true)` - Response received.
    /// - `Ok(false)` - Timed out.
    ///
    /// The response is not necessarily OPERATION_CANCELED.
    /// Depending on the timing, it may be a normal processing result.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(mut job: Job<SqlExecuteResult>) -> Result<(), TgError> {
    ///     job.cancel().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn cancel(self) -> Result<bool, TgError> {
        let timeout = self.default_timeout;
        self.cancel_for(timeout).await
    }

    /// Cancel job.
    ///
    /// # Returns
    /// - `Ok(true)` - Response already received, or cancel already started.
    /// - `Ok(false)` - Timed out.
    ///
    /// The response is not necessarily OPERATION_CANCELED.
    /// Depending on the timing, it may be a normal processing result.
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

    /// Cancel job.
    ///
    /// # Returns
    /// - `Ok(Some(CancelJob))` - Cancellation started.
    /// - `Ok(None)` - Not canceled. (response already received, or cancel already started)
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(mut job: Job<SqlExecuteResult>) -> Result<(), TgError> {
    ///     if let Some(mut cancel_job) = job.cancel_async().await? {
    ///         cancel_job.wait(std::time::Duration::from_secs(10)).await?;
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Disposes this resource.
    ///
    /// If no response is received and no cancellation is made, then execute cancel.
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

    /// for debug
    #[doc(hidden)]
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
                            error!("Job<{}>.drop() runtime::new error. {}", self.name, e);
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
                        warn!("Job<{}>.drop() close error. {}", self.name, e);
                        if self.fail_on_drop_error() {
                            panic!("Job<{}>.drop() close error. {}", self.name, e);
                        }
                    }
                })
            });
        });
    }
}
