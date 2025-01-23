use std::sync::Arc;

use log::trace;
use option::TsurugiFfiConnectionOptionHandle;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    rc_core_error, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
};

mod endpoint;
mod option;

pub(crate) struct TsurugiFfiSession {
    session: Arc<Session>,
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

    if connection_option.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "connection_option", "is null");
    }
    if session_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "session_out", "is null");
    }

    let connection_option = unsafe { &*connection_option };

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(Session::connect(connection_option));
    let session = match result {
        Ok(session) => session,
        Err(e) => return rc_core_error!(context, FUNCTION_NAME, e),
    };
    let session = Box::new(TsurugiFfiSession { session });

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
