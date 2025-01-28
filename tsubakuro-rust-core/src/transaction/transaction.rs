use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use log::debug;

use crate::{
    client_error,
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::response::Response as SqlResponseType,
    prelude::{
        convert_sql_response, sql::SqlClient, sql_result_only_success_processor, ServiceClient,
        Session,
    },
    session::wire::response::WireResponse,
    sql_service_error,
};

#[derive(Debug)]
pub struct Transaction {
    session: Arc<Session>,
    transaction_handle: u64,
    transaction_id: String,
    close_timeout: Duration,
    closed: AtomicBool,
    fail_on_drop_error: AtomicBool,
}

impl Transaction {
    pub(crate) fn new(
        session: Arc<Session>,
        transaction_handle: u64,
        transaction_id: String,
        close_timeout: Duration,
    ) -> Transaction {
        let fail_on_drop_error = session.fail_on_drop_error();
        Transaction {
            session,
            transaction_handle,
            transaction_id,
            close_timeout,
            closed: AtomicBool::new(false),
            fail_on_drop_error: AtomicBool::new(fail_on_drop_error),
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

    // for debug
    pub fn set_fail_on_drop_error(&self, value: bool) {
        self.fail_on_drop_error
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }

    pub(crate) fn fail_on_drop_error(&self) -> bool {
        self.fail_on_drop_error
            .load(std::sync::atomic::Ordering::SeqCst)
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
                            debug!("Transaction.drop() runtime::new error. {}", e);
                            if self.fail_on_drop_error() {
                                panic!("Transaction.drop() runtime::new error. {}", e);
                            }
                            return;
                        }
                    }
                };
                runtime.block_on(async {
                    let sql_client = SqlClient::new(self.session.clone());
                    let tx_handle = self.transaction_handle;
                    if let Err(e) = sql_client.dispose_transaction_send_only(tx_handle).await {
                        debug!("Transaction.drop() dispose error. {}", e);
                        if self.fail_on_drop_error() {
                            panic!("Transaction.drop() dispose error. {}", e);
                        }
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

    let sql_response = convert_sql_response(FUNCTION_NAME, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        FUNCTION_NAME,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;
    match message.response {
        Some(SqlResponseType::Begin(begin)) => match begin.result {
            Some(crate::jogasaki::proto::sql::response::begin::Result::Success(success)) => {
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
            Some(crate::jogasaki::proto::sql::response::begin::Result::Error(error)) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
            None => Err(invalid_response_error!(
                FUNCTION_NAME,
                format!("response Begin.result is None"),
            )),
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not Begin", message.response),
        )),
    }
}

pub(crate) fn transaction_commit_processor(response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "transaction_commit_processor()";

    sql_result_only_success_processor(FUNCTION_NAME, response)
}

pub(crate) fn transaction_rollback_processor(response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "transaction_rollback_processor()";

    sql_result_only_success_processor(FUNCTION_NAME, response)
}

pub(crate) fn transaction_dispose_processor(response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "transaction_dispose_processor()";

    sql_result_only_success_processor(FUNCTION_NAME, response)
}
