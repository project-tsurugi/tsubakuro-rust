//! commit option.

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    context::TsurugiFfiContextHandle,
    ffi_arg_out_initialize, ffi_arg_require_non_null,
    return_code::{rc_ok, TsurugiFfiRc},
};

/// Commit type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
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

/// Commit option.
pub type TsurugiFfiCommitOptionHandle = *mut TsurugiFfiCommitOption;

/// CommitOption: Creates a new instance.
///
/// See [`CommitOption::new`].
///
/// # Returns
/// - `commit_option_out` - commit option. To dispose, call `tsurugi_ffi_commit_option_dispose()`.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_create(
    context: TsurugiFfiContextHandle,
    commit_option_out: *mut TsurugiFfiCommitOptionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_create()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, commit_option_out={:?}",
        context,
        commit_option_out
    );

    ffi_arg_out_initialize!(commit_option_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, commit_option_out);

    let commit_option = Box::new(TsurugiFfiCommitOption {
        commit_option: CommitOption::new(),
    });

    let handle = Box::into_raw(commit_option);
    unsafe {
        *commit_option_out = handle;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. commit_option={:?}",
        rc,
        handle
    );
    rc
}

/// CommitOption: Set commit type.
///
/// See [`CommitOption::set_commit_type`].
///
/// # Receiver
/// - `commit_option` - Commit option.
///
/// # Parameters
/// - `commit_type` - commit type.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_set_commit_type(
    context: TsurugiFfiContextHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
    commit_type: TsurugiFfiCommitType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_set_commit_type()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, commit_option={:?}, commit_type={:?}",
        context,
        commit_option,
        commit_type as i32
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, commit_option);

    let commit_option = unsafe { &mut *commit_option };

    commit_option.set_commit_type(commit_type.into());

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// CommitOption: Get commit type.
///
/// See [`CommitOption::commit_type`].
///
/// # Receiver
/// - `commit_option` - Commit option.
///
/// # Returns
/// - `commit_type_out` - commit type.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_get_commit_type(
    context: TsurugiFfiContextHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
    commit_type_out: *mut TsurugiFfiCommitType,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_get_commit_type()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, commit_option={:?}, commit_type_out={:?}",
        context,
        commit_option,
        commit_type_out
    );

    ffi_arg_out_initialize!(commit_type_out, TsurugiFfiCommitType::Unspecified);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, commit_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, commit_type_out);

    let commit_option = unsafe { &mut *commit_option };

    let commit_type = commit_option.commit_type();

    let value = commit_type.into();
    unsafe {
        *commit_type_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (commit_type={:?})",
        rc,
        value as i32
    );
    rc
}

/// CommitOption: Set auto dispose.
///
/// See [`CommitOption::set_auto_dispose`].
///
/// # Receiver
/// - `commit_option` - Commit option.
///
/// # Parameters
/// - `auto_dispose` - auto dispose.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_set_auto_dispose(
    context: TsurugiFfiContextHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
    auto_dispose: bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_set_auto_dispose()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, commit_option={:?}, auto_dispose={:?}",
        context,
        commit_option,
        auto_dispose
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, commit_option);

    let commit_option = unsafe { &mut *commit_option };

    commit_option.set_auto_dispose(auto_dispose);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// CommitOption: Get auto dispose.
///
/// See [`CommitOption::auto_dispose`].
///
/// # Receiver
/// - `commit_option` - Commit option.
///
/// # Returns
/// - `auto_dispose_out` - auto dispose.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_commit_option_get_auto_dispose(
    context: TsurugiFfiContextHandle,
    commit_option: TsurugiFfiCommitOptionHandle,
    auto_dispose_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_commit_option_get_auto_dispose()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, commit_option={:?}, auto_dispose_out={:?}",
        context,
        commit_option,
        auto_dispose_out
    );

    ffi_arg_out_initialize!(auto_dispose_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, commit_option);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, auto_dispose_out);

    let commit_option = unsafe { &mut *commit_option };

    let value = commit_option.auto_dispose();

    unsafe {
        *auto_dispose_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (auto_dispose={:?})",
        rc,
        value
    );
    rc
}

/// CommitOption: Dispose.
///
/// # Receiver
/// - `commit_option` - Commit option.
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
