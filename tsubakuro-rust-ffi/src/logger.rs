//! logger module.

use std::sync::Once;

use env_logger::{Builder, Target};
use log::trace;

use crate::{
    return_code::{TsurugiFfiRc, TSURUGI_FFI_RC_OK},
    TsurugiFfiStringHandle,
};

static ENV_LOGGER_INIT: Once = Once::new();

/// Initialize env_logger.
///
/// Use the `RUST_LOG` environment variable as filters.
///
/// Calls to `tsurugi_ffi_env_logger_init` and [`tsurugi_ffi_env_logger_init_with_filters`] other than the first one are ignored.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_env_logger_init() -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_env_logger_init()";
    trace!("{FUNCTION_NAME} start");

    ENV_LOGGER_INIT.call_once(|| {
        env_logger::builder().format_timestamp_millis().init();
    });

    let rc = TSURUGI_FFI_RC_OK;
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// Initialize env_logger.
///
/// # Parameters
/// - `filters` - filter string. (e.g. "tsubakuro_rust_ffi=trace")
///               If null, do not log output.
/// - `file_path` - log file path. If null, logs to stderr.
///
/// Calls to [`tsurugi_ffi_env_logger_init`] and `tsurugi_ffi_env_logger_init_with_filters` other than the first one are ignored.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_env_logger_init_with_filters(
    filters: TsurugiFfiStringHandle,
    file_path: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_env_logger_init_with_filters()";
    trace!("{FUNCTION_NAME} start");

    let filters = if !filters.is_null() {
        unsafe { std::ffi::CStr::from_ptr(filters) }
    } else {
        let rc = TSURUGI_FFI_RC_OK;
        trace!("{FUNCTION_NAME} end (filters==null). rc={:x}", rc);
        return rc;
    };

    ENV_LOGGER_INIT.call_once(|| {
        let mut builder = Builder::new();

        if let Ok(s) = filters.to_str() {
            builder.parse_filters(s);
        }

        if !file_path.is_null() {
            let s = unsafe { std::ffi::CStr::from_ptr(file_path) };
            if let Ok(s) = s.to_str() {
                env_logger_init_file(&mut builder, s);
            }
        }

        builder.format_timestamp_millis().init();
    });

    let rc = TSURUGI_FFI_RC_OK;
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

fn env_logger_init_file(builder: &mut Builder, file_path: &str) {
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
    {
        Ok(file) => {
            builder.target(Target::Pipe(Box::new(file)));
        }
        Err(e) => {
            eprintln!(
                "tsurugi_ffi_env_logger_init_with_setting(): log file open error. {}",
                e
            );
        }
    };
}
