use std::{sync::Arc, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc},
    vec_u8_to_field, TsurugiFfiByteArrayHandle, TsurugiFfiDuration,
};

pub(crate) struct TsurugiFfiClobDownloader {
    downloader: ClobDownloader,
    runtime: Arc<tokio::runtime::Runtime>,
    value: Option<Vec<u8>>,
}

impl TsurugiFfiClobDownloader {
    pub(crate) fn new(
        downloader: ClobDownloader,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiClobDownloader {
        TsurugiFfiClobDownloader {
            downloader,
            runtime,
            value: None,
        }
    }

    fn runtime(&self) -> Arc<tokio::runtime::Runtime> {
        self.runtime.clone()
    }
}

impl std::ops::Deref for TsurugiFfiClobDownloader {
    type Target = ClobDownloader;

    fn deref(&self) -> &Self::Target {
        &self.downloader
    }
}

impl std::ops::DerefMut for TsurugiFfiClobDownloader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.downloader
    }
}

/// ClobDownloader.
///
/// since 0.10.0
pub type TsurugiFfiClobDownloaderHandle = *mut TsurugiFfiClobDownloader;

/// Downloads a chunk of data.
///
/// See [`ClobDownloader::download_chunk`].
///
/// # Receiver
/// - `downloader` - ClobDownloader.
///
/// # Arguments
/// - `length` - chunk length (number of bytes).
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `value_out` - chunk data.
/// - `size_out` - chunk data size (number of bytes).
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_downloader_download_chunk_utf8(
    context: TsurugiFfiContextHandle,
    downloader: TsurugiFfiClobDownloaderHandle,
    length: u64,
    timeout: TsurugiFfiDuration,
    value_out: *mut TsurugiFfiByteArrayHandle,
    size_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_downloader_download_chunk_utf8()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, downloader={:?}, length={}, timeout={:?}, value_out={:?}, size_out={:?}",
        context,
        downloader,
        length,
        timeout,
        value_out,
        size_out
    );

    ffi_arg_out_initialize!(value_out, std::ptr::null_mut());
    ffi_arg_out_initialize!(size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, downloader);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, value_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, size_out);

    let downloader = unsafe { &mut *downloader };
    let timeout = Duration::from_nanos(timeout);

    let runtime = downloader.runtime();
    let value = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        downloader.download_chunk_utf8(length as usize, timeout)
    );

    let (ptr, size) = if let Some(vec) = value {
        vec_u8_to_field!(downloader.value, vec)
    } else {
        (std::ptr::null(), 0)
    };

    unsafe {
        *value_out = ptr;
        *size_out = size;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. value_out={:?}, size_out={:?}",
        rc,
        ptr,
        size
    );
    rc
}

/// Downloads a chunk of data as UTF-8 into the provided buffer.
///
/// See [`ClobDownloader::download_chunk_into_utf8`].
///
/// # Receiver
/// - `downloader` - ClobDownloader.
///
/// # Arguments
/// - `buffer` - buffer to store chunk data.
/// - `buffer_size` - buffer size (number of bytes).
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `size_out` - chunk data size (number of bytes) actually downloaded.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_downloader_download_chunk_into_utf8(
    context: TsurugiFfiContextHandle,
    downloader: TsurugiFfiClobDownloaderHandle,
    buffer: *mut u8,
    buffer_size: u64,
    timeout: TsurugiFfiDuration,
    size_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_downloader_download_chunk_into_utf8()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, downloader={:?}, buffer={:?}, buffer_size={}, timeout={:?}, size_out={:?}",
        context,
        downloader,
        buffer,
        buffer_size,
        timeout,
        size_out
    );

    ffi_arg_out_initialize!(size_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, downloader);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, buffer);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, size_out);

    let downloader = unsafe { &mut *downloader };
    let timeout = Duration::from_nanos(timeout);

    let runtime = downloader.runtime();
    let size = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        downloader.download_chunk_into_utf8(
            unsafe { std::slice::from_raw_parts_mut(buffer, buffer_size as usize) },
            timeout
        )
    );

    unsafe {
        *size_out = size as u64;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. size_out={}", rc, size);
    rc
}

/// Disposes the ClobDownloader.
///
/// # Receiver
/// - `downloader` - ClobDownloader.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_downloader_dispose(downloader: TsurugiFfiClobDownloaderHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_downloader_dispose()";
    trace!("{FUNCTION_NAME} start. downloader={:?}", downloader);

    if downloader.is_null() {
        trace!("{FUNCTION_NAME} end. arg[downloader] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(downloader);
    }

    trace!("{FUNCTION_NAME} end");
}
