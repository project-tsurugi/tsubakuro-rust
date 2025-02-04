package com.tsurugidb.tsubakuro.rust.java.job;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiCancelJob extends TgFfiObject {

    TgFfiCancelJob(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized boolean wait(TgFfiContext context, long timeoutNanos) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtr();
        var arg = timeoutNanos;
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_cancel_job_wait(ctx, handle, arg, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized boolean isDone(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_cancel_job_is_done(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_cancel_job_dispose(handle);
    }
}
