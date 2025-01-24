use std::{ffi::c_char, ops::Deref};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_clear, cchar_field_dispose, cchar_field_set,
    context::TsurugiFfiContextHandle,
    ffi_arg_cchar_to_str, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
};

use super::endpoint::TsurugiFfiEndpointHandle;

#[derive(Debug)]
pub(crate) struct TsurugiFfiConnectionOption {
    connection_option: ConnectionOption,
    endpoint_str: *mut c_char,
    application_name: *mut c_char,
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
    trace!("{FUNCTION_NAME} start");

    if connection_option_out.is_null() {
        return rc_ffi_arg_error!(
            context,
            FUNCTION_NAME,
            1,
            "connection_option_out",
            "is null"
        );
    }

    let connection_option = Box::new(TsurugiFfiConnectionOption {
        connection_option: ConnectionOption::new(),
        endpoint_str: std::ptr::null_mut(),
        application_name: std::ptr::null_mut(),
    });

    let handle = Box::into_raw(connection_option);
    unsafe {
        *connection_option_out = handle;
    }

    trace!("{FUNCTION_NAME} end. connection_option={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_endpoint(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    endpoint: TsurugiFfiEndpointHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_endpoint()";
    trace!(
        "{FUNCTION_NAME} start. connection_option={:?}",
        connection_option
    );

    if connection_option.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "connection_option", "is null");
    }
    if endpoint.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "endpoint", "is null");
    }

    let endpoint = unsafe { &*endpoint };

    let connection_option = unsafe { &mut *connection_option };
    connection_option.set_endpoint(endpoint.deref().clone());

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_endpoint_url(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    endpoint: *const c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_endpoint()";
    trace!(
        "{FUNCTION_NAME} start. connection_option={:?}",
        connection_option
    );

    if connection_option.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "connection_option", "is null");
    }
    if endpoint.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "endpoint", "is null");
    }

    let endpoint = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, endpoint);

    let connection_option = unsafe { &mut *connection_option };
    match connection_option.set_endpoint_url(endpoint) {
        Ok(_) => {}
        Err(e) => return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "endpoint", e.message()),
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_endpoint(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    endpoint_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_endpoint()";
    trace!(
        "{FUNCTION_NAME} start. connection_option={:?}",
        connection_option
    );

    if connection_option.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "connection_option", "is null");
    }
    if endpoint_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "endpoint_out", "is null");
    }

    let connection_option = unsafe { &mut *connection_option };

    match connection_option.endpoint() {
        Some(endpoint) => unsafe {
            let endpoint = endpoint.to_string();
            cchar_field_set!(context, connection_option.endpoint_str, endpoint);
        },
        None => unsafe {
            cchar_field_clear!(connection_option.endpoint_str);
        },
    }
    unsafe {
        *endpoint_out = connection_option.endpoint_str;
    }

    trace!(
        "{FUNCTION_NAME} end. endpoint={:?}",
        connection_option.endpoint_str
    );
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_set_application_name(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    application_name: *const c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_set_application_name()";
    trace!(
        "{FUNCTION_NAME} start. connection_option={:?}",
        connection_option
    );

    if connection_option.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "connection_option", "is null");
    }
    if application_name.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "application_name", "is null");
    }

    let application_name = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, application_name);

    let connection_option = unsafe { &mut *connection_option };
    connection_option.set_application_name(application_name);

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_connection_option_get_application_name(
    context: TsurugiFfiContextHandle,
    connection_option: TsurugiFfiConnectionOptionHandle,
    application_name_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_connection_option_get_application_name()";
    trace!(
        "{FUNCTION_NAME} start. connection_option={:?}",
        connection_option
    );

    if connection_option.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "connection_option", "is null");
    }
    if application_name_out.is_null() {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "application_name_out", "is null");
    }

    let connection_option = unsafe { &mut *connection_option };

    match connection_option.application_name() {
        Some(application_name) => unsafe {
            let application_name = application_name.to_string();
            cchar_field_set!(
                context,
                connection_option.application_name,
                application_name
            );
        },
        None => unsafe {
            cchar_field_clear!(connection_option.application_name);
        },
    }
    unsafe {
        *application_name_out = connection_option.application_name;
    }

    trace!(
        "{FUNCTION_NAME} end. application_name={:?}",
        connection_option.application_name
    );
    rc_ok(context)
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
        let connection_option = Box::from_raw(connection_option);

        cchar_field_dispose!(connection_option.endpoint_str);
        cchar_field_dispose!(connection_option.application_name);
    }

    trace!("{FUNCTION_NAME} end");
}
