package com.tsurugidb.tsubakuro.rust.java.transaction;

import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.List;
import java.util.Objects;
import java.util.OptionalInt;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiTransactionOption extends TgFfiObject {

    public static TgFfiTransactionOption create(TgFfiContext context) {
        Objects.requireNonNull(context, "context must not be null");
        return create(context.manager(), context);
    }

    public static TgFfiTransactionOption create(TgFfiObjectManager manager) {
        return create(manager, null);
    }

    public static TgFfiTransactionOption create(TgFfiObjectManager manager, TgFfiContext context) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return createMain(manager, context);
            }
        } else {
            return createMain(manager, null);
        }
    }

    private static TgFfiTransactionOption createMain(TgFfiObjectManager manager, TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_create(ctx, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiTransactionOption(manager, outHandle);
    }

    TgFfiTransactionOption(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized void setTransactionType(TgFfiContext context, TgFfiTransactionType type) {
        Objects.requireNonNull(type, "type must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = type.value();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_type(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized TgFfiTransactionType getTransactionType(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_type(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        int value = outToInt(out);
        return TgFfiTransactionType.forNumber(value);
    }

    public synchronized void setTransactionLabel(TgFfiContext context, String label) {
        Objects.requireNonNull(label, "label must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateString(label);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_label(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized String getTransactionLabel(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_label(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized void setModifiesDefinitions(TgFfiContext context, boolean modifiesDefinitions) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = modifiesDefinitions;
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_modifies_definitions(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized boolean getModifiesDefinitions(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_modifies_definitions(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized void setWritePreserve(TgFfiContext context, List<String> tableNames) {
        Objects.requireNonNull(tableNames, "tableNames must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateStringArray(tableNames);
        var size = tableNames.size();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_write_preserve(ctx, handle, arg, size);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized List<String> getWritePreserve(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var sizeOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_write_preserve(ctx, handle, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToStringList(out, sizeOut);
    }

    public synchronized void setInclusiveReadArea(TgFfiContext context, List<String> tableNames) {
        Objects.requireNonNull(tableNames, "tableNames must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateStringArray(tableNames);
        var size = tableNames.size();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_inclusive_read_area(ctx, handle, arg, size);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized List<String> getInclusiveReadArea(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var sizeOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_inclusive_read_area(ctx, handle, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToStringList(out, sizeOut);
    }

    public synchronized void setExclusiveReadArea(TgFfiContext context, List<String> tableNames) {
        Objects.requireNonNull(tableNames, "tableNames must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateStringArray(tableNames);
        var size = tableNames.size();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_exclusive_read_area(ctx, handle, arg, size);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized List<String> getExclusiveReadArea(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var sizeOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_exclusive_read_area(ctx, handle, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToStringList(out, sizeOut);
    }

    public synchronized void setScanParallel(TgFfiContext context, int scanParallel) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = scanParallel;
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_scan_parallel(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized OptionalInt getScanParallel(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var existsOut = allocateBooleanOut();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_scan_parallel(ctx, handle, existsOut, out);
        TgFfiRcUtil.throwIfError(rc, context);

        boolean exists = outToBoolean(existsOut);
        if (exists) {
            return OptionalInt.of(outToInt(out));
        } else {
            return OptionalInt.empty();
        }
    }

    public synchronized void setPriority(TgFfiContext context, TgFfiTransactionPriority priority) {
        Objects.requireNonNull(priority, "priority must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = priority.value();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_priority(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized TgFfiTransactionPriority getPriority(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_priority(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        int value = outToInt(out);
        return TgFfiTransactionPriority.forNumber(value);
    }

    public synchronized void setCloseTimeout(TgFfiContext context, Duration timeout) {
        Objects.requireNonNull(timeout, "timeout must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_close_timeout(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized Duration getCloseTimeout(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var existsOut = allocateBooleanOut();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_close_timeout(ctx, handle, existsOut, out);
        TgFfiRcUtil.throwIfError(rc, context);

        boolean exists = outToBoolean(existsOut);
        if (exists) {
            return outToDuration(out);
        } else {
            assert outToHandle(out).address() == 0;
            return null;
        }
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_dispose(handle);
    }
}
