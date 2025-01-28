use std::{ffi::c_char, sync::Arc};

use execute_result::{TsurugiFfiSqlExecuteResult, TsurugiFfiSqlExecuteResultHandle};
use log::trace;
use query_result::{TsurugiFfiSqlQueryResult, TsurugiFfiSqlQueryResultHandle};
use table_list::{TsurugiFfiTableList, TsurugiFfiTableListHandle};
use table_metadata::{TsurugiFfiTableMetadata, TsurugiFfiTableMetadataHandle};
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_cchar_to_str, ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc},
    transaction::{
        commit_option::TsurugiFfiCommitOptionHandle, option::TsurugiFfiTransactionOptionHandle,
        TsurugiFfiTransaction, TsurugiFfiTransactionHandle,
    },
};

mod atom_type;
mod column;
mod execute_result;
mod query_result;
mod query_result_metadata;
mod table_list;
mod table_metadata;

pub(crate) struct TsurugiFfiSqlClient {
    sql_client: SqlClient,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl TsurugiFfiSqlClient {
    pub(crate) fn new(
        sql_client: SqlClient,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiSqlClient {
        TsurugiFfiSqlClient {
            sql_client,
            runtime,
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
pub extern "C" fn tsurugi_ffi_sql_client_list_tables(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    table_list_out: *mut TsurugiFfiTableListHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_list_tables()";
    trace!("{FUNCTION_NAME} start");

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

    trace!("{FUNCTION_NAME} end. table_list={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_get_table_metadata(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    table_name: *const c_char,
    table_metadata_out: *mut TsurugiFfiTableMetadataHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_get_table_metadata()";
    trace!("{FUNCTION_NAME} start");

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

    trace!("{FUNCTION_NAME} end. table_metadata={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_start_transaction(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction_option: TsurugiFfiTransactionOptionHandle,
    transaction_out: *mut TsurugiFfiTransactionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_start_transaction()";
    trace!("{FUNCTION_NAME} start");

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

    trace!("{FUNCTION_NAME} end. transaction={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_execute(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    sql: *const c_char,
    execute_result_out: *mut TsurugiFfiSqlExecuteResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_execute()";
    trace!("{FUNCTION_NAME} start");

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

    trace!("{FUNCTION_NAME} end. execute_result={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_query(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    sql: *const c_char,
    query_result_out: *mut TsurugiFfiSqlQueryResultHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_query()";
    trace!("{FUNCTION_NAME} start");

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

    trace!("{FUNCTION_NAME} end. query_result={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_commit(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_commit()";
    trace!("{FUNCTION_NAME} start");

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

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_sql_client_rollback(
    context: TsurugiFfiContextHandle,
    sql_client: TsurugiFfiSqlClientHandle,
    transaction: TsurugiFfiTransactionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_sql_client_rollback()";
    trace!("{FUNCTION_NAME} start");

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

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
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
