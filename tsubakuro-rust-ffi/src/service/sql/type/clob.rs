//! clob.

use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

#[derive(Debug)]
pub(crate) struct TsurugiFfiClob {
    clob: TgClob,
}

impl TsurugiFfiClob {
    pub(crate) fn new(clob: TgClob) -> TsurugiFfiClob {
        TsurugiFfiClob { clob }
    }
}

impl std::ops::Deref for TsurugiFfiClob {
    type Target = TgClob;

    fn deref(&self) -> &Self::Target {
        &self.clob
    }
}

/// Clob.
///
/// since 0.10.0
pub type TsurugiFfiClobHandle = *mut TsurugiFfiClob;

/// Clob: Dispose.
///
/// # Receiver
/// - `clob` - clob.
///
/// since 0.10.0
#[no_mangle]
pub extern "C" fn tsurugi_ffi_clob_dispose(clob: TsurugiFfiClobHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_clob_dispose()";
    trace!("{FUNCTION_NAME} start. clob={:?}", clob);

    if clob.is_null() {
        trace!("{FUNCTION_NAME} end. arg[clob] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(clob);
    }

    trace!("{FUNCTION_NAME} end");
}

#[derive(Debug)]
pub(crate) struct TsurugiFfiClobReference {
    clob_reference: TgClobReference,
    pub(crate) value: Option<CString>,
}

impl TsurugiFfiClobReference {
    pub(crate) fn new(clob_reference: TgClobReference) -> TsurugiFfiClobReference {
        TsurugiFfiClobReference {
            clob_reference,
            value: None,
        }
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
