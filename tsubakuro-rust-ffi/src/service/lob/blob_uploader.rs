use std::{sync::Arc, time::Duration};

use log::{trace, warn};
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::{TsurugiFfiContext, TsurugiFfiContextHandle},
    error::TsurugiFfiError,
    ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{
        rc_ok, TsurugiFfiRc, TSURUGI_FFI_RC_FFI_ALREADY_FINISHED, TSURUGI_FFI_RC_FFI_CANCEL_ERROR,
    },
    service::sql::r#type::blob::{TsurugiFfiBlob, TsurugiFfiBlobHandle},
    TsurugiFfiByteArrayHandle, TsurugiFfiDuration,
};

pub(crate) struct TsurugiFfiBlobUploader {
    uploader: Option<BlobUploader>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl TsurugiFfiBlobUploader {
    pub(crate) fn new(
        uploader: BlobUploader,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiBlobUploader {
        TsurugiFfiBlobUploader {
            uploader: Some(uploader),
            runtime,
        }
    }

    fn runtime(&self) -> Arc<tokio::runtime::Runtime> {
        self.runtime.clone()
    }
}

/// BlobUploader.
///
/// since 0.10.0
pub type TsurugiFfiBlobUploaderHandle = *mut TsurugiFfiBlobUploader;

/// Uploads a chunk of data.
///
/// See [`BlobUploader::upload_chunk`].
///
/// # Receiver
/// - `uploader` - BlobUploader.
///
/// # Arguments
/// - `value` - chunk data.
/// - `size` - chunk data size (number of bytes).
/// - `timeout` - timeout time \[nanoseconds\].
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_blob_uploader_upload_chunk(
    context: TsurugiFfiContextHandle,
    uploader: TsurugiFfiBlobUploaderHandle,
    value: TsurugiFfiByteArrayHandle,
    size: u64,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_uploader_upload_chunk()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, uploader={:?}, value={:?}, size={}, timeout={:?}",
        context,
        uploader,
        value,
        size,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, uploader);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value);

    let uploader = unsafe { &mut *uploader };
    let runtime = uploader.runtime();
    let value = unsafe { std::slice::from_raw_parts(value, size as usize) };
    let timeout = Duration::from_nanos(timeout);

    let uploader = if let Some(uploader) = uploader.uploader.as_mut() {
        uploader
    } else {
        warn!("{FUNCTION_NAME} BlobUploader is already finished");
        let rc = TSURUGI_FFI_RC_FFI_ALREADY_FINISHED;
        let error = TsurugiFfiError::FfiError(rc, "BlobUploader is already finished".to_string());
        TsurugiFfiContext::set_error(context, rc, error);
        trace!("{FUNCTION_NAME} end rc={:x}", rc);
        return rc;
    };

    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        uploader.upload_chunk(value, timeout)
    );

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// Finishes the upload and returns the resulting `TgBlob`.
///
/// See [`BlobUploader::finish`].
///
/// # Receiver
/// - `uploader` - BlobUploader.
///
/// # Arguments
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `blob_out` - BLOB. To dispose, call [`tsurugi_ffi_blob_dispose`](crate::service::sql::type::blob::tsurugi_ffi_blob_dispose).
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_blob_uploader_finish(
    context: TsurugiFfiContextHandle,
    uploader: TsurugiFfiBlobUploaderHandle,
    timeout: TsurugiFfiDuration,
    blob_out: *mut TsurugiFfiBlobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_uploader_finish()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, uploader={:?}, timeout={:?}, blob_out={:?}",
        context,
        uploader,
        timeout,
        blob_out
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, uploader);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, blob_out);

    let uploader = unsafe { &mut *uploader };
    let timeout = Duration::from_nanos(timeout);
    let runtime = uploader.runtime();

    let uploader = if let Some(uploader) = uploader.uploader.take() {
        uploader
    } else {
        warn!("{FUNCTION_NAME} BlobUploader is already finished");
        let rc = TSURUGI_FFI_RC_FFI_ALREADY_FINISHED;
        let error = TsurugiFfiError::FfiError(rc, "BlobUploader is already finished".to_string());
        TsurugiFfiContext::set_error(context, rc, error);
        trace!("{FUNCTION_NAME} end rc={:x}", rc);
        return rc;
    };
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, uploader.finish(timeout));

    let blob = Box::new(TsurugiFfiBlob::new(value));
    let handle = Box::into_raw(blob);
    unsafe {
        *blob_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. blob={:?}", rc, handle);
    rc
}

/// Cancels the upload.
///
/// See [`BlobUploader::cancel`].
///
/// # Receiver
/// - `uploader` - BlobUploader.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_blob_uploader_cancel(
    context: TsurugiFfiContextHandle,
    uploader: TsurugiFfiBlobUploaderHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_uploader_cancel()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, uploader={:?}",
        context,
        uploader
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, uploader);

    let uploader = unsafe { &mut *uploader };

    let uploader = if let Some(uploader) = uploader.uploader.take() {
        uploader
    } else {
        warn!("{FUNCTION_NAME} BlobUploader is already finished");
        let rc = TSURUGI_FFI_RC_FFI_ALREADY_FINISHED;
        let error = TsurugiFfiError::FfiError(rc, "BlobUploader is already finished".to_string());
        TsurugiFfiContext::set_error(context, rc, error);
        trace!("{FUNCTION_NAME} end rc={:x}", rc);
        return rc;
    };
    if let Err(e) = uploader.cancel() {
        warn!("{FUNCTION_NAME} failed to cancel BlobUploader: {:?}", e);
        let rc = TSURUGI_FFI_RC_FFI_CANCEL_ERROR;
        let error = TsurugiFfiError::FfiError(rc, "Failed to cancel BlobUploader".to_string());
        TsurugiFfiContext::set_error(context, rc, error);
        trace!("{FUNCTION_NAME} end rc={:x}", rc);
        return rc;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// Disposes the BlobUploader.
///
/// # Receiver
/// - `uploader` - BlobUploader.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_blob_uploader_dispose(uploader: TsurugiFfiBlobUploaderHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_uploader_dispose()";
    trace!("{FUNCTION_NAME} start. uploader={:?}", uploader);

    if uploader.is_null() {
        trace!("{FUNCTION_NAME} end. arg[uploader] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(uploader);
    }

    trace!("{FUNCTION_NAME} end");
}
