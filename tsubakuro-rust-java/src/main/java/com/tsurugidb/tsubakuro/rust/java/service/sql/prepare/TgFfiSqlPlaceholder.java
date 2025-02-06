package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlPlaceholder extends TgFfiObject {

    public static TgFfiSqlPlaceholder ofAtomType(TgFfiContext context, String name, TgFfiAtomType atomType) {
        Objects.requireNonNull(context, "context must not be null");
        return ofAtomType(context.manager(), context, name, atomType);
    }

    public static TgFfiSqlPlaceholder ofAtomType(TgFfiObjectManager manager, String name, TgFfiAtomType atomType) {
        return ofAtomType(manager, null, name, atomType);
    }

    public static TgFfiSqlPlaceholder ofAtomType(TgFfiObjectManager manager, TgFfiContext context, String name, TgFfiAtomType atomType) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofAtomTypeMain(manager, context, name, atomType);
            }
        } else {
            return ofAtomTypeMain(manager, null, name, atomType);
        }
    }

    private static TgFfiSqlPlaceholder ofAtomTypeMain(TgFfiObjectManager manager, TgFfiContext context, String name, TgFfiAtomType atomType) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = atomType.value();
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_of_atom_type(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlPlaceholder(manager, outHandle);
    }

    TgFfiSqlPlaceholder(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized String getName(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_get_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized TgFfiAtomType getAtomType(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_get_atom_type(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var value = outToInt(out);
        return TgFfiAtomType.forNumber(value);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_dispose(handle);
    }
}
