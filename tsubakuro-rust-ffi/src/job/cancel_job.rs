use std::{sync::Arc, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc},
    TsurugiFfiDuration,
};

pub(crate) struct TsurugiFfiCancelJob {
    cancel_job: CancelJob,
    runtime: Arc<tokio::runtime::Runtime>,
}

impl TsurugiFfiCancelJob {
    pub(crate) fn new(
        cancel_job: CancelJob,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiCancelJob {
        TsurugiFfiCancelJob {
            cancel_job,
            runtime,
        }
    }

    fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

impl std::ops::Deref for TsurugiFfiCancelJob {
    type Target = CancelJob;

    fn deref(&self) -> &Self::Target {
        &self.cancel_job
    }
}

impl std::ops::DerefMut for TsurugiFfiCancelJob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cancel_job
    }
}

pub type TsurugiFfiCancelJobHandle = *mut TsurugiFfiCancelJob;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_cancel_job_wait(
    context: TsurugiFfiContextHandle,
    cancel_job: TsurugiFfiCancelJobHandle,
    timeout: TsurugiFfiDuration,
    done_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_cancel_job_wait()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, cancel_job={:?}, timeout={:?}, done_out={:?}",
        context,
        cancel_job,
        timeout,
        done_out
    );

    ffi_arg_out_initialize!(done_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, cancel_job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 3, done_out);

    let cancel_job = unsafe { &mut *cancel_job };
    let timeout = Duration::from_nanos(timeout);

    let runtime = cancel_job.runtime().clone();
    let done = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, cancel_job.wait(timeout));

    unsafe {
        *done_out = done;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_cancel_job_is_done(
    context: TsurugiFfiContextHandle,
    cancel_job: TsurugiFfiCancelJobHandle,
    done_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_cancel_job_is_done()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, cancel_job={:?}, done_out={:?}",
        context,
        cancel_job,
        done_out
    );

    ffi_arg_out_initialize!(done_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, cancel_job);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, done_out);

    let cancel_job = unsafe { &mut *cancel_job };

    let runtime = cancel_job.runtime().clone();
    let done = ffi_exec_core_async!(context, FUNCTION_NAME, runtime, cancel_job.is_done());

    unsafe {
        *done_out = done;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_cancel_job_dispose(cancel_job: TsurugiFfiCancelJobHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_cancel_job_dispose()";
    trace!("{FUNCTION_NAME} start. cancel_job={:?}", cancel_job);

    if cancel_job.is_null() {
        trace!("{FUNCTION_NAME} end. arg[cancel_job] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(cancel_job);
    }

    trace!("{FUNCTION_NAME} end");
}
