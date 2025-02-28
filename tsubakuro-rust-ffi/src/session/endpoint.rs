use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiStringHandle,
};

pub(crate) struct TsurugiFfiEndpoint {
    endpoint: Endpoint,
}

impl std::ops::Deref for TsurugiFfiEndpoint {
    type Target = Endpoint;

    fn deref(&self) -> &Self::Target {
        &self.endpoint
    }
}

impl std::ops::DerefMut for TsurugiFfiEndpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.endpoint
    }
}

/// Endpoint.
pub type TsurugiFfiEndpointHandle = *mut TsurugiFfiEndpoint;

/// Creates a new endpoint instance.
///
/// # Parameters
/// - `endpoint` - endpoint url. (e.g. `tcp://localhost:12345`)
///
/// # Returns
/// - `endpoint_out` - endpoint. To dispose, call `tsurugi_ffi_endpoint_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_endpoint_parse(
    context: TsurugiFfiContextHandle,
    endpoint: TsurugiFfiStringHandle,
    endpoint_out: *mut TsurugiFfiEndpointHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_endpoint_parse()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, endpoint={:?}, endpoint_out={:?}",
        context,
        endpoint,
        endpoint_out
    );

    ffi_arg_out_initialize!(endpoint_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, endpoint);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, endpoint_out);

    let endpoint = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, endpoint);
    let endpoint = match Endpoint::parse(endpoint) {
        Ok(value) => value,
        Err(e) => return rc_ffi_arg_error!(context, FUNCTION_NAME, 1, "endpoint", e.message()),
    };
    let endpoint = Box::new(TsurugiFfiEndpoint { endpoint });

    let handle = Box::into_raw(endpoint);
    unsafe {
        *endpoint_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. endpoint={:?}", rc, handle);
    rc
}

/// Dispose endpoint instance.
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
