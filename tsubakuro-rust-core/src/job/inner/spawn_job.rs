use std::{future::Future, pin::Pin, sync::Mutex, time::Duration};

use tokio::task::JoinHandle;
use tonic::async_trait;

use crate::{
    client_error,
    error::TgError,
    job::{cancel_job::CancelJob, inner::InnerJob},
    timeout_error,
};

pub(crate) type BoxFuture<T> = Pin<Box<dyn Future<Output = Result<T, TgError>> + Send>>;

pub(crate) struct SpawnInnerJob<T: Send> {
    handle: Mutex<Option<JoinHandle<Result<T, TgError>>>>,
}

impl<T: Send + 'static> SpawnInnerJob<T> {
    pub(crate) fn new(supplier: Box<dyn Fn(Duration) -> BoxFuture<T> + Send>) -> Box<Self> {
        let handle = tokio::spawn(supplier(Duration::ZERO));

        Box::new(SpawnInnerJob {
            handle: Mutex::new(Some(handle)),
        })
    }
}

#[async_trait(?Send)]
impl<T: Send> InnerJob<T> for SpawnInnerJob<T> {
    async fn wait(&self, timeout: Duration) -> Result<bool, TgError> {
        if self.is_finished() {
            return Ok(true);
        }

        if timeout.is_zero() {
            loop {
                tokio::task::yield_now().await;

                if self.is_finished() {
                    return Ok(true);
                }
            }
        }

        match tokio::time::timeout(timeout, async {
            loop {
                tokio::task::yield_now().await;

                if self.is_finished() {
                    break;
                }
            }
        })
        .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn is_done(&self) -> Result<bool, TgError> {
        Ok(self.is_finished())
    }

    async fn take_for(&self, timeout: Duration) -> Result<T, TgError> {
        let handle = {
            let mut guard = self.handle.lock().unwrap();

            guard.take()
        };

        let handle = match handle {
            Some(handle) => handle,
            None => {
                return Err(client_error!("SpawnInnerJob::take_for(): already taken"));
            }
        };

        let join_result = if timeout.is_zero() {
            handle.await
        } else {
            match tokio::time::timeout(timeout, handle).await {
                Ok(result) => result,
                Err(_) => {
                    return Err(timeout_error!("SpawnInnerJob::take_for()"));
                }
            }
        };

        match join_result {
            Ok(result) => result,
            Err(err) if err.is_cancelled() => Err(client_error!("SpawnInnerJob cancelled")),
            Err(err) => Err(client_error!("Join error: {}", err)),
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

    fn set_fail_on_drop_error(&self, _value: bool) {
        // do nothing
    }
}

impl<T: Send> Drop for SpawnInnerJob<T> {
    fn drop(&mut self) {
        self.abort();
    }
}

impl<T: Send> SpawnInnerJob<T> {
    fn is_finished(&self) -> bool {
        let guard = self.handle.lock().unwrap();

        match guard.as_ref() {
            Some(handle) => handle.is_finished(),
            None => true,
        }
    }

    fn abort(&self) {
        let guard = self.handle.lock().unwrap();

        if let Some(handle) = guard.as_ref() {
            handle.abort();
        }
    }
}
