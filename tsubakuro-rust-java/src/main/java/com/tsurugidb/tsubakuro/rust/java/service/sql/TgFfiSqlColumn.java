package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlColumn extends TgFfiObject {

    TgFfiSqlColumn(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized String getName(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized TgFfiAtomType getAtomType(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_atom_type(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        int value = outToInt(out);
        return TgFfiAtomType.forNumber(value);
    }

    public synchronized ArbitraryInt getLength(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var providedOut = allocateBooleanOut();
        var valueOut = allocateIntOut();
        var arbitraryOut = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_length(ctx, handle, providedOut, valueOut, arbitraryOut);
        TgFfiRcUtil.throwIfError(rc, context);

        if (outToBoolean(providedOut)) {
            return new ArbitraryInt(outToInt(valueOut), outToBoolean(arbitraryOut));
        } else {
            return null;
        }
    }

    public synchronized ArbitraryInt getPrecision(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var providedOut = allocateBooleanOut();
        var valueOut = allocateIntOut();
        var arbitraryOut = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_precision(ctx, handle, providedOut, valueOut, arbitraryOut);
        TgFfiRcUtil.throwIfError(rc, context);

        if (outToBoolean(providedOut)) {
            return new ArbitraryInt(outToInt(valueOut), outToBoolean(arbitraryOut));
        } else {
            return null;
        }
    }

    public synchronized ArbitraryInt getScale(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var providedOut = allocateBooleanOut();
        var valueOut = allocateIntOut();
        var arbitraryOut = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_scale(ctx, handle, providedOut, valueOut, arbitraryOut);
        TgFfiRcUtil.throwIfError(rc, context);

        if (outToBoolean(providedOut)) {
            return new ArbitraryInt(outToInt(valueOut), outToBoolean(arbitraryOut));
        } else {
            return null;
        }
    }

    public synchronized Boolean getNullable(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var providedOut = allocateBooleanOut();
        var valueOut = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_nullable(ctx, handle, providedOut, valueOut);
        TgFfiRcUtil.throwIfError(rc, context);

        if (outToBoolean(providedOut)) {
            return outToBoolean(valueOut);
        } else {
            return null;
        }
    }

    public synchronized Boolean getVarying(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var providedOut = allocateBooleanOut();
        var valueOut = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_varying(ctx, handle, providedOut, valueOut);
        TgFfiRcUtil.throwIfError(rc, context);

        if (outToBoolean(providedOut)) {
            return outToBoolean(valueOut);
        } else {
            return null;
        }
    }

    public synchronized String getDescription(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_get_description(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_sql_column_dispose(handle);
    }
}
