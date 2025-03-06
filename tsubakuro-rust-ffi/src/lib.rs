//! [tsubakuro-rust-ffi](https://github.com/project-tsurugi/tsubakuro-rust/tree/master/tsubakuro-rust-ffi)
//! is a library providing functions in C ABI format to access [Tsurugi DB](https://github.com/project-tsurugi/tsurugidb).
//!
//! tsubakuro-rust-ffi uses [tsubakuro-rust-core](https://github.com/project-tsurugi/tsubakuro-rust/tree/master/tsubakuro-rust-core),
//! so refer to that for explanation.
//!
//! # Examples
//!
//! See <https://github.com/project-tsurugi/tsubakuro-rust/tree/master/tsubakuro-rust-ffi#example>.
#![allow(private_interfaces)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::missing_safety_doc)]

use std::ffi::c_char;

pub mod context;
mod error;
pub mod job;
pub mod logger;
pub mod return_code;
pub mod service;
pub mod session;
pub mod transaction;
mod util;

/// Nanosecond.
pub type TsurugiFfiDuration = u64;

/// String (UTF-8 with `nul` termination).
pub type TsurugiFfiStringHandle = *const c_char;

/// String array.
pub type TsurugiFfiStringArrayHandle = *const TsurugiFfiStringHandle;

/// Byte array.
pub type TsurugiFfiByteArrayHandle = *const u8;
