package com.tsurugidb.tsubakuro.rust.java.transaction;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

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

class TgFfiTransactionErrorInfoTest extends TgFfiTester {

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

        public TgFfiTransactionErrorInfo getTransactionErrorInfo(String pattern) {
            switch (pattern) {
            case DIRECT:
                return client.getTransactionErrorInfo(context, transaction);
            case DIRECT_FOR:
                return client.getTransactionErrorInfoFor(context, transaction, Duration.ofSeconds(5));
            default:
                return jobTake(client.getTransactionErrorInfoAsync(context, transaction), pattern);
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

            try (var info = resource.getTransactionErrorInfo(pattern)) {
                assertTrue(info.isNormal(context));
                assertFalse(info.isError(context));
                assertNull(info.getServerErrorName(context));
                assertNull(info.getServerErrorMessage(context));
                assertEquals(0, info.getServerErrorCategoryNumber(context));
                assertNull(info.getServerErrorCategoryStr(context));
                assertEquals(0, info.getServerErrorCodeNumber(context));
                assertNull(info.getServerErrorStructuredCode(context));
            }

            var e = assertThrows(TgFfiRuntimeException.class, () -> {
                resource.execute("insert into test values(1, 11, 'aaa')");
            });
            assertEquals("UNIQUE_CONSTRAINT_VIOLATION_EXCEPTION", e.getErrorName());

            try (var info = resource.getTransactionErrorInfo(pattern)) {
                assertFalse(info.isNormal(context));
                assertTrue(info.isError(context));
                assertEquals(e.getErrorName(), info.getServerErrorName(context));
                assertTrue(e.getMessage().contains(info.getServerErrorMessage(context)));
                assertEquals(e.getServerErrorCategoryNumber(), info.getServerErrorCategoryNumber(context));
                assertEquals(e.getServerErrorCategoryStr(), info.getServerErrorCategoryStr(context));
                assertEquals(e.getServerErrorCodeNumber(), info.getServerErrorCodeNumber(context));
                assertEquals(e.getServerErrorStructuredCode(), info.getServerErrorStructuredCode(context));
            }
        }
    }

    @Test
    void get_argError() {
        try (var resource = new TestResource()) {
            var context = resource.context;

            try (var info = resource.getTransactionErrorInfo(DIRECT)) {
                is_normal_argError(context, info);
                is_error_argError(context, info);
                get_server_error_name_argError(context, info);
                get_server_error_message_argError(context, info);
                get_server_error_category_number_argError(context, info);
                get_server_error_category_str_argError(context, info);
                get_server_error_code_number_argError(context, info);
                get_server_error_structured_code_argError(context, info);
            }
        }
    }

    private void is_normal_argError(TgFfiContext context, TgFfiTransactionErrorInfo info) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_is_normal(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = info.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_is_normal(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void is_error_argError(TgFfiContext context, TgFfiTransactionErrorInfo info) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateBooleanOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_is_error(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = info.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_is_error(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_server_error_name_argError(TgFfiContext context, TgFfiTransactionErrorInfo info) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = info.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_server_error_message_argError(TgFfiContext context, TgFfiTransactionErrorInfo info) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_message(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = info.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_message(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_server_error_category_number_argError(TgFfiContext context, TgFfiTransactionErrorInfo info) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_category_number(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = info.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_category_number(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_server_error_category_str_argError(TgFfiContext context, TgFfiTransactionErrorInfo info) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_category_str(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = info.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_category_str(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_server_error_code_number_argError(TgFfiContext context, TgFfiTransactionErrorInfo info) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_code_number(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = info.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_code_number(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_server_error_structured_code_argError(TgFfiContext context, TgFfiTransactionErrorInfo info) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_structured_code(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = info.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_error_info_get_server_error_structured_code(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }
}
