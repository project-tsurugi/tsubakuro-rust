package com.tsurugidb.tsubakuro.rust.java.job;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;
import java.util.concurrent.TimeUnit;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiCancelJobTest extends TgFfiTester {

    @Test
    void test() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            for (int i = 0; i < 10; i++) {
                try (var sessionJob = TgFfiSession.connectAsync(context, connectionOption)) {
                    try (var cancelJob = sessionJob.cancelAsync(context)) {
                        if (cancelJob != null) {
                            cancelJob.isDone(context);
                            cancelJob.wait(context, TimeUnit.SECONDS.toNanos(1));
                            cancelJob.isDone(context);
                            return;
                        }
                    }
                }
            }
        }
    }

    @Test
    void argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());

            for (int i = 0; i < 10; i++) {
                try (var sessionJob = TgFfiSession.connectAsync(context, connectionOption)) {
                    try (var cancelJob = sessionJob.cancelAsync(context)) {
                        if (cancelJob != null) {
                            wait_argError(cancelJob);
                            is_done_argError(cancelJob);
                            return;
                        }
                    }
                }
            }
        }
    }

    private void wait_argError(TgFfiCancelJob target) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            long arg = 10;
            var out = manager.allocatePtr();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_cancel_job_wait(ctx, handle, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = target.handle();
            long arg = 10;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_cancel_job_wait(ctx, handle, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    private void is_done_argError(TgFfiCancelJob target) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtr();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_cancel_job_is_done(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = target.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_cancel_job_is_done(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }
}
