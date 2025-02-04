use std::{sync::Arc, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core_async, impl_job_delegator,
    job::{TsurugiFfiJob, TsurugiFfiJobHandle, VoidJobDelegator},
    rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    service::sql::{TsurugiFfiSqlClient, TsurugiFfiSqlClientHandle},
    TsurugiFfiDuration,
};

use super::{option::TsurugiFfiConnectionOptionHandle, r#type::TsurugiFfiShutdownType};

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
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, session_out={:?}",
        context,
        connection_option,
        session_out
    );

    ffi_arg_out_initialize!(session_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, session_out);

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
pub extern "C" fn tsurugi_ffi_session_connect_for(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    timeout: TsurugiFfiDuration,
    session_out: *mut TsurugiFfiSessionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_connect_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, timeout={:?}, session_out={:?}",
        context,
        connection_option,
        timeout,
        session_out
    );

    ffi_arg_out_initialize!(session_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, session_out);

    let connection_option = unsafe { &*connection_option };
    let timeout = Duration::from_nanos(timeout);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let session = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        Session::connect_for(connection_option, timeout)
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
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, session_job_out={:?}",
        context,
        connection_option,
        session_job_out
    );

    ffi_arg_out_initialize!(session_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, session_job_out);

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
    let job = TsurugiFfiJob::new(job, Box::new(ConnectJobDelegator {}), runtime);
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
    fn convert(
        value: Arc<Session>,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiSession> {
        Some(TsurugiFfiSession::new(value, runtime))
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_set_default_timeout(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_set_default_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, timeout={:?}",
        context,
        session,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);

    let session = unsafe { &mut *session };
    let timeout = Duration::from_nanos(timeout);

    session.set_default_timeout(timeout);

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_get_default_timeout(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    default_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_get_default_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, default_timeout_out={:?}",
        context,
        session,
        default_timeout_out
    );

    ffi_arg_out_initialize!(default_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, default_timeout_out);

    let session = unsafe { &mut *session };

    let timeout = session.default_timeout();

    unsafe {
        *default_timeout_out = timeout.as_nanos() as TsurugiFfiDuration;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_make_sql_client(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    sql_client_out: *mut TsurugiFfiSqlClientHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_make_sql_client()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, sql_client_out={:?}",
        context,
        session,
        sql_client_out
    );

    ffi_arg_out_initialize!(sql_client_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, sql_client_out);

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
pub extern "C" fn tsurugi_ffi_session_update_expiration_time(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    expiration_time_exists: bool,
    expiration_time: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_update_expiration_time()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, ,expiration_time_exists={:?}, expiration_time={:?}",
        context,
        session,
        expiration_time_exists,
        expiration_time
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);

    let session = unsafe { &*session };
    let expiration_time = if expiration_time_exists {
        Some(Duration::from_nanos(expiration_time))
    } else {
        None
    };

    let runtime = session.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        session.update_expiration_time(expiration_time)
    );

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_update_expiration_time_for(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    expiration_time_exists: bool,
    expiration_time: TsurugiFfiDuration,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_update_expiration_time_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, ,expiration_time_exists={:?}, expiration_time={:?}, timeout={:?}",
        context,
        session,
        expiration_time_exists,
        expiration_time,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);

    let session = unsafe { &*session };
    let expiration_time = if expiration_time_exists {
        Some(Duration::from_nanos(expiration_time))
    } else {
        None
    };
    let timeout = Duration::from_nanos(timeout);

    let runtime = session.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        session.update_expiration_time_for(expiration_time, timeout)
    );

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_update_expiration_time_async(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    expiration_time_exists: bool,
    expiration_time: TsurugiFfiDuration,
    update_expiration_time_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_update_expiration_time_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, ,expiration_time_exists={:?}, expiration_time={:?}, update_expiration_time_job_out={:?}",
        context,
        session,
        expiration_time_exists,
        expiration_time,
        update_expiration_time_job_out
    );

    ffi_arg_out_initialize!(update_expiration_time_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 4, update_expiration_time_job_out);

    let session = unsafe { &*session };
    let expiration_time = if expiration_time_exists {
        Some(Duration::from_nanos(expiration_time))
    } else {
        None
    };

    let runtime = session.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        session.update_expiration_time_async(expiration_time)
    );
    let job = TsurugiFfiJob::new(job, Box::new(VoidJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *update_expiration_time_job_out = handle as TsurugiFfiJobHandle;
    }

    trace!(
        "{FUNCTION_NAME} end. update_expiration_time_job={:?}",
        handle
    );
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_shutdown(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    shutdown_type: TsurugiFfiShutdownType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_shutdown()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, shutdown_type={:?}",
        context,
        session,
        shutdown_type as i32
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    if !TsurugiFfiShutdownType::is_valid(shutdown_type as i32) {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "shutdown_type", "is invalid");
    }

    let session = unsafe { &*session };

    let runtime = session.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        session.shutdown(shutdown_type.into())
    );

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_shutdown_for(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    shutdown_type: TsurugiFfiShutdownType,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_shutdown_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, shutdown_type={:?}, timeout={:?}",
        context,
        session,
        shutdown_type as i32,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    if !TsurugiFfiShutdownType::is_valid(shutdown_type as i32) {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "shutdown_type", "is invalid");
    }

    let session = unsafe { &*session };
    let timeout = Duration::from_nanos(timeout);

    let runtime = session.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        session.shutdown_for(shutdown_type.into(), timeout)
    );

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_shutdown_async(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    shutdown_type: TsurugiFfiShutdownType,
    shutdown_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_shutdown_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, shutdown_type={:?}, shutdown_job_out={:?}",
        context,
        session,
        shutdown_type as i32,
        shutdown_job_out
    );

    ffi_arg_out_initialize!(shutdown_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    if !TsurugiFfiShutdownType::is_valid(shutdown_type as i32) {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "shutdown_type", "is invalid");
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, shutdown_job_out);

    let session = unsafe { &*session };

    let runtime = session.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        session.shutdown_async(shutdown_type.into())
    );
    let job = TsurugiFfiJob::new(job, Box::new(VoidJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *shutdown_job_out = handle as TsurugiFfiJobHandle;
    }

    trace!("{FUNCTION_NAME} end. shutdown_job={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_is_shutdowned(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    is_shutdowned_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_is_shutdowned()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, is_shutdowned_out={:?}",
        context,
        session,
        is_shutdowned_out
    );

    ffi_arg_out_initialize!(is_shutdowned_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_shutdowned_out);

    let session = unsafe { &*session };

    let is_shutdowned = session.is_shutdowned();

    unsafe {
        *is_shutdowned_out = is_shutdowned;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_close(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_close()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}",
        context,
        session
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);

    let session = unsafe { &*session };

    let runtime = session.runtime();
    ffi_exec_core_async!(context, FUNCTION_NAME, runtime, session.close());

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_session_is_closed(
    context: TsurugiFfiContextHandle,
    session: TsurugiFfiSessionHandle,
    is_closed_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_session_is_closed()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, session={:?}, is_closed_out={:?}",
        context,
        session,
        is_closed_out
    );

    ffi_arg_out_initialize!(is_closed_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, session);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_closed_out);

    let session = unsafe { &*session };

    let is_closed = session.is_closed();

    unsafe {
        *is_closed_out = is_closed;
    }

    trace!("{FUNCTION_NAME} end");
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
