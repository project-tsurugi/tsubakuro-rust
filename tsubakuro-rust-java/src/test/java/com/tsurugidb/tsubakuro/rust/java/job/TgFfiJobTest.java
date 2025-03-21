package com.tsurugidb.tsubakuro.rust.java.job;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.fail;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiJobTest extends TgFfiTester {

    @Test
    void argError() {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        try (var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            try (var job = TgFfiSession.connectAsync(context, connectionOption)) {
                get_name_argError(job);
                wait_argError(job);
                is_done_argError(job);
                take_argError(job);
                take_for_argError(job);
                take_if_ready_argError(job);
                cancel_argError(job);
                cancel_for_argError(job);
                cancel_async_argError(job);
                close_argError(job);
            }
        }
    }

    private void get_name_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_get_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_get_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void wait_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateBooleanOut();
            var t = 10;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_wait(ctx, handle, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var t = 10;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_wait(ctx, handle, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    private void is_done_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_is_done(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_is_done(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void take() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            try (var job = TgFfiSession.connectAsync(context, connectionOption)) {
                assertEquals("Handshake", job.getName(context));

                assertTrue(job.wait(context, Duration.ofSeconds(5)));
                assertTrue(job.isDone(context));

                try (var session = job.take(context)) {
                    var e = assertThrows(TgFfiRuntimeException.class, () -> {
                        job.take(context);
                    });
                    assertTrue(e.getMessage().contains("already taked"));
                }
            }
        }
    }

    private void take_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void take_for() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            try (var job = TgFfiSession.connectAsync(context, connectionOption)) {
                try (var session = job.takeFor(context, Duration.ofSeconds(5))) {
                    var e = assertThrows(TgFfiRuntimeException.class, () -> {
                        job.takeFor(context, Duration.ofSeconds(5));
                    });
                    assertTrue(e.getMessage().contains("already taked"));
                }
            }
        }
    }

    private void take_for_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var t = Duration.ofSeconds(5).toNanos();
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take_for(ctx, handle, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var t = Duration.ofSeconds(5).toNanos();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take_for(ctx, handle, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void take_if_ready() throws Exception {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            try (var job = TgFfiSession.connectAsync(context, connectionOption)) {
                assertEquals("Handshake", job.getName(context));

                for (int i = 0; i < 10; i++) {
                    var sessionOpt = job.takeIfReady(context);
                    if (sessionOpt == null) {
                        Thread.sleep(200);
                        continue;
                    }
                    try (var session = sessionOpt.get()) {
                        assertTrue(job.isDone(context));

                        var e = assertThrows(TgFfiRuntimeException.class, () -> {
                            job.takeIfReady(context);
                        });
                        assertTrue(e.getMessage().contains("already taked"));

                        return;
                    }
                }
            }
            fail("take_if_ready() was not ready");
        }
    }

    private void take_if_ready_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var isReadyOut = manager.allocateBooleanOut();
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take_if_ready(ctx, handle, isReadyOut, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var isReadyOut = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take_if_ready(ctx, handle, isReadyOut, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var isReadyOut = manager.allocateBooleanOut();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_take_if_ready(ctx, handle, isReadyOut, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void cancel() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            try (var job = TgFfiSession.connectAsync(context, connectionOption)) {
                job.cancel(context);
            }
        }
    }

    private void cancel_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void cancel_for() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            try (var job = TgFfiSession.connectAsync(context, connectionOption)) {
                var timeout = Duration.ofSeconds(5);
                job.cancelFor(context, timeout);
            }
        }
    }

    private void cancel_for_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var t = Duration.ofSeconds(5).toNanos();
            var out = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel_for(ctx, handle, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var t = Duration.ofSeconds(5).toNanos();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel_for(ctx, handle, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    // cancel_async() → TgFfiCancelJobTest

    private void cancel_async_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel_async(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = job.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_cancel_async(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void close() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            try (var job = TgFfiSession.connectAsync(context, connectionOption)) {
                job.close(context);
            }
        }
    }

    private void close_argError(TgFfiJob<TgFfiSession> job) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_job_close(ctx, handle);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }
}
