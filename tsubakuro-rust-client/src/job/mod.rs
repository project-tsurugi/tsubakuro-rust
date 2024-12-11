use std::{future::Future, pin::Pin, time::Duration};

use crate::error::TgError;

pub struct Job<T> {
    take: Box<
        dyn FnOnce(Duration) -> Pin<Box<dyn Future<Output = Result<T, TgError>> + Send>> + Send,
    >,
    default_timeout: Duration,
}

impl<T> Job<T> {
    pub(crate) fn new<F>(take: F, default_timeout: Duration) -> Job<T>
    where
        F: FnOnce(Duration) -> Pin<Box<dyn Future<Output = Result<T, TgError>> + Send>>
            + Send
            + 'static,
    {
        Job {
            take: Box::new(take),
            default_timeout,
        }
    }

    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    // TODO &self
    pub async fn take(self) -> Result<T, TgError> {
        let timeout = self.default_timeout;
        self.take_for(timeout).await
    }

    // TODO &self
    pub async fn take_for(self, timeout: Duration) -> Result<T, TgError> {
        (self.take)(timeout).await
    }
}
