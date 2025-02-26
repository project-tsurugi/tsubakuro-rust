use log::trace;
use tsubakuro_rust_core::prelude::*;

#[derive(Debug)]
pub(crate) struct TsurugiFfiTgBlobReference {
    _blob_reference: TgBlobReference,
}

impl TsurugiFfiTgBlobReference {
    pub(crate) fn new(blob_reference: TgBlobReference) -> TsurugiFfiTgBlobReference {
        TsurugiFfiTgBlobReference {
            _blob_reference: blob_reference,
        }
    }
}

pub type TsurugiFfiBlobReferenceHandle = *mut TsurugiFfiTgBlobReference;

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
