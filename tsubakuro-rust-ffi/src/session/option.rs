use std::{ffi::CString, ops::Deref, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_clear, cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null,
    rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiDuration, TsurugiFfiStringHandle,
};

use super::endpoint::TsurugiFfiEndpointHandle;

#[derive(Debug)]
pub(crate) struct TsurugiFfiConnectionOption {
    connection_option: ConnectionOption,
    endpoint: Option<CString>,
    application_name: Option<CString>,
    session_label: Option<CString>,
}

impl std::ops::Deref for TsurugiFfiConnectionOption {
    type Target = ConnectionOption;

    fn deref(&self) -> &Self::Target {
        &self.connection_option
    }
}

impl std::ops::DerefMut for TsurugiFfiConnectionOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.connection_option
    }
}

pub type TsurugiFfiConnectionOptionHandle = *mut TsurugiFfiConnectionOption;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_create(
    context: TsurugiFfiContextHandle,
    connection_option_out: *mut TsurugiFfiConnectionOptionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_create()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option_out={:?}",
        context,
        connection_option_out
    );

    ffi_arg_out_initialize!(connection_option_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option_out);

    let connection_option = Box::new(TsurugiFfiConnectionOption {
        connection_option: ConnectionOption::new(),
        endpoint: None,
        application_name: None,
        session_label: None,
    });

    let handle = Box::into_raw(connection_option);
    unsafe {
        *connection_option_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. connection_option={:?}",
        rc,
        handle
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_endpoint(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    endpoint: TsurugiFfiEndpointHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_endpoint()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, endpoint={:?}",
        context,
        connection_option,
        endpoint
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, endpoint);

    let endpoint = unsafe { &*endpoint };

    let connection_option = unsafe { &mut *connection_option };
    connection_option.set_endpoint(endpoint.deref().clone());

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_endpoint_url(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    endpoint: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_endpoint_url()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, endpoint={:?}",
        context,
        connection_option,
        endpoint
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, endpoint);

    let endpoint = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, endpoint);

    let connection_option = unsafe { &mut *connection_option };

    match connection_option.set_endpoint_url(endpoint) {
        Ok(_) => {}
        Err(e) => return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "endpoint", e.message()),
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_endpoint(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    endpoint_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_endpoint()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, endpoint_out={:?}",
        context,
        connection_option,
        endpoint_out
    );

    ffi_arg_out_initialize!(endpoint_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, endpoint_out);

    let connection_option = unsafe { &mut *connection_option };

    match connection_option.endpoint() {
        Some(endpoint) => {
            let endpoint = endpoint.to_string();
            cchar_field_set!(context, connection_option.endpoint, endpoint);
        }
        None => cchar_field_clear!(connection_option.endpoint),
    }

    let ptr = cstring_to_cchar!(connection_option.endpoint);
    unsafe {
        *endpoint_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (endpoint={:?})", rc, ptr);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_application_name(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    application_name: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_application_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, application_name={:?}",
        context,
        connection_option,
        application_name
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, application_name);

    let application_name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, application_name);

    let connection_option = unsafe { &mut *connection_option };

    connection_option.set_application_name(application_name);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_application_name(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    application_name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_application_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, application_name_out={:?}",
        context,
        connection_option,
        application_name_out
    );

    ffi_arg_out_initialize!(application_name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, application_name_out);

    let connection_option = unsafe { &mut *connection_option };

    match connection_option.application_name() {
        Some(application_name) => {
            let application_name = application_name.to_string();
            cchar_field_set!(
                context,
                connection_option.application_name,
                application_name
            );
        }
        None => cchar_field_clear!(connection_option.application_name),
    }

    let ptr = cstring_to_cchar!(connection_option.application_name);
    unsafe {
        *application_name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (application_name={:?})",
        rc,
        ptr
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_session_label(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    session_label: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_session_label()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, session_label={:?}",
        context,
        connection_option,
        session_label
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, session_label);

    let label = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, session_label);

    let connection_option = unsafe { &mut *connection_option };

    connection_option.set_session_label(label);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_session_label(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    session_label_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_session_label()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, session_label_out={:?}",
        context,
        connection_option,
        session_label_out
    );

    ffi_arg_out_initialize!(session_label_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, session_label_out);

    let connection_option = unsafe { &mut *connection_option };

    match connection_option.session_label() {
        Some(label) => {
            let label = label.to_string();
            cchar_field_set!(context, connection_option.session_label, label);
        }
        None => cchar_field_clear!(connection_option.session_label),
    }

    let ptr = cstring_to_cchar!(connection_option.session_label);
    unsafe {
        *session_label_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (session_label={:?})", rc, ptr);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_keep_alive(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    keep_alive: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_keep_alive()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, keep_alive={:?}",
        context,
        connection_option,
        keep_alive
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);

    let connection_option = unsafe { &mut *connection_option };
    let keep_alive = Duration::from_nanos(keep_alive);

    connection_option.set_keep_alive(keep_alive);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_keep_alive(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    keep_alive_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_keep_alive()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, keep_alive_out={:?}",
        context,
        connection_option,
        keep_alive_out
    );

    ffi_arg_out_initialize!(keep_alive_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, keep_alive_out);

    let connection_option = unsafe { &*connection_option };

    let keep_alive = connection_option.keep_alive();

    let value = keep_alive.as_nanos() as TsurugiFfiDuration;
    unsafe {
        *keep_alive_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (keep_alive={:?})", rc, value);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_default_timeout(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    default_timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_default_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, default_timeout={:?}",
        context,
        connection_option,
        default_timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);

    let connection_option = unsafe { &mut *connection_option };
    let default_timeout = Duration::from_nanos(default_timeout);

    connection_option.set_default_timeout(default_timeout);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_default_timeout(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    default_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_default_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, default_timeout_out={:?}",
        context,
        connection_option,
        default_timeout_out
    );

    ffi_arg_out_initialize!(default_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, default_timeout_out);

    let connection_option = unsafe { &mut *connection_option };

    let default_timeout = connection_option.default_timeout();

    let value = default_timeout.as_nanos() as TsurugiFfiDuration;
    unsafe {
        *default_timeout_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (default_timeout={:?})",
        rc,
        value
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_send_timeout(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    send_timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_send_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, send_timeout={:?}",
        context,
        connection_option,
        send_timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);

    let connection_option = unsafe { &mut *connection_option };
    let send_timeout = Duration::from_nanos(send_timeout);

    connection_option.set_send_timeout(send_timeout);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_send_timeout(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    send_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_send_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, send_timeout_out={:?}",
        context,
        connection_option,
        send_timeout_out
    );

    ffi_arg_out_initialize!(send_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, send_timeout_out);

    let connection_option = unsafe { &mut *connection_option };

    let send_timeout = connection_option.send_timeout();

    let value = send_timeout.as_nanos() as TsurugiFfiDuration;
    unsafe {
        *send_timeout_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (send_timeout={:?})",
        rc,
        value
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_recv_timeout(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    recv_timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_recv_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, recv_timeout={:?}",
        context,
        connection_option,
        recv_timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);

    let connection_option = unsafe { &mut *connection_option };
    let recv_timeout = Duration::from_nanos(recv_timeout);

    connection_option.set_recv_timeout(recv_timeout);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_recv_timeout(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    recv_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_recv_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, recv_timeout_out={:?}",
        context,
        connection_option,
        recv_timeout_out
    );

    ffi_arg_out_initialize!(recv_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, recv_timeout_out);

    let connection_option = unsafe { &mut *connection_option };

    let recv_timeout = connection_option.recv_timeout();

    let value = recv_timeout.as_nanos() as TsurugiFfiDuration;
    unsafe {
        *recv_timeout_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (recv_timeout={:?})",
        rc,
        value
    );
    rc
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_dispose(
    connection_option: TsurugiFfiConnectionOptionHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_dispose()";
    trace!(
        "{FUNCTION_NAME} start. connection_option={:?}",
        connection_option
    );

    if connection_option.is_null() {
        trace!("{FUNCTION_NAME} end. arg[connection_option] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(connection_option);
    }

    trace!("{FUNCTION_NAME} end");
}
