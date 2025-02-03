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

pub type TsurugiFfiStringArrayHandle = *mut *mut std::ffi::c_char;
