package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlParameter extends TgFfiObject {

    public static TgFfiSqlParameter ofNull(TgFfiContext context, String name) {
        Objects.requireNonNull(context, "context must not be null");
        return ofNull(context.manager(), context, name);
    }

    public static TgFfiSqlParameter ofNull(TgFfiObjectManager manager, String name) {
        return ofNull(manager, null, name);
    }

    public static TgFfiSqlParameter ofNull(TgFfiObjectManager manager, TgFfiContext context, String name) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofNullMain(manager, context, name);
            }
        } else {
            return ofNullMain(manager, null, name);
        }
    }

    private static TgFfiSqlParameter ofNullMain(TgFfiObjectManager manager, TgFfiContext context, String name) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var out = manager.allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_null(ctx, arg1, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofInt4(TgFfiContext context, String name, int value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofInt4(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofInt4(TgFfiObjectManager manager, String name, int value) {
        return ofInt4(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofInt4(TgFfiObjectManager manager, TgFfiContext context, String name, int value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofInt4Main(manager, context, name, value);
            }
        } else {
            return ofInt4Main(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofInt4Main(TgFfiObjectManager manager, TgFfiContext context, String name, int value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value;
        var out = manager.allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int4(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofInt8(TgFfiContext context, String name, long value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofInt8(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofInt8(TgFfiObjectManager manager, String name, long value) {
        return ofInt8(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofInt8(TgFfiObjectManager manager, TgFfiContext context, String name, long value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofInt8Main(manager, context, name, value);
            }
        } else {
            return ofInt8Main(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofInt8Main(TgFfiObjectManager manager, TgFfiContext context, String name, long value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value;
        var out = manager.allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int8(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofFloat4(TgFfiContext context, String name, float value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofFloat4(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofFloat4(TgFfiObjectManager manager, String name, float value) {
        return ofFloat4(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofFloat4(TgFfiObjectManager manager, TgFfiContext context, String name, float value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofFloat4Main(manager, context, name, value);
            }
        } else {
            return ofFloat4Main(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofFloat4Main(TgFfiObjectManager manager, TgFfiContext context, String name, float value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value;
        var out = manager.allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float4(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofFloat8(TgFfiContext context, String name, double value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofFloat8(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofFloat8(TgFfiObjectManager manager, String name, double value) {
        return ofFloat8(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofFloat8(TgFfiObjectManager manager, TgFfiContext context, String name, double value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofFloat8Main(manager, context, name, value);
            }
        } else {
            return ofFloat8Main(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofFloat8Main(TgFfiObjectManager manager, TgFfiContext context, String name, double value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value;
        var out = manager.allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float8(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofCharacter(TgFfiContext context, String name, String value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofCharacter(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofCharacter(TgFfiObjectManager manager, String name, String value) {
        return ofCharacter(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofCharacter(TgFfiObjectManager manager, TgFfiContext context, String name, String value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofCharacterMain(manager, context, name, value);
            }
        } else {
            return ofCharacterMain(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofCharacterMain(TgFfiObjectManager manager, TgFfiContext context, String name, String value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = manager.allocateString(value);
        var out = manager.allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_character(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    TgFfiSqlParameter(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized String getName(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtr();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_get_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_dispose(handle);
    }
}
