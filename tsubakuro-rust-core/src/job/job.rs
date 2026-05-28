use std::{sync::Arc, time::Duration};

use super::cancel_job::CancelJob;

use crate::{
    client_error,
    error::TgError,
    job::inner::{
        convert_job::ConvertInnerJob,
        spawn_job::{BoxFuture, SpawnInnerJob},
        value_job::ValueInnerJob,
        wire_slot_job::WireSlotInnerJob,
        InnerJob,
    },
    session::wire::{response::WireResponse, response_box::SlotEntryHandle, Wire},
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
pub struct Job<T: Send> {
    name: String,
    inner: Box<dyn InnerJob<T> + Send>,
    default_timeout: Duration,
    taked: bool,
    closed: bool,
}

impl<T: Send> std::fmt::Debug for Job<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("name", &self.name)
            .field("default_timeout", &self.default_timeout)
            .field("taked", &self.taked)
            .field("closed", &self.closed)
            .finish()
    }
}

impl<T: Send + 'static> Job<T> {
    pub(crate) fn new(
        name: &str,
        wire: Arc<Wire>,
        slot_handle: Arc<SlotEntryHandle>,
        converter: Box<
            dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send + Sync,
        >,
        default_timeout: Duration,
    ) -> Job<T> {
        let inner = Box::new(WireSlotInnerJob::new(
            name.to_string(),
            wire,
            slot_handle,
            converter,
        ));

        Job {
            name: name.to_string(),
            inner,
            default_timeout,
            taked: false,
            closed: false,
        }
    }

    pub(crate) fn returns(name: &str, value: T) -> Job<T> {
        let inner = Box::new(ValueInnerJob::new(value));

        Job {
            name: name.to_string(),
            inner,
            default_timeout: Duration::ZERO,
            taked: false,
            closed: false,
        }
    }

    pub(crate) fn supplier(
        name: &str,
        supplier: Box<dyn Fn(Duration) -> BoxFuture<T> + Send + Sync>,
        default_timeout: Duration,
    ) -> Job<T> {
        let inner = SpawnInnerJob::new(supplier);

        Job {
            name: name.to_string(),
            inner,
            default_timeout,
            taked: false,
            closed: false,
        }
    }

    pub(crate) fn convert<R: Send + 'static>(
        self,
        name: &str,
        converter: Box<dyn Fn(T) -> Result<R, TgError> + Send>,
    ) -> Job<R> {
        let inner = Box::new(ConvertInnerJob::new(self.inner, converter));

        let default_timeout = self.default_timeout;
        let taked = self.taked;
        let closed = self.closed;

        Job {
            name: name.to_string(),
            inner,
            default_timeout,
            taked,
            closed,
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
        self.inner.wait(timeout).await
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
        self.inner.is_done().await
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

        self.taked = true;
        let value = self.inner.take_for(timeout).await?;
        Ok(value)
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
        if self.is_done().await? {
            return Ok(None);
        }

        self.inner.cancel_async().await
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

        if self.is_done().await? {
            return Ok(());
        }

        self.inner.send_cancel().await // send only (do not check response)
    }

    // fn check_close(&self) -> Result<(), TgError> {
    //     if self.closed {
    //         return Err(client_error!(format!("Job<{}> already closed", self.name)));
    //     }
    //     Ok(())
    // }

    /// for debug
    #[doc(hidden)]
    pub fn set_fail_on_drop_error(&mut self, value: bool) {
        self.inner.set_fail_on_drop_error(value);
    }
}
