use std::{ffi::c_char, sync::Arc};

use log::trace;
use table_list::{TsurugiFfiTableList, TsurugiFfiTableListHandle};
use table_metadata::{TsurugiFfiTableMetadata, TsurugiFfiTableMetadataHandle};
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_cchar_to_str, ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc},
    session::TsurugiFfiSessionHandle,
    transaction::{
        option::TsurugiFfiTransactionOptionHandle, TsurugiFfiTransaction,
        TsurugiFfiTransactionHandle,
    },
};

mod atom_type;
mod column;
mod table_list;
mod table_metadata;

pub(crate) struct TsurugiFfiSqlClient {
    sql_client: SqlClient,
    runtime: Arc<tokio::runtime::Runtime>,
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

impl TsurugiFfiSqlClient {
    pub(crate) fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

pub type TsurugiFfiSqlClientHandle = *mut TsurugiFfiSqlClient;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_make_sql_client(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    sql_client_out: *mut TsurugiFfiSqlClientHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_make_sql_client()";
    trace!("{FUNCTION_NAME} start");

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, sql_client_out);

    let session = unsafe { &*session };
    let sql_client: SqlClient = session.make_client();
    let client = Box::new(TsurugiFfiSqlClient {
        sql_client,
        runtime: session.runtime().clone(),
    });

    let handle = Box::into_raw(client);
    unsafe {
        *sql_client_out = handle;
    }

    trace!("{FUNCTION_NAME} end. sql_client={:?}", handle);
    rc_ok(context)
}

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
