package com.tsurugidb.tsubakuro.rust.java.transaction;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTransactionTest extends TgFfiTester {

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void get_transaction_id(String pattern) {
        getTransactionId(pattern, true);
        getTransactionId(pattern, false);
    }

    private void getTransactionId(String pattern, boolean close) {
        var manager = getFfiObjectManager();
        try (var client = createSqlClient(); //
                var transaction = createTransaction(client, pattern); //
                var context = TgFfiContext.create(manager)) {
            String transactionId = transaction.getTransactionId(context);
            assertTrue(transactionId.startsWith("TID-"));

            if (close) {
                transaction.close(context);
            }
        }
    }

    @Test
    void argError() {
        var manager = getFfiObjectManager();
        try (var client = createSqlClient(); //
                var transaction = createTransaction(client, DIRECT); //
                var context = TgFfiContext.create(manager)) {
            get_transaction_id_argError(context, transaction);
            set_close_timeout_argError(context, transaction);
            get_close_timeout_argError(context, transaction);
            is_closed_argError(context, transaction);
        }
    }

    private void get_transaction_id_argError(TgFfiContext context, TgFfiTransaction transaction) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_get_transaction_id(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = transaction.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_get_transaction_id(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_close_timeout() {
        var manager = getFfiObjectManager();
        try (var client = createSqlClient(); //
                var context = TgFfiContext.create(manager); //
                var transaction = createTransaction(client, DIRECT)) {
            transaction.setCloseTimeout(context, Duration.ofSeconds(5));

            var timeout = transaction.getCloseTimeout(context);
            assertEquals(Duration.ofSeconds(5), timeout);
        }
    }

    private void set_close_timeout_argError(TgFfiContext context, TgFfiTransaction transaction) {
        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = Duration.ofSeconds(5).toNanos();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_set_close_timeout(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    private void get_close_timeout_argError(TgFfiContext context, TgFfiTransaction transaction) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_get_close_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = transaction.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_get_close_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void close(String pattern) {
        close(pattern, true);
        close(pattern, false);
    }

    private void close(String pattern, boolean close) {
        var manager = getFfiObjectManager();
        try (var client = createSqlClient(); //
                var context = TgFfiContext.create(manager); //
                var transaction = createTransaction(client, pattern)) {
            assertFalse(transaction.isClosed(context));

            if (close) {
                doClose(transaction, pattern);
                assertTrue(transaction.isClosed(context));
            }
        }
    }

    @Test
    void close_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_close(ctx, handle);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void close_for_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var t = Duration.ofSeconds(5).toNanos();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_close_for(ctx, handle, t);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    private void is_closed_argError(TgFfiContext context, TgFfiTransaction transaction) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_is_closed(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = transaction.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_is_closed(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private TgFfiTransaction createTransaction(TgFfiSqlClient client, String pattern) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var transactionOption = TgFfiTransactionOption.create(context)) {
            switch (pattern) {
            case DIRECT:
                return client.startTransaction(context, transactionOption);
            case DIRECT_FOR:
                return client.startTransactionFor(context, transactionOption, Duration.ofSeconds(5));
            default:
                try (var job = client.startTransactionAsync(context, transactionOption)) {
                    return jobTake(job, pattern);
                }
            }
        }
    }

    private void doClose(TgFfiTransaction transaction, String pattern) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            switch (pattern) {
            case DIRECT:
            default:
                transaction.close(context);
                break;
            case DIRECT_FOR:
                transaction.closeFor(context, Duration.ofSeconds(5));
                break;
            }
        }
    }
}
