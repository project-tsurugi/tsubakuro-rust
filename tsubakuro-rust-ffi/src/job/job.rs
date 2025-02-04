use std::{
    ffi::{c_void, CString},
    sync::Arc,
    time::Duration,
};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core_async,
    job::cancel_job::TsurugiFfiCancelJob,
    return_code::{rc_ok, TsurugiFfiRc, TSURUGI_FFI_RC_OK},
    TsurugiFfiDuration, TsurugiFfiStringHandle,
};

use super::cancel_job::TsurugiFfiCancelJobHandle;

pub(crate) struct TsurugiFfiJob<T> {
    job: Option<Job<T>>,
    delegater: Box<dyn TsurugiFfiJobDelegator>,
    runtime: Arc<tokio::runtime::Runtime>,
    name: Option<CString>,
}

impl<T> TsurugiFfiJob<T> {
    pub(crate) fn new(
        job: Job<T>,
        delegater: Box<dyn TsurugiFfiJobDelegator>,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiJob<T> {
        TsurugiFfiJob {
            job: Some(job),
            delegater,
            runtime,
            name: None,
        }
    }

    fn value_name(&self) -> &str {
        self.delegater.value_name()
    }

    fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }

    fn raw_job(&mut self) -> Option<&mut Job<T>> {
        self.job.as_mut()
    }

    fn take_raw_job(&mut self) -> Option<Job<T>> {
        self.job.take()
    }
}

pub(crate) trait TsurugiFfiJobDelegator {
    fn value_name(&self) -> &str;

    fn take(
        &self,
        context: TsurugiFfiContextHandle,
        job: TsurugiFfiJobHandle,
        value_out: *mut *mut c_void,
    ) -> TsurugiFfiRc;

    fn take_for(
        &self,
        context: TsurugiFfiContextHandle,
        job: TsurugiFfiJobHandle,
        timeout: TsurugiFfiDuration,
        value_out: *mut *mut c_void,
    ) -> TsurugiFfiRc;

    fn take_if_ready(
        &self,
        context: TsurugiFfiContextHandle,
        job: TsurugiFfiJobHandle,
        is_ready_out: *mut bool,
        value_out: *mut *mut c_void,
    ) -> TsurugiFfiRc;
}

#[macro_export]
macro_rules! impl_job_delegator {
    ( $struct_name:ident, $src:ty, $ffi:ty, $value_name:expr $(,)?) => {
        pub(crate) struct $struct_name;

        impl $crate::job::TsurugiFfiJobDelegator for $struct_name {
            fn value_name(&self) -> &str {
                $value_name
            }

            fn take(
                &self,
                context: TsurugiFfiContextHandle,
                job: TsurugiFfiJobHandle,
                value_out: *mut *mut std::ffi::c_void,
            ) -> TsurugiFfiRc {
                let job = unsafe { &mut *(job as *mut TsurugiFfiJob<$src>) };
                let value_out = value_out as *mut *mut $ffi;
                job.take(context, $struct_name::convert, value_out)
            }

            fn take_for(
                &self,
                context: TsurugiFfiContextHandle,
                job: TsurugiFfiJobHandle,
                timeout: $crate::TsurugiFfiDuration,
                value_out: *mut *mut std::ffi::c_void,
            ) -> TsurugiFfiRc {
                let job = unsafe { &mut *(job as *mut TsurugiFfiJob<$src>) };
                let value_out = value_out as *mut *mut $ffi;
                job.take_for(context, timeout, $struct_name::convert, value_out)
            }

            fn take_if_ready(
                &self,
                context: TsurugiFfiContextHandle,
                job: TsurugiFfiJobHandle,
                is_ready_out: *mut bool,
                value_out: *mut *mut std::ffi::c_void,
            ) -> TsurugiFfiRc {
                let job = unsafe { &mut *(job as *mut TsurugiFfiJob<$src>) };
                let value_out = value_out as *mut *mut $ffi;
                job.take_if_ready(context, $struct_name::convert, is_ready_out, value_out)
            }
        }
    };
}

impl_job_delegator! {
VoidJobDelegator,
(),
c_void,
"void",
}

impl VoidJobDelegator {
    fn convert(_value: (), _runtime: Arc<tokio::runtime::Runtime>) -> Option<c_void> {
        None
    }
}

pub type TsurugiFfiJobHandle = *mut c_void; // *mut TsurugiFfiJob<T>

fn unknown_job(job: TsurugiFfiJobHandle) -> *mut TsurugiFfiJob<c_void> {
    job as *mut TsurugiFfiJob<c_void>
}

macro_rules! get_raw_job {
    ($context:expr, $function_name:expr, $job_getter:expr) => {{
        match $job_getter {
            Some(value) => value,
            None => {
                let message = format!("{} error. job already closed", $function_name);
                log::trace!("{message}");

                let rc = $crate::return_code::TSURUGI_FFI_RC_FFI_JOB_ALREADY_CLOSED;
                let error = $crate::error::TsurugiFfiError::FfiError(rc, message);
                $crate::context::TsurugiFfiContext::set_error($context, rc, error);

                return rc;
            }
        }
    }};
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_get_name(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    name_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_get_name()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(name_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, name_out);

    let job = unsafe { &mut *unknown_job(job) };

    if job.name.is_none() {
        let raw_job = get_raw_job!(context, FUNCTION_NAME, job.raw_job());
        let name = raw_job.name().clone();
        cchar_field_set!(context, job.name, name);
    }

    unsafe {
        *name_out = cstring_to_cchar!(job.name);
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_wait(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    timeout: TsurugiFfiDuration,
    done_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_wait()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(done_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, done_out);

    let job = unsafe { &mut *unknown_job(job) };
    let timeout = Duration::from_nanos(timeout);

    let runtime = job.runtime().clone();
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.raw_job());
    let done = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.wait(timeout));

    unsafe {
        *done_out = done;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_is_done(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    done_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_is_done()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(done_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, done_out);

    let job = unsafe { &mut *unknown_job(job) };

    let runtime = job.runtime().clone();
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.raw_job());
    let done = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.is_done());

    unsafe {
        *done_out = done;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_take(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    value_out: *mut *mut c_void, // FFI Handle out
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_take()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(value_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);

    unsafe { &mut *unknown_job(job) }
        .delegater
        .take(context, job, value_out)
}

impl<T> TsurugiFfiJob<T> {
    pub(crate) fn take<FFI>(
        &mut self,
        context: TsurugiFfiContextHandle,
        converter: fn(T, Arc<tokio::runtime::Runtime>) -> Option<FFI>,
        value_out: *mut *mut FFI,
    ) -> TsurugiFfiRc {
        const FUNCTION_NAME: &str = "tsurugi_ffi_job_take()";

        let runtime = self.runtime().clone();
        let raw_job = get_raw_job!(context, FUNCTION_NAME, self.raw_job());
        let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.take());
        let value = converter(value, runtime);

        let handle = match value {
            Some(value) => Box::into_raw(Box::new(value)),
            None => std::ptr::null_mut(),
        };
        unsafe {
            *value_out = handle;
        }

        trace!("{FUNCTION_NAME} end. {}={:?}", self.value_name(), handle);
        rc_ok(context)
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_take_for(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    timeout: TsurugiFfiDuration,
    value_out: *mut *mut c_void, // FFI Handle out
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_take_for()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(value_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    unsafe { &mut *unknown_job(job) }
        .delegater
        .take_for(context, job, timeout, value_out)
}

impl<T> TsurugiFfiJob<T> {
    pub(crate) fn take_for<FFI>(
        self: &mut Self,
        context: TsurugiFfiContextHandle,
        timeout: TsurugiFfiDuration,
        converter: fn(T, Arc<tokio::runtime::Runtime>) -> Option<FFI>,
        value_out: *mut *mut FFI,
    ) -> TsurugiFfiRc {
        const FUNCTION_NAME: &str = "tsurugi_ffi_job_take_for()";

        let timeout = Duration::from_nanos(timeout);

        let runtime = self.runtime().clone();
        let raw_job = get_raw_job!(context, FUNCTION_NAME, self.raw_job());
        let value =
            ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.take_for(timeout));
        let value = converter(value, runtime);

        let handle = match value {
            Some(value) => Box::into_raw(Box::new(value)),
            None => std::ptr::null_mut(),
        };
        unsafe {
            *value_out = handle;
        }

        trace!("{FUNCTION_NAME} end. {}={:?}", self.value_name(), handle);
        rc_ok(context)
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_take_if_ready(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    is_ready_out: *mut bool,
    value_out: *mut *mut c_void, // FFI Handle out
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_take_if_ready()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(is_ready_out, false);
    ffi_arg_out_initialize!(value_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_ready_out);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, value_out);

    unsafe { &mut *unknown_job(job) }
        .delegater
        .take_if_ready(context, job, is_ready_out, value_out)
}

impl<T> TsurugiFfiJob<T> {
    pub(crate) fn take_if_ready<FFI>(
        self: &mut Self,
        context: TsurugiFfiContextHandle,
        converter: fn(T, Arc<tokio::runtime::Runtime>) -> Option<FFI>,
        is_ready_out: *mut bool,
        value_out: *mut *mut FFI,
    ) -> TsurugiFfiRc {
        const FUNCTION_NAME: &str = "tsurugi_ffi_job_take_if_ready()";

        let runtime = self.runtime().clone();
        let raw_job = get_raw_job!(context, FUNCTION_NAME, self.raw_job());
        let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.take_if_ready());
        let value = match value {
            Some(value) => converter(value, runtime),
            None => {
                unsafe {
                    *is_ready_out = false;
                    *value_out = std::ptr::null_mut();
                }
                trace!("{FUNCTION_NAME} end. not ready {}=null", self.value_name());
                return rc_ok(context);
            }
        };

        let handle = match value {
            Some(value) => Box::into_raw(Box::new(value)),
            None => std::ptr::null_mut(),
        };
        unsafe {
            *is_ready_out = true;
            *value_out = handle;
        }

        trace!("{FUNCTION_NAME} end. ready {}={:?}", self.value_name(), handle);
        rc_ok(context)
    }
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_cancel(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    cancell_done_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_cancel()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(cancell_done_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, cancell_done_out);

    let job = unsafe { &mut *unknown_job(job) };

    let runtime = job.runtime().clone();
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.take_raw_job());
    let cancel_done = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.cancel());

    unsafe {
        *cancell_done_out = cancel_done;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_cancel_for(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    timeout: TsurugiFfiDuration,
    cancell_done_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_cancel_for()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(cancell_done_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, cancell_done_out);

    let job = unsafe { &mut *unknown_job(job) };
    let timeout = Duration::from_nanos(timeout);

    let runtime = job.runtime().clone();
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.take_raw_job());
    let cancel_done =
        ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.cancel_for(timeout));

    unsafe {
        *cancell_done_out = cancel_done;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_cancel_async(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    cancel_job_out: *mut TsurugiFfiCancelJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_cancel_async()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_out_initialize!(cancel_job_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, cancel_job_out);

    let job = unsafe { &mut *unknown_job(job) };

    let runtime = job.runtime().clone();
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.take_raw_job());
    let cancel_job =
        match ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.cancel_async()) {
            Some(value) => value,
            None => {
                unsafe {
                    *cancel_job_out = std::ptr::null_mut();
                }
                trace!("{FUNCTION_NAME} end. cancel_job=null");
                return rc_ok(context);
            }
        };
    let cancel_job = TsurugiFfiCancelJob::new(cancel_job, runtime.clone());
    let cancel_job = Box::new(cancel_job);

    let handle = Box::into_raw(cancel_job);
    unsafe {
        *cancel_job_out = handle;
    }

    trace!("{FUNCTION_NAME} end. cancel_job={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_close(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_close()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);

    let job = unsafe { &mut *unknown_job(job) };

    let runtime = job.runtime().clone();
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.take_raw_job());
    ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.close());

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_dispose(job: TsurugiFfiJobHandle) {
    job_dispose(job);
}

fn job_dispose(job: TsurugiFfiJobHandle) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_dispose()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    if job.is_null() {
        trace!("{FUNCTION_NAME} end. arg[job] is null");
        return TSURUGI_FFI_RC_OK;
    }

    unsafe {
        let mut job = Box::from_raw(unknown_job(job));

        let raw_job = job.take_raw_job();
        if let Some(raw_job) = raw_job {
            let context = std::ptr::null_mut();

            let runtime = job.runtime().clone();
            ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.close());
        }
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}
