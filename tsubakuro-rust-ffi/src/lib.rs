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

pub type TsurugiFfiStringHandle = *const c_char;
pub type TsurugiFfiStringArrayHandle = *const TsurugiFfiStringHandle;

/// nanosecond
pub type TsurugiFfiDuration = u64;
