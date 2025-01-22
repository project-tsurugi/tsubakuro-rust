use log::trace;

use crate::return_code::{TsurugiFfiRc, TSURUGI_FFI_RC_OK};

#[no_mangle]
pub extern "C" fn tsurugi_ffi_env_logger_init() -> TsurugiFfiRc {
    env_logger::init();
    trace!("tsurugi_ffi_env_logger_init() end");
    TSURUGI_FFI_RC_OK
}
