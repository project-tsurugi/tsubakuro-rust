package com.tsurugidb.tsubakuro.rust.java.transaction;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.io.Closeable;
import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTransactionStatusWithMessageTest extends TgFfiTester {

    @BeforeAll
    static void beforeAll() {
        dropAndCreateTable("test", """
                create table test (
                  foo int primary key,
                  bar bigint,
                  zzz varchar(10)
                )""");
    }

    private class TestResource implements Closeable {
        final TgFfiContext context;
        final TgFfiSqlClient client;
        final TgFfiTransaction transaction;

        public TestResource() {
            var manager = getFfiObjectManager();
            this.context = TgFfiContext.create(manager);
            this.client = createSqlClient();
            this.transaction = startOcc(client);
        }

        public void execute(String sql) {
            try (var _ = client.execute(context, transaction, sql)) {
            }
        }

        public TgFfiTransactionStatusWithMessage getTransactionStatus(String pattern) {
            switch (pattern) {
            case DIRECT:
                return client.getTransactionStatus(context, transaction);
            case DIRECT_FOR:
                return client.getTransactionStatusFor(context, transaction, Duration.ofSeconds(5));
            default:
                return jobTake(client.getTransactionStatusAsync(context, transaction), pattern);
            }
        }

        @Override
        public void close() {
            try (context; client; transaction) {
                client.rollback(context, transaction);
            }
        }
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void get(String pattern) {
        try (var resource = new TestResource()) {
            var context = resource.context;

            resource.execute("insert into test values(1, 11, 'aaa')");

            try (var status = resource.getTransactionStatus(pattern)) {
                assertEquals(TgFfiTransactionStatus.RUNNING, status.getStatus(context));
                assertEquals("", status.getMessage(context));
            }

            var e = assertThrows(TgFfiRuntimeException.class, () -> {
                resource.execute("insert into test values(1, 11, 'aaa')");
            });
            assertEquals("UNIQUE_CONSTRAINT_VIOLATION_EXCEPTION", e.getErrorName());

            try (var status = resource.getTransactionStatus(pattern)) {
                assertEquals(TgFfiTransactionStatus.ABORTED, status.getStatus(context));
                assertEquals("", status.getMessage(context));
            }
        }
    }

    @Test
    void get_argError() {
        try (var resource = new TestResource()) {
            var context = resource.context;

            try (var status = resource.getTransactionStatus(DIRECT)) {
                get_status_argError(context, status);
                get_message_argError(context, status);
            }
        }
    }

    private void get_status_argError(TgFfiContext context, TgFfiTransactionStatusWithMessage status) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_status_with_message_get_status(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = status.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_status_with_message_get_status(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_message_argError(TgFfiContext context, TgFfiTransactionStatusWithMessage status) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_status_with_message_get_message(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = status.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_status_with_message_get_message(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }
}
