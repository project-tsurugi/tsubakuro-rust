package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlPlaceholderTest extends TgFfiTester {

    @Test
    void of_atom_type() {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var placeholder = TgFfiSqlPlaceholder.ofAtomType(context, "test", TgFfiAtomType.INT4)) {
            assertEquals("test", placeholder.getName(context));
            assertEquals(TgFfiAtomType.INT4, placeholder.getAtomType(context));
        }
    }

    @Test
    void of_atom_type_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = TgFfiAtomType.INT4.value();
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_of_atom_type(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = -1;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_of_atom_type(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("test");
            var arg2 = TgFfiAtomType.INT4.value();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_of_atom_type(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void get_name_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_get_name(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiSqlPlaceholder.ofAtomType(context, "test", TgFfiAtomType.INT4)) {
            var ctx = context.handle();
            var arg = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_get_name(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_atom_type_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_get_atom_type(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiSqlPlaceholder.ofAtomType(context, "test", TgFfiAtomType.INT4)) {
            var ctx = context.handle();
            var arg = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_placeholder_get_atom_type(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }
}
