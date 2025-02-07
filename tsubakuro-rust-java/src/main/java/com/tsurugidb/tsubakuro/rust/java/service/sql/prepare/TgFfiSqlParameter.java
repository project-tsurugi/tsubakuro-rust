package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import java.lang.foreign.MemorySegment;
import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
import java.time.OffsetDateTime;
import java.time.OffsetTime;
import java.time.ZoneOffset;
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
        var out = manager.allocateHandleOut();
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
        var out = manager.allocateHandleOut();
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
        var out = manager.allocateHandleOut();
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
        var out = manager.allocateHandleOut();
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
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float8(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofDecimal(TgFfiContext context, String name, BigDecimal value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofDecimal(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofDecimal(TgFfiObjectManager manager, String name, BigDecimal value) {
        return ofDecimal(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofDecimal(TgFfiObjectManager manager, TgFfiContext context, String name, BigDecimal value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofDecimalMain(manager, context, name, value);
            }
        } else {
            return ofDecimalMain(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofDecimalMain(TgFfiObjectManager manager, TgFfiContext context, String name, BigDecimal value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);

        byte[] unscaledValue = value.unscaledValue().toByteArray();
        var arg2 = manager.allocateBytes(unscaledValue);
        int size = unscaledValue.length;
        int exponent = -value.scale();

        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_decimal(ctx, arg1, arg2, size, exponent, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofDecimalI128(TgFfiContext context, String name, BigDecimal value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofDecimalI128(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofDecimalI128(TgFfiObjectManager manager, String name, BigDecimal value) {
        return ofDecimalI128(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofDecimalI128(TgFfiObjectManager manager, TgFfiContext context, String name, BigDecimal value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofDecimalI128Main(manager, context, name, value);
            }
        } else {
            return ofDecimalI128Main(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofDecimalI128Main(TgFfiObjectManager manager, TgFfiContext context, String name, BigDecimal value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);

        var unscaledValue = value.unscaledValue();
        long high = unscaledValue.shiftRight(64).longValueExact();
        long low = unscaledValue.longValue();
        int exponent = -value.scale();

        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_decimal_i128(ctx, arg1, high, low, exponent, out);
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
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_character(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofOctet(TgFfiContext context, String name, byte[] value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofOctet(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofOctet(TgFfiObjectManager manager, String name, byte[] value) {
        return ofOctet(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofOctet(TgFfiObjectManager manager, TgFfiContext context, String name, byte[] value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofOctetMain(manager, context, name, value);
            }
        } else {
            return ofOctetMain(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofOctetMain(TgFfiObjectManager manager, TgFfiContext context, String name, byte[] value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = manager.allocateBytes(value);
        long size = value.length;
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_octet(ctx, arg1, arg2, size, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofDate(TgFfiContext context, String name, LocalDate value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofDate(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofDate(TgFfiObjectManager manager, String name, LocalDate value) {
        return ofDate(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofDate(TgFfiObjectManager manager, TgFfiContext context, String name, LocalDate value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofDateMain(manager, context, name, value);
            }
        } else {
            return ofDateMain(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofDateMain(TgFfiObjectManager manager, TgFfiContext context, String name, LocalDate value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value.toEpochDay();
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_date(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofTimeOfDay(TgFfiContext context, String name, LocalTime value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofTimeOfDay(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofTimeOfDay(TgFfiObjectManager manager, String name, LocalTime value) {
        return ofTimeOfDay(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofTimeOfDay(TgFfiObjectManager manager, TgFfiContext context, String name, LocalTime value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofTimeOfDayMain(manager, context, name, value);
            }
        } else {
            return ofTimeOfDayMain(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofTimeOfDayMain(TgFfiObjectManager manager, TgFfiContext context, String name, LocalTime value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value.toNanoOfDay();
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_of_day(ctx, arg1, arg2, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofTimePoint(TgFfiContext context, String name, LocalDateTime value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofTimePoint(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofTimePoint(TgFfiObjectManager manager, String name, LocalDateTime value) {
        return ofTimePoint(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofTimePoint(TgFfiObjectManager manager, TgFfiContext context, String name, LocalDateTime value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofTimePointMain(manager, context, name, value);
            }
        } else {
            return ofTimePointMain(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofTimePointMain(TgFfiObjectManager manager, TgFfiContext context, String name, LocalDateTime value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value.toEpochSecond(ZoneOffset.UTC);
        var nanos = value.getNano();
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_point(ctx, arg1, arg2, nanos, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofTimeOfDayWithTimeZone(TgFfiContext context, String name, OffsetTime value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofTimeOfDayWithTimeZone(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofTimeOfDayWithTimeZone(TgFfiObjectManager manager, String name, OffsetTime value) {
        return ofTimeOfDayWithTimeZone(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofTimeOfDayWithTimeZone(TgFfiObjectManager manager, TgFfiContext context, String name, OffsetTime value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofTimeOfDayWithTimeZoneMain(manager, context, name, value);
            }
        } else {
            return ofTimeOfDayWithTimeZoneMain(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofTimeOfDayWithTimeZoneMain(TgFfiObjectManager manager, TgFfiContext context, String name, OffsetTime value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value.toLocalTime().toNanoOfDay();
        var offset = value.getOffset().getTotalSeconds() / 60;
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_of_day_with_time_zone(ctx, arg1, arg2, offset, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlParameter(manager, outHandle);
    }

    public static TgFfiSqlParameter ofTimePointWithTimeZone(TgFfiContext context, String name, OffsetDateTime value) {
        Objects.requireNonNull(context, "context must not be null");
        return ofTimePointWithTimeZone(context.manager(), context, name, value);
    }

    public static TgFfiSqlParameter ofTimePointWithTimeZone(TgFfiObjectManager manager, String name, OffsetDateTime value) {
        return ofTimePointWithTimeZone(manager, null, name, value);
    }

    public static TgFfiSqlParameter ofTimePointWithTimeZone(TgFfiObjectManager manager, TgFfiContext context, String name, OffsetDateTime value) {
        Objects.requireNonNull(manager, "manager must not be null");

        if (context != null) {
            synchronized (context) {
                return ofTimePointWithTimeZoneMain(manager, context, name, value);
            }
        } else {
            return ofTimePointWithTimeZoneMain(manager, null, name, value);
        }
    }

    private static TgFfiSqlParameter ofTimePointWithTimeZoneMain(TgFfiObjectManager manager, TgFfiContext context, String name, OffsetDateTime value) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var arg1 = manager.allocateString(name);
        var arg2 = value.toLocalDateTime().toEpochSecond(ZoneOffset.UTC);
        var nanos = value.toLocalDateTime().getNano();
        var offset = value.getOffset().getTotalSeconds() / 60;
        var out = manager.allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_point_with_time_zone(ctx, arg1, arg2, nanos, offset, out);
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
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_get_name(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_dispose(handle);
    }
}
