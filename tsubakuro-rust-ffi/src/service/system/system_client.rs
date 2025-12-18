use std::{ffi::CString, sync::Arc, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core_async,
    impl_job_delegator,
    job::{TsurugiFfiJob, TsurugiFfiJobHandle},
    return_code::{rc_ok, TsurugiFfiRc},
    service::system::system_info::{TsurugiFfiSystemInfo, TsurugiFfiSystemInfoHandle},
    TsurugiFfiDuration, TsurugiFfiStringHandle,
};

pub(crate) struct TsurugiFfiSystemClient {
    system_client: SystemClient,
    runtime: Arc<tokio::runtime::Runtime>,
    service_message_version: Option<CString>,
}

impl TsurugiFfiSystemClient {
    pub(crate) fn new(
        system_client: SystemClient,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiSystemClient {
        TsurugiFfiSystemClient {
            system_client,
            runtime,
            service_message_version: None,
        }
    }

    pub(crate) fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

impl std::ops::Deref for TsurugiFfiSystemClient {
    type Target = SystemClient;

    fn deref(&self) -> &Self::Target {
        &self.system_client
    }
}

impl std::ops::DerefMut for TsurugiFfiSystemClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.system_client
    }
}

/// System client.
pub type TsurugiFfiSystemClientHandle = *mut TsurugiFfiSystemClient;

/// SystemClient: Get service message version.
///
/// See [`SystemClient::service_message_version`].
///
/// # Receiver
/// - `system_client` - System client.
///
/// # Returns
/// - `version_out` - service message version.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_system_client_get_service_message_version(
    context: TsurugiFfiContextHandle,
    system_client: TsurugiFfiSystemClientHandle,
    version_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_system_client_get_service_message_version()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, system_client={:?}, version_out={:?}",
        context,
        system_client,
        version_out
    );

    ffi_arg_out_initialize!(version_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, system_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, version_out);

    let client = unsafe { &mut *system_client };

    let smv = SystemClient::service_message_version();
    cchar_field_set!(context, client.service_message_version, smv);

    let ptr = cstring_to_cchar!(client.service_message_version);
    unsafe {
        *version_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (version={:?})", rc, ptr);
    rc
}

/// SystemClient: Get system info.
///
/// See [`SystemClient::get_system_info`].
///
/// # Receiver
/// - `system_client` - System client.
///
/// # Returns
/// - `system_info_out` - system info. To dispose, call [`tsurugi_ffi_system_info_dispose`](crate::service::system::system_info::tsurugi_ffi_system_info_dispose).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_system_client_get_system_info(
    context: TsurugiFfiContextHandle,
    system_client: TsurugiFfiSystemClientHandle,
    system_info_out: *mut TsurugiFfiSystemInfoHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_system_client_get_system_info()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, system_client={:?}, system_info_out={:?}",
        context,
        system_client,
        system_info_out
    );

    ffi_arg_out_initialize!(system_info_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, system_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, system_info_out);

    let client = unsafe { &*system_client };

    let runtime = client.runtime();
    let system_info =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, client.get_system_info());

    let system_info = Box::new(TsurugiFfiSystemInfo::new(system_info));

    let handle = Box::into_raw(system_info);
    unsafe {
        *system_info_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. system_info={:?}", rc, handle);
    rc
}

/// SystemClient: Get system info.
///
/// See [`SystemClient::get_system_info_for`].
///
/// # Receiver
/// - `system_client` - System client.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
///
/// # Returns
/// - `system_info_out` - system info. To dispose, call [`tsurugi_ffi_system_info_dispose`](crate::service::system::system_info::tsurugi_ffi_system_info_dispose).
#[no_mangle]
pub extern "C" fn tsurugi_ffi_system_client_get_system_info_for(
    context: TsurugiFfiContextHandle,
    system_client: TsurugiFfiSystemClientHandle,
    timeout: TsurugiFfiDuration,
    system_info_out: *mut TsurugiFfiSystemInfoHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_system_client_get_system_info_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, system_client={:?}, timeout={:?}, system_info_out={:?}",
        context,
        system_client,
        timeout,
        system_info_out
    );

    ffi_arg_out_initialize!(system_info_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, system_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, system_info_out);

    let client = unsafe { &*system_client };
    let timeout = Duration::from_nanos(timeout);

    let runtime = client.runtime();
    let system_info = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.get_system_info_for(timeout)
    );

    let system_info = Box::new(TsurugiFfiSystemInfo::new(system_info));

    let handle = Box::into_raw(system_info);
    unsafe {
        *system_info_out = handle;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. system_info={:?}", rc, handle);
    rc
}

/// SystemClient: Get system info.
///
/// See [`SystemClient::get_system_info_async`].
///
/// # Receiver
/// - `system_client` - System client.
///
/// # Returns
/// - `system_info_job_out` - Job for `TsurugiFfiSystemInfoHandle`. To dispose, call [`tsurugi_ffi_job_dispose`](crate::job::tsurugi_ffi_job_dispose).
///   Handle taken from Job casts to `TsurugiFfiSystemInfoHandle` and call [`tsurugi_ffi_system_info_dispose`](crate::service::system::system_info::tsurugi_ffi_system_info_dispose) to dispose.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_system_client_get_system_info_async(
    context: TsurugiFfiContextHandle,
    system_client: TsurugiFfiSystemClientHandle,
    system_info_job_out: *mut TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_system_client_get_system_info_async()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, system_client={:?}, system_info_job_out={:?}",
        context,
        system_client,
        system_info_job_out
    );

    ffi_arg_out_initialize!(system_info_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, system_client);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, system_info_job_out);

    let client = unsafe { &*system_client };

    let runtime = client.runtime();
    let job = ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        client.get_system_info_async()
    );
    let job = TsurugiFfiJob::new(job, Box::new(SystemInfoJobDelegator {}), runtime.clone());
    let job = Box::new(job);

    let handle = Box::into_raw(job);
    unsafe {
        *system_info_job_out = handle as TsurugiFfiJobHandle;
    }
    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. system_info_job={:?}",
        rc,
        handle
    );
    rc
}

impl_job_delegator! {
    SystemInfoJobDelegator,
    SystemInfo,
    TsurugiFfiSystemInfo,
    "system_info",
}

impl SystemInfoJobDelegator {
    fn convert(
        value: SystemInfo,
        _runtime: Arc<tokio::runtime::Runtime>,
    ) -> Option<TsurugiFfiSystemInfo> {
        Some(TsurugiFfiSystemInfo::new(value))
    }
}

/// SystemClient: Dispose.
///
/// # Receiver
/// - `system_client` - System client.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_system_client_dispose(system_client: TsurugiFfiSystemClientHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_system_client_dispose()";
    trace!("{FUNCTION_NAME} start. system_client={:?}", system_client);

    if system_client.is_null() {
        trace!("{FUNCTION_NAME} end. arg[system_client] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(system_client);
    }

    trace!("{FUNCTION_NAME} end");
}
