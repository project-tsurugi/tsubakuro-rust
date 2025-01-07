use std::{sync::Arc, time::Duration};

use execute_result::{execute_result_processor, SqlExecuteResult};
use log::trace;
use prepare::{prepare_dispose_processor, prepare_processor, SqlPreparedStatement};
use query_result::{query_result_processor, SqlQueryResult};
use table_list::{list_tables_processor, TableList};
use table_metadata::{table_metadata_processor, TableMetadata};

use crate::{
    error::TgError,
    invalid_response_error,
    job::Job,
    jogasaki::proto::sql::{
        request::{request::Request as SqlRequestCommand, Request as SqlRequest},
        response::{response::Response as SqlResponseType, Response as SqlResponse},
    },
    prelude::{CommitOption, ServiceClient, SqlParameter, SqlPlaceholder},
    prost_decode_error,
    session::{
        wire::{response::WireResponse, Wire},
        Session,
    },
    sql_service_error,
    transaction::{
        option::TransactionOption, transaction_begin_processor, transaction_commit_processor,
        transaction_dispose_processor, transaction_rollback_processor, Transaction,
    },
};

use prost::{alloc::string::String as ProstString, Message};

pub mod column;
pub(crate) mod error;
pub mod execute_result;
pub mod name;
pub mod prepare;
pub mod query_result;
pub mod table_list;
pub mod table_metadata;

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
        let default_timeout = session.default_timeout();
        SqlClient {
            session,
            default_timeout,
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

    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }
}

impl SqlClient {
    pub async fn list_tables(&self) -> Result<TableList, TgError> {
        let timeout = self.default_timeout;
        self.list_tables_for(timeout).await
    }

    pub async fn list_tables_for(&self, timeout: Duration) -> Result<TableList, TgError> {
        const FUNCTION_NAME: &str = "list_tables()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::list_tables_command();
        let response = self.send_and_pull_response(command, timeout).await?;
        let table_list = list_tables_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(table_list)
    }

    pub async fn list_tables_async(&self) -> Result<Job<TableList>, TgError> {
        const FUNCTION_NAME: &str = "list_tables_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::list_tables_command();
        let job = self
            .send_and_pull_async("ListTables", command, Box::new(list_tables_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn list_tables_command() -> SqlRequestCommand {
        let request = crate::jogasaki::proto::sql::request::ListTables {};
        SqlRequestCommand::ListTables(request)
    }

    pub async fn get_table_metadata(&self, table_name: &str) -> Result<TableMetadata, TgError> {
        let timeout = self.default_timeout;
        self.get_table_metadata_for(table_name, timeout).await
    }

    pub async fn get_table_metadata_for(
        &self,
        table_name: &str,
        timeout: Duration,
    ) -> Result<TableMetadata, TgError> {
        const FUNCTION_NAME: &str = "get_table_metadata()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::table_metadata_command(table_name);
        let response = self.send_and_pull_response(command, timeout).await?;
        let metadata = table_metadata_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(metadata)
    }

    pub async fn get_table_metadata_async(
        &self,
        table_name: &str,
    ) -> Result<Job<TableMetadata>, TgError> {
        const FUNCTION_NAME: &str = "get_table_metadata_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::table_metadata_command(table_name);
        let job = self
            .send_and_pull_async("TableMetadata", command, Box::new(table_metadata_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn table_metadata_command(table_name: &str) -> SqlRequestCommand {
        let request = crate::jogasaki::proto::sql::request::DescribeTable {
            name: table_name.to_string(),
        };
        SqlRequestCommand::DescribeTable(request)
    }

    pub async fn prepare(
        &self,
        sql: &str,
        placeholders: Vec<SqlPlaceholder>,
    ) -> Result<SqlPreparedStatement, TgError> {
        let timeout = self.default_timeout;
        self.prepare_for(sql, placeholders, timeout).await
    }

    pub async fn prepare_for(
        &self,
        sql: &str,
        placeholders: Vec<SqlPlaceholder>,
        timeout: Duration,
    ) -> Result<SqlPreparedStatement, TgError> {
        const FUNCTION_NAME: &str = "prepare()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::prepare_command(sql, placeholders);
        let response = self.send_and_pull_response(command, timeout).await?;

        let session = self.session.clone();
        let close_timeout = self.default_timeout;
        let ps = prepare_processor(session, response, close_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(ps)
    }

    pub async fn prepare_async(
        &self,
        sql: &str,
        placeholders: Vec<SqlPlaceholder>,
    ) -> Result<Job<SqlPreparedStatement>, TgError> {
        const FUNCTION_NAME: &str = "prepare_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::prepare_command(sql, placeholders);
        let session = self.session.clone();
        let close_timeout = self.default_timeout;
        let job = self
            .send_and_pull_async(
                "Prepare",
                command,
                Box::new(move |response| {
                    prepare_processor(session.clone(), response, close_timeout)
                }),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn prepare_command(sql: &str, placeholders: Vec<SqlPlaceholder>) -> SqlRequestCommand {
        let request = crate::jogasaki::proto::sql::request::Prepare {
            sql: sql.to_string(),
            placeholders,
        };
        SqlRequestCommand::Prepare(request)
    }

    pub(crate) async fn dispose_prepare(
        &self,
        prepare_handle: u64,
        has_result_records: bool,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "dispose_prepare()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::dispose_prepare_statement_command(prepare_handle, has_result_records);
        let response = self.send_and_pull_response(command, timeout).await?;
        let _ = prepare_dispose_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    fn dispose_prepare_statement_command(
        prepare_handle: u64,
        has_result_records: bool,
    ) -> SqlRequestCommand {
        let ps = crate::jogasaki::proto::sql::common::PreparedStatement {
            handle: prepare_handle,
            has_result_records,
        };

        let request = crate::jogasaki::proto::sql::request::DisposePreparedStatement {
            prepared_statement_handle: Some(ps),
        };
        SqlRequestCommand::DisposePreparedStatement(request)
    }

    pub async fn start_transaction(
        &self,
        transaction_option: &TransactionOption,
    ) -> Result<Transaction, TgError> {
        let timeout = self.default_timeout;
        self.start_transaction_for(transaction_option, timeout)
            .await
    }

    pub async fn start_transaction_for(
        &self,
        transaction_option: &TransactionOption,
        timeout: Duration,
    ) -> Result<Transaction, TgError> {
        const FUNCTION_NAME: &str = "start_transaction()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::begin_transaction_command(transaction_option);
        let response = self.send_and_pull_response(command, timeout).await?;

        let session = self.session.clone();
        let close_timeout = transaction_option
            .close_timeout()
            .unwrap_or(self.default_timeout);
        let transaction = transaction_begin_processor(session, response, close_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(transaction)
    }

    pub async fn start_transaction_async(
        &self,
        transaction_option: &TransactionOption,
    ) -> Result<Job<Transaction>, TgError> {
        const FUNCTION_NAME: &str = "start_transaction_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::begin_transaction_command(transaction_option);
        let session = self.session.clone();
        let close_timeout = transaction_option
            .close_timeout()
            .unwrap_or(self.default_timeout);
        let job = self
            .send_and_pull_async(
                "StartTransaction",
                command,
                Box::new(move |response| {
                    transaction_begin_processor(session.clone(), response, close_timeout)
                }),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn begin_transaction_command(transaction_option: &TransactionOption) -> SqlRequestCommand {
        let tx_option = transaction_option.request();

        let request = crate::jogasaki::proto::sql::request::Begin {
            option: Some(tx_option),
        };
        SqlRequestCommand::Begin(request)
    }

    pub async fn execute(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<SqlExecuteResult, TgError> {
        let timeout = self.default_timeout;
        self.execute_for(transaction, sql, timeout).await
    }

    pub async fn execute_for(
        &self,
        transaction: &Transaction,
        sql: &str,
        timeout: Duration,
    ) -> Result<SqlExecuteResult, TgError> {
        const FUNCTION_NAME: &str = "execute()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::execute_statement_command(tx_handle, sql);
        let response = self.send_and_pull_response(command, timeout).await?;
        let execute_result = execute_result_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(execute_result)
    }

    pub async fn execute_async(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<Job<SqlExecuteResult>, TgError> {
        const FUNCTION_NAME: &str = "execute_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::execute_statement_command(tx_handle, sql);
        let job = self
            .send_and_pull_async("Execute", command, Box::new(execute_result_processor))
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

    pub async fn prepared_execute(
        &self,
        transaction: &Transaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> Result<SqlExecuteResult, TgError> {
        let timeout = self.default_timeout;
        self.prepared_execute_for(transaction, prepared_statement, parameters, timeout)
            .await
    }

    pub async fn prepared_execute_for(
        &self,
        transaction: &Transaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
        timeout: Duration,
    ) -> Result<SqlExecuteResult, TgError> {
        const FUNCTION_NAME: &str = "prepared_execute()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command =
            Self::execute_prepared_statement_command(tx_handle, prepared_statement, parameters);
        let response = self.send_and_pull_response(command, timeout).await?;
        let execute_result = execute_result_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(execute_result)
    }

    pub async fn prepared_execute_async(
        &self,
        transaction: &Transaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> Result<Job<SqlExecuteResult>, TgError> {
        const FUNCTION_NAME: &str = "execute_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command =
            Self::execute_prepared_statement_command(tx_handle, prepared_statement, parameters);
        let job = self
            .send_and_pull_async(
                "PreparedExecute",
                command,
                Box::new(execute_result_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_prepared_statement_command(
        transaction_handle: u64,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> SqlRequestCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let ps_handle = crate::jogasaki::proto::sql::common::PreparedStatement {
            handle: prepared_statement.prepare_handle(),
            has_result_records: prepared_statement.has_result_records(),
        };
        let request = crate::jogasaki::proto::sql::request::ExecutePreparedStatement {
            transaction_handle: Some(tx_handle),
            prepared_statement_handle: Some(ps_handle),
            parameters,
        };
        SqlRequestCommand::ExecutePreparedStatement(request)
    }

    pub async fn query(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<SqlQueryResult, TgError> {
        let timeout = self.default_timeout;
        self.query_for(transaction, sql, timeout).await
    }

    pub async fn query_for(
        &self,
        transaction: &Transaction,
        sql: &str,
        timeout: Duration,
    ) -> Result<SqlQueryResult, TgError> {
        const FUNCTION_NAME: &str = "query()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::execute_query_command(tx_handle, sql);
        let response = self.send_and_pull_response(command, timeout).await?;

        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let query_result = query_result_processor(wire, response, default_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(query_result)
    }

    pub async fn query_async(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<Job<SqlQueryResult>, TgError> {
        const FUNCTION_NAME: &str = "query_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::execute_query_command(tx_handle, sql);
        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let job = self
            .send_and_pull_async(
                "Query",
                command,
                Box::new(move |response| {
                    query_result_processor(wire.clone(), response, default_timeout)
                }),
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

    pub async fn prepared_query(
        &self,
        transaction: &Transaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> Result<SqlQueryResult, TgError> {
        let timeout = self.default_timeout;
        self.prepared_query_for(transaction, prepared_statement, parameters, timeout)
            .await
    }

    pub async fn prepared_query_for(
        &self,
        transaction: &Transaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
        timeout: Duration,
    ) -> Result<SqlQueryResult, TgError> {
        const FUNCTION_NAME: &str = "prepared_query()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command =
            Self::execute_prepared_query_command(tx_handle, prepared_statement, parameters);
        let response = self.send_and_pull_response(command, timeout).await?;

        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let query_result = query_result_processor(wire, response, default_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(query_result)
    }

    pub async fn prepared_query_async(
        &self,
        transaction: &Transaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> Result<Job<SqlQueryResult>, TgError> {
        const FUNCTION_NAME: &str = "prepared_query_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command =
            Self::execute_prepared_query_command(tx_handle, prepared_statement, parameters);
        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let job = self
            .send_and_pull_async(
                "PreparedQuery",
                command,
                Box::new(move |response| {
                    query_result_processor(wire.clone(), response, default_timeout)
                }),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_prepared_query_command(
        transaction_handle: u64,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> SqlRequestCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let ps_handle = crate::jogasaki::proto::sql::common::PreparedStatement {
            handle: prepared_statement.prepare_handle(),
            has_result_records: prepared_statement.has_result_records(),
        };
        let request = crate::jogasaki::proto::sql::request::ExecutePreparedQuery {
            transaction_handle: Some(tx_handle),
            prepared_statement_handle: Some(ps_handle),
            parameters,
        };
        SqlRequestCommand::ExecutePreparedQuery(request)
    }

    pub async fn commit(
        &self,
        transaction: &Transaction,
        commit_option: &CommitOption,
    ) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.commit_for(transaction, commit_option, timeout).await
    }

    pub async fn commit_for(
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
        const FUNCTION_NAME: &str = "commit_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::commit_command(tx_handle, commit_option);
        let job = self
            .send_and_pull_async("Commit", command, Box::new(transaction_commit_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn commit_command(transaction_handle: u64, commit_option: &CommitOption) -> SqlRequestCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };

        let request = crate::jogasaki::proto::sql::request::Commit {
            transaction_handle: Some(tx_handle),
            notification_type: commit_option.commit_type().into(),
            auto_dispose: commit_option.auto_dispose(),
        };
        SqlRequestCommand::Commit(request)
    }

    pub async fn rollback(&self, transaction: &Transaction) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.rollback_for(transaction, timeout).await
    }

    pub async fn rollback_for(
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
        const FUNCTION_NAME: &str = "rollback_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::rollback_command(tx_handle);
        let job = self
            .send_and_pull_async(
                "Rollback",
                command,
                Box::new(transaction_rollback_processor),
            )
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
        job_name: &str,
        command: SqlRequestCommand,
        converter: Box<dyn Fn(WireResponse) -> Result<T, TgError> + Send>,
    ) -> Result<Job<T>, TgError> {
        let request = Self::new_request(command);
        self.wire()
            .send_and_pull_async(
                job_name,
                SERVICE_ID_SQL,
                request,
                converter,
                self.default_timeout,
            )
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

pub(crate) fn convert_sql_response(
    function_name: &str,
    response: &WireResponse,
) -> Result<Option<SqlResponse>, TgError> {
    match response {
        WireResponse::ResponseSessionPayload(_slot, payload, error) => {
            if let Some(e) = error {
                return Err(e.to_tg_error());
            }
            // let payload = payload.as_deref().unwrap();
            let payload = &payload.as_ref().unwrap()[..];
            let sql_response = SqlResponse::decode_length_delimited(payload)
                .map_err(|e| prost_decode_error!(function_name, "SqlResponse", e))?;
            match &sql_response.response {
                Some(SqlResponseType::ResultOnly(result_only)) => match &result_only.result {
                    Some(crate::jogasaki::proto::sql::response::result_only::Result::Success(
                        _,
                    )) => Ok(Some(sql_response)),
                    Some(crate::jogasaki::proto::sql::response::result_only::Result::Error(
                        error,
                    )) => {
                        let error = error.clone();
                        Err(sql_service_error!(function_name, error))
                    }
                    _ => Ok(Some(sql_response)),
                },
                _ => Ok(Some(sql_response)),
            }
        }
        _ => Ok(None),
    }
}

pub(crate) fn sql_result_only_success_processor(
    function_name: &str,
    response: WireResponse,
) -> Result<(), TgError> {
    let sql_response = convert_sql_response(function_name, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        function_name,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;
    match message.response {
        Some(SqlResponseType::ResultOnly(result_only)) => match result_only.result {
            Some(crate::jogasaki::proto::sql::response::result_only::Result::Success(_)) => Ok(()),
            _ => Err(invalid_response_error!(
                function_name,
                format!("fail. {:?}", result_only),
            )),
        },
        _ => Err(invalid_response_error!(
            function_name,
            format!("response {:?} is not ResultOnly", message.response),
        )),
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
