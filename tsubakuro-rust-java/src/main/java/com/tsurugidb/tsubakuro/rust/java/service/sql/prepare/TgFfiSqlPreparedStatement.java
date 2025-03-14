package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlPreparedStatement extends TgFfiObject {

    public TgFfiSqlPreparedStatement(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized boolean hasResultRecords(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_has_result_records(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized void setCloseTimeout(TgFfiContext context, Duration timeout) {
        Objects.requireNonNull(timeout, "timeout must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_set_close_timeout(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized Duration getCloseTimeout(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_get_close_timeout(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToDuration(out);
    }

    public synchronized void close(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_close(ctx, handle);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized void closeFor(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_close_for(ctx, handle, t);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized boolean isClosed(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_is_closed(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    @Override
    public void close() {
        RuntimeException re = null;
        try {
            close(null);
        } catch (RuntimeException e) {
            re = e;
        }

        try {
            super.close();
        } catch (RuntimeException e) {
            if (re != null) {
                e.addSuppressed(re);
            }
            throw e;
        }

        if (re != null) {
            throw re;
        }
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_dispose(handle);
    }
}
