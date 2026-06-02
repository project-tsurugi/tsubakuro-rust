use std::{sync::Arc, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc},
    vec_u8_to_field, TsurugiFfiByteArrayHandle, TsurugiFfiDuration,
};

pub(crate) struct TsurugiFfiBlobDownloader {
    downloader: BlobDownloader,
    runtime: Arc<tokio::runtime::Runtime>,
    value: Option<Vec<u8>>,
}

impl TsurugiFfiBlobDownloader {
    pub(crate) fn new(
        downloader: BlobDownloader,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiBlobDownloader {
        TsurugiFfiBlobDownloader {
            downloader,
            runtime,
            value: None,
        }
    }

    fn runtime(&self) -> Arc<tokio::runtime::Runtime> {
        self.runtime.clone()
    }
}

impl std::ops::Deref for TsurugiFfiBlobDownloader {
    type Target = BlobDownloader;

    fn deref(&self) -> &Self::Target {
        &self.downloader
    }
}

impl std::ops::DerefMut for TsurugiFfiBlobDownloader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.downloader
    }
}

/// BlobDownloader.
///
/// since 0.10.0
pub type TsurugiFfiBlobDownloaderHandle = *mut TsurugiFfiBlobDownloader;

/// Downloads a chunk of data.
///
/// See [`BlobDownloader::download_chunk`].
///
/// # Receiver
/// - `downloader` - BlobDownloader.
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
pub extern "C" fn tsurugi_ffi_blob_downloader_download_chunk(
    context: TsurugiFfiContextHandle,
    downloader: TsurugiFfiBlobDownloaderHandle,
    length: u64,
    timeout: TsurugiFfiDuration,
    value_out: *mut TsurugiFfiByteArrayHandle,
    size_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_downloader_download_chunk()";
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
        downloader.download_chunk(length as usize, timeout)
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
        "{FUNCTION_NAME} end rc={:x}. (value={:?}, size={:?})",
        rc,
        ptr,
        size
    );
    rc
}

/// Downloads a chunk of data into the provided buffer.
///
/// See [`BlobDownloader::download_chunk_into`].
///
/// # Receiver
/// - `downloader` - BlobDownloader.
///
/// # Arguments
/// - `buffer` - buffer to store the chunk data.
/// - `buffer_size` - buffer size (number of bytes).
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `size_out` - chunk data size (number of bytes) actually downloaded.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_blob_downloader_download_chunk_into(
    context: TsurugiFfiContextHandle,
    downloader: TsurugiFfiBlobDownloaderHandle,
    buffer: *mut u8,
    buffer_size: u64,
    timeout: TsurugiFfiDuration,
    size_out: *mut u64,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_downloader_download_chunk_into()";
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
        downloader.download_chunk_into(
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

/// Disposes the BlobDownloader.
///
/// # Receiver
/// - `downloader` - BlobDownloader.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_blob_downloader_dispose(downloader: TsurugiFfiBlobDownloaderHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_downloader_dispose()";
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
