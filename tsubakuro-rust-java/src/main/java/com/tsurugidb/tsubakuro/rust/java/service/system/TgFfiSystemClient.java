package com.tsurugidb.tsubakuro.rust.java.service.system;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiJob;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSystemClient extends TgFfiObject {

    public TgFfiSystemClient(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized String getServiceMessageVersion(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_system_client_get_service_message_version(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized TgFfiSystemInfo getSystemInfo(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_system_client_get_system_info(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSystemInfo(manager(), outHandle);
    }

    public synchronized TgFfiSystemInfo getSystemInfoFor(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_system_client_get_system_info_for(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSystemInfo(manager(), outHandle);
    }

    public synchronized TgFfiJob<TgFfiSystemInfo> getSystemInfoAsync(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_system_client_get_system_info_async(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiJob<>(manager(), outHandle) {
            @Override
            protected TgFfiSystemInfo valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
                return new TgFfiSystemInfo(manager, valueHandle);
            }
        };
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_system_client_dispose(handle);
    }
}
