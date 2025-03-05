use std::{collections::HashMap, sync::Arc, time::Duration};

use log::trace;

use crate::{
    error::TgError,
    invalid_response_error,
    job::Job,
    jogasaki::proto::sql::{
        request::{request::Request as SqlCommand, Request as SqlRequest},
        response::{response::Response as SqlResponseType, Response as SqlResponse},
    },
    prelude::{
        convert_lob_parameters, execute_result_processor,
        explain::explain_processor,
        list_tables_processor, lob_copy_to_processor, lob_open_processor,
        prepare_dispose_processor, prepare_processor, query_result_processor,
        status::{transaction_status_processor, TransactionStatus},
        table_metadata_processor, CommitOption, ServiceClient, SqlExecuteResult, SqlParameter,
        SqlPlaceholder, SqlQueryResult, TableList, TableMetadata, TgBlobReference,
        TgLargeObjectReference,
    },
    prost_decode_error,
    session::{
        wire::{response::WireResponse, response_box::SlotEntryHandle, Wire},
        Session,
    },
    sql_service_error,
    tateyama::proto::framework::common::BlobInfo,
    transaction::{
        option::TransactionOption, transaction_begin_processor, transaction_commit_processor,
        transaction_dispose_processor, transaction_rollback_processor, Transaction,
    },
};

use prost::{alloc::string::String as ProstString, Message};

use super::{explain::SqlExplainResult, prepare::SqlPreparedStatement};

/// The symbolic ID of the destination service.
const SERVICE_SYMBOLIC_ID: &str = "sql";

/// The major service message version which this client requests.
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 1;

/// The minor service message version which this client requests.
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 4;

pub(crate) const SERVICE_ID_SQL: i32 = 3;

/// A SQL service client.
///
/// # Examples
/// ```
/// use std::sync::Arc;
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example(session: &Arc<Session>) -> Result<(), TgError> {
///     let client: SqlClient = session.make_client();
///
///     // In Tsurugi, DDL is also executed in a transaction.
///     // (DDL and DML must not be executed in the same transaction)
///     let transaction = client.start_transaction(&TransactionOption::default()).await?;
///     let result = {
///         let sql = "
///           create table customer (
///             c_id bigint primary key,
///             c_name varchar(30) not null,
///             c_age int
///           )
///         ";
///         let result = client.execute(&transaction, sql).await;
///         match result {
///            Ok(_) => client.commit(&transaction, &CommitOption::default()).await,
///            Err(e) => Err(e)
///         }
///     };
///     transaction.close().await?;
///     result?;
///
///     let table_list = client.list_tables().await?;
///     let table_metadata = client.get_table_metadata("customer").await?;
///
///     Ok(())
/// }
/// ```
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
    /// Get service message version.
    pub fn service_message_version() -> String {
        format!(
            "{}-{}.{}",
            SERVICE_SYMBOLIC_ID, SERVICE_MESSAGE_VERSION_MAJOR, SERVICE_MESSAGE_VERSION_MINOR
        )
    }

    /// Set default timeout.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Get default timeout.
    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }
}

impl SqlClient {
    /// Returns the list of available table names in the database, except system tables.
    ///
    /// The table names are each fully qualified (maybe with a schema name).
    /// To retrieve more details for the individual tables, you can use [Self::get_table_metadata].
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient) -> Result<(), TgError> {
    ///     let table_list = client.list_tables().await?;
    ///
    ///     let table_names = table_list.table_names();
    ///     for table_name in table_names {
    ///         println!("{}", table_name);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_tables(&self) -> Result<TableList, TgError> {
        let timeout = self.default_timeout;
        self.list_tables_for(timeout).await
    }

    /// Returns the list of available table names in the database, except system tables.
    ///
    /// The table names are each fully qualified (maybe with a schema name).
    /// To retrieve more details for the individual tables, you can use [Self::get_table_metadata_for].
    pub async fn list_tables_for(&self, timeout: Duration) -> Result<TableList, TgError> {
        const FUNCTION_NAME: &str = "list_tables()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::list_tables_command();
        let response = self.send_and_pull_response(command, None, timeout).await?;
        let table_list = list_tables_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(table_list)
    }

    /// Returns the list of available table names in the database, except system tables.
    ///
    /// The table names are each fully qualified (maybe with a schema name).
    /// To retrieve more details for the individual tables, you can use [Self::get_table_metadata_async].
    pub async fn list_tables_async(&self) -> Result<Job<TableList>, TgError> {
        const FUNCTION_NAME: &str = "list_tables_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::list_tables_command();
        let job = self
            .send_and_pull_async("ListTables", command, None, Box::new(list_tables_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn list_tables_command() -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::ListTables {};
        SqlCommand::ListTables(request)
    }

    /// Retrieves metadata for a table.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient) -> Result<(), TgError> {
    ///     let table_metadata = client.get_table_metadata("customer").await?;
    ///     println!("table name={}", table_metadata.table_name());
    ///
    ///     let columns = table_metadata.columns();
    ///     for column in columns {
    ///         println!("column name={}", column.name());
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_table_metadata(&self, table_name: &str) -> Result<TableMetadata, TgError> {
        let timeout = self.default_timeout;
        self.get_table_metadata_for(table_name, timeout).await
    }

    /// Retrieves metadata for a table.
    pub async fn get_table_metadata_for(
        &self,
        table_name: &str,
        timeout: Duration,
    ) -> Result<TableMetadata, TgError> {
        const FUNCTION_NAME: &str = "get_table_metadata()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::table_metadata_command(table_name);
        let response = self.send_and_pull_response(command, None, timeout).await?;
        let metadata = table_metadata_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(metadata)
    }

    /// Retrieves metadata for a table.
    pub async fn get_table_metadata_async(
        &self,
        table_name: &str,
    ) -> Result<Job<TableMetadata>, TgError> {
        const FUNCTION_NAME: &str = "get_table_metadata_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::table_metadata_command(table_name);
        let job = self
            .send_and_pull_async(
                "TableMetadata",
                command,
                None,
                Box::new(table_metadata_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn table_metadata_command(table_name: &str) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::DescribeTable {
            name: table_name.to_string(),
        };
        SqlCommand::DescribeTable(request)
    }

    /// Prepares a SQL statement.
    ///
    /// Note: Should invoke [`SqlPreparedStatement::close`] before [`SqlPreparedStatement::drop`] to dispose the prepared statement.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient) -> Result<(), TgError> {
    ///     let sql = "insert into customer values(:id, :name, :age)";
    ///     let placeholders = vec![
    ///         SqlPlaceholder::of::<i64>("id"),
    ///         SqlPlaceholder::of::<String>("name"),
    ///         SqlPlaceholder::of::<i32>("age"),
    ///     ];
    ///     let prepared_statement = client.prepare(sql, placeholders).await?;
    ///
    ///     prepared_statement.close().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn prepare(
        &self,
        sql: &str,
        placeholders: Vec<SqlPlaceholder>,
    ) -> Result<SqlPreparedStatement, TgError> {
        let timeout = self.default_timeout;
        self.prepare_for(sql, placeholders, timeout).await
    }

    /// Prepares a SQL statement.
    ///
    /// Note: Should invoke [`SqlPreparedStatement::close`] before [`SqlPreparedStatement::drop`] to dispose the prepared statement.
    pub async fn prepare_for(
        &self,
        sql: &str,
        placeholders: Vec<SqlPlaceholder>,
        timeout: Duration,
    ) -> Result<SqlPreparedStatement, TgError> {
        const FUNCTION_NAME: &str = "prepare()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::prepare_command(sql, placeholders);
        let response = self.send_and_pull_response(command, None, timeout).await?;

        let session = self.session.clone();
        let close_timeout = self.default_timeout;
        let ps = prepare_processor(session, response, close_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(ps)
    }

    /// Prepares a SQL statement.
    ///
    /// Note: Should invoke [`SqlPreparedStatement::close`] before [`SqlPreparedStatement::drop`] to dispose the prepared statement.
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
                None,
                Box::new(move |response| {
                    prepare_processor(session.clone(), response, close_timeout)
                }),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn prepare_command(sql: &str, placeholders: Vec<SqlPlaceholder>) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::Prepare {
            sql: sql.to_string(),
            placeholders,
        };
        SqlCommand::Prepare(request)
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
        let response = self.send_and_pull_response(command, None, timeout).await?;
        prepare_dispose_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    pub(crate) async fn dispose_prepare_send_only(
        &self,
        prepare_handle: u64,
        has_result_records: bool,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "dispose_prepare_send_only()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::dispose_prepare_statement_command(prepare_handle, has_result_records);
        let _ = self.send_only(command).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    fn dispose_prepare_statement_command(
        prepare_handle: u64,
        has_result_records: bool,
    ) -> SqlCommand {
        let ps = crate::jogasaki::proto::sql::common::PreparedStatement {
            handle: prepare_handle,
            has_result_records,
        };

        let request = crate::jogasaki::proto::sql::request::DisposePreparedStatement {
            prepared_statement_handle: Some(ps),
        };
        SqlCommand::DisposePreparedStatement(request)
    }

    /// Retrieves execution plan of the statement.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient) -> Result<(), TgError> {
    ///     let sql = "select * from customer oder by c_id";
    ///     let explain_result = client.explain(sql).await?;
    ///     println!("json={}", explain_result.contents());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn explain(&self, sql: &str) -> Result<SqlExplainResult, TgError> {
        let timeout = self.default_timeout;
        self.explain_for(sql, timeout).await
    }

    /// Retrieves execution plan of the statement.
    pub async fn explain_for(
        &self,
        sql: &str,
        timeout: Duration,
    ) -> Result<SqlExplainResult, TgError> {
        const FUNCTION_NAME: &str = "explain()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::explain_text_command(sql);
        let response = self.send_and_pull_response(command, None, timeout).await?;

        let explain_result = explain_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(explain_result)
    }

    /// Retrieves execution plan of the statement.
    pub async fn explain_async(&self, sql: &str) -> Result<Job<SqlExplainResult>, TgError> {
        const FUNCTION_NAME: &str = "explain_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::explain_text_command(sql);
        let job = self
            .send_and_pull_async("Explain", command, None, Box::new(explain_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn explain_text_command(sql: &str) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::ExplainByText {
            sql: sql.to_string(),
        };
        SqlCommand::ExplainByText(request)
    }

    /// Retrieves execution plan of the statement.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, prepared_statement: &SqlPreparedStatement) -> Result<(), TgError> {
    ///     // prepared_statement: "select * from customer where c_id = :id"
    ///     let parameters = vec![SqlParameter::of("id", 3_i64)];
    ///     let explain_result = client.prepared_explain(prepared_statement, parameters).await?;
    ///     println!("json={}", explain_result.contents());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn prepared_explain(
        &self,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> Result<SqlExplainResult, TgError> {
        let timeout = self.default_timeout;
        self.prepared_explain_for(prepared_statement, parameters, timeout)
            .await
    }

    /// Retrieves execution plan of the statement.
    pub async fn prepared_explain_for(
        &self,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
        timeout: Duration,
    ) -> Result<SqlExplainResult, TgError> {
        const FUNCTION_NAME: &str = "prepared_explain()";
        trace!("{} start", FUNCTION_NAME);

        let (parameters, lobs) = convert_lob_parameters(parameters);
        let command = Self::explain_prepared_command(prepared_statement, parameters);
        let response = self.send_and_pull_response(command, lobs, timeout).await?;
        let explain_result = explain_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(explain_result)
    }

    /// Retrieves execution plan of the statement.
    pub async fn prepared_explain_async(
        &self,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> Result<Job<SqlExplainResult>, TgError> {
        const FUNCTION_NAME: &str = "prepared_explain_async()";
        trace!("{} start", FUNCTION_NAME);

        let (parameters, lobs) = convert_lob_parameters(parameters);
        let command = Self::explain_prepared_command(prepared_statement, parameters);
        let job = self
            .send_and_pull_async("Explain", command, lobs, Box::new(explain_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn explain_prepared_command(
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> SqlCommand {
        let ps_handle = crate::jogasaki::proto::sql::common::PreparedStatement {
            handle: prepared_statement.prepare_handle(),
            has_result_records: prepared_statement.has_result_records(),
        };
        let request = crate::jogasaki::proto::sql::request::Explain {
            prepared_statement_handle: Some(ps_handle),
            parameters,
        };
        SqlCommand::Explain(request)
    }

    /// Starts a new transaction.
    ///
    /// Note: Should invoke [`Transaction::close`] before [`Transaction::drop`] to dispose the transaction.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction_option: &TransactionOption) -> Result<(), TgError> {
    ///     let transaction = client.start_transaction(transaction_option).await?;
    ///
    ///     let result = client.commit(&transaction, &CommitOption::default()).await;
    ///
    ///     transaction.close().await?;
    ///
    ///     result
    /// }
    /// ```
    pub async fn start_transaction(
        &self,
        transaction_option: &TransactionOption,
    ) -> Result<Transaction, TgError> {
        let timeout = self.default_timeout;
        self.start_transaction_for(transaction_option, timeout)
            .await
    }

    /// Starts a new transaction.
    ///
    /// Note: Should invoke [`Transaction::close`] before [`Transaction::drop`] to dispose the transaction.
    pub async fn start_transaction_for(
        &self,
        transaction_option: &TransactionOption,
        timeout: Duration,
    ) -> Result<Transaction, TgError> {
        const FUNCTION_NAME: &str = "start_transaction()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::begin_transaction_command(transaction_option);
        let response = self.send_and_pull_response(command, None, timeout).await?;

        let session = self.session.clone();
        let close_timeout = transaction_option
            .close_timeout()
            .unwrap_or(self.default_timeout);
        let transaction = transaction_begin_processor(session, response, close_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(transaction)
    }

    /// Starts a new transaction.
    ///
    /// Note: Should invoke [`Transaction::close`] before [`Transaction::drop`] to dispose the transaction.
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
                None,
                Box::new(move |response| {
                    transaction_begin_processor(session.clone(), response, close_timeout)
                }),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn begin_transaction_command(transaction_option: &TransactionOption) -> SqlCommand {
        let tx_option = transaction_option.request();

        let request = crate::jogasaki::proto::sql::request::Begin {
            option: Some(tx_option),
        };
        SqlCommand::Begin(request)
    }

    /// Returns occurred error in the target transaction.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    ///     let status = client.get_transaction_status(transaction).await?;
    ///     println!("is_error={}", status.is_error());
    ///
    ///     if let Some(code) = status.diagnostic_code() {
    ///         println!("diagnostic_code={}", code);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_transaction_status(
        &self,
        transaction: &Transaction,
    ) -> Result<TransactionStatus, TgError> {
        let timeout = self.default_timeout;
        self.get_transaction_status_for(transaction, timeout).await
    }

    /// Returns occurred error in the target transaction.
    pub async fn get_transaction_status_for(
        &self,
        transaction: &Transaction,
        timeout: Duration,
    ) -> Result<TransactionStatus, TgError> {
        const FUNCTION_NAME: &str = "get_transaction_status()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::transaction_status_command(transaction.transaction_handle()?);
        let response = self.send_and_pull_response(command, None, timeout).await?;
        let status = transaction_status_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(status)
    }

    /// Returns occurred error in the target transaction.
    pub async fn get_transaction_status_async(
        &self,
        transaction: &Transaction,
    ) -> Result<Job<TransactionStatus>, TgError> {
        const FUNCTION_NAME: &str = "get_transaction_status_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::transaction_status_command(transaction.transaction_handle()?);
        let job = self
            .send_and_pull_async(
                "TransactionStatus",
                command,
                None,
                Box::new(transaction_status_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn transaction_status_command(transaction_handle: u64) -> SqlCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let request = crate::jogasaki::proto::sql::request::GetErrorInfo {
            transaction_handle: Some(tx_handle),
        };
        SqlCommand::GetErrorInfo(request)
    }

    /// Executes a SQL statement.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    ///     let sql = "insert into customer values(4, 'example', 20)";
    ///     let execute_result = client.execute(&transaction, sql).await?;
    ///     println!("inserted rows={}", execute_result.inserted_rows());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<SqlExecuteResult, TgError> {
        let timeout = self.default_timeout;
        self.execute_for(transaction, sql, timeout).await
    }

    /// Executes a SQL statement.
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
        let response = self.send_and_pull_response(command, None, timeout).await?;
        let execute_result = execute_result_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(execute_result)
    }

    /// Executes a SQL statement.
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
            .send_and_pull_async("Execute", command, None, Box::new(execute_result_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_statement_command(transaction_handle: u64, sql: &str) -> SqlCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let request = crate::jogasaki::proto::sql::request::ExecuteStatement {
            transaction_handle: Some(tx_handle),
            sql: ProstString::from(sql),
        };
        SqlCommand::ExecuteStatement(request)
    }

    /// Executes a SQL statement.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, prepared_statement: &SqlPreparedStatement) -> Result<(), TgError> {
    ///     // prepared_statement: "insert into customer values(:id, :name, :age)"
    ///     let parameters = vec![
    ///         SqlParameter::of("id", 4_i64),
    ///         SqlParameter::of("name", "example"),
    ///         SqlParameter::of("age", 20),
    ///     ];
    ///     let execute_result = client.prepared_execute(&transaction, prepared_statement, parameters).await?;
    ///     println!("inserted rows={}", execute_result.inserted_rows());
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Executes a SQL statement.
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
        let (parameters, lobs) = convert_lob_parameters(parameters);

        let command =
            Self::execute_prepared_statement_command(tx_handle, prepared_statement, parameters);
        let response = self.send_and_pull_response(command, lobs, timeout).await?;
        let execute_result = execute_result_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(execute_result)
    }

    /// Executes a SQL statement.
    pub async fn prepared_execute_async(
        &self,
        transaction: &Transaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> Result<Job<SqlExecuteResult>, TgError> {
        const FUNCTION_NAME: &str = "prepared_execute_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;
        let (parameters, lobs) = convert_lob_parameters(parameters);

        let command =
            Self::execute_prepared_statement_command(tx_handle, prepared_statement, parameters);
        let job = self
            .send_and_pull_async("Execute", command, lobs, Box::new(execute_result_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_prepared_statement_command(
        transaction_handle: u64,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> SqlCommand {
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
        SqlCommand::ExecutePreparedStatement(request)
    }

    /// Executes a SQL statement and retrieve its result.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    ///     let sql = "select c_id, c_name, c_age from customer order by c_id";
    ///     let mut query_result = client.query(&transaction, sql).await?;
    ///
    ///     while query_result.next_row().await? {
    ///         if query_result.next_column().await? {
    ///             let id: i64 = query_result.fetch().await?;
    ///         }
    ///         if query_result.next_column().await? {
    ///             let name: String = query_result.fetch().await?;
    ///         }
    ///         if query_result.next_column().await? {
    ///             let age: i32 = query_result.fetch().await?;
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query(
        &self,
        transaction: &Transaction,
        sql: &str,
    ) -> Result<SqlQueryResult, TgError> {
        let timeout = self.default_timeout;
        self.query_for(transaction, sql, timeout).await
    }

    /// Executes a SQL statement and retrieve its result.
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
        let response = self.send_and_pull_response(command, None, timeout).await?;

        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let query_result = query_result_processor(wire, response, default_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(query_result)
    }

    /// Executes a SQL statement and retrieve its result.
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
                None,
                Box::new(move |response| {
                    query_result_processor(wire.clone(), response, default_timeout)
                }),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_query_command(transaction_handle: u64, sql: &str) -> SqlCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let request = crate::jogasaki::proto::sql::request::ExecuteQuery {
            transaction_handle: Some(tx_handle),
            sql: ProstString::from(sql),
        };
        SqlCommand::ExecuteQuery(request)
    }

    /// Executes a SQL statement and retrieve its result.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, prepared_statement: &SqlPreparedStatement) -> Result<(), TgError> {
    ///     // prepared_statement: "select c_id, c_name, c_age from customer where c_id = :id"
    ///     let parameters = vec![SqlParameter::of("id", 3_i64)];
    ///     let mut query_result = client.prepared_query(&transaction, prepared_statement, parameters).await?;
    ///
    ///     while query_result.next_row().await? {
    ///         if query_result.next_column().await? {
    ///             let id: i64 = query_result.fetch().await?;
    ///         }
    ///         if query_result.next_column().await? {
    ///             let name: String = query_result.fetch().await?;
    ///         }
    ///         if query_result.next_column().await? {
    ///             let age: i32 = query_result.fetch().await?;
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Executes a SQL statement and retrieve its result.
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
        let (parameters, lobs) = convert_lob_parameters(parameters);

        let command =
            Self::execute_prepared_query_command(tx_handle, prepared_statement, parameters);
        let response = self.send_and_pull_response(command, lobs, timeout).await?;

        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let query_result = query_result_processor(wire, response, default_timeout)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(query_result)
    }

    /// Executes a SQL statement and retrieve its result.
    pub async fn prepared_query_async(
        &self,
        transaction: &Transaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> Result<Job<SqlQueryResult>, TgError> {
        const FUNCTION_NAME: &str = "prepared_query_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;
        let (parameters, lobs) = convert_lob_parameters(parameters);

        let command =
            Self::execute_prepared_query_command(tx_handle, prepared_statement, parameters);
        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let job = self
            .send_and_pull_async(
                "Query",
                command,
                lobs,
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
    ) -> SqlCommand {
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
        SqlCommand::ExecutePreparedQuery(request)
    }

    /// Open BLOB file.
    ///
    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, query_result: &mut SqlQueryResult) -> Result<Vec<u8>, TgError> {
    ///     let blob: TgBlobReference = query_result.fetch().await?;
    ///     let mut file = client.open_blob(transaction, &blob).await?;
    ///
    ///     let mut buffer = Vec::new();
    ///     file.read_to_end(&mut buffer).unwrap();
    ///
    ///     Ok(buffer)
    /// }
    /// ```
    pub async fn open_blob(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
    ) -> Result<std::fs::File, TgError> {
        let timeout = self.default_timeout;
        self.open_blob_for(transaction, blob, timeout).await
    }

    /// Open BLOB file.
    pub async fn open_blob_for(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        timeout: Duration,
    ) -> Result<std::fs::File, TgError> {
        const FUNCTION_NAME: &str = "open_blob()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::open_lob_command(tx_handle, blob);
        let response = self.send_and_pull_response(command, None, timeout).await?;
        let file = lob_open_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(file)
    }

    /// Open BLOB file.
    pub async fn open_blob_async(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
    ) -> Result<Job<std::fs::File>, TgError> {
        const FUNCTION_NAME: &str = "open_blob_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::open_lob_command(tx_handle, blob);
        let job = self
            .send_and_pull_async("File", command, None, Box::new(lob_open_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn open_lob_command<T: TgLargeObjectReference>(transaction_handle: u64, lob: &T) -> SqlCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let lob = crate::jogasaki::proto::sql::common::LargeObjectReference {
            provider: lob.provider().into(),
            object_id: lob.object_id(),
            contents_opt: None,
        };

        let request = crate::jogasaki::proto::sql::request::GetLargeObjectData {
            transaction_handle: Some(tx_handle),
            reference: Some(lob),
        };
        SqlCommand::GetLargeObjectData(request)
    }

    /// Copy BLOB to local file.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, query_result: &mut SqlQueryResult) -> Result<(), TgError> {
    ///     let blob: TgBlobReference = query_result.fetch().await?;
    ///     client.copy_blob_to(transaction, &blob, "/path/to/blob.bin").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn copy_blob_to(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        destination: &str,
    ) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.copy_blob_to_for(transaction, blob, destination, timeout)
            .await
    }

    /// Copy BLOB to local file.
    pub async fn copy_blob_to_for(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        destination: &str,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "copy_blob_to()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::copy_blob_to_command(tx_handle, blob);
        let response = self.send_and_pull_response(command, None, timeout).await?;
        lob_copy_to_processor(response, destination)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    /// Copy BLOB to local file.
    pub async fn copy_blob_to_async(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        destination: &'static str,
    ) -> Result<Job<()>, TgError> {
        const FUNCTION_NAME: &str = "copy_blob_to_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::copy_blob_to_command(tx_handle, blob);
        let job = self
            .send_and_pull_async(
                "BlobCopy",
                command,
                None,
                Box::new(move |response| lob_copy_to_processor(response, destination)),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn copy_blob_to_command(transaction_handle: u64, blob: &TgBlobReference) -> SqlCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };
        let lob = crate::jogasaki::proto::sql::common::LargeObjectReference {
            provider: blob.provider().into(),
            object_id: blob.object_id(),
            contents_opt: None,
        };

        let request = crate::jogasaki::proto::sql::request::GetLargeObjectData {
            transaction_handle: Some(tx_handle),
            reference: Some(lob),
        };
        SqlCommand::GetLargeObjectData(request)
    }

    /// Request commit to the SQL service.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    ///     let commit_option = CommitOption::default();
    ///     client.commit(transaction, &commit_option).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn commit(
        &self,
        transaction: &Transaction,
        commit_option: &CommitOption,
    ) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.commit_for(transaction, commit_option, timeout).await
    }

    /// Request commit to the SQL service.
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
        let response = self.send_and_pull_response(command, None, timeout).await?;
        transaction_commit_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    /// Request commit to the SQL service.
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
            .send_and_pull_async(
                "Commit",
                command,
                None,
                Box::new(transaction_commit_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn commit_command(transaction_handle: u64, commit_option: &CommitOption) -> SqlCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };

        let request = crate::jogasaki::proto::sql::request::Commit {
            transaction_handle: Some(tx_handle),
            notification_type: commit_option.commit_type().into(),
            auto_dispose: commit_option.auto_dispose(),
        };
        SqlCommand::Commit(request)
    }

    /// Request rollback to the SQL service.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    ///     client.rollback(transaction).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn rollback(&self, transaction: &Transaction) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.rollback_for(transaction, timeout).await
    }

    /// Request rollback to the SQL service.
    pub async fn rollback_for(
        &self,
        transaction: &Transaction,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "rollback()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::rollback_command(tx_handle);
        let response = self.send_and_pull_response(command, None, timeout).await?;
        transaction_rollback_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    /// Request rollback to the SQL service.
    pub async fn rollback_async(&self, transaction: &Transaction) -> Result<Job<()>, TgError> {
        const FUNCTION_NAME: &str = "rollback_async()";
        trace!("{} start", FUNCTION_NAME);

        let tx_handle = transaction.transaction_handle()?;

        let command = Self::rollback_command(tx_handle);
        let job = self
            .send_and_pull_async(
                "Rollback",
                command,
                None,
                Box::new(transaction_rollback_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn rollback_command(transaction_handle: u64) -> SqlCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };

        let request = crate::jogasaki::proto::sql::request::Rollback {
            transaction_handle: Some(tx_handle),
        };
        SqlCommand::Rollback(request)
    }

    pub(crate) async fn dispose_transaction(
        &self,
        transaction_handle: u64,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "dispose_transaction()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::dispose_transaction_command(transaction_handle);
        let response = self.send_and_pull_response(command, None, timeout).await?;
        transaction_dispose_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    pub(crate) async fn dispose_transaction_send_only(
        &self,
        transaction_handle: u64,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "dispose_transaction()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::dispose_transaction_command(transaction_handle);
        let _ = self.send_only(command).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    fn dispose_transaction_command(transaction_handle: u64) -> SqlCommand {
        let tx_handle = crate::jogasaki::proto::sql::common::Transaction {
            handle: transaction_handle,
        };

        let request = crate::jogasaki::proto::sql::request::DisposeTransaction {
            transaction_handle: Some(tx_handle),
        };
        SqlCommand::DisposeTransaction(request)
    }
}

impl SqlClient {
    fn wire(&self) -> Arc<Wire> {
        self.session.wire()
    }

    async fn send_only(&self, command: SqlCommand) -> Result<Arc<SlotEntryHandle>, TgError> {
        let request = Self::new_request(command);
        self.wire().send_only(SERVICE_ID_SQL, request, None).await
    }

    async fn send_and_pull_response(
        &self,
        command: SqlCommand,
        lobs: Option<Vec<BlobInfo>>,
        timeout: Duration,
    ) -> Result<WireResponse, TgError> {
        let request = Self::new_request(command);
        self.wire()
            .send_and_pull_response(SERVICE_ID_SQL, request, lobs, timeout)
            .await
    }

    async fn send_and_pull_async<T: 'static>(
        &self,
        job_name: &str,
        command: SqlCommand,
        lobs: Option<Vec<BlobInfo>>,
        converter: Box<dyn Fn(WireResponse) -> Result<T, TgError> + Send>,
    ) -> Result<Job<T>, TgError> {
        let request = Self::new_request(command);
        self.wire()
            .send_and_pull_async(
                job_name,
                SERVICE_ID_SQL,
                request,
                lobs,
                converter,
                self.default_timeout,
                self.session.fail_on_drop_error(),
            )
            .await
    }

    fn new_request(command: SqlCommand) -> SqlRequest {
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
) -> Result<(Option<SqlResponse>, Option<HashMap<String, BlobInfo>>), TgError> {
    match response {
        WireResponse::ResponseSessionPayload(_slot, payload, lobs, error) => {
            if let Some(e) = error {
                return Err(e.to_tg_error());
            }
            if payload.is_none() {
                return Err(invalid_response_error!(function_name, "payload is None"));
            }
            // let payload = payload.as_deref().unwrap();
            let payload = &payload.as_ref().unwrap()[..];
            let sql_response = SqlResponse::decode_length_delimited(payload)
                .map_err(|e| prost_decode_error!(function_name, "SqlResponse", e))?;
            match &sql_response.response {
                Some(SqlResponseType::ResultOnly(result_only)) => match &result_only.result {
                    Some(crate::jogasaki::proto::sql::response::result_only::Result::Success(
                        _,
                    )) => Ok((Some(sql_response), lobs.clone())),
                    Some(crate::jogasaki::proto::sql::response::result_only::Result::Error(
                        error,
                    )) => {
                        let error = error.clone();
                        Err(sql_service_error!(function_name, error))
                    }
                    _ => Ok((Some(sql_response), lobs.clone())),
                },
                _ => Ok((Some(sql_response), lobs.clone())),
            }
        }
        _ => Ok((None, None)),
    }
}

pub(crate) fn sql_result_only_success_processor(
    function_name: &str,
    response: WireResponse,
) -> Result<(), TgError> {
    let (sql_response, _) = convert_sql_response(function_name, &response)?;
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
        assert_eq!("sql-1.4", smv);
    }
}
