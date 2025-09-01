//! Credential.

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_cchar_to_str, ffi_arg_out_initialize, ffi_arg_require_non_null, rc_core_error,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiStringHandle,
};

pub(crate) struct TsurugiFfiCredential {
    credential: Credential,
}

impl std::ops::Deref for TsurugiFfiCredential {
    type Target = Credential;

    fn deref(&self) -> &Self::Target {
        &self.credential
    }
}

/// Credential.
pub type TsurugiFfiCredentialHandle = *mut TsurugiFfiCredential;

/// Credential: Creates a null credential.
///
/// See [`Credential::null`].
///
/// # Returns
/// - `credential_out` - credential. To dispose, call [`tsurugi_ffi_credential_dispose`].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_credential_null(
    context: TsurugiFfiContextHandle,
    credential_out: *mut TsurugiFfiCredentialHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_credential_null()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, credential_out={:?}",
        context,
        credential_out
    );

    ffi_arg_out_initialize!(credential_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, credential_out);

    let credential = Credential::null();
    let credential = Box::new(TsurugiFfiCredential { credential });

    let handle = Box::into_raw(credential);
    unsafe {
        *credential_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. credential={:?}", rc, handle);
    rc
}

/// Credential: Creates a new user/password credential.
///
/// See [`Credential::from_user_password`].
///
/// # Parameters
/// - `user` - The user name.
/// - `password` - The password. (nullable)
///
/// # Returns
/// - `credential_out` - credential. To dispose, call [`tsurugi_ffi_credential_dispose`].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_credential_from_user_password(
    context: TsurugiFfiContextHandle,
    user: TsurugiFfiStringHandle,
    password: TsurugiFfiStringHandle,
    credential_out: *mut TsurugiFfiCredentialHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_credential_from_user_password()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, user={:?}, password={:?}, credential_out={:?}",
        context,
        user,
        password,
        credential_out
    );

    ffi_arg_out_initialize!(credential_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, user);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, credential_out);

    let user = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, user);
    let password = if password.is_null() {
        None
    } else {
        Some(ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 2, password))
    };

    let credential = Credential::from_user_password(user, password);
    let credential = Box::new(TsurugiFfiCredential { credential });

    let handle = Box::into_raw(credential);
    unsafe {
        *credential_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. credential={:?}", rc, handle);
    rc
}

/// Credential: Creates a new authentication token credential.
///
/// See [`Credential::from_auth_token`].
///
/// # Parameters
/// - `token` - The auth token.
///
/// # Returns
/// - `credential_out` - credential. To dispose, call [`tsurugi_ffi_credential_dispose`].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_credential_from_auth_token(
    context: TsurugiFfiContextHandle,
    token: TsurugiFfiStringHandle,
    credential_out: *mut TsurugiFfiCredentialHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_credential_from_auth_token()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, token={:?}, credential_out={:?}",
        context,
        token,
        credential_out
    );

    ffi_arg_out_initialize!(credential_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, token);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, credential_out);

    let token = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, token);

    let credential = Credential::from_auth_token(token);
    let credential = Box::new(TsurugiFfiCredential { credential });

    let handle = Box::into_raw(credential);
    unsafe {
        *credential_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. credential={:?}", rc, handle);
    rc
}

/// Credential: Load credential from file.
///
/// See [`Credential::load`].
///
/// # Parameters
/// - `path` - The credential file path.
///
/// # Returns
/// - `credential_out` - credential. To dispose, call [`tsurugi_ffi_credential_dispose`].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_credential_load(
    context: TsurugiFfiContextHandle,
    path: TsurugiFfiStringHandle,
    credential_out: *mut TsurugiFfiCredentialHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_credential_load()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, path={:?}, credential_out={:?}",
        context,
        path,
        credential_out
    );

    ffi_arg_out_initialize!(credential_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, path);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, credential_out);

    let path = ffi_arg_cchar_to_str!(context, FUNCTION_NAME, 1, path);

    let credential = match Credential::load(path) {
        Ok(credential) => credential,
        Err(e) => return rc_core_error!(context, FUNCTION_NAME, e),
    };
    let credential = Box::new(TsurugiFfiCredential { credential });

    let handle = Box::into_raw(credential);
    unsafe {
        *credential_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. credential={:?}", rc, handle);
    rc
}

/// Credential: Dispose.
///
/// # Receiver
/// - `credential` - credential.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_credential_dispose(credential: TsurugiFfiCredentialHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_credential_dispose()";
    trace!("{FUNCTION_NAME} start. credential={:?}", credential);

    if credential.is_null() {
        trace!("{FUNCTION_NAME} end. arg[credential] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(credential);
    }

    trace!("{FUNCTION_NAME} end");
}
