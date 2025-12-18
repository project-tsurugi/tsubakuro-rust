//! table metadata.

use std::ffi::CString;

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiStringHandle,
};

pub(crate) struct TsurugiFfiSystemInfo {
    system_info: SystemInfo,
    name: Option<CString>,
    version: Option<CString>,
}

impl TsurugiFfiSystemInfo {
    pub(crate) fn new(system_info: SystemInfo) -> TsurugiFfiSystemInfo {
        TsurugiFfiSystemInfo {
            system_info,
            name: None,
            version: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiSystemInfo {
    type Target = SystemInfo;

    fn deref(&self) -> &Self::Target {
        &self.system_info
    }
}

impl std::ops::DerefMut for TsurugiFfiSystemInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.system_info
    }
}

/// System info.
pub type TsurugiFfiSystemInfoHandle = *mut TsurugiFfiSystemInfo;

/// SystemInfo: Get tsurugidb name.
///
/// See [`SystemInfo::name`].
///
/// # Receiver
/// - `system_info` - System info.
///
/// # Returns
/// - `database_name_out` - database name.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_system_info_get_name(
    context: TsurugiFfiContextHandle,
    system_info: TsurugiFfiSystemInfoHandle,
    name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_system_info_get_name()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, system_info={:?}, name_out={:?}",
        context,
        system_info,
        name_out
    );

    ffi_arg_out_initialize!(name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, system_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, name_out);

    let system_info = unsafe { &mut *system_info };

    if system_info.name.is_none() {
        let name = system_info.name().clone();
        cchar_field_set!(context, system_info.name, name);
    }

    let ptr = cstring_to_cchar!(system_info.name);
    unsafe {
        *name_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (name={:?})", rc, ptr);
    rc
}

/// SystemInfo: Get tsurugidb version.
///
/// See [`SystemInfo::version`].
///
/// # Receiver
/// - `system_info` - System info.
///
/// # Returns
/// - `version_out` - version.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_system_info_get_version(
    context: TsurugiFfiContextHandle,
    system_info: TsurugiFfiSystemInfoHandle,
    version_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_system_info_get_version()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, system_info={:?}, version_out={:?}",
        context,
        system_info,
        version_out
    );

    ffi_arg_out_initialize!(version_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, system_info);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, version_out);

    let system_info = unsafe { &mut *system_info };

    if system_info.version.is_none() {
        let version = system_info.version().clone();
        cchar_field_set!(context, system_info.version, version);
    }

    let ptr = cstring_to_cchar!(system_info.version);
    unsafe {
        *version_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (version={:?})", rc, ptr);
    rc
}

/// SystemInfo: Dispose.
///
/// # Receiver
/// - `system_info` - System info.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_system_info_dispose(system_info: TsurugiFfiSystemInfoHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_system_info_dispose()";
    trace!("{FUNCTION_NAME} start. system_info={:?}", system_info);

    if system_info.is_null() {
        trace!("{FUNCTION_NAME} end. arg[system_info] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(system_info);
    }

    trace!("{FUNCTION_NAME} end");
}
