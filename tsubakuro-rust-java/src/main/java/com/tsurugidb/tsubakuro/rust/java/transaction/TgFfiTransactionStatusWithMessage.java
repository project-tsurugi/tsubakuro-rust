package com.tsurugidb.tsubakuro.rust.java.transaction;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiTransactionStatusWithMessage extends TgFfiObject {

    public TgFfiTransactionStatusWithMessage(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized TgFfiTransactionStatus getStatus(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_status_with_message_get_status(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        int value = outToInt(out);
        return TgFfiTransactionStatus.forNumber(value);
    }

    public synchronized String getMessage(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_status_with_message_get_message(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_status_with_message_dispose(handle);
    }
}
