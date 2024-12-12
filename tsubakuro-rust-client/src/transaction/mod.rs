use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use log::debug;
use prost::Message;

use crate::{
    client_error,
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::{
        response::Response as SqlResponseCase, Response as SqlResponse,
    },
    prelude::{sql::SqlClient, ServiceClient, Session},
    prost_decode_error,
    session::wire::WireResponse,
    sql_service_error,
};

pub mod option;

#[derive(Debug)]
pub struct Transaction {
    session: Arc<Session>,
    transaction_handle: u64,
    transaction_id: String,
    close_timeout: Duration,
    closed: AtomicBool,
}

impl Transaction {
    pub(crate) fn new(
        session: Arc<Session>,
        transaction_handle: u64,
        transaction_id: String,
        close_timeout: Duration,
    ) -> Transaction {
        Transaction {
            session,
            transaction_handle,
            transaction_id,
            close_timeout,
            closed: AtomicBool::new(false),
        }
    }

    pub(crate) fn transaction_handle(&self) -> Result<u64, TgError> {
        if self.is_closed() {
            Err(client_error!("transaction already closed"))
        } else {
            Ok(self.transaction_handle)
        }
    }

    pub fn transaction_id(&self) -> &String {
        &self.transaction_id
    }

    pub fn set_close_timeout(&mut self, timeout: Duration) {
        self.close_timeout = timeout;
    }

    pub fn close_timeout(&self) -> Duration {
        self.close_timeout
    }

    pub async fn close(&self) -> Result<(), TgError> {
        self.close_for(self.close_timeout).await
    }

    pub async fn close_for(&self, timeout: Duration) -> Result<(), TgError> {
        if let Ok(_) = self.closed.compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
        ) {
            let sql_client = SqlClient::new(self.session.clone());
            let tx_handle = self.transaction_handle;
            sql_client.dispose_transaction(tx_handle, timeout).await?;
        }
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.is_closed() {
            return;
        }

        std::thread::scope(|scope| {
            scope.spawn(move || {
                let runtime = {
                    match tokio::runtime::Runtime::new() {
                        Ok(runtime) => runtime,
                        Err(e) => {
                            debug!("Transaction.drop() error. {}", e);
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    if let Err(e) = self.close().await {
                        debug!("Transaction.drop() error. {}", e);
                    }
                })
            });
        });
    }
}

pub(crate) fn transaction_begin_processor(
    session: Arc<Session>,
    response: WireResponse,
    close_timeout: Duration,
) -> Result<Transaction, TgError> {
    const FUNCTION_NAME: &str = "transaction_begin_processor()";

    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload) = response {
        payload.unwrap()
    } else {
        return Err(invalid_response_error!(
            FUNCTION_NAME,
            "response is not ResponseSessionPayload",
        ));
    };

    let message = SqlResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "SqlResponse", e))?;
    match message.response {
        Some(SqlResponseCase::Begin(begin)) => match begin.result.unwrap() {
            crate::jogasaki::proto::sql::response::begin::Result::Success(success) => {
                let tx_handle = success
                    .transaction_handle
                    .ok_or(invalid_response_error!(
                        FUNCTION_NAME,
                        "response.transaction_handle is None"
                    ))?
                    .handle;
                let tx_id = success
                    .transaction_id
                    .ok_or(invalid_response_error!(
                        FUNCTION_NAME,
                        "response.transaction_id is None"
                    ))?
                    .id;
                Ok(Transaction::new(session, tx_handle, tx_id, close_timeout))
            }
            crate::jogasaki::proto::sql::response::begin::Result::Error(error) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not Begin", message.response),
        )),
    }
}

pub(crate) fn transaction_commit_processor(response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "transaction_commit_processor()";

    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload) = response {
        payload.unwrap()
    } else {
        return Err(invalid_response_error!(
            FUNCTION_NAME,
            "response is not ResponseSessionPayload",
        ));
    };

    let message = SqlResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "SqlResponse", e))?;
    match message.response {
        Some(SqlResponseCase::ResultOnly(result_only)) => match result_only.result.unwrap() {
            crate::jogasaki::proto::sql::response::result_only::Result::Success(_) => Ok(()),
            crate::jogasaki::proto::sql::response::result_only::Result::Error(error) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not ResultOnly", message.response),
        )),
    }
}

pub(crate) fn transaction_rollback_processor(response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "transaction_rollback_processor()";

    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload) = response {
        payload.unwrap()
    } else {
        return Err(invalid_response_error!(
            FUNCTION_NAME,
            "response is not ResponseSessionPayload",
        ));
    };

    let message = SqlResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "SqlResponse", e))?;
    match message.response {
        Some(SqlResponseCase::ResultOnly(result_only)) => match result_only.result.unwrap() {
            crate::jogasaki::proto::sql::response::result_only::Result::Success(_) => Ok(()),
            crate::jogasaki::proto::sql::response::result_only::Result::Error(error) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not ResultOnly", message.response),
        )),
    }
}

pub(crate) fn transaction_dispose_processor(response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "transaction_dispose_processor()";

    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload) = response {
        payload.unwrap()
    } else {
        return Err(invalid_response_error!(
            FUNCTION_NAME,
            "response is not ResponseSessionPayload",
        ));
    };

    let message = SqlResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "SqlResponse", e))?;
    match message.response {
        Some(SqlResponseCase::ResultOnly(result_only)) => match result_only.result.unwrap() {
            crate::jogasaki::proto::sql::response::result_only::Result::Success(_) => Ok(()),
            crate::jogasaki::proto::sql::response::result_only::Result::Error(error) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not ResultOnly", message.response),
        )),
    }
}
