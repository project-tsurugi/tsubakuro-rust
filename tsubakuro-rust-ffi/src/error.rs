use tsubakuro_rust_core::error::TgError;

use crate::return_code::TsurugiFfiRc;

#[derive(Debug)]
pub(crate) enum TsurugiFfiError {
    FfiError(TsurugiFfiRc, /* message */ String),
    CoreError(TsurugiFfiRc, TgError),
}

impl TsurugiFfiError {
    pub(crate) fn message(&self) -> String {
        match self {
            TsurugiFfiError::FfiError(_rc, message) => message.clone(),
            TsurugiFfiError::CoreError(_rc, error) => error.to_string(),
        }
    }
}
