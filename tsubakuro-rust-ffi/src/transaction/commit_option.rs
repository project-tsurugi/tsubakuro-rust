use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_require_non_null, rc_ffi_arg_error,
    return_code::{rc_ok, TsurugiFfiRc},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
#[allow(dead_code)]
pub enum TsurugiFfiCommitType {
    /// the default commit status (rely on the database settings).
    Unspecified = 0,
    /// commit operation has accepted, and the transaction will never abort except system errors.
    Accepted = 10,
    /// commit data has been visible for others.
    Available = 20,
    /// commit data has been saved on the local disk.
    Stored = 30,
    /// commit data has been propagated to the all suitable nodes.
    Propagated = 40,
}

impl TsurugiFfiCommitType {
    fn is_valid(value: i32) -> bool {
        matches!(value, 0 | 10 | 20 | 30 | 40)
    }
}

impl From<CommitType> for TsurugiFfiCommitType {
    fn from(value: CommitType) -> Self {
        match value {
            CommitType::Unspecified => TsurugiFfiCommitType::Unspecified,
            CommitType::Accepted => TsurugiFfiCommitType::Accepted,
            CommitType::Available => TsurugiFfiCommitType::Available,
            CommitType::Stored => TsurugiFfiCommitType::Stored,
            CommitType::Propagated => TsurugiFfiCommitType::Propagated,
        }
    }
}

impl From<TsurugiFfiCommitType> for CommitType {
    fn from(value: TsurugiFfiCommitType) -> Self {
        match value {
            TsurugiFfiCommitType::Unspecified => Self::Unspecified,
            TsurugiFfiCommitType::Accepted => Self::Accepted,
            TsurugiFfiCommitType::Available => Self::Available,
            TsurugiFfiCommitType::Stored => Self::Stored,
            TsurugiFfiCommitType::Propagated => Self::Propagated,
        }
    }
}

#[derive(Debug)]
pub(crate) struct TsurugiFfiCommitOption {
    commit_option: CommitOption,
}

impl std::ops::Deref for TsurugiFfiCommitOption {
    type Target = CommitOption;

    fn deref(&self) -> &Self::Target {
        &self.commit_option
    }
}

impl std::ops::DerefMut for TsurugiFfiCommitOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.commit_option
    }
}

pub type TsurugiFfiCommitOptionHandle = *mut TsurugiFfiCommitOption;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_create(
    context: TsurugiFfiContextHandle,
    commit_option_out: *mut TsurugiFfiCommitOptionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_create()";
    trace!("{FUNCTION_NAME} start");

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, commit_option_out);

    let commit_option = Box::new(TsurugiFfiCommitOption {
        commit_option: CommitOption::new(),
    });

    let handle = Box::into_raw(commit_option);
    unsafe {
        *commit_option_out = handle;
    }

    trace!("{FUNCTION_NAME} end. commit_option={:?}", handle);
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_set_commit_type(
    context: TsurugiFfiContextHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
    commit_type: TsurugiFfiCommitType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_set_commit_type()";
    trace!("{FUNCTION_NAME} start. commit_option={:?}", commit_option);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, commit_option);
    if !TsurugiFfiCommitType::is_valid(commit_type as i32) {
        return rc_ffi_arg_error!(context, FUNCTION_NAME, 2, "commit_type", "is invalid");
    }

    let commit_option = unsafe { &mut *commit_option };

    commit_option.set_commit_type(commit_type.into());

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_get_commit_type(
    context: TsurugiFfiContextHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
    commit_type_out: *mut TsurugiFfiCommitType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_get_commit_type()";
    trace!("{FUNCTION_NAME} start. commit_option={:?}", commit_option);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, commit_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, commit_type_out);

    let commit_option = unsafe { &mut *commit_option };

    let commit_type = commit_option.commit_type();
    unsafe {
        *commit_type_out = commit_type.into();
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

// TODO tsurugi_ffi_commit_option_set_auto_dispose()

#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_dispose(commit_option: TsurugiFfiCommitOptionHandle) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_dispose()";
    trace!("{FUNCTION_NAME} start. commit_option={:?}", commit_option);

    if commit_option.is_null() {
        trace!("{FUNCTION_NAME} end. arg[commit_option] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(commit_option);
    }

    trace!("{FUNCTION_NAME} end");
}
