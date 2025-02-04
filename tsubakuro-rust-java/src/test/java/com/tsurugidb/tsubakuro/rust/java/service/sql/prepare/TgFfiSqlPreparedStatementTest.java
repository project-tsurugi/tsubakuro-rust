package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

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

class TgFfiSqlPreparedStatementTest extends TgFfiTester {

    @Test
    void argError() {
        var manager = getFfiObjectManager();

        try (var client = createSqlClient(); //
                var ps = getSqlPreparedStatement(client, DIRECT); //
                var context = TgFfiContext.create(manager)) {
            has_result_records_argError(context, ps);
            set_close_timeout_argError(context, ps);
            get_close_timeout_argError(context, ps);
            is_closed_argError(context, ps);
        }
    }

    private void has_result_records_argError(TgFfiContext context, TgFfiSqlPreparedStatement ps) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtr();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_has_result_records(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = ps.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_has_result_records(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void set_close_timeout() {
        var manager = getFfiObjectManager();
        try (var client = createSqlClient(); //
                var context = TgFfiContext.create(manager); //
                var ps = getSqlPreparedStatement(client, DIRECT)) {
            ps.setCloseTimeout(context, Duration.ofSeconds(5));

            var timeout = ps.getCloseTimeout(context);
            assertEquals(Duration.ofSeconds(5), timeout);
        }
    }

    private void set_close_timeout_argError(TgFfiContext context, TgFfiSqlPreparedStatement ps) {
        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg = Duration.ofSeconds(5).toNanos();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_set_close_timeout(ctx, handle, arg);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    private void get_close_timeout_argError(TgFfiContext context, TgFfiSqlPreparedStatement ps) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtr();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_get_close_timeout(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = ps.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_get_close_timeout(ctx, handle, out);
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
                var ps = getSqlPreparedStatement(client, pattern)) {
            assertFalse(ps.isClosed(context));

            if (close) {
                doClose(ps, pattern);
                assertTrue(ps.isClosed(context));
            }
        }
    }

    @Test
    void close_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var client = createSqlClient()) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_close(ctx, handle);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void close_for_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var client = createSqlClient()) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var t = Duration.ofSeconds(5).toNanos();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_close_for(ctx, handle, t);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    private void is_closed_argError(TgFfiContext context, TgFfiSqlPreparedStatement ps) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtr();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_is_closed(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = ps.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_is_closed(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private TgFfiSqlPreparedStatement getSqlPreparedStatement(TgFfiSqlClient client, String pattern) {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        switch (pattern) {
        case DIRECT:
            return client.prepare(context, "select * from test", null);
        case DIRECT_FOR:
            return client.prepareFor(context, "select * from test", null, Duration.ofSeconds(5));
        default:
            try (var job = client.prepareAsync(context, "select * from test", null)) {
                return jobTake(job, pattern);
            }
        }
    }

    private void doClose(TgFfiSqlPreparedStatement ps, String pattern) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            switch (pattern) {
            case DIRECT:
            default:
                ps.close(context);
                break;
            case DIRECT_FOR:
                ps.closeFor(context, Duration.ofSeconds(5));
                break;
            }
        }
    }
}
