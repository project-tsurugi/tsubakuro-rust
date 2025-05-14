//! Transaction error information.

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

/// Transaction status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum TsurugiFfiTransactionStatus {
    /// the transaction status is not specified (should not be used normally).
    Unspecified = 0,
    /// the transaction status unknown or not provided.
    Untracked = 1,
    /// the transaction is started and running.
    Running = 10,
    /// the transaction is in the process of committing.
    Committing = 20,
    /// the transaction has been committed and visible for others.
    Available = 30,
    /// the transaction has been committed and saved on the local disk.
    Stored = 40,
    /// the transaction has been committed and propagated to all the suitable nodes.
    Propagated = 50,
    /// the transaction is in the process of aborting.
    Aborting = 60,
    /// the transaction has been aborted.
    Aborted = 70,
}

impl From<TransactionStatus> for TsurugiFfiTransactionStatus {
    fn from(value: TransactionStatus) -> Self {
        match value {
            TransactionStatus::Unspecified => TsurugiFfiTransactionStatus::Unspecified,
            TransactionStatus::Untracked => TsurugiFfiTransactionStatus::Untracked,
            TransactionStatus::Running => TsurugiFfiTransactionStatus::Running,
            TransactionStatus::Committing => TsurugiFfiTransactionStatus::Committing,
            TransactionStatus::Available => TsurugiFfiTransactionStatus::Available,
            TransactionStatus::Stored => TsurugiFfiTransactionStatus::Stored,
            TransactionStatus::Propagated => TsurugiFfiTransactionStatus::Propagated,
            TransactionStatus::Aborting => TsurugiFfiTransactionStatus::Aborting,
            TransactionStatus::Aborted => TsurugiFfiTransactionStatus::Aborted,
        }
    }
}

impl From<TsurugiFfiTransactionStatus> for TransactionStatus {
    fn from(value: TsurugiFfiTransactionStatus) -> Self {
        match value {
            TsurugiFfiTransactionStatus::Unspecified => Self::Unspecified,
            TsurugiFfiTransactionStatus::Untracked => Self::Untracked,
            TsurugiFfiTransactionStatus::Running => Self::Running,
            TsurugiFfiTransactionStatus::Committing => Self::Committing,
            TsurugiFfiTransactionStatus::Available => Self::Available,
            TsurugiFfiTransactionStatus::Stored => Self::Stored,
            TsurugiFfiTransactionStatus::Propagated => Self::Propagated,
            TsurugiFfiTransactionStatus::Aborting => Self::Aborting,
            TsurugiFfiTransactionStatus::Aborted => Self::Aborted,
        }
    }
}

#[derive(Debug)]
pub struct TsurugiFfiTransactionStatusWithMessage {
    transaction_status: TransactionStatusWithMessage,
    message: Option<CString>,
}

impl TsurugiFfiTransactionStatusWithMessage {
    pub(crate) fn new(
        transaction_status: TransactionStatusWithMessage,
    ) -> TsurugiFfiTransactionStatusWithMessage {
        TsurugiFfiTransactionStatusWithMessage {
            transaction_status,
            message: None,
        }
    }
}

impl std::ops::Deref for TsurugiFfiTransactionStatusWithMessage {
    type Target = TransactionStatusWithMessage;

    fn deref(&self) -> &Self::Target {
        &self.transaction_status
    }
}

impl std::ops::DerefMut for TsurugiFfiTransactionStatusWithMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction_status
    }
}

/// Transaction status.
pub type TsurugiFfiTransactionStatusWithMessageHandle = *mut TsurugiFfiTransactionStatusWithMessage;

/// TransactionStatusWithMessage: Get transaction status.
///
/// See [`TransactionStatusWithMessage::status`].
///
/// # Receiver
/// - `transaction_status` - Transaction status.
///
/// # Returns
/// - `status_out` - transaction status.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_with_message_get_status(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusWithMessageHandle,
    status_out: *mut TsurugiFfiTransactionStatus,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_with_message_get_status()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, status_out={:?}",
        context,
        transaction_status,
        status_out
    );

    ffi_arg_out_initialize!(status_out, TsurugiFfiTransactionStatus::Unspecified);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, status_out);

    let status = unsafe { &mut *transaction_status };

    let value = status.status().into();

    unsafe {
        *status_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (status={:?})",
        rc,
        value as i32
    );
    rc
}

/// TransactionStatus: Returns additional information for the transaction status.
///
/// See [`TransactionStatusWithMessage::message`].
///
/// # Receiver
/// - `transaction_status` - Transaction status.
///
/// # Returns
/// - `message_out` - message.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_with_message_get_message(
    context: TsurugiFfiContextHandle,
    transaction_status: TsurugiFfiTransactionStatusWithMessageHandle,
    message_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_with_message_get_message()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction_status={:?}, message_out={:?}",
        context,
        transaction_status,
        message_out
    );

    ffi_arg_out_initialize!(message_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction_status);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, message_out);

    let status = unsafe { &mut *transaction_status };

    if status.message.is_none() {
        let message = status.message().clone();
        cchar_field_set!(context, status.message, message);
    }

    let ptr = cstring_to_cchar!(status.message);
    unsafe {
        *message_out = ptr;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (message={:?})", rc, ptr);
    rc
}

/// TransactionStatusWithMessage: Dispose.
///
/// # Receiver
/// - `transaction_status` - Transaction status.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_status_with_message_dispose(
    transaction_status: TsurugiFfiTransactionStatusWithMessageHandle,
) {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_status_with_message_dispose()";
    trace!(
        "{FUNCTION_NAME} start. transaction_status={:?}",
        transaction_status
    );

    if transaction_status.is_null() {
        trace!("{FUNCTION_NAME} end. arg[transaction_status] is null");
        return;
    }

    unsafe {
        let _ = Box::from_raw(transaction_status);
    }

    trace!("{FUNCTION_NAME} end");
}
