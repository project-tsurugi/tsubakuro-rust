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

pub(crate) fn calculate_timeout(
    function_name: &str,
    timeout: Duration,
    start: Instant,
) -> Result<Duration, TgError> {
    if timeout.is_zero() {
        return Ok(timeout);
    }

    let elapsed = start.elapsed();
    if timeout > elapsed {
        Ok(timeout - elapsed)
    } else {
        Err(timeout_error!(function_name))
    }
}
