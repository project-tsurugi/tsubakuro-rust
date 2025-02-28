use std::time::Duration;

use prost::alloc::string::String as ProstString;
use tokio::time::Instant;

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
}

#[doc(hidden)]
#[macro_export]
macro_rules! return_err_if_timeout {
    ($timeout:expr, $function_name:expr) => {
        if $timeout.is_timeout() {
            ::log::trace!("{}: timeout", $function_name);
            return Err($crate::timeout_error!($function_name))
        }
    };
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
