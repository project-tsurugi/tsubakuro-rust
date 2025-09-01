package com.tsurugidb.tsubakuro.rust.java.session;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrowsExactly;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiEndpointTest extends TgFfiTester {

    @Test
    void parse() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var _ = TgFfiEndpoint.parse(context, "tcp://localhost:12345")) {
        }

        try (var _ = TgFfiEndpoint.parse(manager, "tcp://localhost:12345")) {
        }
    }

    @Test
    void parse_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_endpoint_parse(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = manager.allocateString("tcp://localhost:12345");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_endpoint_parse(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void parse_error() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var e = assertThrowsExactly(TgFfiRuntimeException.class, () -> {
                TgFfiEndpoint.parse(context, "ipc://tsurugidb");
            });
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), e.getReturnCode());
            assertNotNull(e.getMessage());
        }
    }
}
