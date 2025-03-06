use std::{ffi::CString, sync::Arc, time::Duration};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_set,
    context::TsurugiFfiContextHandle,
    cstring_to_cchar, ffi_arg_out_initialize, ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc, TSURUGI_FFI_RC_OK},
    TsurugiFfiDuration, TsurugiFfiStringHandle,
};

pub(crate) struct TsurugiFfiTransaction {
    transaction: Transaction,
    runtime: Arc<tokio::runtime::Runtime>,
    transaction_id: Option<CString>,
}

impl TsurugiFfiTransaction {
    pub(crate) fn new(
        transaction: Transaction,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiTransaction {
        TsurugiFfiTransaction {
            transaction,
            runtime,
            transaction_id: None,
        }
    }

    fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

impl std::ops::Deref for TsurugiFfiTransaction {
    type Target = Transaction;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl std::ops::DerefMut for TsurugiFfiTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.transaction
    }
}

/// Transaction.
pub type TsurugiFfiTransactionHandle = *mut TsurugiFfiTransaction;

/// Transaction: Get transaction id.
///
/// See [`Transaction::transaction_id`].
///
/// # Receiver
/// - `transaction` - Transaction.
///
/// # Returns
/// - `transaction_id_out` - transaction id.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_get_transaction_id(
    context: TsurugiFfiContextHandle,
    transaction: TsurugiFfiTransactionHandle,
    transaction_id_out: *mut TsurugiFfiStringHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_get_transaction_id()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction={:?}, transaction_id_out={:?}",
        context,
        transaction,
        transaction_id_out
    );

    ffi_arg_out_initialize!(transaction_id_out, std::ptr::null_mut());
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_id_out);

    let transaction = unsafe { &mut *transaction };

    if transaction.transaction_id.is_none() {
        let transaction_id = transaction.transaction_id().clone();
        cchar_field_set!(context, transaction.transaction_id, transaction_id);
    }

    let ptr = cstring_to_cchar!(transaction.transaction_id);
    unsafe {
        *transaction_id_out = ptr;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (transaction_id={:?})",
        rc,
        ptr
    );
    rc
}

/// Transaction: Set close timeout.
///
/// See [`Transaction::set_close_timeout`].
///
/// # Receiver
/// - `transaction` - Transaction.
///
/// # Parameters
/// - `close_timeout` - timeout time \[nanoseconds\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_set_close_timeout(
    context: TsurugiFfiContextHandle,
    transaction: TsurugiFfiTransactionHandle,
    close_timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_set_close_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction={:?}, close_timeout={:?}",
        context,
        transaction,
        close_timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction);

    let transaction = unsafe { &mut *transaction };
    let close_timeout = Duration::from_nanos(close_timeout);

    transaction.set_close_timeout(close_timeout);

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// Transaction: Get close timeout.
///
/// See [`Transaction::close_timeout`].
///
/// # Receiver
/// - `transaction` - Transaction.
///
/// # Returns
/// - `close_timeout_out` - timeout time \[nanoseconds\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_get_close_timeout(
    context: TsurugiFfiContextHandle,
    transaction: TsurugiFfiTransactionHandle,
    close_timeout_out: *mut TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_get_close_timeout()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction={:?}, close_timeout_out={:?}",
        context,
        transaction,
        close_timeout_out
    );

    ffi_arg_out_initialize!(close_timeout_out, 0);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, close_timeout_out);

    let transaction = unsafe { &mut *transaction };

    let close_timeout = transaction.close_timeout();

    let value = close_timeout.as_nanos() as TsurugiFfiDuration;
    unsafe {
        *close_timeout_out = value;
    }

    let rc = rc_ok(context);
    trace!(
        "{FUNCTION_NAME} end rc={:x}. (close_timeout={:?})",
        rc,
        value
    );
    rc
}

/// Transaction: Close.
///
/// See [`Transaction::close`].
///
/// Note: Close is called in [`tsurugi_ffi_transaction_dispose`].
///
/// # Receiver
/// - `transaction` - Transaction.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_close(
    context: TsurugiFfiContextHandle,
    transaction: TsurugiFfiTransactionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_close()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction={:?}",
        context,
        transaction
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction);

    let transaction = unsafe { &mut *transaction };

    let runtime = transaction.runtime();
    ffi_exec_core_async!(context, FUNCTION_NAME, runtime, transaction.close());

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// Transaction: Close.
///
/// See [`Transaction::close_for`].
///
/// Note: Close is called in [`tsurugi_ffi_transaction_dispose`].
///
/// # Receiver
/// - `transaction` - Transaction.
///
/// # Parameters
/// - `timeout` - timeout time \[nanoseconds\].
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_close_for(
    context: TsurugiFfiContextHandle,
    transaction: TsurugiFfiTransactionHandle,
    timeout: TsurugiFfiDuration,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_close_for()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction={:?}, timeout={:?}",
        context,
        transaction,
        timeout
    );

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction);

    let transaction = unsafe { &mut *transaction };
    let timeout = Duration::from_nanos(timeout);

    let runtime = transaction.runtime();
    ffi_exec_core_async!(
        context,
        FUNCTION_NAME,
        runtime,
        transaction.close_for(timeout)
    );

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}", rc);
    rc
}

/// Transaction: Check if the session is closed.
///
/// See [`Transaction::is_closed`].
///
/// # Receiver
/// - `transaction` - Transaction.
///
/// # Returns
/// - `is_closed_out` - `true`: Already closed / `false`: Not closed.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_is_closed(
    context: TsurugiFfiContextHandle,
    transaction: TsurugiFfiTransactionHandle,
    is_closed_out: *mut bool,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_is_closed()";
    trace!(
        "{FUNCTION_NAME} start. context={:?}, transaction={:?}, is_closed_out={:?}",
        context,
        transaction,
        is_closed_out
    );

    ffi_arg_out_initialize!(is_closed_out, false);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, is_closed_out);

    let transaction = unsafe { &*transaction };

    let value = transaction.is_closed();

    unsafe {
        *is_closed_out = value;
    }

    let rc = rc_ok(context);
    trace!("{FUNCTION_NAME} end rc={:x}. (is_closed={:?})", rc, value);
    rc
}

/// Transaction: Dispose.
///
/// # Receiver
/// - `transaction` - Transaction.
#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_dispose(transaction: TsurugiFfiTransactionHandle) {
    transaction_dispose(transaction);
}

fn transaction_dispose(transaction: TsurugiFfiTransactionHandle) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_dispose()";
    trace!("{FUNCTION_NAME} start. transaction={:?}", transaction);

    if transaction.is_null() {
        trace!("{FUNCTION_NAME} end. arg[transaction] is null");
        return TSURUGI_FFI_RC_OK;
    }

    unsafe {
        let transaction = Box::from_raw(transaction);

        if !transaction.is_closed() {
            let context = std::ptr::null_mut();

            let runtime = transaction.runtime();
            ffi_exec_core_async!(context, FUNCTION_NAME, runtime, transaction.close());
        }
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}
