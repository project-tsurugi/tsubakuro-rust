use std::{sync::Arc, time::Duration};

use execute_result::{execute_result_processor, SqlExecuteResult};
use log::trace;
use query_result::{query_result_processor, SqlQueryResult};
use table_list::{list_tables_processor, TableList};

use crate::{
    error::TgError,
    job::Job,
    jogasaki::proto::sql::request::{request::Request as SqlRequestCommand, Request as SqlRequest},
    prelude::{CommitOption, ServiceClient},
    session::{
        wire::{Wire, WireResponse},
        Session,
    },
    transaction::{
        option::{CommitType, TransactionOption},
        transaction_begin_processor, transaction_commit_processor, transaction_dispose_processor,
        transaction_rollback_processor, Transaction,
    },
};

use prost::alloc::string::String as ProstString;

pub(crate) mod error;
pub mod execute_result;
pub mod name;
pub mod query_result;
pub mod table_list;

/// The symbolic ID of the destination service.
const SERVICE_SYMBOLIC_ID: &str = "sql";

/// The major service message version which this client requests.
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 1;

/// The minor service message version which this client requests.
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 3;

const SERVICE_ID_SQL: i32 = 3;

pub struct SqlClient {
    session: Arc<Session>,
    default_timeout: Duration,
}

impl ServiceClient for SqlClient {
    fn new(session: Arc<Session>) -> Self {
        SqlClient {
            session,
            default_timeout: Duration::ZERO,
        }
    }
}

impl SqlClient {
    pub fn service_message_version() -> String {
        format!(
            "{}-{}.{}",
            SERVICE_SYMBOLIC_ID, SERVICE_MESSAGE_VERSION_MAJOR, SERVICE_MESSAGE_VERSION_MINOR
        )
    }

    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    pub async fn list_tables(&self) -> Result<TableList, TgError> {
        self.list_tables_with_timeout(self.default_timeout).await
    }

    pub async fn list_tables_with_timeout(&self, timeout: Duration) -> Result<TableList, TgError> {
        const FUNCTION_NAME: &str = "list_tables()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::list_table_command();
        let response = self.send_and_pull_response(command, timeout).await?;
        let table_list = list_tables_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(table_list)
    }

    pub async fn list_tables_async(&self) -> Result<Job<TableList>, TgError> {
        self.list_tables_async_with_timeout(self.default_timeout)
            .await
    }

    pub async fn list_tables_async_with_timeout(
        &self,
        timeout: Duration,
    ) -> Result<Job<TableList>, TgError> {
        const FUNCTION_NAME: &str = "list_tables_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::list_table_command();
        let job = self
            .send_and_pull_async(command, Box::new(list_tables_processor), timeout)
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn list_table_command() -> SqlRequestCommand {
        let request = crate::jogasaki::proto::sql::request::ListTables {};
        SqlRequestCommand::ListTables(request)
    }

    // TODO SqlClient::get_table_metadata()

    // TODO SqlClient::create_prepared_statement()

    pub async fn start_transaction(
        &self,
        transaction_option: &TransactionOption,
    ) -> Result<Transaction, TgError> {
        self.start_transaction_with_timeout(transaction_option, self.default_timeout)
            .await
    }

    pub async fn start_transaction_with_timeout(
        &self,
        transaction_option: &TransactionOption,
        timeout: Duration,
    ) -> Result<Transaction, TgError> {
        const FUNCTION_NAME: &str = "start_transaction()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::begin_transaction_command(transaction_option);
        let response = self.send_and_pull_response(command, timeout).await?;
        let session = self.session.clone();
        let transaction = transaction_begin_processor(session, response, self.default_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(transaction)
    }

    pub async fn start_transaction_async(
        &self,
        transaction_option: &TransactionOption,
    ) -> Result<Job<Transaction>, TgError> {
        self.start_transaction_async_with_timeout(transaction_option, self.default_timeout)
            .await
    }

    pub async fn start_transaction_async_with_timeout(
        &self,
        transaction_option: &TransactionOption,
        timeout: Duration,
    ) -> Result<Job<Transaction>, TgError> {
        const FUNCTION_NAME: &str = "start_transaction_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::begin_transaction_command(transaction_option);
        let session = self.session.clone();
        let close_timeout = self.default_timeout;
        let job = self
            .send_and_pull_async(
                command,
                Box::new(move |response| {
                    transaction_begin_processor(session.clone(), response, close_timeout)
                }),
                timeout,
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn begin_transaction_command(transaction_option: &TransactionOption) -> SqlRequestCommand {
        let tx_option = transaction_option.as_request();

        let request = crate::jogasaki::proto::sql::request::Begin {
            option: Some(tx_option),
        };
        SqlRequestCommand::Begin(request)
    }

    pub async fn execute_statement(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<SqlExecuteResult, TgError> {
        self.execute_statement_with_timeout(transaction, sql, self.default_timeout)
            .await
    }

    pub async fn execute_statement_with_timeout(
        &self,
        transaction: &Transaction,
        sql: &str,
        timeout: Duration,
    ) -> Result<SqlExecuteResult, TgError> {
        const FUNCTION_NAME: &str = "execute_statement()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::execute_statement_command(tx_handle, sql);
        let response = self.send_and_pull_response(command, timeout).await?;
        let execute_result = execute_result_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(execute_result)
    }

    pub async fn execute_statement_async(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<Job<SqlExecuteResult>, TgError> {
        self.execute_statement_async_with_timeout(transaction, sql, self.default_timeout)
            .await
    }

    pub async fn execute_statement_async_with_timeout(
        &self,
        transaction: &Transaction,
        sql: &str,
        timeout: Duration,
    ) -> Result<Job<SqlExecuteResult>, TgError> {
        const FUNCTION_NAME: &str = "execute_statement_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::execute_statement_command(tx_handle, sql);
        let job = self
            .send_and_pull_async(command, Box::new(execute_result_processor), timeout)
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_statement_command(transaction_handle: u64, sql: &str) -> SqlRequestCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let request = crate::jogasaki::proto::sql::request::ExecuteStatement {
            transaction_handle: Some(tx_handle),
            sql: ProstString::from(sql),
        };
        SqlRequestCommand::ExecuteStatement(request)
    }

    pub async fn execute_query(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<SqlQueryResult, TgError> {
        self.execute_query_with_timeout(transaction, sql, self.default_timeout)
            .await
    }

    pub async fn execute_query_with_timeout(
        &self,
        transaction: &Transaction,
        sql: &str,
        timeout: Duration,
    ) -> Result<SqlQueryResult, TgError> {
        const FUNCTION_NAME: &str = "execute_query()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::execute_query_command(tx_handle, sql);
        let response = self.send_and_pull_response(command, timeout).await?;
        let result_set =
            query_result_processor(self.wire().clone(), response, self.default_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(result_set)
    }

    pub async fn execute_query_async(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<Job<SqlQueryResult>, TgError> {
        self.execute_query_async_with_timeout(transaction, sql, self.default_timeout)
            .await
    }

    pub async fn execute_query_async_with_timeout(
        &self,
        transaction: &Transaction,
        sql: &str,
        timeout: Duration,
    ) -> Result<Job<SqlQueryResult>, TgError> {
        const FUNCTION_NAME: &str = "execute_query_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::execute_query_command(tx_handle, sql);
        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let job = self
            .send_and_pull_async(
                command,
                Box::new(move |response| query_result_processor(wire, response, default_timeout)),
                timeout,
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_query_command(transaction_handle: u64, sql: &str) -> SqlRequestCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let request = crate::jogasaki::proto::sql::request::ExecuteQuery {
            transaction_handle: Some(tx_handle),
            sql: ProstString::from(sql),
        };
        SqlRequestCommand::ExecuteQuery(request)
    }

    pub async fn commit(
        &self,
        transaction: &Transaction,
        commit_option: &CommitOption,
    ) -> Result<(), TgError> {
        self.commit_with_timeout(transaction, commit_option, self.default_timeout)
            .await
    }

    pub async fn commit_with_timeout(
        &self,
        transaction: &Transaction,
        commit_option: &CommitOption,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "commit()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::commit_command(tx_handle, commit_option);
        let response = self.send_and_pull_response(command, timeout).await?;
        let _ = transaction_commit_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    pub async fn commit_async(
        &self,
        transaction: &Transaction,
        commit_option: &CommitOption,
    ) -> Result<Job<()>, TgError> {
        self.commit_async_with_timeout(transaction, commit_option, self.default_timeout)
            .await
    }

    pub async fn commit_async_with_timeout(
        &self,
        transaction: &Transaction,
        commit_option: &CommitOption,
        timeout: Duration,
    ) -> Result<Job<()>, TgError> {
        const FUNCTION_NAME: &str = "commit_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::commit_command(tx_handle, commit_option);
        let job = self
            .send_and_pull_async(command, Box::new(transaction_commit_processor), timeout)
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn commit_command(transaction_handle: u64, commit_option: &CommitOption) -> SqlRequestCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };

        use crate::jogasaki::proto::sql::request::CommitStatus;
        let commit_type = match commit_option.commit_type() {
            CommitType::Default => CommitStatus::Unspecified,
            CommitType::Accepted => CommitStatus::Accepted,
            CommitType::Available => CommitStatus::Available,
            CommitType::Stored => CommitStatus::Stored,
            CommitType::Propagated => CommitStatus::Propagated,
        };

        let request = crate::jogasaki::proto::sql::request::Commit {
            transaction_handle: Some(tx_handle),
            notification_type: commit_type.into(),
            auto_dispose: commit_option.auto_dispose(),
        };
        SqlRequestCommand::Commit(request)
    }

    pub async fn rollback(&self, transaction: &Transaction) -> Result<(), TgError> {
        self.rollback_with_timeout(transaction, self.default_timeout)
            .await
    }

    pub async fn rollback_with_timeout(
        &self,
        transaction: &Transaction,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "rollback()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::rollback_command(tx_handle);
        let response = self.send_and_pull_response(command, timeout).await?;
        let _ = transaction_rollback_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    pub async fn rollback_async(&self, transaction: &Transaction) -> Result<Job<()>, TgError> {
        self.rollback_async_with_timeout(transaction, self.default_timeout)
            .await
    }

    pub async fn rollback_async_with_timeout(
        &self,
        transaction: &Transaction,
        timeout: Duration,
    ) -> Result<Job<()>, TgError> {
        const FUNCTION_NAME: &str = "rollback_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::rollback_command(tx_handle);
        let job = self
            .send_and_pull_async(command, Box::new(transaction_rollback_processor), timeout)
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn rollback_command(transaction_handle: u64) -> SqlRequestCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };

        let request = crate::jogasaki::proto::sql::request::Rollback {
            transaction_handle: Some(tx_handle),
        };
        SqlRequestCommand::Rollback(request)
    }

    pub(crate) async fn dispose_transaction(
        &self,
        transaction_handle: u64,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "dispose_transaction()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::dispose_transaction_command(transaction_handle);
        let response = self.send_and_pull_response(command, timeout).await?;
        let _ = transaction_dispose_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    fn dispose_transaction_command(transaction_handle: u64) -> SqlRequestCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };

        let request = crate::jogasaki::proto::sql::request::DisposeTransaction {
            transaction_handle: Some(tx_handle),
        };
        SqlRequestCommand::DisposeTransaction(request)
    }
}

impl SqlClient {
    fn wire(&self) -> Arc<Wire> {
        self.session.wire()
    }

    async fn send_and_pull_response(
        &self,
        command: SqlRequestCommand,
        timeout: Duration,
    ) -> Result<WireResponse, TgError> {
        let request = Self::new_request(command);
        self.wire()
            .send_and_pull_response(SERVICE_ID_SQL, request, timeout)
            .await
    }

    async fn send_and_pull_async<T: 'static>(
        &self,
        command: SqlRequestCommand,
        converter: Box<dyn FnOnce(WireResponse) -> Result<T, TgError> + Send>,
        timeout: Duration,
    ) -> Result<Job<T>, TgError> {
        let request = Self::new_request(command);
        self.wire()
            .send_and_pull_async(SERVICE_ID_SQL, request, converter, timeout)
            .await
    }

    fn new_request(command: SqlRequestCommand) -> SqlRequest {
        SqlRequest {
            session_handle: None,
            service_message_version_major: SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: SERVICE_MESSAGE_VERSION_MINOR,
            request: Some(command),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn service_message_version() {
        let smv = SqlClient::service_message_version();
        assert_eq!("sql-1.3", smv);
    }
}
