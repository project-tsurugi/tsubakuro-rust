use std::sync::Once;

use log::trace;

static ENV_LOGGER_INIT: Once = Once::new();

pub(crate) fn env_logger_init() {
    ENV_LOGGER_INIT.call_once(|| {
        const FUNCTION_NAME: &str = "env_logger_init()";
        trace!("{FUNCTION_NAME} start");

        env_logger::builder().format_timestamp_millis().init();

        trace!("{FUNCTION_NAME} end");
    })
}
