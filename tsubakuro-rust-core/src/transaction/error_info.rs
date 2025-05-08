use crate::{
    error::{DiagnosticCode, TgError},
    invalid_response_error,
    jogasaki::proto::sql::response::response::Response as SqlResponseType,
    prelude::convert_sql_response,
    session::wire::response::WireResponse,
    sql_service_error,
};

/// Transaction error information.
///
/// since 0.2.0
#[derive(Debug)]
pub struct TransactionErrorInfo {
    server_error: Option<TgError>,
}

impl TransactionErrorInfo {
    pub(crate) fn new(server_error: Option<TgError>) -> TransactionErrorInfo {
        TransactionErrorInfo { server_error }
    }

    /// Returns occurred error in the target transaction, only if the transaction has been accidentally aborted.
    pub fn server_error(&self) -> Option<&TgError> {
        self.server_error.as_ref()
    }

    /// Whether the status is normal.
    pub fn is_normal(&self) -> bool {
        self.server_error.is_none()
    }

    /// Whether the status is error.
    pub fn is_error(&self) -> bool {
        self.server_error.is_some()
    }

    /// Returns diagnostic code if error occurred in the target transaction.
    pub fn diagnostic_code(&self) -> Option<&DiagnosticCode> {
        match &self.server_error {
            Some(TgError::ServerError(_, _, code, _)) => Some(code),
            _ => None,
        }
    }
}

pub(crate) fn transaction_error_info_processor(
    response: WireResponse,
) -> Result<TransactionErrorInfo, TgError> {
    const FUNCTION_NAME: &str = "transaction_error_info_processor()";

    let (sql_response, _) = convert_sql_response(FUNCTION_NAME, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        FUNCTION_NAME,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;

    use crate::jogasaki::proto::sql::response::get_error_info::Result;
    match message.response {
        Some(SqlResponseType::GetErrorInfo(info)) => match info.result {
            Some(Result::Success(error)) => Ok(TransactionErrorInfo::new(Some(
                sql_service_error!(FUNCTION_NAME, error),
            ))),
            Some(Result::ErrorNotFound(_)) => Ok(TransactionErrorInfo::new(None)),
            Some(Result::Error(error)) => Err(sql_service_error!(FUNCTION_NAME, error)),
            None => Err(invalid_response_error!(
                FUNCTION_NAME,
                format!("response GetErrorInfo.result is None"),
            )),
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not GetErrorInfo", message.response),
        )),
    }
}
