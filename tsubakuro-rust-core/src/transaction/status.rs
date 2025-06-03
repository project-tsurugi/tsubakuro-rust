use std::sync::Arc;

use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::{
        get_transaction_status::Success, response::Response as SqlResponseType,
    },
    prelude::{convert_sql_response, TransactionStatus},
    session::wire::{response::WireResponse, response_box::SlotEntryHandle},
    sql_service_error,
};

/// Transaction status.
///
/// since 0.2.0
#[derive(Debug)]
pub struct TransactionStatusWithMessage {
    status: Success,
}

impl TransactionStatusWithMessage {
    pub(crate) fn new(status: Success) -> TransactionStatusWithMessage {
        TransactionStatusWithMessage { status }
    }

    /// Returns the enum value of status.
    pub fn status(&self) -> TransactionStatus {
        self.status.status()
    }

    /// Returns additional information for the transaction status.
    pub fn message(&self) -> &String {
        &self.status.message
    }
}

pub(crate) fn transaction_status_processor(
    _: Arc<SlotEntryHandle>,
    response: WireResponse,
) -> Result<TransactionStatusWithMessage, TgError> {
    const FUNCTION_NAME: &str = "transaction_status_processor()";

    let (sql_response, _) = convert_sql_response(FUNCTION_NAME, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        FUNCTION_NAME,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;

    use crate::jogasaki::proto::sql::response::get_transaction_status::Result;
    match message.response {
        Some(SqlResponseType::GetTransactionStatus(status)) => match status.result {
            Some(Result::Success(success)) => Ok(TransactionStatusWithMessage::new(success)),
            Some(Result::Error(error)) => Err(sql_service_error!(FUNCTION_NAME, error)),
            None => Err(invalid_response_error!(
                FUNCTION_NAME,
                format!("response GetTransactionStatus.result is None"),
            )),
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!(
                "response {:?} is not GetTransactionStatus",
                message.response
            ),
        )),
    }
}
