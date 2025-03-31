package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.time.Duration;
import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
import java.time.OffsetDateTime;
import java.time.OffsetTime;
import java.time.ZoneOffset;
import java.util.Arrays;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiBlobReference;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiClobReference;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiSqlQueryResult extends TgFfiObject {

    TgFfiSqlQueryResult(TgFfiObjectManager manager, MemorySegment handle) {
        super(manager, handle);
    }

    public synchronized void setDefaultTimeout(TgFfiContext context, Duration timeout) {
        Objects.requireNonNull(timeout, "timeout must not be null");

        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var arg = allocateDuration(timeout);
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_set_default_timeout(ctx, handle, arg);
        TgFfiRcUtil.throwIfError(rc, context);
    }

    public synchronized Duration getDefaultTimeout(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_default_timeout(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToDuration(out);
    }

    public synchronized TgFfiSqlQueryResultMetadata getMetadata(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_metadata(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiSqlQueryResultMetadata(manager(), outHandle);
    }

    public synchronized boolean nextRow(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized boolean nextRowFor(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row_for(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized boolean nextColumn(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized boolean nextColumnFor(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column_for(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized boolean isNull(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_is_null(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBoolean(out);
    }

    public synchronized boolean fetchBoolean(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_boolean(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_BOOLEAN, 0);
    }

    public synchronized boolean fetchForBoolean(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateBooleanOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_boolean(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_BOOLEAN, 0);
    }

    public synchronized int fetchInt4(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int4(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_INT, 0);
    }

    public synchronized int fetchForInt4(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_int4(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_INT, 0);
    }

    public synchronized long fetchInt8(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int8(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_LONG, 0);
    }

    public synchronized long fetchForInt8(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_int8(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_LONG, 0);
    }

    public synchronized float fetchFloat4(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = manager().allocateFloatOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float4(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_FLOAT, 0);
    }

    public synchronized float fetchForFloat4(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = manager().allocateFloatOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_float4(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_FLOAT, 0);
    }

    public synchronized double fetchFloat8(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = manager().allocateDoubleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float8(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_DOUBLE, 0);
    }

    public synchronized double fetchForFloat8(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = manager().allocateDoubleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_float8(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return out.get(ValueLayout.JAVA_DOUBLE, 0);
    }

    public synchronized BigDecimal fetchDecimal(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var bytesOut = allocatePtrOut();
        var sizeOut = allocateIntOut();
        var valueOut = allocateLongOut();
        var exponentOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_decimal(ctx, handle, bytesOut, sizeOut, valueOut, exponentOut);
        TgFfiRcUtil.throwIfError(rc, context);

        int scale = -outToInt(exponentOut);

        if (outToHandle(bytesOut).address() == 0 || outToInt(sizeOut) == 0) {
            long unscaledValue = outToLong(valueOut);
            return BigDecimal.valueOf(unscaledValue, scale);
        } else {
            var bytes = outToBytesInt(bytesOut, sizeOut);
            var unscaledValue = new BigInteger(bytes);
            return new BigDecimal(unscaledValue, scale);
        }
    }

    public synchronized BigDecimal fetchForDecimal(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var bytesOut = allocatePtrOut();
        var sizeOut = allocateIntOut();
        var valueOut = allocateLongOut();
        var exponentOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_decimal(ctx, handle, t, bytesOut, sizeOut, valueOut, exponentOut);
        TgFfiRcUtil.throwIfError(rc, context);

        int scale = -outToInt(exponentOut);

        if (outToHandle(bytesOut).address() == 0 || outToInt(sizeOut) == 0) {
            long unscaledValue = outToLong(valueOut);
            return BigDecimal.valueOf(unscaledValue, scale);
        } else {
            var bytes = outToBytesInt(bytesOut, sizeOut);
            var unscaledValue = new BigInteger(bytes);
            return new BigDecimal(unscaledValue, scale);
        }
    }

    private static final BigInteger MASK;
    static {
        var buf = new byte[Long.BYTES + 1];
        Arrays.fill(buf, 1, buf.length, (byte) 0xff);
        MASK = new BigInteger(buf);
    }

    public synchronized BigDecimal fetchDecimalI128(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var highOut = allocateLongOut();
        var lowOut = allocateLongOut();
        var exponentOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_decimal_i128(ctx, handle, highOut, lowOut, exponentOut);
        TgFfiRcUtil.throwIfError(rc, context);

        long high = outToLong(highOut);
        long low = outToLong(lowOut);
        int scale = -outToInt(exponentOut);

        var h = BigInteger.valueOf(high).shiftLeft(64);
        var l = BigInteger.valueOf(low).and(MASK);
        var unscaledValue = h.or(l);
        return new BigDecimal(unscaledValue, scale);
    }

    public synchronized BigDecimal fetchForDecimalI128(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var highOut = allocateLongOut();
        var lowOut = allocateLongOut();
        var exponentOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_decimal_i128(ctx, handle, t, highOut, lowOut, exponentOut);
        TgFfiRcUtil.throwIfError(rc, context);

        long high = outToLong(highOut);
        long low = outToLong(lowOut);
        int scale = -outToInt(exponentOut);

        var h = BigInteger.valueOf(high).shiftLeft(64);
        var l = BigInteger.valueOf(low).and(MASK);
        var unscaledValue = h.or(l);
        return new BigDecimal(unscaledValue, scale);
    }

    public synchronized String fetchCharacter(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_character(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized String fetchForCharacter(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocatePtrOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_character(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToString(out);
    }

    public synchronized byte[] fetchOctet(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocatePtrOut();
        var sizeOut = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_octet(ctx, handle, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBytesLong(out, sizeOut);
    }

    public synchronized byte[] fetchForOctet(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocatePtrOut();
        var sizeOut = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_octet(ctx, handle, t, out, sizeOut);
        TgFfiRcUtil.throwIfError(rc, context);

        return outToBytesLong(out, sizeOut);
    }

    public synchronized LocalDate fetchDate(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_date(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        long value = outToLong(out);
        return LocalDate.ofEpochDay(value);
    }

    public synchronized LocalDate fetchForDate(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_date(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        long value = outToLong(out);
        return LocalDate.ofEpochDay(value);
    }

    public synchronized LocalTime fetchTimeOfDay(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_time_of_day(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        long value = outToLong(out);
        return LocalTime.ofNanoOfDay(value);
    }

    public synchronized LocalTime fetchForTimeOfDay(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateLongOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_time_of_day(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        long value = outToLong(out);
        return LocalTime.ofNanoOfDay(value);
    }

    public synchronized LocalDateTime fetchTimePoint(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var nanosOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_time_point(ctx, handle, out, nanosOut);
        TgFfiRcUtil.throwIfError(rc, context);

        long seconds = outToLong(out);
        int nanos = outToInt(nanosOut);
        return LocalDateTime.ofEpochSecond(seconds, nanos, ZoneOffset.UTC);
    }

    public synchronized LocalDateTime fetchForTimePoint(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateLongOut();
        var nanosOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_time_point(ctx, handle, t, out, nanosOut);
        TgFfiRcUtil.throwIfError(rc, context);

        long seconds = outToLong(out);
        int nanos = outToInt(nanosOut);
        return LocalDateTime.ofEpochSecond(seconds, nanos, ZoneOffset.UTC);
    }

    public synchronized OffsetTime fetchTimeOfDayWithTimeZone(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var offsetOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_time_of_day_with_time_zone(ctx, handle, out, offsetOut);
        TgFfiRcUtil.throwIfError(rc, context);

        long value = outToLong(out);
        int offset = outToInt(offsetOut);
        return OffsetTime.of(LocalTime.ofNanoOfDay(value), ZoneOffset.ofTotalSeconds(offset * 60));
    }

    public synchronized OffsetTime fetchForTimeOfDayWithTimeZone(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateLongOut();
        var offsetOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_time_of_day_with_time_zone(ctx, handle, t, out, offsetOut);
        TgFfiRcUtil.throwIfError(rc, context);

        long value = outToLong(out);
        int offset = outToInt(offsetOut);
        return OffsetTime.of(LocalTime.ofNanoOfDay(value), ZoneOffset.ofTotalSeconds(offset * 60));
    }

    public synchronized OffsetDateTime fetchTimePointWithTimeZone(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateLongOut();
        var nanosOut = allocateIntOut();
        var offsetOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_time_point_with_time_zone(ctx, handle, out, nanosOut, offsetOut);
        TgFfiRcUtil.throwIfError(rc, context);

        long value = outToLong(out);
        int nanos = outToInt(nanosOut);
        int offset = outToInt(offsetOut);
        return OffsetDateTime.of(LocalDateTime.ofEpochSecond(value, nanos, ZoneOffset.UTC), ZoneOffset.ofTotalSeconds(offset * 60));
    }

    public synchronized OffsetDateTime fetchForTimePointWithTimeZone(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateLongOut();
        var nanosOut = allocateIntOut();
        var offsetOut = allocateIntOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_time_point_with_time_zone(ctx, handle, t, out, nanosOut, offsetOut);
        TgFfiRcUtil.throwIfError(rc, context);

        long value = outToLong(out);
        int nanos = outToInt(nanosOut);
        int offset = outToInt(offsetOut);
        return OffsetDateTime.of(LocalDateTime.ofEpochSecond(value, nanos, ZoneOffset.UTC), ZoneOffset.ofTotalSeconds(offset * 60));
    }

    public synchronized TgFfiBlobReference fetchBlob(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_blob(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiBlobReference(manager(), outHandle);
    }

    public synchronized TgFfiBlobReference fetchForBlob(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_blob(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiBlobReference(manager(), outHandle);
    }

    public synchronized TgFfiClobReference fetchClob(TgFfiContext context) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_clob(ctx, handle, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiClobReference(manager(), outHandle);
    }

    public synchronized TgFfiClobReference fetchForClob(TgFfiContext context, Duration timeout) {
        var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
        var handle = handle();
        var t = allocateDuration(timeout);
        var out = allocateHandleOut();
        var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_clob(ctx, handle, t, out);
        TgFfiRcUtil.throwIfError(rc, context);

        var outHandle = outToHandle(out);
        return new TgFfiClobReference(manager(), outHandle);
    }

    @Override
    protected void dispose(MemorySegment handle) {
        tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_dispose(handle);
    }
}
