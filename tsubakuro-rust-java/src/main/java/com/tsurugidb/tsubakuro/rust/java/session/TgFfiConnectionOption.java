package com.tsurugidb.tsubakuro.rust.java.session;

import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiConnectionOption extends TgFfiObject {

    public static TgFfiConnectionOption create(TgFfiContext context) {
        Objects.requireNonNull(context, "context must not be null");
        return create(context.manager(), context);
    }

    public static TgFfiConnectionOption create(TgFfiObjectManager manager) {
        return create(manager, null);
    }

    public static TgFfiConnectionOption create(TgFfiObjectManager manager, TgFfiContext context) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return createMain(manager, context);
            }
        } else {
            return createMain(manager, null);
        }
    }

    private static TgFfiConnectionOption createMain(TgFfiObjectManager manager, TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_create(ctx, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiConnectionOption(manager, outHandle);
    }

    TgFfiConnectionOption(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized void setEndpoint(TgFfiContext context, TgFfiEndpoint endpoint) {
        Objects.requireNonNull(endpoint, "endpoint must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = endpoint.handle();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized void setEndpointUrl(TgFfiContext context, String endpoint) {
        Objects.requireNonNull(endpoint, "endpoint must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(endpoint);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized String getEndpoint(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_endpoint(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized void setApplicationName(TgFfiContext context, String applicationName) {
        Objects.requireNonNull(applicationName, "applicationName must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(applicationName);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_application_name(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized String getApplicationName(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_application_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized void setSessionLabel(TgFfiContext context, String label) {
        Objects.requireNonNull(label, "label must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(label);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_session_label(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized String getSessionLabel(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_session_label(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized void setKeepAlive(TgFfiContext context, Duration keepAlive) {
        Objects.requireNonNull(keepAlive, "keepAlive must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateDuration(keepAlive);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_keep_alive(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized Duration getKeepAlive(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_keep_alive(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToDuration(out);
    }

    public synchronized void setDefaultTimeout(TgFfiContext context, Duration timeout) {
        Objects.requireNonNull(timeout, "timeout must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_default_timeout(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized Duration getDefaultTimeout(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_default_timeout(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToDuration(out);
    }

    public synchronized void setSendTimeout(TgFfiContext context, Duration timeout) {
        Objects.requireNonNull(timeout, "timeout must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_send_timeout(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized Duration getSendTimeout(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_send_timeout(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToDuration(out);
    }

    public synchronized void setRecvTimeout(TgFfiContext context, Duration timeout) {
        Objects.requireNonNull(timeout, "timeout must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_recv_timeout(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized Duration getRecvTimeout(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_recv_timeout(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToDuration(out);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_dispose(handle);
    }
}
