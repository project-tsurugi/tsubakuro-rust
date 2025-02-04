use std::{ffi::CString, sync::Arc, time::Duration, vec};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null,
    ffi_exec_core_async, impl_job_delegator,
    job::{TsurugiFfiJob, TsurugiFfiJobHandle, VoidJobDelegator},
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::{
        execute_result::TsurugiFfiSqlExecuteResult,
        prepare::prepared_statement::TsurugiFfiSqlPreparedStatement,
        query_result::TsurugiFfiSqlQueryResult, table_list::TsurugiFfiTableList,
        table_metadata::TsurugiFfiTableMetadata,
    },
    transaction::{
        commit_option::TsurugiFfiCommitOptionHandle,
        option::TsurugiFfiTransactionOptionHandle,
        status::{TsurugiFfiTransactionStatus, TsurugiFfiTransactionStatusHandle},
        TsurugiFfiTransaction, TsurugiFfiTransactionHandle,
    },
    TsurugiFfiDuration, TsurugiFfiStringHandle,
};

use super::{
    execute_result::TsurugiFfiSqlExecuteResultHandle,
    prepare::{
        parameter::TsurugiFfiSqlParameterHandle, placeholder::TsurugiFfiSqlPlaceholderHandle,
        prepared_statement::TsurugiFfiSqlPreparedStatementHandle,
    },
    query_result::TsurugiFfiSqlQueryResultHandle,
    table_list::TsurugiFfiTableListHandle,
    table_metadata::TsurugiFfiTableMetadataHandle,
};

pub(crate) struct TsurugiFfiSqlClient {
    sql_client: SqlClient,
    runtime: Arc<tokio::runtime::Runtime>,
    service_message_version: Option<CString>,
}

impl TsurugiFfiSqlClient {
    pub(crate) fn new(
        sql_client: SqlClient,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiSqlClient {
        TsurugiFfiSqlClient {
            sql_client,
            runtime,
            service_message_version: None,
        }
    }

    pub(crate) fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

impl std::ops::Deref for TsurugiFfiSqlClient {
    type Target = SqlClient;

    fn deref(&self) -> &Self::Target {
        &self.sql_client
    }
}

impl std::ops::DerefMut for TsurugiFfiSqlClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sql_client
    }
}

pub type TsurugiFfiSqlClientHandle = *mut TsurugiFfiSqlClient;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_get_service_message_version(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    version_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_get_service_message_version()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, version_out={:?}",
        context,
        sql_client,
        version_out
    );

    ffi_arg_out_initialize!(version_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, version_out);

    let client = unsafe { &mut *sql_client };

    let smv = SqlClient::service_message_version();
    cchar_field_set!(context, client.service_message_version, smv);

    let ptr = cstring_to_cchar!(client.service_message_version);
    unsafe {
        *version_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (version={:?})", rc, ptr);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_list_tables(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    table_list_out: *mut TsurugiFfiTableListHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_list_tables()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, table_list_out={:?}",
        context,
        sql_client,
        table_list_out
    );

    ffi_arg_out_initialize!(table_list_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_list_out);

    let client = unsafe { &*sql_client };

    let runtime = client.runtime();
    let table_list = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, client.list_tables());

    let table_list = Box::new(TsurugiFfiTableList::new(table_list));

    let handle = Box::into_raw(table_list);
    unsafe {
        *table_list_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. table_list={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_list_tables_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    timeout: TsurugiFfiDuration,
    table_list_out: *mut TsurugiFfiTableListHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_list_tables_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, timeout={:?}, table_list_out={:?}",
        context,
        sql_client,
        timeout,
        table_list_out
    );

    ffi_arg_out_initialize!(table_list_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_list_out);

    let client = unsafe { &*sql_client };
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let table_list = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.list_tables_for(timeout)
    );

    let table_list = Box::new(TsurugiFfiTableList::new(table_list));

    let handle = Box::into_raw(table_list);
    unsafe {
        *table_list_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. table_list={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_list_tables_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    table_list_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_list_tables_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, table_list_job_out={:?}",
        context,
        sql_client,
        table_list_job_out
    );

    ffi_arg_out_initialize!(table_list_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_list_job_out);

    let client = unsafe { &*sql_client };

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, client.list_tables_async());
    let job = TsurugiFfiJob::new(job, Box::new(TableListJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *table_list_job_out = handle as TsurugiFfiJobHandle;
    }
    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. table_list_job={:?}",
        rc,
        handle
    );
    rc
}

impl_job_delegator! {
    TableListJobDelegator,
    TableList,
    TsurugiFfiTableList,
    "table_list",
}

impl TableListJobDelegator {
    fn convert(
        value: TableList,
        _runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiTableList> {
        Some(TsurugiFfiTableList::new(value))
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_get_table_metadata(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    table_name: TsurugiFfiStringHandle,
    table_metadata_out: *mut TsurugiFfiTableMetadataHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_get_table_metadata()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, table_name={:?}, table_metadata_out={:?}",
        context,
        sql_client,
        table_name,
        table_metadata_out
    );

    ffi_arg_out_initialize!(table_metadata_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_metadata_out);

    let client = unsafe { &*sql_client };
    let table_name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, table_name);

    let runtime = client.runtime();
    let table_metadata = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.get_table_metadata(table_name)
    );

    let table_metadata = Box::new(TsurugiFfiTableMetadata::new(table_metadata));

    let handle = Box::into_raw(table_metadata);
    unsafe {
        *table_metadata_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. table_metadata={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_get_table_metadata_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    table_name: TsurugiFfiStringHandle,
    timeout: TsurugiFfiDuration,
    table_metadata_out: *mut TsurugiFfiTableMetadataHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_get_table_metadata_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, table_name={:?}, timeout={:?}, table_metadata_out={:?}",
        context,
        sql_client,
        table_name,
        timeout,
        table_metadata_out
    );

    ffi_arg_out_initialize!(table_metadata_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, table_metadata_out);

    let client = unsafe { &*sql_client };
    let table_name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, table_name);
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let table_metadata = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.get_table_metadata_for(table_name, timeout)
    );

    let table_metadata = Box::new(TsurugiFfiTableMetadata::new(table_metadata));

    let handle = Box::into_raw(table_metadata);
    unsafe {
        *table_metadata_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. table_metadata={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_get_table_metadata_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    table_name: TsurugiFfiStringHandle,
    table_metadata_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_get_table_metadata_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, table_name={:?}, table_metadata_job_out={:?}",
        context,
        sql_client,
        table_name,
        table_metadata_job_out
    );

    ffi_arg_out_initialize!(table_metadata_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, table_name);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, table_metadata_job_out);

    let client = unsafe { &*sql_client };
    let table_name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, table_name);

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.get_table_metadata_async(table_name)
    );
    let job = TsurugiFfiJob::new(job, Box::new(TableMetadataJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *table_metadata_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. table_metadata_job={:?}",
        rc,
        handle
    );
    rc
}

impl_job_delegator! {
    TableMetadataJobDelegator,
    TableMetadata,
    TsurugiFfiTableMetadata,
    "table_metadata",
}

impl TableMetadataJobDelegator {
    fn convert(
        value: TableMetadata,
        _runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiTableMetadata> {
        Some(TsurugiFfiTableMetadata::new(value))
    }
}

macro_rules! convert_placeholders {
    ($context:expr, $function_name:expr, $arg_index:expr, $placeholders:expr, $placeholders_size:expr) => {
        if $placeholders_size > 0 {
            let src =
                unsafe { std::slice::from_raw_parts($placeholders, $placeholders_size as usize) };
            let mut dst = Vec::with_capacity(src.len());
            for &placeholder in src {
                ffi_arg_require_non_null!($context, $function_name, $arg_index, placeholder);
                let placeholder = unsafe { &*placeholder }.raw_clone();
                dst.push(placeholder);
            }
            dst
        } else {
            vec![]
        }
    };
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepare(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    sql: TsurugiFfiStringHandle,
    placeholders: *const TsurugiFfiSqlPlaceholderHandle,
    placeholders_size: u32,
    prepared_statement_out: *mut TsurugiFfiSqlPreparedStatementHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepare()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, sql={:?}, placeholders={:?}, placeholders_size={:?}, prepared_statement_out={:?}",
        context,
        sql_client,
        sql,
        placeholders,
        placeholders_size,
        prepared_statement_out
    );

    ffi_arg_out_initialize!(prepared_statement_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, sql);
    if placeholders_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, placeholders);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, prepared_statement_out);

    let client = unsafe { &*sql_client };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, sql);
    let placeholders: Vec<SqlPlaceholder> =
        convert_placeholders!(context, FUNCTION_NAME, 3, placeholders, placeholders_size);

    let runtime = client.runtime();
    let prepared_statement = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepare(sql, placeholders)
    );

    let prepared_statement = Box::new(TsurugiFfiSqlPreparedStatement::new(
        prepared_statement,
        runtime.clone(),
    ));

    let handle = Box::into_raw(prepared_statement);
    unsafe {
        *prepared_statement_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. prepared_statement={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepare_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    sql: TsurugiFfiStringHandle,
    placeholders: *const TsurugiFfiSqlPlaceholderHandle,
    placeholders_size: u32,
    timeout: TsurugiFfiDuration,
    prepared_statement_out: *mut TsurugiFfiSqlPreparedStatementHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepare_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, sql={:?}, placeholders={:?}, placeholders_size={:?}, timeout={:?}, prepared_statement_out={:?}",
        context,
        sql_client,
        sql,
        placeholders,
        placeholders_size,
        timeout,
        prepared_statement_out
    );

    ffi_arg_out_initialize!(prepared_statement_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, sql);
    if placeholders_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, placeholders);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 6, prepared_statement_out);

    let client = unsafe { &*sql_client };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, sql);
    let placeholders: Vec<SqlPlaceholder> =
        convert_placeholders!(context, FUNCTION_NAME, 3, placeholders, placeholders_size);
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let prepared_statement = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepare_for(sql, placeholders, timeout)
    );

    let prepared_statement = Box::new(TsurugiFfiSqlPreparedStatement::new(
        prepared_statement,
        runtime.clone(),
    ));

    let handle = Box::into_raw(prepared_statement);
    unsafe {
        *prepared_statement_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. prepared_statement={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepare_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    sql: TsurugiFfiStringHandle,
    placeholders: *const TsurugiFfiSqlPlaceholderHandle,
    placeholders_size: u32,
    prepared_statement_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepare_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, sql={:?}, placeholders={:?}, placeholders_size={:?}, prepared_statement_job_out={:?}",
        context,
        sql_client,
        sql,
        placeholders,
        placeholders_size,
        prepared_statement_job_out
    );

    ffi_arg_out_initialize!(prepared_statement_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, sql);
    if placeholders_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, placeholders);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, prepared_statement_job_out);

    let client = unsafe { &*sql_client };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, sql);
    let placeholders: Vec<SqlPlaceholder> =
        convert_placeholders!(context, FUNCTION_NAME, 3, placeholders, placeholders_size);

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepare_async(sql, placeholders)
    );
    let job = TsurugiFfiJob::new(
        job,
        Box::new(SqlPreparedStatementJobDelegator {}),
        runtime.clone(),
    );
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *prepared_statement_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. prepared_statement_job={:?}",
        rc,
        handle
    );
    rc
}

impl_job_delegator! {
    SqlPreparedStatementJobDelegator,
    SqlPreparedStatement,
    TsurugiFfiSqlPreparedStatement,
    "prepared_statement",
}

impl SqlPreparedStatementJobDelegator {
    fn convert(
        value: SqlPreparedStatement,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiSqlPreparedStatement> {
        Some(TsurugiFfiSqlPreparedStatement::new(value, runtime))
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_start_transaction(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_out: *mut TsurugiFfiTransactionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_start_transaction()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction_option={:?}, transaction_out={:?}",
        context,
        sql_client,
        transaction_option,
        transaction_out
    );

    ffi_arg_out_initialize!(transaction_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, transaction_out);

    let client = unsafe { &*sql_client };
    let transaction_option = unsafe { &*transaction_option };

    let runtime = client.runtime();
    let transaction = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.start_transaction(transaction_option)
    );

    let transaction = Box::new(TsurugiFfiTransaction::new(transaction, runtime.clone()));

    let handle = Box::into_raw(transaction);
    unsafe {
        *transaction_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. transaction={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_start_transaction_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    timeout: TsurugiFfiDuration,
    transaction_out: *mut TsurugiFfiTransactionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_start_transaction_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction_option={:?}, timeout={:?}, transaction_out={:?}",
        context,
        sql_client,
        transaction_option,
        timeout,
        transaction_out
    );

    ffi_arg_out_initialize!(transaction_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, transaction_out);

    let client = unsafe { &*sql_client };
    let transaction_option = unsafe { &*transaction_option };
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let transaction = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.start_transaction_for(transaction_option, timeout)
    );

    let transaction = Box::new(TsurugiFfiTransaction::new(transaction, runtime.clone()));

    let handle = Box::into_raw(transaction);
    unsafe {
        *transaction_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. transaction={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_start_transaction_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_start_transaction_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction_option={:?}, transaction_job_out={:?}",
        context,
        sql_client,
        transaction_option,
        transaction_job_out
    );

    ffi_arg_out_initialize!(transaction_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, transaction_job_out);

    let client = unsafe { &*sql_client };
    let transaction_option = unsafe { &*transaction_option };

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.start_transaction_async(transaction_option)
    );
    let job = TsurugiFfiJob::new(job, Box::new(TransactionJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *transaction_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. transaction_job={:?}",
        rc,
        handle
    );
    rc
}

impl_job_delegator! {
    TransactionJobDelegator,
    Transaction,
    TsurugiFfiTransaction,
    "transaction",
}

impl TransactionJobDelegator {
    fn convert(
        value: Transaction,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiTransaction> {
        Some(TsurugiFfiTransaction::new(value, runtime))
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_get_transaction_status(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    transaction_status_out: *mut TsurugiFfiTransactionStatusHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_get_transaction_status()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, transaction_status_out={:?}",
        context,
        sql_client,
        transaction,
        transaction_status_out
    );

    ffi_arg_out_initialize!(transaction_status_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, transaction_status_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };

    let runtime = client.runtime();
    let status = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.get_transaction_status(transaction)
    );

    let status = Box::new(TsurugiFfiTransactionStatus::new(status));

    let handle = Box::into_raw(status);
    unsafe {
        *transaction_status_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. transaction_status={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_get_transaction_status_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    timeout: TsurugiFfiDuration,
    transaction_status_out: *mut TsurugiFfiTransactionStatusHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_get_transaction_status_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, timeout={:?}, transaction_status_out={:?}",
        context,
        sql_client,
        transaction,
        timeout,
        transaction_status_out
    );

    ffi_arg_out_initialize!(transaction_status_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, transaction_status_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let status = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.get_transaction_status_for(transaction, timeout)
    );

    let status = Box::new(TsurugiFfiTransactionStatus::new(status));

    let handle = Box::into_raw(status);
    unsafe {
        *transaction_status_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. transaction_status={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_get_transaction_status_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    transaction_status_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_get_transaction_status_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, transaction_status_job_out={:?}",
        context,
        sql_client,
        transaction,
        transaction_status_job_out
    );

    ffi_arg_out_initialize!(transaction_status_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, transaction_status_job_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.get_transaction_status_async(transaction)
    );
    let job = TsurugiFfiJob::new(
        job,
        Box::new(TransactionStatusJobDelegator {}),
        runtime.clone(),
    );
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *transaction_status_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. transaction_status_job={:?}",
        rc,
        handle
    );
    rc
}

impl_job_delegator! {
    TransactionStatusJobDelegator,
    TransactionStatus,
    TsurugiFfiTransactionStatus,
    "transaction_status",
}

impl TransactionStatusJobDelegator {
    fn convert(
        value: TransactionStatus,
        _runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiTransactionStatus> {
        Some(TsurugiFfiTransactionStatus::new(value))
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_execute(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    sql: TsurugiFfiStringHandle,
    execute_result_out: *mut TsurugiFfiSqlExecuteResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_execute()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, sql={:?}, execute_result_out={:?}",
        context,
        sql_client,
        transaction,
        sql,
        execute_result_out
    );

    ffi_arg_out_initialize!(execute_result_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, execute_result_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, sql);

    let runtime = client.runtime();
    let execute_result = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.execute(transaction, sql)
    );

    let execute_result = Box::new(TsurugiFfiSqlExecuteResult::new(execute_result));

    let handle = Box::into_raw(execute_result);
    unsafe {
        *execute_result_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. execute_result={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_execute_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    sql: TsurugiFfiStringHandle,
    timeout: TsurugiFfiDuration,
    execute_result_out: *mut TsurugiFfiSqlExecuteResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_execute_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, sql={:?}, timeout={:?}, execute_result_out={:?}",
        context,
        sql_client,
        transaction,
        sql,
        timeout,
        execute_result_out
    );

    ffi_arg_out_initialize!(execute_result_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, execute_result_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, sql);
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let execute_result = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.execute_for(transaction, sql, timeout)
    );

    let execute_result = Box::new(TsurugiFfiSqlExecuteResult::new(execute_result));

    let handle = Box::into_raw(execute_result);
    unsafe {
        *execute_result_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. execute_result={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_execute_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    sql: TsurugiFfiStringHandle,
    execute_result_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_execute_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, sql={:?}, execute_result_job_out={:?}",
        context,
        sql_client,
        transaction,
        sql,
        execute_result_job_out
    );

    ffi_arg_out_initialize!(execute_result_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, execute_result_job_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, sql);

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.execute_async(transaction, sql)
    );
    let job = TsurugiFfiJob::new(
        job,
        Box::new(SqlExecuteResultJobDelegator {}),
        runtime.clone(),
    );
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *execute_result_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. execute_result_job={:?}",
        rc,
        handle
    );
    rc
}

impl_job_delegator! {
    SqlExecuteResultJobDelegator,
    SqlExecuteResult,
    TsurugiFfiSqlExecuteResult,
    "execute_result",
}

impl SqlExecuteResultJobDelegator {
    fn convert(
        value: SqlExecuteResult,
        _runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiSqlExecuteResult> {
        Some(TsurugiFfiSqlExecuteResult::new(value))
    }
}

macro_rules! convert_parameters {
    ($context:expr, $function_name:expr, $arg_index:expr, $parameters:expr, $parameter_size:expr) => {
        if $parameter_size > 0 {
            let src = unsafe { std::slice::from_raw_parts($parameters, $parameter_size as usize) };
            let mut dst = Vec::with_capacity(src.len());
            for &parameter in src {
                ffi_arg_require_non_null!($context, $function_name, $arg_index, parameter);
                let parameter = unsafe { &*parameter }.raw_clone();
                dst.push(parameter);
            }
            dst
        } else {
            vec![]
        }
    };
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepared_execute(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    parameters: *const TsurugiFfiSqlParameterHandle,
    parameter_size: u32,
    execute_result_out: *mut TsurugiFfiSqlExecuteResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepared_execute()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, prepared_statement={:?}, parameters={:?}, parameter_size={:?}, execute_result_out={:?}",
        context,
        sql_client,
        transaction,
        prepared_statement,
        parameters,
        parameter_size,
        execute_result_out
    );

    ffi_arg_out_initialize!(execute_result_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, prepared_statement);
    if parameter_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameters);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 6, execute_result_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let prepared_statement = unsafe { &*prepared_statement };
    let parameters: Vec<SqlParameter> =
        convert_parameters!(context, FUNCTION_NAME, 4, parameters, parameter_size);

    let runtime = client.runtime();
    let execute_result = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepared_execute(transaction, prepared_statement, parameters)
    );

    let execute_result = Box::new(TsurugiFfiSqlExecuteResult::new(execute_result));

    let handle = Box::into_raw(execute_result);
    unsafe {
        *execute_result_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. execute_result={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepared_execute_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    parameters: *const TsurugiFfiSqlParameterHandle,
    parameter_size: u32,
    timeout: TsurugiFfiDuration,
    execute_result_out: *mut TsurugiFfiSqlExecuteResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepared_execute_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, prepared_statement={:?}, parameters={:?}, parameter_size={:?}, timeout={:?}, execute_result_out={:?}",
        context,
        sql_client,
        transaction,
        prepared_statement,
        parameters,
        parameter_size,
        timeout,
        execute_result_out
    );

    ffi_arg_out_initialize!(execute_result_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, prepared_statement);
    if parameter_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameters);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 7, execute_result_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let prepared_statement = unsafe { &*prepared_statement };
    let parameters: Vec<SqlParameter> =
        convert_parameters!(context, FUNCTION_NAME, 4, parameters, parameter_size);
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let execute_result = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepared_execute_for(transaction, prepared_statement, parameters, timeout)
    );

    let execute_result = Box::new(TsurugiFfiSqlExecuteResult::new(execute_result));

    let handle = Box::into_raw(execute_result);
    unsafe {
        *execute_result_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. execute_result={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepared_execute_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    parameters: *const TsurugiFfiSqlParameterHandle,
    parameter_size: u32,
    execute_result_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepared_execute_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, prepared_statement={:?}, parameters={:?}, parameter_size={:?}, execute_result_job_out={:?}",
        context,
        sql_client,
        transaction,
        prepared_statement,
        parameters,
        parameter_size,
        execute_result_job_out
    );

    ffi_arg_out_initialize!(execute_result_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, prepared_statement);
    if parameter_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameters);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 6, execute_result_job_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let prepared_statement = unsafe { &*prepared_statement };
    let parameters: Vec<SqlParameter> =
        convert_parameters!(context, FUNCTION_NAME, 4, parameters, parameter_size);

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepared_execute_async(transaction, prepared_statement, parameters)
    );
    let job = TsurugiFfiJob::new(
        job,
        Box::new(SqlExecuteResultJobDelegator {}),
        runtime.clone(),
    );
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *execute_result_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. execute_result_job={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_query(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    sql: TsurugiFfiStringHandle,
    query_result_out: *mut TsurugiFfiSqlQueryResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_query()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, sql={:?}, query_result_out={:?}",
        context,
        sql_client,
        transaction,
        sql,
        query_result_out
    );

    ffi_arg_out_initialize!(query_result_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, query_result_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, sql);

    let runtime = client.runtime();
    let query_result = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.query(transaction, sql)
    );

    let query_result = Box::new(TsurugiFfiSqlQueryResult::new(query_result, runtime.clone()));

    let handle = Box::into_raw(query_result);
    unsafe {
        *query_result_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. query_result={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_query_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    sql: TsurugiFfiStringHandle,
    timeout: TsurugiFfiDuration,
    query_result_out: *mut TsurugiFfiSqlQueryResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_query_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, sql={:?}, timeout={:?}, query_result_out={:?}",
        context,
        sql_client,
        transaction,
        sql,
        timeout,
        query_result_out
    );

    ffi_arg_out_initialize!(query_result_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 5, query_result_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, sql);
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let query_result = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.query_for(transaction, sql, timeout)
    );

    let query_result = Box::new(TsurugiFfiSqlQueryResult::new(query_result, runtime.clone()));

    let handle = Box::into_raw(query_result);
    unsafe {
        *query_result_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. query_result={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_query_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    sql: TsurugiFfiStringHandle,
    query_result_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_query_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, sql={:?}, query_result_job_out={:?}",
        context,
        sql_client,
        transaction,
        sql,
        query_result_job_out
    );

    ffi_arg_out_initialize!(query_result_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, sql);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, query_result_job_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let sql = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, sql);

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.query_async(transaction, sql)
    );
    let job = TsurugiFfiJob::new(
        job,
        Box::new(SqlQueryResultJobDelegator {}),
        runtime.clone(),
    );
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *query_result_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. query_result_job={:?}",
        rc,
        handle
    );
    rc
}

impl_job_delegator! {
SqlQueryResultJobDelegator,
SqlQueryResult,
TsurugiFfiSqlQueryResult,
"quert_result",
}

impl SqlQueryResultJobDelegator {
    fn convert(
        value: SqlQueryResult,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiSqlQueryResult> {
        Some(TsurugiFfiSqlQueryResult::new(value, runtime))
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepared_query(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    parameters: *const TsurugiFfiSqlParameterHandle,
    parameter_size: u32,
    query_result_out: *mut TsurugiFfiSqlQueryResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepared_query()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, prepared_statement={:?}, parameters={:?}, parameter_size={:?}, query_result_out={:?}",
        context,
        sql_client,
        transaction,
        prepared_statement,
        parameters,
        parameter_size,
        query_result_out
    );

    ffi_arg_out_initialize!(query_result_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, prepared_statement);
    if parameter_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameters);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 6, query_result_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let prepared_statement = unsafe { &*prepared_statement };
    let parameters = convert_parameters!(context, FUNCTION_NAME, 4, parameters, parameter_size);

    let runtime = client.runtime();
    let query_result = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepared_query(transaction, prepared_statement, parameters)
    );

    let query_result = Box::new(TsurugiFfiSqlQueryResult::new(query_result, runtime.clone()));

    let handle = Box::into_raw(query_result);
    unsafe {
        *query_result_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. query_result={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepared_query_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    parameters: *const TsurugiFfiSqlParameterHandle,
    parameter_size: u32,
    timeout: TsurugiFfiDuration,
    query_result_out: *mut TsurugiFfiSqlQueryResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepared_query_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, prepared_statement={:?}, parameters={:?}, parameter_size={:?}, timeout={:?}, query_result_out={:?}",
        context,
        sql_client,
        transaction,
        prepared_statement,
        parameters,
        parameter_size,
        timeout,
        query_result_out
    );

    ffi_arg_out_initialize!(query_result_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, prepared_statement);
    if parameter_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameters);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 7, query_result_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let prepared_statement = unsafe { &*prepared_statement };
    let parameters = convert_parameters!(context, FUNCTION_NAME, 4, parameters, parameter_size);
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let query_result = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepared_query_for(transaction, prepared_statement, parameters, timeout)
    );

    let query_result = Box::new(TsurugiFfiSqlQueryResult::new(query_result, runtime.clone()));

    let handle = Box::into_raw(query_result);
    unsafe {
        *query_result_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. query_result={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_prepared_query_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    prepared_statement: TsurugiFfiSqlPreparedStatementHandle,
    parameters: *const TsurugiFfiSqlParameterHandle,
    parameter_size: u32,
    query_result_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_prepared_query_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, prepared_statement={:?}, parameters={:?}, parameter_size={:?}, query_result_job_out={:?}",
        context,
        sql_client,
        transaction,
        prepared_statement,
        parameters,
        parameter_size,
        query_result_job_out
    );

    ffi_arg_out_initialize!(query_result_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, prepared_statement);
    if parameter_size > 0 {
        ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, parameters);
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 6, query_result_job_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let prepared_statement = unsafe { &*prepared_statement };
    let parameters = convert_parameters!(context, FUNCTION_NAME, 4, parameters, parameter_size);

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.prepared_query_async(transaction, prepared_statement, parameters)
    );
    let job = TsurugiFfiJob::new(
        job,
        Box::new(SqlQueryResultJobDelegator {}),
        runtime.clone(),
    );
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *query_result_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. query_result_job={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_commit(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_commit()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, commit_option={:?}",
        context,
        sql_client,
        transaction,
        commit_option
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, commit_option);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let commit_option = unsafe { &*commit_option };

    let runtime = client.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.commit(transaction, commit_option)
    );

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_commit_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_commit_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, commit_option={:?}, timeout={:?}",
        context,
        sql_client,
        transaction,
        commit_option,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, commit_option);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let commit_option = unsafe { &*commit_option };
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.commit_for(transaction, commit_option, timeout)
    );

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_commit_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
    commit_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_commit_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, commit_option={:?}, commit_job_out={:?}",
        context,
        sql_client,
        transaction,
        commit_option,
        commit_job_out
    );

    ffi_arg_out_initialize!(commit_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, commit_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, commit_job_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let commit_option = unsafe { &*commit_option };

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.commit_async(transaction, commit_option)
    );
    let job = TsurugiFfiJob::new(job, Box::new(VoidJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *commit_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. commit_job={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_rollback(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_rollback()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}",
        context,
        sql_client,
        transaction
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };

    let runtime = client.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.rollback(transaction)
    );

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_rollback_for(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_rollback_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, timeout={:?}",
        context,
        sql_client,
        transaction,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.rollback_for(transaction, timeout)
    );

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_rollback_async(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    rollback_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_rollback_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, sql_client={:?}, transaction={:?}, rollback_job_out={:?}",
        context,
        sql_client,
        transaction,
        rollback_job_out
    );

    ffi_arg_out_initialize!(rollback_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, sql_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, rollback_job_out);

    let client = unsafe { &*sql_client };
    let transaction = unsafe { &*transaction };

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.rollback_async(transaction)
    );
    let job = TsurugiFfiJob::new(job, Box::new(VoidJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *rollback_job_out = handle as TsurugiFfiJobHandle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. rollback_job={:?}", rc, handle);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_dispose(sql_client: TsurugiFfiSqlClientHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_dispose()";
    trace!("{FUNCTION_NAME} start. sql_client={:?}", sql_client);

    if sql_client.is_null() {
        trace!("{FUNCTION_NAME} end. arg[sql_client] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(sql_client);
    }

    trace!("{FUNCTION_NAME} end");
}
