package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;
import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
import java.time.OffsetDateTime;
import java.time.OffsetTime;
import java.time.ZoneOffset;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlParameterTest extends TgFfiTester {

    @Test
    void of_null() {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var placeholder = TgFfiSqlParameter.ofNull(context, "test")) {
            assertEquals("test", placeholder.getName(context));
        }
    }

    @Test
    void of_null_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_null(ctx, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_null(ctx, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void of_int4_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = 123;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int4(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = 123;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int4(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void of_int8_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = 123;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int8(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = 123;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int8(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void of_float4_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = 123;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float4(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = 123;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float4(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void of_float8_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = 123;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float8(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = 123;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float8(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void of_decimal_argError() {
        var manager = getFfiObjectManager();

        var value = new BigDecimal("1234.56");
        byte[] unscaledValue = value.unscaledValue().toByteArray();
        int size = unscaledValue.length;
        int exponent = -value.scale();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = manager.allocateBytes(unscaledValue);
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_decimal(ctx, arg1, arg2, size, exponent, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_decimal(ctx, arg1, arg2, size, exponent, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = manager.allocateBytes(unscaledValue);
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_decimal(ctx, arg1, arg2, size, exponent, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), rc);
        }
    }

    @Test
    void of_decimal_i128_argError() {
        var manager = getFfiObjectManager();

        long high = 123;
        long low = 456;
        int exponent = -2;

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_decimal_i128(ctx, arg1, high, low, exponent, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_decimal_i128(ctx, arg1, high, low, exponent, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), rc);
        }
    }

    @Test
    void of_character_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = manager.allocateString("abc");
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_character(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_character(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = manager.allocateString("abc");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_character(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void of_octet_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = manager.allocateBytes(new byte[0]);
            var size = 0;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_octet(ctx, arg1, arg2, size, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = MemorySegment.NULL;
            var size = 0;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_octet(ctx, arg1, arg2, size, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = manager.allocateString("abc");
            var size = 0;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_octet(ctx, arg1, arg2, size, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
        }
    }

    @Test
    void of_date_argError() {
        var manager = getFfiObjectManager();
        var value = LocalDate.now();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = value.toEpochDay();
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_date(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = value.toEpochDay();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_date(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void of_time_of_day_argError() {
        var manager = getFfiObjectManager();
        var value = LocalTime.now();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = value.toNanoOfDay();
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_of_day(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = value.toNanoOfDay();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_of_day(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void of_time_point_argError() {
        var manager = getFfiObjectManager();
        var value = LocalDateTime.now();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = value.toEpochSecond(ZoneOffset.UTC);
            var nanos = value.getNano();
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_point(ctx, arg1, arg2, nanos, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = value.toEpochSecond(ZoneOffset.UTC);
            var nanos = value.getNano();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_point(ctx, arg1, arg2, nanos, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
        }
    }

    @Test
    void of_time_of_day_with_time_zone_argError() {
        var manager = getFfiObjectManager();
        var value = OffsetTime.now();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = value.toLocalTime().toNanoOfDay();
            var offset = value.getOffset().getTotalSeconds() / 60;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_of_day_with_time_zone(ctx, arg1, arg2, offset, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = value.toLocalTime().toNanoOfDay();
            var offset = value.getOffset().getTotalSeconds() / 60;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_of_day_with_time_zone(ctx, arg1, arg2, offset, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
        }
    }

    @Test
    void of_time_point_with_time_zone_argError() {
        var manager = getFfiObjectManager();
        var value = OffsetDateTime.now();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = value.toLocalDateTime().toEpochSecond(ZoneOffset.UTC);
            var nanos = value.toLocalDateTime().getNano();
            var offset = value.getOffset().getTotalSeconds() / 60;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_point_with_time_zone(ctx, arg1, arg2, nanos, offset, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = value.toLocalDateTime().toEpochSecond(ZoneOffset.UTC);
            var nanos = value.toLocalDateTime().getNano();
            var offset = value.getOffset().getTotalSeconds() / 60;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_time_point_with_time_zone(ctx, arg1, arg2, nanos, offset, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), rc);
        }
    }

    @Test
    void get_name_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_get_name(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiSqlParameter.ofNull(context, "test")) {
            var ctx = context.handle();
            var arg = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_get_name(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }
}
