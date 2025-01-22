use log::trace;

pub type TsurugiFfiRc = u32;
pub const TSURUGI_FFI_RC_OK: u32 = 0;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_env_logger_init() -> TsurugiFfiRc {
    env_logger::init();
    trace!("tsurugi_ffi_env_logger_init() end");
    TSURUGI_FFI_RC_OK
}
