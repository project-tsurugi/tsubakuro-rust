use std::sync::Arc;

use log::trace;
use table_list::{TsurugiFfiTableList, TsurugiFfiTableListHandle};
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    rc_core_error, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    session::TsurugiFfiSessionHandle,
};

mod table_list;

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

    if session.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "session", "is null");
    }
    if sql_client_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "sql_client_out", "is null");
    }

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

    if sql_client.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "session", "is null");
    }
    if table_list_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "sql_client_out", "is null");
    }

    let client = unsafe { &*sql_client };

    let runtime = client.runtime();
    let result = runtime.block_on(client.list_tables());
    let table_list = match result {
        Ok(session) => session,
        Err(e) => return rc_core_error!(context, FUNCTION_NAME, e),
    };

    let table_list = Box::new(TsurugiFfiTableList::new(table_list));

    let handle = Box::into_raw(table_list);
    unsafe {
        *table_list_out = handle;
    }

    trace!("{FUNCTION_NAME} end. table_list={:?}", handle);
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
