use std::time::Duration;

use prost::alloc::string::String as ProstString;
use tokio::time::Instant;

use crate::{error::TgError, timeout_error};

pub(crate) fn string_to_prost_string(s: Option<&String>) -> ProstString {
    if let Some(s) = s {
        ProstString::from(s)
    } else {
        ProstString::new()
    }
}

#[derive(Debug)]
pub(crate) struct Timeout {
    timeout: Duration,
    start: Instant,
}

impl Timeout {
    pub(crate) fn new(timeout: Duration) -> Timeout {
        Timeout {
            timeout,
            start: Instant::now(),
        }
    }

    pub(crate) fn is_timeout(&self) -> bool {
        let timeout = self.timeout;
        if timeout.is_zero() {
            return false;
        }

        let elapsed = self.start.elapsed();
        elapsed > timeout
    }

    pub(crate) fn return_err_if_timeout(&self, function_name: &str) -> Result<(), TgError> {
        if self.is_timeout() {
            Err(timeout_error!(function_name))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn timeout() {
        let zero = Timeout::new(Duration::ZERO);
        let timeout = Timeout::new(Duration::from_millis(100));
        assert_eq!(false, zero.is_timeout());
        assert_eq!(false, timeout.is_timeout());

        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(false, zero.is_timeout());
        assert_eq!(false, timeout.is_timeout());

        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(false, zero.is_timeout());
        assert_eq!(true, timeout.is_timeout());
    }
}
