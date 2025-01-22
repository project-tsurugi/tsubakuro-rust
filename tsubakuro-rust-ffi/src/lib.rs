mod context;
mod logger;

pub type TsurugiFfiRc = u32;
pub const TSURUGI_FFI_RC_OK: u32 = 0;
pub const TSURUGI_FFI_RC_NG_FFI_HEADER: u32 = 0xc000_0000;
pub const TSURUGI_FFI_RC_NG_FFI_ARG0: u32 = TSURUGI_FFI_RC_NG_FFI_HEADER | 0;
