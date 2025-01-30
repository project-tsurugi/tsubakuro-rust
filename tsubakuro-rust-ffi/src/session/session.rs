use std::sync::Arc;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_require_non_null, ffi_exec_core_async, impl_job_delegator,
    job::{TsurugiFfiJob, TsurugiFfiJobHandle},
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::{TsurugiFfiSqlClient, TsurugiFfiSqlClientHandle},
};

use super::option::TsurugiFfiConnectionOptionHandle;

pub(crate) struct TsurugiFfiSession {
    session: Arc<Session>,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl TsurugiFfiSession {
    fn new(session: Arc<Session>, runtime: Arc<tokio::runtime::Runtime>) -> TsurugiFfiSession {
        TsurugiFfiSession { session, runtime }
    }

    pub(crate) fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

impl std::ops::Deref for TsurugiFfiSession {
    type Target = Arc<Session>;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

impl std::ops::DerefMut for TsurugiFfiSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.session
    }
}

pub type TsurugiFfiSessionHandle = *mut TsurugiFfiSession;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_connect(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    session_out: *mut TsurugiFfiSessionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_connect()";
    trace!("{FUNCTION_NAME} start");

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, session_out);
    unsafe {
        *session_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);

    let connection_option = unsafe { &*connection_option };

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let session = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        Session::connect(connection_option)
    );
    let session = Box::new(TsurugiFfiSession::new(session, Arc::new(runtime)));

    let handle = Box::into_raw(session);
    unsafe {
        *session_out = handle;
    }

    trace!("{FUNCTION_NAME} end. session={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_connect_async(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    session_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_connect_async()";
    trace!("{FUNCTION_NAME} start");

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, session_job_out);
    unsafe {
        *session_job_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);

    let connection_option = unsafe { &*connection_option };

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        Session::connect_async(connection_option)
    );
    let runtime = Arc::new(runtime);
    let job = TsurugiFfiJob::new(job, Box::new(ConnectJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *session_job_out = handle as TsurugiFfiJobHandle;
    }

    trace!("{FUNCTION_NAME} end. session_job={:?}", handle);
    rc_ok(context)
}

impl_job_delegator! {
    ConnectJobDelegator,
    Arc<Session>,
    TsurugiFfiSession,
    "session",
}

impl ConnectJobDelegator {
    fn convert(value: Arc<Session>, runtime: Arc<tokio::runtime::Runtime>) -> TsurugiFfiSession {
        TsurugiFfiSession::new(value, runtime)
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_make_sql_client(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    sql_client_out: *mut TsurugiFfiSqlClientHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_make_sql_client()";
    trace!("{FUNCTION_NAME} start. session={:?}", session);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, sql_client_out);
    unsafe {
        *sql_client_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);

    let session = unsafe { &*session };
    let sql_client: SqlClient = session.make_client();
    let client = Box::new(TsurugiFfiSqlClient::new(
        sql_client,
        session.runtime().clone(),
    ));

    let handle = Box::into_raw(client);
    unsafe {
        *sql_client_out = handle;
    }

    trace!("{FUNCTION_NAME} end. sql_client={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_dispose(session: TsurugiFfiSessionHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_dispose()";
    trace!("{FUNCTION_NAME} start. session={:?}", session);

    if session.is_null() {
        trace!("{FUNCTION_NAME} end. arg[session] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(session);
    }

    trace!("{FUNCTION_NAME} end");
}
