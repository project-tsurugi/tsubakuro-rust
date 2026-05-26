use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{atomic::AtomicI64, Arc},
    time::Duration,
};

use log::trace;

use crate::{
    client_error,
    error::TgError,
    invalid_response_error, io_error,
    job::Job,
    jogasaki::proto::sql::{
        common::Transaction as ProtoTransaction,
        request::{
            request::Request as SqlCommand, ClientOnlyLargeObjectInfo, Request as SqlRequest,
        },
        response::{response::Response as SqlResponseType, Response as SqlResponse},
    },
    prelude::{
        error_info::{transaction_error_info_processor, TransactionErrorInfo},
        execute_result_processor,
        explain::explain_processor,
        list_tables_processor, prepare_dispose_processor, prepare_processor,
        query_result_processor,
        r#type::large_object::TgLargeObjectCache,
        table_metadata_processor, transaction_status_processor, CommitOption, ServiceClient,
        SqlExecuteResult, SqlParameter, SqlPlaceholder, SqlQueryResult, TableList, TableMetadata,
        TgBlobReference, TgClobReference, TransactionStatusWithMessage,
    },
    prost_decode_error,
    service::{
        lob::{
            downloader::{BlobDownloader, ClobDownloader},
            lob_client::{create_lob_client, LobClient, LobClientMethod, RemoteLob},
            uploader::{BlobUploader, ClobUploader},
        },
        sql::r#type::{blob::TgBlob, clob::TgClob},
        ServiceMessageVersion,
    },
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

use super::{
    explain::SqlExplainResult, prepare::SqlPreparedStatement,
    r#type::large_object::TgLargeObjectReference,
};

/// The symbolic ID of the destination service.
const SERVICE_SYMBOLIC_ID: &str = "sql";

/// The major service message version which this client requests.
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 2;

/// The minor service message version which this client requests.
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 1;

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
    lob_client: tokio::sync::OnceCell<Box<dyn LobClient>>,
    default_timeout: Duration,
}

impl ServiceClient for SqlClient {
    fn new(session: Arc<Session>) -> Self {
        let default_timeout = session.default_timeout();
        SqlClient {
            session,
            lob_client: tokio::sync::OnceCell::new(),
            default_timeout,
        }
    }
}

impl ServiceMessageVersion for SqlClient {
    fn service_message_version() -> String {
        format!(
            "{}-{}.{}",
            SERVICE_SYMBOLIC_ID, SERVICE_MESSAGE_VERSION_MAJOR, SERVICE_MESSAGE_VERSION_MINOR
        )
    }
}

impl SqlClient {
    async fn get_lob_client(&self) -> Result<&Box<dyn LobClient + 'static>, TgError> {
        self.lob_client
            .get_or_try_init(|| create_lob_client(&self.session))
            .await
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
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;
        let table_list = list_tables_processor(slot_handle, response)?;

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
            .send_and_pull_async("ListTables", command, None, Arc::new(list_tables_processor))
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
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;
        let metadata = table_metadata_processor(slot_handle, response)?;

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
                Arc::new(table_metadata_processor),
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
        let (_, response) = self.send_and_pull_response(command, None, timeout).await?;

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
                Arc::new(move |_, response| {
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
        let (_, response) = self.send_and_pull_response(command, None, timeout).await?;
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
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;

        let explain_result = explain_processor(slot_handle, response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(explain_result)
    }

    /// Retrieves execution plan of the statement.
    pub async fn explain_async(&self, sql: &str) -> Result<Job<SqlExplainResult>, TgError> {
        const FUNCTION_NAME: &str = "explain_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::explain_text_command(sql);
        let job = self
            .send_and_pull_async("Explain", command, None, Arc::new(explain_processor))
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

        let (parameters, lobs) = self.convert_lob_parameters(parameters, timeout).await?;
        let command = Self::explain_prepared_command(prepared_statement, parameters);
        let (slot_handle, response) = self.send_and_pull_response(command, lobs, timeout).await?;
        let explain_result = explain_processor(slot_handle, response)?;

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

        let timeout = self.default_timeout;
        let (parameters, lobs) = self.convert_lob_parameters(parameters, timeout).await?;
        let command = Self::explain_prepared_command(prepared_statement, parameters);
        let job = self
            .send_and_pull_async("Explain", command, lobs, Arc::new(explain_processor))
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
        let (_, response) = self.send_and_pull_response(command, None, timeout).await?;

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
                Arc::new(move |_, response| {
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
    ///     let status = client.get_transaction_error_info(transaction).await?;
    ///     println!("is_error={}", status.is_error());
    ///
    ///     if let Some(code) = status.diagnostic_code() {
    ///         println!("diagnostic_code={}", code);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// since 0.2.0
    pub async fn get_transaction_error_info(
        &self,
        transaction: &Transaction,
    ) -> Result<TransactionErrorInfo, TgError> {
        let timeout = self.default_timeout;
        self.get_transaction_error_info_for(transaction, timeout)
            .await
    }

    /// Returns occurred error in the target transaction.
    ///
    /// since 0.2.0
    pub async fn get_transaction_error_info_for(
        &self,
        transaction: &Transaction,
        timeout: Duration,
    ) -> Result<TransactionErrorInfo, TgError> {
        const FUNCTION_NAME: &str = "get_transaction_error_info()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::transaction_error_info_command(transaction.transaction_handle()?);
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;
        let status = transaction_error_info_processor(slot_handle, response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(status)
    }

    /// Returns occurred error in the target transaction.
    ///
    /// since 0.2.0
    pub async fn get_transaction_error_info_async(
        &self,
        transaction: &Transaction,
    ) -> Result<Job<TransactionErrorInfo>, TgError> {
        const FUNCTION_NAME: &str = "get_transaction_error_info_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::transaction_error_info_command(transaction.transaction_handle()?);
        let job = self
            .send_and_pull_async(
                "TransactionErrorInfo",
                command,
                None,
                Arc::new(transaction_error_info_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn transaction_error_info_command(transaction_handle: &ProtoTransaction) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::GetErrorInfo {
            transaction_handle: Some(*transaction_handle),
        };
        SqlCommand::GetErrorInfo(request)
    }

    /// Get the transaction status on the server.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    ///     let status = client.get_transaction_status(transaction).await?;
    ///     println!("status={:?}", status.status());
    ///     println!("message={}", status.message());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// since 0.2.0
    pub async fn get_transaction_status(
        &self,
        transaction: &Transaction,
    ) -> Result<TransactionStatusWithMessage, TgError> {
        let timeout = self.default_timeout;
        self.get_transaction_status_for(transaction, timeout).await
    }

    /// Get the transaction status on the server.
    ///
    /// since 0.2.0
    pub async fn get_transaction_status_for(
        &self,
        transaction: &Transaction,
        timeout: Duration,
    ) -> Result<TransactionStatusWithMessage, TgError> {
        const FUNCTION_NAME: &str = "get_transaction_status()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::transaction_status_command(transaction.transaction_handle()?);
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;
        let status = transaction_status_processor(slot_handle, response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(status)
    }

    /// Get the transaction status on the server.
    ///
    /// since 0.2.0
    pub async fn get_transaction_status_async(
        &self,
        transaction: &Transaction,
    ) -> Result<Job<TransactionStatusWithMessage>, TgError> {
        const FUNCTION_NAME: &str = "get_transaction_status_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::transaction_status_command(transaction.transaction_handle()?);
        let job = self
            .send_and_pull_async(
                "TransactionStatus",
                command,
                None,
                Arc::new(transaction_status_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn transaction_status_command(transaction_handle: &ProtoTransaction) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::GetTransactionStatus {
            transaction_handle: Some(*transaction_handle),
        };
        SqlCommand::GetTransactionStatus(request)
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
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;
        let execute_result = execute_result_processor(slot_handle, response)?;

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
            .send_and_pull_async("Execute", command, None, Arc::new(execute_result_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_statement_command(transaction_handle: &ProtoTransaction, sql: &str) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::ExecuteStatement {
            transaction_handle: Some(*transaction_handle),
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
        let (parameters, lobs) = self.convert_lob_parameters(parameters, timeout).await?;

        let command =
            Self::execute_prepared_statement_command(tx_handle, prepared_statement, parameters);
        let (slot_handle, response) = self.send_and_pull_response(command, lobs, timeout).await?;
        let execute_result = execute_result_processor(slot_handle, response)?;

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
        let timeout = self.default_timeout;
        let (parameters, lobs) = self.convert_lob_parameters(parameters, timeout).await?;

        let command =
            Self::execute_prepared_statement_command(tx_handle, prepared_statement, parameters);
        let job = self
            .send_and_pull_async("Execute", command, lobs, Arc::new(execute_result_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_prepared_statement_command(
        transaction_handle: &ProtoTransaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> SqlCommand {
        let ps_handle = crate::jogasaki::proto::sql::common::PreparedStatement {
            handle: prepared_statement.prepare_handle(),
            has_result_records: prepared_statement.has_result_records(),
        };
        let request = crate::jogasaki::proto::sql::request::ExecutePreparedStatement {
            transaction_handle: Some(*transaction_handle),
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
    ///             let name: Option<String> = query_result.fetch().await?;
    ///         }
    ///         if query_result.next_column().await? {
    ///             let age: Option<i32> = query_result.fetch().await?;
    ///         }
    ///     }
    ///
    ///     query_result.close().await?;
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
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;

        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let query_result = query_result_processor(wire, slot_handle, response, default_timeout)?;

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
                Arc::new(move |slot_handle, response| {
                    query_result_processor(wire.clone(), slot_handle, response, default_timeout)
                }),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_query_command(transaction_handle: &ProtoTransaction, sql: &str) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::ExecuteQuery {
            transaction_handle: Some(*transaction_handle),
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
    ///             let name: Option<String> = query_result.fetch().await?;
    ///         }
    ///         if query_result.next_column().await? {
    ///             let age: Option<i32> = query_result.fetch().await?;
    ///         }
    ///     }
    ///
    ///     query_result.close().await?;
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
        let (parameters, lobs) = self.convert_lob_parameters(parameters, timeout).await?;

        let command =
            Self::execute_prepared_query_command(tx_handle, prepared_statement, parameters);
        let (slot_handle, response) = self.send_and_pull_response(command, lobs, timeout).await?;

        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let query_result = query_result_processor(wire, slot_handle, response, default_timeout)?;

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
        let timeout = self.default_timeout;
        let (parameters, lobs) = self.convert_lob_parameters(parameters, timeout).await?;

        let command =
            Self::execute_prepared_query_command(tx_handle, prepared_statement, parameters);
        let wire = self.wire().clone();
        let default_timeout = self.default_timeout;
        let job = self
            .send_and_pull_async(
                "Query",
                command,
                lobs,
                Arc::new(move |slot_handle, response| {
                    query_result_processor(wire.clone(), slot_handle, response, default_timeout)
                }),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn execute_prepared_query_command(
        transaction_handle: &ProtoTransaction,
        prepared_statement: &SqlPreparedStatement,
        parameters: Vec<SqlParameter>,
    ) -> SqlCommand {
        let ps_handle = crate::jogasaki::proto::sql::common::PreparedStatement {
            handle: prepared_statement.prepare_handle(),
            has_result_records: prepared_statement.has_result_records(),
        };
        let request = crate::jogasaki::proto::sql::request::ExecutePreparedQuery {
            transaction_handle: Some(*transaction_handle),
            prepared_statement_handle: Some(ps_handle),
            parameters,
        };
        SqlCommand::ExecutePreparedQuery(request)
    }

    pub async fn allows_lob_operation(&self, operation: LobOperation) -> Result<bool, TgError> {
        let lob_client = self.get_lob_client().await?;

        use LobClientMethod::*;
        let method_list = match operation {
            LobOperation::UploadLobFile => vec![UploadLobFile],
            LobOperation::UploadLob => vec![UploadLob],
            LobOperation::CreateLobUploader => vec![CreateLobUploader],
            LobOperation::OpenLob => vec![DownloadLobFile],
            LobOperation::GetLobCache => vec![DownloadLobFile, DownloadLob],
            LobOperation::ReadLob => vec![DownloadLob],
            LobOperation::CopyLobTo => vec![DownloadLobFile, DownloadLob],
            LobOperation::CreateLobDownloader => vec![CreateLobDownloader],
        };

        for m in method_list {
            let supported = lob_client.supports_method(m);
            if supported {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn upload_blob_file<T: AsRef<Path>>(&self, path: T) -> Result<TgBlob, TgError> {
        let timeout = self.default_timeout;
        self.upload_blob_file_for(path, timeout).await
    }

    pub async fn upload_blob_file_for<T: AsRef<Path>>(
        &self,
        path: T,
        timeout: Duration,
    ) -> Result<TgBlob, TgError> {
        const FUNCTION_NAME: &str = "upload_blob_file()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let lob = lob_client.upload_lob_file(path.as_ref(), timeout).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(TgBlob::RemoteLob(lob))
    }

    pub async fn upload_blob_file_async<T: AsRef<Path>>(
        &self,
        path: T,
    ) -> Result<Job<TgBlob>, TgError> {
        const FUNCTION_NAME: &str = "upload_blob_file_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let job = lob_client.upload_lob_file_async(path.as_ref()).await?;
        let job = job.convert("TgBlob", Arc::new(|lob| Ok(TgBlob::RemoteLob(lob))));

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    pub async fn upload_clob_file<T: AsRef<Path>>(&self, path: T) -> Result<TgClob, TgError> {
        let timeout = self.default_timeout;
        self.upload_clob_file_for(path, timeout).await
    }

    pub async fn upload_clob_file_for<T: AsRef<Path>>(
        &self,
        path: T,
        timeout: Duration,
    ) -> Result<TgClob, TgError> {
        const FUNCTION_NAME: &str = "upload_clob_file()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let lob = lob_client.upload_lob_file(path.as_ref(), timeout).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(TgClob::RemoteLob(lob))
    }

    pub async fn upload_clob_file_async<T: AsRef<Path>>(
        &self,
        path: T,
    ) -> Result<Job<TgClob>, TgError> {
        const FUNCTION_NAME: &str = "upload_clob_file_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let job = lob_client.upload_lob_file_async(path.as_ref()).await?;
        let job = job.convert("TgClob", Arc::new(|lob| Ok(TgClob::RemoteLob(lob))));

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    pub async fn upload_blob(&self, value: &[u8]) -> Result<TgBlob, TgError> {
        let timeout = self.default_timeout;
        self.upload_blob_for(value, timeout).await
    }

    pub async fn upload_blob_for(
        &self,
        value: &[u8],
        timeout: Duration,
    ) -> Result<TgBlob, TgError> {
        const FUNCTION_NAME: &str = "upload_blob()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let lob = lob_client.upload_lob(value, timeout).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(TgBlob::RemoteLob(lob))
    }

    pub async fn upload_blob_async(&self, value: &[u8]) -> Result<Job<TgBlob>, TgError> {
        const FUNCTION_NAME: &str = "upload_blob_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let job = lob_client.upload_lob_async(value).await?;
        let job = job.convert("TgBlob", Arc::new(|lob| Ok(TgBlob::RemoteLob(lob))));

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    pub async fn upload_clob(&self, value: &str) -> Result<TgClob, TgError> {
        let timeout = self.default_timeout;
        self.upload_clob_for(value, timeout).await
    }

    pub async fn upload_clob_for(&self, value: &str, timeout: Duration) -> Result<TgClob, TgError> {
        const FUNCTION_NAME: &str = "upload_clob()";
        trace!("{} start", FUNCTION_NAME);

        let value = value.as_bytes();

        let lob_client = self.get_lob_client().await?;
        let lob = lob_client.upload_lob(value, timeout).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(TgClob::RemoteLob(lob))
    }

    pub async fn upload_clob_async(&self, value: &str) -> Result<Job<TgClob>, TgError> {
        const FUNCTION_NAME: &str = "upload_clob_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let value = value.as_bytes();
        let job = lob_client.upload_lob_async(value).await?;
        let job = job.convert("TgClob", Arc::new(|lob| Ok(TgClob::RemoteLob(lob))));

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    pub async fn create_blob_uploader(&self) -> Result<BlobUploader, TgError> {
        let lob_client = self.get_lob_client().await?;
        let uploader = lob_client.create_lob_uploader().await?;
        Ok(BlobUploader::new(uploader))
    }

    pub async fn create_clob_uploader(&self) -> Result<ClobUploader, TgError> {
        let lob_client = self.get_lob_client().await?;
        let uploader = lob_client.create_lob_uploader().await?;
        Ok(ClobUploader::new(uploader))
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

        let lob_client = self.get_lob_client().await?;
        let client_path = lob_client
            .download_lob_file(transaction, blob, timeout)
            .await?;
        let file = Self::open_lob_file(client_path)?;

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

        let lob_client = self.get_lob_client().await?;
        let job = lob_client
            .download_lob_file_async(transaction, blob)
            .await?;
        let job = job.convert("File", Arc::new(Self::open_lob_file));

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    /// Open CLOB file.
    ///
    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, query_result: &mut SqlQueryResult) -> Result<String, TgError> {
    ///     let clob: TgClobReference = query_result.fetch().await?;
    ///     let mut file = client.open_clob(transaction, &clob).await?;
    ///
    ///     let mut buffer = String::new();
    ///     file.read_to_string(&mut buffer).unwrap();
    ///
    ///     Ok(buffer)
    /// }
    /// ```
    pub async fn open_clob(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
    ) -> Result<std::fs::File, TgError> {
        let timeout = self.default_timeout;
        self.open_clob_for(transaction, clob, timeout).await
    }

    /// Open CLOB file.
    pub async fn open_clob_for(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
        timeout: Duration,
    ) -> Result<std::fs::File, TgError> {
        const FUNCTION_NAME: &str = "open_clob()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let client_path = lob_client
            .download_lob_file(transaction, clob, timeout)
            .await?;
        let file = Self::open_lob_file(client_path)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(file)
    }

    /// Open CLOB file.
    pub async fn open_clob_async(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
    ) -> Result<Job<std::fs::File>, TgError> {
        const FUNCTION_NAME: &str = "open_clob_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let job = lob_client
            .download_lob_file_async(transaction, clob)
            .await?;
        let job = job.convert("File", Arc::new(Self::open_lob_file));

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn open_lob_file(client_path: PathBuf) -> Result<std::fs::File, TgError> {
        std::fs::File::open(client_path).map_err(|e| io_error!("Failed to open lob file: {}", e))
    }

    /// Get BLOB cache.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, query_result: &mut SqlQueryResult) -> Result<(), TgError> {
    ///     let blob: TgBlobReference = query_result.fetch().await?;
    ///     let cache = client.get_blob_cache(transaction, &blob).await?;
    ///
    ///     println!("BLOB.path={:?}", cache.path());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// since 0.5.0
    pub async fn get_blob_cache(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
    ) -> Result<TgLargeObjectCache, TgError> {
        let timeout = self.default_timeout;
        self.get_blob_cache_for(transaction, blob, timeout).await
    }

    /// Get BLOB cache.
    ///
    /// since 0.5.0
    pub async fn get_blob_cache_for(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        timeout: Duration,
    ) -> Result<TgLargeObjectCache, TgError> {
        const FUNCTION_NAME: &str = "get_blob_cache()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let cache = if lob_client.supports_method(LobClientMethod::DownloadLobFile) {
            let client_path = lob_client
                .download_lob_file(transaction, blob, timeout)
                .await?;
            Self::create_large_object_cache(client_path)?
        } else {
            TgLargeObjectCache::new(None)
        };

        trace!("{} end", FUNCTION_NAME);
        Ok(cache)
    }

    /// Get BLOB cache.
    ///
    /// since 0.5.0
    pub async fn get_blob_cache_async(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
    ) -> Result<Job<TgLargeObjectCache>, TgError> {
        const FUNCTION_NAME: &str = "get_blob_cache_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let job = if lob_client.supports_method(LobClientMethod::DownloadLobFile) {
            let job = lob_client
                .download_lob_file_async(transaction, blob)
                .await?;
            job.convert(
                "LargeObjectCache",
                Arc::new(Self::create_large_object_cache),
            )
        } else {
            Job::returns("LargeObjectCache", TgLargeObjectCache::new(None))
        };

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    /// Get CLOB cache.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, query_result: &mut SqlQueryResult) -> Result<(), TgError> {
    ///     let clob: TgClobReference = query_result.fetch().await?;
    ///     let cache = client.get_clob_cache(transaction, &clob).await?;
    ///
    ///     println!("CLOB.path={:?}", cache.path());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// since 0.5.0
    pub async fn get_clob_cache(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
    ) -> Result<TgLargeObjectCache, TgError> {
        let timeout = self.default_timeout;
        self.get_clob_cache_for(transaction, clob, timeout).await
    }

    /// Get CLOB cache.
    ///
    /// since 0.5.0
    pub async fn get_clob_cache_for(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
        timeout: Duration,
    ) -> Result<TgLargeObjectCache, TgError> {
        const FUNCTION_NAME: &str = "get_clob_cache()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let cache = if lob_client.supports_method(LobClientMethod::DownloadLobFile) {
            let client_path = lob_client
                .download_lob_file(transaction, clob, timeout)
                .await?;
            Self::create_large_object_cache(client_path)?
        } else {
            TgLargeObjectCache::new(None)
        };

        trace!("{} end", FUNCTION_NAME);
        Ok(cache)
    }

    /// Get CLOB cache.
    ///
    /// since 0.5.0
    pub async fn get_clob_cache_async(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
    ) -> Result<Job<TgLargeObjectCache>, TgError> {
        const FUNCTION_NAME: &str = "get_clob_cache_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let job = if lob_client.supports_method(LobClientMethod::DownloadLobFile) {
            let job = lob_client
                .download_lob_file_async(transaction, clob)
                .await?;
            job.convert(
                "LargeObjectCache",
                Arc::new(Self::create_large_object_cache),
            )
        } else {
            Job::returns("LargeObjectCache", TgLargeObjectCache::new(None))
        };

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn create_large_object_cache(client_path: PathBuf) -> Result<TgLargeObjectCache, TgError> {
        let cache = TgLargeObjectCache::new(Some(client_path));
        Ok(cache)
    }

    /// Read BLOB.
    ///
    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, query_result: &mut SqlQueryResult) -> Result<Vec<u8>, TgError> {
    ///     let blob: TgBlobReference = query_result.fetch().await?;
    ///     let bytes = client.read_blob(transaction, &blob).await?;
    ///
    ///     Ok(bytes)
    /// }
    /// ```
    ///
    /// since 0.2.0
    pub async fn read_blob(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
    ) -> Result<Vec<u8>, TgError> {
        let timeout = self.default_timeout;
        self.read_blob_for(transaction, blob, timeout).await
    }

    /// Read BLOB.
    ///
    /// since 0.2.0
    pub async fn read_blob_for(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        timeout: Duration,
    ) -> Result<Vec<u8>, TgError> {
        const FUNCTION_NAME: &str = "read_blob()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let buf = lob_client.download_lob(transaction, blob, timeout).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(buf)
    }

    /// Read BLOB.
    ///
    /// since 0.2.0
    pub async fn read_blob_async(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
    ) -> Result<Job<Vec<u8>>, TgError> {
        const FUNCTION_NAME: &str = "read_blob_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let job = lob_client.download_lob_async(transaction, blob).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    /// Read CLOB.
    ///
    /// # Examples
    /// ```
    /// use std::io::Read;
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, query_result: &mut SqlQueryResult) -> Result<String, TgError> {
    ///     let clob: TgClobReference = query_result.fetch().await?;
    ///     let text = client.read_clob(transaction, &clob).await?;
    ///
    ///     Ok(text)
    /// }
    /// ```
    ///
    /// since 0.2.0
    pub async fn read_clob(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
    ) -> Result<String, TgError> {
        let timeout = self.default_timeout;
        self.read_clob_for(transaction, clob, timeout).await
    }

    /// Read CLOB.
    ///
    /// since 0.2.0
    pub async fn read_clob_for(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
        timeout: Duration,
    ) -> Result<String, TgError> {
        const FUNCTION_NAME: &str = "read_clob()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let buf = lob_client.download_lob(transaction, clob, timeout).await?;
        let buf =
            String::from_utf8(buf).map_err(|e| io_error!("CLOB data is not valid UTF-8: {}", e))?;

        trace!("{} end", FUNCTION_NAME);
        Ok(buf)
    }

    /// Read CLOB.
    ///
    /// since 0.2.0
    pub async fn read_clob_async(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
    ) -> Result<Job<String>, TgError> {
        const FUNCTION_NAME: &str = "read_clob_async()";
        trace!("{} start", FUNCTION_NAME);

        let lob_client = self.get_lob_client().await?;
        let job = lob_client.download_lob_async(transaction, clob).await?;
        let job = job.convert(
            "CLOB",
            Arc::new(|buf| {
                String::from_utf8(buf).map_err(|e| io_error!("CLOB data is not valid UTF-8: {}", e))
            }),
        );

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
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
    pub async fn copy_blob_to<T: AsRef<Path>>(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        destination: T,
    ) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.copy_blob_to_for(transaction, blob, destination, timeout)
            .await
    }

    /// Copy BLOB to local file.
    pub async fn copy_blob_to_for<T: AsRef<Path>>(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        destination: T,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "copy_blob_to()";
        trace!("{} start", FUNCTION_NAME);

        self.copy_lob_to(transaction, blob, destination.as_ref(), timeout)
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    /// Copy BLOB to local file.
    pub async fn copy_blob_to_async<T: AsRef<Path> + Send + Sync + Clone>(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        destination: T,
    ) -> Result<Job<()>, TgError> {
        const FUNCTION_NAME: &str = "copy_blob_to_async()";
        trace!("{} start", FUNCTION_NAME);

        let job = self
            .copy_lob_to_async(transaction, blob, destination.as_ref())
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    /// Copy CLOB to local file.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SqlClient, transaction: &Transaction, query_result: &mut SqlQueryResult) -> Result<(), TgError> {
    ///     let clob: TgClobReference = query_result.fetch().await?;
    ///     client.copy_clob_to(transaction, &clob, "/path/to/clob.txt").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn copy_clob_to<T: AsRef<Path>>(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
        destination: T,
    ) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.copy_clob_to_for(transaction, clob, destination, timeout)
            .await
    }

    /// Copy CLOB to local file.
    pub async fn copy_clob_to_for<T: AsRef<Path>>(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
        destination: T,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "copy_clob_to()";
        trace!("{} start", FUNCTION_NAME);

        self.copy_lob_to(transaction, clob, destination.as_ref(), timeout)
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    /// Copy CLOB to local file.
    pub async fn copy_clob_to_async<T: AsRef<Path> + Send + Sync + Clone>(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
        destination: T,
    ) -> Result<Job<()>, TgError> {
        const FUNCTION_NAME: &str = "copy_clob_to_async()";
        trace!("{} start", FUNCTION_NAME);

        let job = self
            .copy_lob_to_async(transaction, clob, destination.as_ref())
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    async fn copy_lob_to(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        destination: &Path,
        timeout: Duration,
    ) -> Result<(), TgError> {
        let lob_client = self.get_lob_client().await?;
        if lob_client.supports_method(LobClientMethod::DownloadLobFile) {
            let client_path = lob_client
                .download_lob_file(transaction, lob, timeout)
                .await?;
            std::fs::copy(client_path, destination)
                .map_err(|e| io_error!("Failed to copy lob file: {}", e))?;
        } else {
            let buf = lob_client.download_lob(transaction, lob, timeout).await?;
            std::fs::write(destination, buf)
                .map_err(|e| io_error!("Failed to write lob file: {}", e))?;
        }
        Ok(())
    }

    async fn copy_lob_to_async(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        destination: &Path,
    ) -> Result<Job<()>, TgError> {
        let destination = destination.to_path_buf();

        let lob_client = self.get_lob_client().await?;
        let job = if lob_client.supports_method(LobClientMethod::DownloadLobFile) {
            let job = lob_client.download_lob_file_async(transaction, lob).await?;
            job.convert(
                "LobFileCopy",
                Arc::new(move |client_path| {
                    std::fs::copy(client_path, destination.clone())
                        .map_err(|e| io_error!("Failed to copy lob file: {}", e))?;
                    Ok(())
                }),
            )
        } else {
            let job = lob_client.download_lob_async(transaction, lob).await?;
            job.convert(
                "LobCopy",
                Arc::new(move |buf| {
                    std::fs::write(destination.clone(), buf)
                        .map_err(|e| io_error!("Failed to write lob file: {}", e))?;
                    Ok(())
                }),
            )
        };
        Ok(job)
    }

    pub async fn create_blob_downloader(
        &self,
        transaction: &Transaction,
        blob: &TgBlobReference,
        timeout: Duration,
    ) -> Result<BlobDownloader, TgError> {
        let lob_client = self.get_lob_client().await?;
        let downloader = lob_client
            .create_lob_downloader(transaction, blob, timeout)
            .await?;
        Ok(BlobDownloader::new(downloader))
    }

    pub async fn create_clob_downloader(
        &self,
        transaction: &Transaction,
        clob: &TgClobReference,
        timeout: Duration,
    ) -> Result<ClobDownloader, TgError> {
        let lob_client = self.get_lob_client().await?;
        let downloader = lob_client
            .create_lob_downloader(transaction, clob, timeout)
            .await?;
        Ok(ClobDownloader::new(downloader))
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
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;
        transaction_commit_processor(slot_handle, response)?;

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
                Arc::new(transaction_commit_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn commit_command(
        transaction_handle: &ProtoTransaction,
        commit_option: &CommitOption,
    ) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::Commit {
            transaction_handle: Some(*transaction_handle),
            notification_type: commit_option.notification_type,
            auto_dispose: commit_option.auto_dispose,
            option: Some(*commit_option),
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
        let (slot_handle, response) = self.send_and_pull_response(command, None, timeout).await?;
        transaction_rollback_processor(slot_handle, response)?;

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
                Arc::new(transaction_rollback_processor),
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn rollback_command(transaction_handle: &ProtoTransaction) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::Rollback {
            transaction_handle: Some(*transaction_handle),
        };
        SqlCommand::Rollback(request)
    }

    pub(crate) async fn dispose_transaction(
        &self,
        transaction_handle: &ProtoTransaction,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "dispose_transaction()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::dispose_transaction_command(transaction_handle);
        let (_, response) = self.send_and_pull_response(command, None, timeout).await?;
        transaction_dispose_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    pub(crate) async fn dispose_transaction_send_only(
        &self,
        transaction_handle: &ProtoTransaction,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "dispose_transaction()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::dispose_transaction_command(transaction_handle);
        let _ = self.send_only(command).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    fn dispose_transaction_command(transaction_handle: &ProtoTransaction) -> SqlCommand {
        let request = crate::jogasaki::proto::sql::request::DisposeTransaction {
            transaction_handle: Some(*transaction_handle),
        };
        SqlCommand::DisposeTransaction(request)
    }
}

static BLOB_NUMBER: AtomicI64 = AtomicI64::new(0);
static CLOB_NUMBER: AtomicI64 = AtomicI64::new(0);

impl SqlClient {
    async fn convert_lob_parameters(
        &self,
        parameters: Vec<SqlParameter>,
        timeout: Duration,
    ) -> Result<(Vec<SqlParameter>, Option<Vec<BlobInfo>>), TgError> {
        use crate::jogasaki::proto::sql::common::blob::Data as BlobData;
        use crate::jogasaki::proto::sql::common::clob::Data as ClobData;
        use crate::jogasaki::proto::sql::common::Blob;
        use crate::jogasaki::proto::sql::common::Clob;
        use crate::jogasaki::proto::sql::request::parameter::Value;

        let mut parameters_result = Vec::with_capacity(parameters.len());
        let mut lobs = Vec::new();
        for parameter in parameters {
            let parameter = match parameter {
                SqlParameter {
                    placement,
                    value: Some(Value::LargeObjectInfoBlob(data)),
                } => {
                    let channel_name = Self::create_channel_name("Blob", &BLOB_NUMBER);
                    let lob_info = self
                        .create_lob_info(channel_name.clone(), data, timeout)
                        .await?;
                    lobs.push(lob_info);

                    let data = BlobData::ChannelName(channel_name);
                    let value = Blob { data: Some(data) };
                    let value = Value::Blob(value);
                    SqlParameter {
                        placement,
                        value: Some(value),
                    }
                }
                SqlParameter {
                    placement,
                    value: Some(Value::LargeObjectInfoClob(data)),
                } => {
                    let channel_name = Self::create_channel_name("Clob", &CLOB_NUMBER);
                    let lob_info = self
                        .create_lob_info(channel_name.clone(), data, timeout)
                        .await?;
                    lobs.push(lob_info);

                    let data = ClobData::ChannelName(channel_name);
                    let value = Clob { data: Some(data) };
                    let value = Value::Clob(value);
                    SqlParameter {
                        placement,
                        value: Some(value),
                    }
                }
                parameter => parameter,
            };
            parameters_result.push(parameter);
        }

        if lobs.is_empty() {
            Ok((parameters_result, None))
        } else {
            Ok((parameters_result, Some(lobs)))
        }
    }

    fn create_channel_name(prefix: &str, number: &AtomicI64) -> String {
        let pid = std::process::id();
        let n = number.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        format!("Rust{prefix}Channel-{pid}-{n}")
    }

    async fn create_lob_info(
        &self,
        channel_name: String,
        info: ClientOnlyLargeObjectInfo,
        timeout: Duration,
    ) -> Result<BlobInfo, TgError> {
        use crate::jogasaki::proto::sql::request::client_only_large_object_info::Data;
        use crate::tateyama::proto::framework::common::blob_info::BlobLocation;
        use crate::tateyama::proto::framework::common::BlobRelayReference;

        let lob_location = match info.data {
            Some(Data::ClientPath(path)) => {
                let client_path = Path::new(&path);
                let lob_client = self.get_lob_client().await?;
                let lob = lob_client.upload_lob_file(client_path, timeout).await?;
                match lob {
                    RemoteLob::ServerPath(path) => BlobLocation::Path(path),
                    RemoteLob::LobReference(storage_id, object_id, tag) => {
                        BlobLocation::Blob(BlobRelayReference {
                            storage_id,
                            object_id,
                            tag,
                        })
                    }
                }
            }
            Some(Data::ServerPath(path)) => BlobLocation::Path(path),
            Some(Data::BlobRelayReference(lob_ref)) => BlobLocation::Blob(BlobRelayReference {
                storage_id: lob_ref.storage_id,
                object_id: lob_ref.object_id,
                tag: lob_ref.tag,
            }),
            None => return Err(client_error!("Large object info data is None")),
        };
        Ok(BlobInfo {
            channel_name,
            temporary: false,
            blob_location: Some(lob_location),
        })
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
    ) -> Result<(Arc<SlotEntryHandle>, WireResponse), TgError> {
        let request = Self::new_request(command);
        self.wire()
            .send_and_pull_response(SERVICE_ID_SQL, request, lobs, timeout)
            .await
    }

    async fn send_and_pull_async<T: Send + Sync + 'static>(
        &self,
        job_name: &str,
        command: SqlCommand,
        lobs: Option<Vec<BlobInfo>>,
        converter: Arc<
            dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send + Sync,
        >,
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

/// Large object (BLOB/CLOB) operation type.
///
/// since 0.10.0
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LobOperation {
    /// upload_lob_file
    UploadLobFile,
    /// upload_lob
    UploadLob,
    /// create_lob_uploader
    CreateLobUploader,
    /// open_lob
    OpenLob,
    /// get_lob_cache
    GetLobCache,
    /// read_lob
    ReadLob,
    /// copy_lob_to
    CopyLobTo,
    /// create_lob_downloader
    CreateLobDownloader,
}

#[allow(clippy::type_complexity)]
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
        assert_eq!("sql-2.1", smv);
    }
}
