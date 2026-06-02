//! blob.

use log::trace;
use tsubakuro_rust_core::prelude::*;

#[derive(Debug)]
pub(crate) struct TsurugiFfiBlob {
    blob: TgBlob,
}

impl TsurugiFfiBlob {
    pub(crate) fn new(blob: TgBlob) -> TsurugiFfiBlob {
        TsurugiFfiBlob { blob }
    }
}

impl std::ops::Deref for TsurugiFfiBlob {
    type Target = TgBlob;

    fn deref(&self) -> &Self::Target {
        &self.blob
    }
}

/// Blob.
///
/// since 0.10.0
pub type TsurugiFfiBlobHandle = *mut TsurugiFfiBlob;

/// Blob: Dispose.
///
/// # Receiver
/// - `blob` - blob.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_blob_dispose(blob: TsurugiFfiBlobHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_dispose()";
    trace!("{FUNCTION_NAME} start. blob={:?}", blob);

    if blob.is_null() {
        trace!("{FUNCTION_NAME} end. arg[blob] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(blob);
    }

    trace!("{FUNCTION_NAME} end");
}

#[derive(Debug)]
pub(crate) struct TsurugiFfiBlobReference {
    blob_reference: TgBlobReference,
    pub(crate) value: Option<Vec<u8>>,
}

impl TsurugiFfiBlobReference {
    pub(crate) fn new(blob_reference: TgBlobReference) -> TsurugiFfiBlobReference {
        TsurugiFfiBlobReference {
            blob_reference,
            value: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiBlobReference {
    type Target = TgBlobReference;

    fn deref(&self) -> &Self::Target {
        &self.blob_reference
    }
}

/// Blob.
pub type TsurugiFfiBlobReferenceHandle = *mut TsurugiFfiBlobReference;

/// BlobReference: Dispose.
///
/// # Receiver
/// - `blob_reference` - blob reference.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_blob_reference_dispose(
    blob_reference: TsurugiFfiBlobReferenceHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_blob_reference_dispose()";
    trace!("{FUNCTION_NAME} start. blob_reference={:?}", blob_reference);

    if blob_reference.is_null() {
        trace!("{FUNCTION_NAME} end. arg[blob_reference] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(blob_reference);
    }

    trace!("{FUNCTION_NAME} end");
}
