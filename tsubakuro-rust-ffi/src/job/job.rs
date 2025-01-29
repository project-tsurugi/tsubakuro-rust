use std::{
    ffi::{c_char, c_void},
    sync::Arc,
    time::Duration,
};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_dispose, cchar_field_set,
    context::TsurugiFfiContextHandle,
    ffi_arg_require_non_null, ffi_exec_core_async,
    job::cancel_job::TsurugiFfiCancelJob,
    return_code::{rc_ok, TsurugiFfiRc, TSURUGI_FFI_RC_OK},
    session::session::{TsurugiFfiSession, TsurugiFfiSessionHandle},
    TsurugiFfiDuration,
};

use super::cancel_job::TsurugiFfiCancelJobHandle;

#[derive(Clone, Copy, Debug)]
pub(crate) enum TsurugiFfiJobValueType {
    Session,
}

impl TsurugiFfiJobValueType {
    fn name(&self) -> &str {
        match self {
            TsurugiFfiJobValueType::Session => "session",
        }
    }
}

pub(crate) struct TsurugiFfiJob<T, FFI> {
    job_type: TsurugiFfiJobValueType,
    runtime: Arc<tokio::runtime::Runtime>,
    job: Option<Job<T>>,
    converter: Box<dyn Fn(T, Arc<tokio::runtime::Runtime>) -> FFI>,
    name: *mut c_char,
}

impl<T, FFI> TsurugiFfiJob<T, FFI> {
    pub(crate) fn new(
        job_type: TsurugiFfiJobValueType,
        job: Job<T>,
        converter: Box<dyn Fn(T, Arc<tokio::runtime::Runtime>) -> FFI>,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiJob<T, FFI> {
        TsurugiFfiJob {
            job_type,
            runtime,
            job: Some(job),
            converter,
            name: std::ptr::null_mut(),
        }
    }

    fn value_name(&self) -> &str {
        self.job_type.name()
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

pub type TsurugiFfiJobHandle = *mut c_void; // *mut TsurugiFfiJob<T, FFI>

fn unknown_job(job: TsurugiFfiJobHandle) -> *mut TsurugiFfiJob<c_void, c_void> {
    job as *mut TsurugiFfiJob<c_void, c_void>
}

fn job_type(job: TsurugiFfiJobHandle) -> TsurugiFfiJobValueType {
    unsafe { &*unknown_job(job) }.job_type
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
    name_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_get_name()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, name_out);
    unsafe {
        *name_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);

    let job = unsafe { &mut *unknown_job(job) };

    if job.name.is_null() {
        let raw_job = get_raw_job!(context, FUNCTION_NAME, job.raw_job());
        let name = raw_job.name().clone();
        unsafe {
            cchar_field_set!(context, job.name, name);
        }
    }

    unsafe {
        *name_out = job.name;
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

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, done_out);
    unsafe {
        *done_out = false;
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);

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

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, done_out);
    unsafe {
        *done_out = false;
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);

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
    value_out: *mut *mut c_void, // FFI out
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_take()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);
    unsafe {
        *value_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);

    match job_type(job) {
        TsurugiFfiJobValueType::Session => job_take(
            context,
            job as *mut TsurugiFfiJob<Arc<Session>, TsurugiFfiSession>,
            value_out as *mut TsurugiFfiSessionHandle,
        ),
    }
}

fn job_take<T, FFI>(
    context: TsurugiFfiContextHandle,
    job: *mut TsurugiFfiJob<T, FFI>,
    value_out: *mut *mut FFI,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_take()";

    let job = unsafe { &mut *job };

    let runtime = job.runtime().clone();
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.raw_job());
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.take());
    let value = (job.converter)(value, runtime);
    let value = Box::new(value);

    let handle = Box::into_raw(value);
    unsafe {
        *value_out = handle;
    }

    trace!("{FUNCTION_NAME} end. {}={:?}", job.value_name(), handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_take_if_ready(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    value_out: *mut *mut c_void, // FFI out
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_take_if_ready()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, value_out);
    unsafe {
        *value_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);

    match job_type(job) {
        TsurugiFfiJobValueType::Session => job_take_if_ready(
            context,
            job as *mut TsurugiFfiJob<Arc<Session>, TsurugiFfiSession>,
            value_out as *mut TsurugiFfiSessionHandle,
        ),
    }
}

fn job_take_if_ready<T, FFI>(
    context: TsurugiFfiContextHandle,
    job: *mut TsurugiFfiJob<T, FFI>,
    value_out: *mut *mut FFI,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_take_if_ready()";

    let job = unsafe { &mut *job };

    let runtime = job.runtime().clone();
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.raw_job());
    let value = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.take_if_ready());
    let value = match value {
        Some(value) => (job.converter)(value, runtime),
        None => {
            unsafe {
                *value_out = std::ptr::null_mut();
            }
            trace!("{FUNCTION_NAME} end. {}=null", job.value_name());
            return rc_ok(context);
        }
    };
    let value = Box::new(value);

    let handle = Box::into_raw(value);
    unsafe {
        *value_out = handle;
    }

    trace!("{FUNCTION_NAME} end. {}={:?}", job.value_name(), handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_job_cancel(
    context: TsurugiFfiContextHandle,
    job: TsurugiFfiJobHandle,
    cancell_done_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_job_cancel()";
    trace!("{FUNCTION_NAME} start. job={:?}", job);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, cancell_done_out);
    unsafe {
        *cancell_done_out = false;
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);

    let job = unsafe { &mut *unknown_job(job) };
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.take_raw_job());

    let runtime = job.runtime().clone();
    let cancel_done = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, raw_job.cancel());

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

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, cancel_job_out);
    unsafe {
        *cancel_job_out = std::ptr::null_mut();
    }
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, job);

    let job = unsafe { &mut *unknown_job(job) };
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.take_raw_job());

    let runtime = job.runtime().clone();
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
    let raw_job = get_raw_job!(context, FUNCTION_NAME, job.take_raw_job());

    let runtime = job.runtime().clone();
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

        cchar_field_dispose!(job.name);

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
