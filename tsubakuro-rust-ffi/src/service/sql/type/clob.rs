//! clob.

use log::trace;
use tsubakuro_rust_core::prelude::*;

#[derive(Debug)]
pub(crate) struct TsurugiFfiClobReference {
    clob_reference: TgClobReference,
}

impl TsurugiFfiClobReference {
    pub(crate) fn new(clob_reference: TgClobReference) -> TsurugiFfiClobReference {
        TsurugiFfiClobReference { clob_reference }
    }
}

impl std::ops::Deref for TsurugiFfiClobReference {
    type Target = TgClobReference;

    fn deref(&self) -> &Self::Target {
        &self.clob_reference
    }
}

/// Clob.
pub type TsurugiFfiClobReferenceHandle = *mut TsurugiFfiClobReference;

/// ClobReference: Dispose.
///
/// # Receiver
/// - `clob_reference` - clob reference.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_reference_dispose(
    clob_reference: TsurugiFfiClobReferenceHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_reference_dispose()";
    trace!("{FUNCTION_NAME} start. clob_reference={:?}", clob_reference);

    if clob_reference.is_null() {
        trace!("{FUNCTION_NAME} end. arg[clob_reference] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(clob_reference);
    }

    trace!("{FUNCTION_NAME} end");
}
