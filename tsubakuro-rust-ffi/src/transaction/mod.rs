use std::{ffi::c_char, sync::Arc};

use log::trace;
use tsubakuro_rust_core::prelude::*;

use crate::{
    cchar_field_dispose, cchar_field_set,
    context::TsurugiFfiContextHandle,
    ffi_arg_require_non_null, ffi_exec_core_async,
    return_code::{rc_ok, TsurugiFfiRc, TSURUGI_FFI_RC_OK},
};

pub(crate) mod option;

pub(crate) struct TsurugiFfiTransaction {
    transaction: Transaction,
    runtime: Arc<tokio::runtime::Runtime>,
    transaction_id: *mut c_char,
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

impl TsurugiFfiTransaction {
    pub(crate) fn new(
        transaction: Transaction,
        runtime: Arc<tokio::runtime::Runtime>,
    ) -> TsurugiFfiTransaction {
        TsurugiFfiTransaction {
            transaction,
            runtime,
            transaction_id: std::ptr::null_mut(),
        }
    }

    pub(crate) fn runtime(&self) -> &Arc<tokio::runtime::Runtime> {
        &self.runtime
    }
}

pub type TsurugiFfiTransactionHandle = *mut TsurugiFfiTransaction;

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_get_transaction_id(
    context: TsurugiFfiContextHandle,
    transaction: TsurugiFfiTransactionHandle,
    transaction_id_out: *mut *mut c_char,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_get_transaction_id()";
    trace!("{FUNCTION_NAME} start. transaction={:?}", transaction);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction);
    ffi_arg_require_non_null!(context, FUNCTION_NAME, 2, transaction_id_out);

    let transaction = unsafe { &mut *transaction };

    if transaction.transaction_id.is_null() {
        let transaction_id = transaction.transaction_id().clone();
        unsafe {
            cchar_field_set!(context, transaction.transaction_id, transaction_id);
        }
    }

    unsafe {
        *transaction_id_out = transaction.transaction_id;
    }

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

#[no_mangle]
pub extern "C" fn tsurugi_ffi_transaction_close(
    context: TsurugiFfiContextHandle,
    transaction: TsurugiFfiTransactionHandle,
) -> TsurugiFfiRc {
    const FUNCTION_NAME: &str = "tsurugi_ffi_transaction_close()";
    trace!("{FUNCTION_NAME} start. transaction={:?}", transaction);

    ffi_arg_require_non_null!(context, FUNCTION_NAME, 1, transaction);

    let transaction = unsafe { &mut *transaction };

    let runtime = transaction.runtime();
    ffi_exec_core_async!(context, FUNCTION_NAME, runtime, transaction.close());

    trace!("{FUNCTION_NAME} end");
    rc_ok(context)
}

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

        cchar_field_dispose!(transaction.transaction_id);

        if !transaction.is_closed() {
            let context = std::ptr::null_mut();

            let runtime = transaction.runtime();
            ffi_exec_core_async!(context, FUNCTION_NAME, runtime, transaction.close());
        }
    }

    trace!("{FUNCTION_NAME} end");
    TSURUGI_FFI_RC_OK
}
