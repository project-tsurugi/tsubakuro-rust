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

/// nanosecond
pub type TsurugiFfiDuration = u64;

pub type TsurugiFfiStringHandle = *const c_char;
pub type TsurugiFfiStringArrayHandle = *const TsurugiFfiStringHandle;

pub type TsurugiFfiByteArrayHandle = *const u8;
