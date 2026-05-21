use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};

use log::debug;
use tonic::async_trait;

use crate::{
    client_error,
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
};

pub(crate) type BoxFuture<T> = Pin<Box<dyn Future<Output = Result<T, TgError>> + Send>>;

pub(crate) struct SupplierJob<T: Send> {
    supplier: Arc<dyn Fn(Duration) -> BoxFuture<T> + Send + Sync>,
    called: AtomicBool,
    done: AtomicBool,
    value: Mutex<Option<T>>,
}

impl<T: Send> SupplierJob<T> {
    pub(crate) fn new(supplier: Arc<dyn Fn(Duration) -> BoxFuture<T> + Send + Sync>) -> Self {
        SupplierJob {
            supplier,
            called: AtomicBool::new(false),
            done: AtomicBool::new(false),
            value: Mutex::new(None),
        }
    }
}

#[async_trait]
impl<T: Send> InnerJob<T> for SupplierJob<T> {
    async fn wait(&self, timeout: Duration) -> Result<bool, TgError> {
        if self.done.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(true);
        }

        if self
            .called
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            )
            .is_ok()
        {
            let result = (self.supplier)(timeout).await;
            match result {
                Ok(value) => {
                    *self.value.lock().unwrap() = Some(value);
                    self.done.store(true, std::sync::atomic::Ordering::SeqCst);
                    Ok(true)
                }
                Err(TgError::TimeoutError(_)) => Ok(false),
                Err(e) => Err(e),
            }
        } else {
            debug!("SupplierJob::wait(): supplier already called, operation does not complete");
            Ok(false)
        }
    }

    async fn is_done(&self) -> Result<bool, TgError> {
        Ok(self.done.load(std::sync::atomic::Ordering::SeqCst))
    }

    async fn take_for(&self, timeout: Duration) -> Result<T, TgError> {
        {
            let mut self_value = self.value.lock().unwrap();
            if let Some(value) = self_value.take() {
                return Ok(value);
            }
        }

        if self
            .called
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            )
            .is_ok()
        {
            let result = (self.supplier)(timeout).await;
            self.done.store(true, std::sync::atomic::Ordering::SeqCst);
            result
        } else {
            Err(client_error!("SupplierJob::take_for(): supplier already called"))
        }
    }

    async fn cancel_async(&self) -> Result<Option<CancelJob>, TgError> {
        Ok(None)
    }

    async fn send_cancel(&self) -> Result<(), TgError> {
        Ok(())
    }

    fn dispose(&self, _name: &str, _fail_on_error: bool) {
        // do nothing
    }
}
