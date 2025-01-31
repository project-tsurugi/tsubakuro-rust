use tsubakuro_rust_core::error::{DiagnosticCode, TgError};

use crate::return_code::{rc_to_name, TsurugiFfiRc};

#[derive(Debug)]
pub(crate) enum TsurugiFfiError {
    FfiError(TsurugiFfiRc, /* message */ String),
    CoreError(TsurugiFfiRc, TgError),
}

impl TsurugiFfiError {
    pub(crate) fn name(&self) -> String {
        match self {
            TsurugiFfiError::FfiError(rc, _message) => rc_to_name(*rc).to_string(),
            TsurugiFfiError::CoreError(_rc, error) => match error {
                TgError::ClientError(_, _error) => "TSURUGI_CORE_CLIENT_ERROR".to_string(),
                TgError::TimeoutError(_) => "TSURUGI_CORE_CLIENT_TIMEOUT_ERROR".to_string(),
                TgError::IoError(_, _error) => "TSURUGI_CORE_CLIENT_IO_ERROR".to_string(),
                TgError::ServerError(_, code, _) => code.name().clone(),
            },
        }
    }

    pub(crate) fn message(&self) -> String {
        match self {
            TsurugiFfiError::FfiError(_rc, message) => message.clone(),
            TsurugiFfiError::CoreError(_rc, error) => error.to_string(),
        }
    }

    pub(crate) fn diagnostic_code(&self) -> Option<&DiagnosticCode> {
        match self {
            TsurugiFfiError::CoreError(_, e) => e.diagnostic_code(),
            _ => None,
        }
    }
}
