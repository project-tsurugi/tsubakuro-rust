use log::trace;
use tsubakuro_rust_core::prelude::*;

#[derive(Debug)]
pub(crate) struct TsurugiFfiTgClobReference {
    _clob_reference: TgClobReference,
}

impl TsurugiFfiTgClobReference {
    pub(crate) fn new(clob_reference: TgClobReference) -> TsurugiFfiTgClobReference {
        TsurugiFfiTgClobReference {
            _clob_reference: clob_reference,
        }
    }
}

pub type TsurugiFfiClobReferenceHandle = *mut TsurugiFfiTgClobReference;

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
