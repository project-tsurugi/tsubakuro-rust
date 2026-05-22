use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};

use tokio::{sync::Notify, task::JoinHandle};
use tonic::async_trait;

use crate::{
    client_error,
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
    timeout_error,
};

pub(crate) type BoxFuture<T> = Pin<Box<dyn Future<Output = Result<T, TgError>> + Send>>;

pub(crate) struct SpawnInnerJob<T: Send> {
    handle: Mutex<Option<JoinHandle<()>>>,
    done: AtomicBool,
    notify: Notify,
    result: tokio::sync::Mutex<Option<Result<T, TgError>>>,
}

impl<T: Send + 'static> SpawnInnerJob<T> {
    pub(crate) fn new(supplier: Arc<dyn Fn(Duration) -> BoxFuture<T> + Send + Sync>) -> Arc<Self> {
        let job = Arc::new(SpawnInnerJob {
            handle: Mutex::new(None),
            done: AtomicBool::new(false),
            notify: Notify::new(),
            result: tokio::sync::Mutex::new(None),
        });

        let cloned = job.clone();
        let handle = tokio::spawn(async move {
            let result = supplier(Duration::ZERO).await;
            *cloned.result.lock().await = Some(result);
            cloned
                .done
                .store(true, std::sync::atomic::Ordering::Release);
            cloned.notify.notify_waiters();
        });

        *job.handle.lock().unwrap() = Some(handle);

        job
    }
}

#[async_trait]
impl<T: Send> InnerJob<T> for SpawnInnerJob<T> {
    async fn wait(&self, timeout: Duration) -> Result<bool, TgError> {
        if self.done.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(true);
        }

        match tokio::time::timeout(timeout, self.notify.notified()).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn is_done(&self) -> Result<bool, TgError> {
        Ok(self.done.load(std::sync::atomic::Ordering::Acquire))
    }

    async fn take_for(&self, timeout: Duration) -> Result<T, TgError> {
        if !self.wait(timeout).await? {
            return Err(timeout_error!("SpawnJob::take_for()"));
        }

        let mut result = self.result.lock().await;

        match result.take() {
            Some(result) => result,
            None => Err(client_error!("SpawnJob::take_for(): result already taken")),
        }
    }

    async fn cancel_async(&self) -> Result<Option<CancelJob>, TgError> {
        self.abort();
        Ok(None)
    }

    async fn send_cancel(&self) -> Result<(), TgError> {
        self.abort();
        Ok(())
    }

    fn dispose(&self, _name: &str, _fail_on_error: bool) {
        if self.done.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }

        self.abort();
    }
}

impl<T: Send> SpawnInnerJob<T> {
    fn abort(&self) {
        let handle = {
            let mut self_handle = self.handle.lock().unwrap();
            self_handle.take()
        };

        if let Some(handle) = handle {
            handle.abort();
        }
    }
}
