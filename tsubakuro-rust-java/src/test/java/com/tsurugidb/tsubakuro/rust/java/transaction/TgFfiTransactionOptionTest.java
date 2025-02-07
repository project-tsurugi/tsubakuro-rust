package com.tsurugidb.tsubakuro.rust.java.transaction;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.List;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTransactionOptionTest extends TgFfiTester {

    @Test
    void create() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
        }

        try (var target = TgFfiTransactionOption.create(manager)) {
        }
    }

    @Test
    void create_argError() {
        var manager = getFfiObjectManager();

        try (var contextObject = TgFfiContext.create(manager)) {
            var context = contextObject.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_create(context, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void set_transaction_type() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            assertEquals(TgFfiTransactionType.SHORT, target.getTransactionType(context));

            target.setTransactionType(context, TgFfiTransactionType.LONG);

            var type = target.getTransactionType(context);
            assertEquals(TgFfiTransactionType.LONG, type);
        }
    }

    @Test
    void set_transaction_type_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = TgFfiTransactionType.LONG.value();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_type(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_transaction_type_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_type(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_type(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_transaction_label() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            assertNull(target.getTransactionLabel(context));

            target.setTransactionLabel(context, "tsubakuro-rust-java/test");

            String s = target.getTransactionLabel(context);
            assertEquals("tsubakuro-rust-java/test", s);
        }
    }

    @Test
    void set_transaction_label_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = manager.allocateString("tsubakuro-rust-java/test");
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_label(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_label(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_transaction_label_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_label(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_label(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_modifies_definitions() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            assertFalse(target.getModifiesDefinitions(context));

            target.setModifiesDefinitions(context, true);

            assertTrue(target.getModifiesDefinitions(context));
        }
    }

    @Test
    void set_modifies_definitions_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = true;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_modifies_definitions(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_modifies_definitions_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_modifies_definitions(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_modifies_definitions(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_write_preserve() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            assertTrue(target.getWritePreserve(context).isEmpty());

            var list = List.of("abc", "def", "ghi");
            target.setWritePreserve(context, list);

            var actual = target.getWritePreserve(context);
            assertEquals(list, actual);
        }
    }

    @Test
    void set_write_preserve_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = manager.allocateStringArray(List.of("a"));
            var size = 1;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_write_preserve(ctx, handle, arg, size);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = MemorySegment.NULL;
            var size = 1;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_write_preserve(ctx, handle, arg, size);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_write_preserve_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var sout = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_write_preserve(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var sout = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_write_preserve(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = manager.allocatePtrOut();
            var sout = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_write_preserve(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void set_inclusive_read_area() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            assertTrue(target.getInclusiveReadArea(context).isEmpty());

            var list = List.of("abc", "def", "ghi");
            target.setInclusiveReadArea(context, list);

            var actual = target.getInclusiveReadArea(context);
            assertEquals(list, actual);
        }
    }

    @Test
    void set_inclusive_read_area_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = manager.allocateStringArray(List.of("a"));
            var size = 1;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_inclusive_read_area(ctx, handle, arg, size);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = MemorySegment.NULL;
            var size = 1;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_inclusive_read_area(ctx, handle, arg, size);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_inclusive_read_area_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var sout = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_inclusive_read_area(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var sout = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_inclusive_read_area(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = manager.allocatePtrOut();
            var sout = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_inclusive_read_area(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void set_exclusive_read_area() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            assertTrue(target.getExclusiveReadArea(context).isEmpty());

            var list = List.of("abc", "def", "ghi");
            target.setExclusiveReadArea(context, list);

            var actual = target.getExclusiveReadArea(context);
            assertEquals(list, actual);
        }
    }

    @Test
    void set_exclusive_read_area_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = manager.allocateStringArray(List.of("a"));
            var size = 1;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_exclusive_read_area(ctx, handle, arg, size);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = MemorySegment.NULL;
            var size = 1;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_exclusive_read_area(ctx, handle, arg, size);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_exclusive_read_area_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var sout = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_exclusive_read_area(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var sout = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_exclusive_read_area(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = manager.allocatePtrOut();
            var sout = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_exclusive_read_area(ctx, handle, out, sout);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void set_priority() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            assertEquals(TgFfiTransactionPriority.UNSPECIFIED, target.getPriority(context));

            target.setPriority(context, TgFfiTransactionPriority.WAIT);

            var priority = target.getPriority(context);
            assertEquals(TgFfiTransactionPriority.WAIT, priority);
        }
    }

    @Test
    void set_priority_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = TgFfiTransactionType.LONG.value();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_priority(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_priority_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_priority(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_priority(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_close_timeout() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            assertNull(target.getCloseTimeout(context));

            target.setCloseTimeout(context, Duration.ofSeconds(5));

            var timeout = target.getCloseTimeout(context);
            assertEquals(Duration.ofSeconds(5), timeout);
        }
    }

    @Test
    void set_close_timeout_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = TgFfiTransactionType.LONG.value();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_close_timeout(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_close_timeout_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var existsOut = manager.allocateBooleanOut();
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_close_timeout(ctx, handle, existsOut, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var existsOut = MemorySegment.NULL;
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_close_timeout(ctx, handle, existsOut, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiTransactionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var existsOut = manager.allocateBooleanOut();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_close_timeout(ctx, handle, existsOut, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }
}
