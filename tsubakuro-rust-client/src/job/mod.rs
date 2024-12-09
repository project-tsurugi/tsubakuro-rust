use std::{future::Future, pin::Pin, time::Duration};

use crate::error::TgError;

pub struct Job<T> {
    take: Box<
        dyn FnOnce(Duration) -> Pin<Box<dyn Future<Output = Result<T, TgError>> + Send>> + Send,
    >,
}

impl<T> Job<T> {
    pub(crate) fn new<F>(take: F) -> Job<T>
    where
        F: FnOnce(Duration) -> Pin<Box<dyn Future<Output = Result<T, TgError>> + Send>>
            + Send
            + 'static,
    {
        Job {
            take: Box::new(take),
        }
    }

    pub async fn take(self, timeout: Duration) -> Result<T, TgError> {
        (self.take)(timeout).await
    }
}
