//! connection option.

use std::{ffi::CString, ops::Deref, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_clear, cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null,
    rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    session::credential::TsurugiFfiCredentialHandle,
    TsurugiFfiDuration, TsurugiFfiStringHandle,
};

use super::endpoint::TsurugiFfiEndpointHandle;

#[derive(Debug)]
pub(crate) struct TsurugiFfiConnectionOption {
    connection_option: ConnectionOption,
    endpoint_url: Option<CString>,
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

/// Connection option.
pub type TsurugiFfiConnectionOptionHandle = *mut TsurugiFfiConnectionOption;

/// ConnectionOption: Creates a new instance.
///
/// See [`ConnectionOption::new`].
///
/// # Returns
/// - `connection_option_out` - connection option. To dispose, call [`tsurugi_ffi_connection_option_dispose`].
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
        endpoint_url: None,
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

/// ConnectionOption: Set endpoint.
///
/// See [`ConnectionOption::set_endpoint`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `endpoint` - endpoint.
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

/// ConnectionOption: Set endpoint.
///
/// See [`ConnectionOption::set_endpoint_url`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `endpoint` - endpoint url. (e.g. `tcp://localhost:12345`)
#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_endpoint_url(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    endpoint_url: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_endpoint_url()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, endpoint_url={:?}",
        context,
        connection_option,
        endpoint_url
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, endpoint_url);

    let endpoint = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, endpoint_url);

    let connection_option = unsafe { &mut *connection_option };

    match connection_option.set_endpoint_url(endpoint) {
        Ok(_) => {}
        Err(e) => return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "endpoint", e.message()),
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// ConnectionOption: Get endpoint.
///
/// See [`ConnectionOption::endpoint`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Returns
/// - `endpoint_url_out` - endpoint url.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_endpoint_url(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    endpoint_url_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_endpoint_url()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, endpoint_url_out={:?}",
        context,
        connection_option,
        endpoint_url_out
    );

    ffi_arg_out_initialize!(endpoint_url_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, endpoint_url_out);

    let connection_option = unsafe { &mut *connection_option };

    match connection_option.endpoint() {
        Some(endpoint) => {
            let endpoint = endpoint.to_string();
            cchar_field_set!(context, connection_option.endpoint_url, endpoint);
        }
        None => cchar_field_clear!(connection_option.endpoint_url),
    }

    let ptr = cstring_to_cchar!(connection_option.endpoint_url);
    unsafe {
        *endpoint_url_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (endpoint_url={:?})", rc, ptr);
    rc
}

/// ConnectionOption: Set credential.
///
/// See [`ConnectionOption::set_credential`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `credential` - credential.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_credential(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    credential: TsurugiFfiCredentialHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_credential()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, credential={:?}",
        context,
        connection_option,
        credential
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, credential);

    let credential = unsafe { &*credential };

    let connection_option = unsafe { &mut *connection_option };
    connection_option.set_credential(credential.deref().clone());

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// ConnectionOption: Set application name.
///
/// See [`ConnectionOption::set_application_name`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `application_name` - application name.
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

/// ConnectionOption: Get application name.
///
/// See [`ConnectionOption::application_name`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Returns
/// - `application_name_out` - application name.
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

/// ConnectionOption: Set session label.
///
/// See [`ConnectionOption::set_session_label`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `session_label` - session label.
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

/// ConnectionOption: Get session label.
///
/// See [`ConnectionOption::session_label`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Returns
/// - `session_label_out` - session label.
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

/// ConnectionOption: Set keep alive interval.
///
/// See [`ConnectionOption::set_keep_alive`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `keep_alive` - keep alive interval \[nanosecond\].
///   Do not keep alive when `keep_alive` is 0.
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

/// ConnectionOption: Get keep alive interval.
///
/// See [`ConnectionOption::keep_alive`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Returns
/// - `keep_alive_out` - keep alive interval \[nanosecond\].
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

/// ConnectionOption: Adds a path mapping entry for both sending and receiving BLOB/CLOB.
///
/// See [`ConnectionOption::add_large_object_path_mapping`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `client_path` - the client path, must be a directory
/// - `server_path` - the server path, must be a directory
#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_add_large_object_path_mapping(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    client_path: TsurugiFfiStringHandle,
    server_path: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_add_large_object_path_mapping()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, client_path={:?}, server_path={:?}",
        context,
        connection_option,
        client_path,
        server_path
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, client_path);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, server_path);

    let connection_option = unsafe { &mut *connection_option };
    let client_path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, client_path);
    let server_path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, server_path);

    connection_option.add_large_object_path_mapping(client_path, server_path);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// ConnectionOption: Adds a path mapping entry for sending BLOB/CLOB.
///
/// See [`ConnectionOption::add_large_object_path_mapping_on_send`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `client_path` - the client path to be transformed, must be a directory
/// - `server_path` - the server path, must be a directory
#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_add_large_object_path_mapping_on_send(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    client_path: TsurugiFfiStringHandle,
    server_path: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str =
        "tsurugi_ffi_connection_option_add_large_object_path_mapping_on_send()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, client_path={:?}, server_path={:?}",
        context,
        connection_option,
        client_path,
        server_path
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, client_path);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, server_path);

    let connection_option = unsafe { &mut *connection_option };
    let client_path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, client_path);
    let server_path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, server_path);

    connection_option.add_large_object_path_mapping_on_send(client_path, server_path);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// ConnectionOption: Adds a path mapping entry for receiving BLOB/CLOB.
///
/// See [`ConnectionOption::add_large_object_path_mapping_on_recv`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `server_path` - the target server path to be transformed, must be a directory
/// - `client_path` - the target client path, must be a directory
#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_add_large_object_path_mapping_on_recv(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    server_path: TsurugiFfiStringHandle,
    client_path: TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str =
        "tsurugi_ffi_connection_option_add_large_object_path_mapping_on_recv()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, connection_option={:?}, server_path={:?}, client_path={:?}",
        context,
        connection_option,
        server_path,
        client_path
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, connection_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, server_path);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, client_path);

    let connection_option = unsafe { &mut *connection_option };
    let server_path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, server_path);
    let client_path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 3, client_path);

    connection_option.add_large_object_path_mapping_on_recv(server_path, client_path);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// ConnectionOption: Set default timeout.
///
/// See [`ConnectionOption::set_default_timeout`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `default_timeout` - default timeout \[nanosecond\].
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

/// ConnectionOption: Get default timeout.
///
/// See [`ConnectionOption::default_timeout`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Returns
/// - `default_timeout_out` - default timeout \[nanosecond\].
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

/// ConnectionOption: Set communication send timeout.
///
/// See [`ConnectionOption::set_send_timeout`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `send_timeout` - send timeout \[nanosecond\].
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

/// ConnectionOption: Get communication send timeout.
///
/// See [`ConnectionOption::send_timeout`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Returns
/// - `send_timeout_out` - send timeout \[nanosecond\].
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

/// ConnectionOption: Set communication recv timeout.
///
/// See [`ConnectionOption::set_recv_timeout`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Parameters
/// - `recv_timeout` - recv timeout \[nanosecond\].
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

/// ConnectionOption: Get communication recv timeout.
///
/// See [`ConnectionOption::recv_timeout`].
///
/// # Receiver
/// - `connection_option` - Connection option.
///
/// # Returns
/// - `recv_timeout_out` - recv timeout \[nanosecond\].
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

/// ConnectionOption: Dispose.
///
/// # Receiver
/// - `connection_option` - Connection option.
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
