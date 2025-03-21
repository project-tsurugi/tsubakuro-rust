//! logger module.

use log::trace;

use crate::return_code::{TsurugiFfiRc, TSURUGI_FFI_RC_OK};

/// Initialize env_logger.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_env_logger_init() -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_env_logger_init()";
    trace!("{FUNCTION_NAME} start");

    env_logger::builder().format_timestamp_millis().init();

    let rc = TSURUGI_FFI_RC_OK;
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}
