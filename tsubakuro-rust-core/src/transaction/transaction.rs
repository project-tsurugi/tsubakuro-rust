use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use log::{error, warn};

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

/// Transaction.
///
/// See [SqlClient::start_transaction()](crate::prelude::SqlClient::start_transaction),
/// [execute()](crate::prelude::SqlClient::execute),
/// [query()](crate::prelude::SqlClient::query),
/// [get_transaction_status()](crate::prelude::SqlClient::get_transaction_status),
/// [commit()](crate::prelude::SqlClient::commit),
/// [rollback()](crate::prelude::SqlClient::rollback).
///
/// Note: Should invoke [`Self::close`] before [`Self::drop`] to dispose the transaction.
///
/// # Examples
/// ```
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example(client: &SqlClient) -> Result<(), TgError> {
///     let mut transaction_option = TransactionOption::new();
///     transaction_option.set_transaction_type(TransactionType::Short); // OCC
///     let transaction = client.start_transaction(&transaction_option).await?;
///
///     let mut result: Result<(), TgError> = todo!(); // execute SQL using transaction
///
///     if result.is_ok() {
///         let commit_option = CommitOption::default();
///         result = client.commit(&transaction, &commit_option).await;
///     }
///     transaction.close().await?;
///
///     result
/// }
/// ```
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

    /// Provides transaction id that is unique to for the duration of the database server's lifetime.
    pub fn transaction_id(&self) -> &String {
        &self.transaction_id
    }

    /// Set close timeout.
    pub fn set_close_timeout(&mut self, timeout: Duration) {
        self.close_timeout = timeout;
    }

    /// Get close timeout.
    pub fn close_timeout(&self) -> Duration {
        self.close_timeout
    }

    /// Disposes this resource.
    ///
    /// Note: Should invoke `close` before [`Self::drop`] to dispose the transaction.
    pub async fn close(&self) -> Result<(), TgError> {
        self.close_for(self.close_timeout).await
    }

    /// Disposes this resource.
    ///
    /// Note: Should invoke `close_for` before [`Self::drop`] to dispose the transaction.
    pub async fn close_for(&self, timeout: Duration) -> Result<(), TgError> {
        if self
            .closed
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            )
            .is_ok()
        {
            let sql_client = SqlClient::new(self.session.clone());
            let tx_handle = self.transaction_handle;
            sql_client.dispose_transaction(tx_handle, timeout).await?;
        }
        Ok(())
    }

    /// Check if this resource is closed.
    pub fn is_closed(&self) -> bool {
        self.closed.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// for debug
    #[doc(hidden)]
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
                            error!("Transaction.drop() runtime::new error. {}", e);
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
                        warn!("Transaction.drop() dispose error. {}", e);
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
