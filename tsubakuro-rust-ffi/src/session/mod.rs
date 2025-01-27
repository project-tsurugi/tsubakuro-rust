use std::sync::Arc;

use log::trace;
use option::TsurugiFfiConnectionOptionHandle;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc},
};

mod endpoint;
mod option;

pub(crate) struct TsurugiFfiSession {
    session: Arc<Session>,
    runtime: Arc<tokio::runtime::Runtime>,
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

impl TsurugiFfiSession {
    pub(crate) fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
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
    let session = Box::new(TsurugiFfiSession {
        session,
        runtime: Arc::new(runtime),
    });

    let handle = Box::into_raw(session);
    unsafe {
        *session_out = handle;
    }

    trace!("{FUNCTION_NAME} end. session={:?}", handle);
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
