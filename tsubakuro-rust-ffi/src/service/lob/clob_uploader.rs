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
    service::sql::r#type::clob::{TsurugiFfiClob, TsurugiFfiClobHandle},
    TsurugiFfiByteArrayHandle, TsurugiFfiDuration,
};

pub(crate) struct TsurugiFfiClobUploader {
    uploader: Option<ClobUploader>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl TsurugiFfiClobUploader {
    pub(crate) fn new(
        uploader: ClobUploader,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiClobUploader {
        TsurugiFfiClobUploader {
            uploader: Some(uploader),
            runtime,
        }
    }

    fn runtime(&self) -> Arc<tokio::runtime::Runtime> {
        self.runtime.clone()
    }
}

/// ClobUploader.
///
/// since 0.10.0
pub type TsurugiFfiClobUploaderHandle = *mut TsurugiFfiClobUploader;

/// Uploads a chunk of data.
///
/// See [`ClobUploader::upload_chunk_utf8`].
///
/// # Receiver
/// - `uploader` - ClobUploader.
///
/// # Arguments
/// - `value` - chunk data as UTF-8 bytes.
/// - `size` - chunk data size (number of bytes).
/// - `timeout` - timeout time \[nanoseconds\].
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_uploader_upload_chunk_utf8(
    context: TsurugiFfiContextHandle,
    uploader: TsurugiFfiClobUploaderHandle,
    value: TsurugiFfiByteArrayHandle,
    size: u64,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_uploader_upload_chunk_utf8()";
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
        warn!("{FUNCTION_NAME} ClobUploader is already finished");
        let rc = TSURUGI_FFI_RC_FFI_ALREADY_FINISHED;
        let error = TsurugiFfiError::FfiError(rc, "ClobUploader is already finished".to_string());
        TsurugiFfiContext::set_error(context, rc, error);
        trace!("{FUNCTION_NAME} end rc={:x}", rc);
        return rc;
    };

    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        uploader.upload_chunk_utf8(value, timeout)
    );

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// Finishes the upload and returns the resulting `TgClob`.
///
/// See [`ClobUploader::finish`].
///
/// # Receiver
/// - `uploader` - ClobUploader.
///
/// # Arguments
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `clob_out` - CLOB. To dispose, call [`tsurugi_ffi_clob_dispose`](crate::service::sql::type::clob::tsurugi_ffi_clob_dispose).
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_uploader_finish(
    context: TsurugiFfiContextHandle,
    uploader: TsurugiFfiClobUploaderHandle,
    timeout: TsurugiFfiDuration,
    clob_out: *mut TsurugiFfiClobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_uploader_finish()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, uploader={:?}, timeout={:?}, clob_out={:?}",
        context,
        uploader,
        timeout,
        clob_out
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, uploader);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, clob_out);

    let uploader = unsafe { &mut *uploader };
    let timeout = Duration::from_nanos(timeout);
    let runtime = uploader.runtime();

    let uploader = if let Some(uploader) = uploader.uploader.take() {
        uploader
    } else {
        warn!("{FUNCTION_NAME} ClobUploader is already finished");
        let rc = TSURUGI_FFI_RC_FFI_ALREADY_FINISHED;
        let error = TsurugiFfiError::FfiError(rc, "ClobUploader is already finished".to_string());
        TsurugiFfiContext::set_error(context, rc, error);
        trace!("{FUNCTION_NAME} end rc={:x}", rc);
        return rc;
    };
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, uploader.finish(timeout));

    let clob = Box::new(TsurugiFfiClob::new(value));
    let handle = Box::into_raw(clob);
    unsafe {
        *clob_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. clob={:?}", rc, handle);
    rc
}

/// Cancels the upload.
///
/// See [`ClobUploader::cancel`].
///
/// # Receiver
/// - `uploader` - ClobUploader.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_uploader_cancel(
    context: TsurugiFfiContextHandle,
    uploader: TsurugiFfiClobUploaderHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_uploader_cancel()";
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
        warn!("{FUNCTION_NAME} ClobUploader is already finished");
        let rc = TSURUGI_FFI_RC_FFI_ALREADY_FINISHED;
        let error = TsurugiFfiError::FfiError(rc, "ClobUploader is already finished".to_string());
        TsurugiFfiContext::set_error(context, rc, error);
        trace!("{FUNCTION_NAME} end rc={:x}", rc);
        return rc;
    };
    if let Err(e) = uploader.cancel() {
        warn!("{FUNCTION_NAME} failed to cancel ClobUploader: {:?}", e);
        let rc = TSURUGI_FFI_RC_FFI_CANCEL_ERROR;
        let error = TsurugiFfiError::FfiError(rc, "Failed to cancel ClobUploader".to_string());
        TsurugiFfiContext::set_error(context, rc, error);
        trace!("{FUNCTION_NAME} end rc={:x}", rc);
        return rc;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// Disposes the ClobUploader.
///
/// # Receiver
/// - `uploader` - ClobUploader.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_uploader_dispose(uploader: TsurugiFfiClobUploaderHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_uploader_dispose()";
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
