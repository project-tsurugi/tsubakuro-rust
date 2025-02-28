use std::ffi::c_char;

mod context;
mod error;
mod job;
mod logger;
mod return_code;
mod service;
mod session;
mod transaction;
mod util;

/// Nanosecond.
pub type TsurugiFfiDuration = u64;

/// String.
///
/// UTF-8 with `nul` termination.
pub type TsurugiFfiStringHandle = *const c_char;

/// String array.
pub type TsurugiFfiStringArrayHandle = *const TsurugiFfiStringHandle;

/// Byte array.
pub type TsurugiFfiByteArrayHandle = *const u8;
