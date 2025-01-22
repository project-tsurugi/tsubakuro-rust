use tsubakuro_rust_core::error::TgError;

use crate::return_code::TsurugiFfiRc;

#[derive(Debug)]
pub(crate) enum TsurugiFfiError {
    FfiError(/*rc*/ TsurugiFfiRc, /* message */ String),
    _CoreError(TgError),
}

impl TsurugiFfiError {
    pub(crate) fn message(&self) -> &String {
        match self {
            TsurugiFfiError::FfiError(_rc, message) => message,
            TsurugiFfiError::_CoreError(error) => error.message(),
        }
    }
}
