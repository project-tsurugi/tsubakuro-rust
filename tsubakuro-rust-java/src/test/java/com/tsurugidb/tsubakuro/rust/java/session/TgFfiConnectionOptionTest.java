package com.tsurugidb.tsubakuro.rust.java.session;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiConnectionOptionTest extends TgFfiTester {

    @Test
    void create() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
        }

        try (var target = TgFfiConnectionOption.create(manager)) {
        }
    }

    @Test
    void create_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_create(ctx, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void set_endpoint() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            assertNull(target.getEndpointUrl(context));

            var endpoint = TgFfiEndpoint.parse(context, "tcp://localhost:12345");
            target.setEndpoint(context, endpoint);

            String s = target.getEndpointUrl(context);
            assertEquals("tcp://localhost:12345", s);
        }
    }

    @Test
    void set_endpoint_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = TgFfiEndpoint.parse(context, "tcp://localhost:12345").handle();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_endpoint_url() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            assertNull(target.getEndpointUrl(context));

            target.setEndpointUrl(context, "tcp://localhost:12345");

            String s = target.getEndpointUrl(context);
            assertEquals("tcp://localhost:12345", s);
        }
    }

    @Test
    void set_endpoint_url_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = manager.allocateString("tcp://localhost:12345");
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = manager.allocateString("ipc:tsurugi");
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_endpoint_url_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_endpoint_url(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_endpoint_url(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_application_name() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            assertNull(target.getEndpointUrl(context));

            target.setApplicationName(context, "tsubakuro-rust-java/test");

            String s = target.getApplicationName(context);
            assertEquals("tsubakuro-rust-java/test", s);
        }
    }

    @Test
    void set_application_name_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = manager.allocateString("tsubakuro-rust-java/test");
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_application_name(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_application_name(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_application_name_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_application_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_application_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_session_label() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            assertNull(target.getEndpointUrl(context));

            target.setSessionLabel(context, "tsubakuro-rust-java/test");

            String s = target.getSessionLabel(context);
            assertEquals("tsubakuro-rust-java/test", s);
        }
    }

    @Test
    void set_session_label_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = manager.allocateString("tsubakuro-rust-java/test");
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_session_label(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var arg = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_session_label(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_session_label_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_session_label(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_session_label(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_keep_alive() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            target.setKeepAlive(context, Duration.ofMinutes(1));

            var keepAlive = target.getKeepAlive(context);
            assertEquals(Duration.ofMinutes(1), keepAlive);
        }
    }

    @Test
    void set_keep_alive_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = Duration.ofMinutes(1).toNanos();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_keep_alive(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_keep_alive_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_keep_alive(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_keep_alive(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_default_timeout() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            target.setDefaultTimeout(context, Duration.ofSeconds(5));

            var timeout = target.getDefaultTimeout(context);
            assertEquals(Duration.ofSeconds(5), timeout);
        }
    }

    @Test
    void set_default_timeout_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = Duration.ofSeconds(5).toNanos();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_default_timeout(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_default_timeout_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_default_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_default_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_send_timeout() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            target.setSendTimeout(context, Duration.ofSeconds(5));

            var timeout = target.getSendTimeout(context);
            assertEquals(Duration.ofSeconds(5), timeout);
        }
    }

    @Test
    void set_send_timeout_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = Duration.ofSeconds(5).toNanos();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_send_timeout(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_send_timeout_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_send_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_send_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_recv_timeout() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            target.setRecvTimeout(context, Duration.ofSeconds(5));

            var timeout = target.getRecvTimeout(context);
            assertEquals(Duration.ofSeconds(5), timeout);
        }
    }

    @Test
    void set_recv_timeout_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = Duration.ofSeconds(5).toNanos();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_recv_timeout(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void get_recv_timeout_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_recv_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager); //
                var target = TgFfiConnectionOption.create(context)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_recv_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }
}
