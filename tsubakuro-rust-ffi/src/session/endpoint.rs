use std::ffi::{c_char, CStr};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    return_code::{rc_ffi_arg_error, TsurugiFfiRc, TSURUGI_FFI_RC_OK},
};

pub(crate) struct TsurugiFfiEndpoint {
    endpoint: Endpoint,
}

impl TsurugiFfiEndpoint {
    pub(crate) fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }
}

pub type TsurugiFfiEndpointHandle = *mut TsurugiFfiEndpoint;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_endpoint_parse(
    context: TsurugiFfiContextHandle,
    endpoint: *const c_char,
    endpoint_out: *mut TsurugiFfiEndpointHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_endpoint_parse()";
    trace!("{FUNCTION_NAME} start");

    if endpoint.is_null() {
        return rc_ffi_arg_error(context, FUNCTION_NAME, 1, "endpoint", "is null");
    }
    if endpoint_out.is_null() {
        return rc_ffi_arg_error(context, FUNCTION_NAME, 2, "endpoint_out", "is null");
    }

    let endpoint = unsafe { CStr::from_ptr(endpoint) };
    let endpoint = match endpoint.to_str() {
        Ok(value) => value,
        Err(e) => return rc_ffi_arg_error(context, FUNCTION_NAME, 1, "endpoint", &e.to_string()),
    };
    let endpoint = match Endpoint::parse(endpoint) {
        Ok(value) => value,
        Err(e) => return rc_ffi_arg_error(context, FUNCTION_NAME, 1, "endpoint", e.message()),
    };
    let endpoint = Box::new(TsurugiFfiEndpoint { endpoint });

    let handle = Box::into_raw(endpoint);
    unsafe {
        *endpoint_out = handle;
    }

    trace!("{FUNCTION_NAME} end. endpoint={:?}", handle);
    TSURUGI_FFI_RC_OK
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_endpoint_dispose(endpoint: TsurugiFfiEndpointHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_endpoint_dispose()";
    trace!("{FUNCTION_NAME} start. endpoint={:?}", endpoint);

    if endpoint.is_null() {
        trace!("{FUNCTION_NAME} end. arg[endpoint] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(endpoint);
    }

    trace!("{FUNCTION_NAME} end");
}
