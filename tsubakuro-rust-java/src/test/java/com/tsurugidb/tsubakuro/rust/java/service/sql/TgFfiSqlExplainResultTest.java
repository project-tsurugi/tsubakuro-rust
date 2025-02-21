package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.List;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlExplainResultTest extends TgFfiTester {

    @BeforeAll
    static void beforeAll() {
        dropAndCreateTable("test", """
                create table test (
                  foo int primary key,
                  bar bigint,
                  zzz varchar(10)
                )""");
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void explain(String pattern) {
        var manager = getFfiObjectManager();

        try (var explainResult = getExplain(pattern, "select * from test"); //
                var context = TgFfiContext.create(manager)) {
            explain(context, explainResult);
        }
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void preparedExplain(String pattern) {
        var manager = getFfiObjectManager();

        try (var explainResult = getPreparedExplain(pattern, "select * from test"); //
                var context = TgFfiContext.create(manager)) {
            explain(context, explainResult);
        }
    }

    private void explain(TgFfiContext context, TgFfiSqlExplainResult explainResult) {
        var formatId = explainResult.getFormatId(context);
        assertEquals("jogasaki-statement.json", formatId);

        var formatVersion = explainResult.getFormatVersion(context);
        assertEquals(1L, formatVersion);

        var contents = explainResult.getContents(context);
        assertNotNull(contents);

        var columns = explainResult.getColumns(context);
        assertEquals(3, columns.size());

        int i = 0;
        {
            var column = columns.get(i++);
            assertEquals("foo", column.getName(context));
            assertEquals(TgFfiAtomType.INT4, column.getAtomType(context));
        }
        {
            var column = columns.get(i++);
            assertEquals("bar", column.getName(context));
            assertEquals(TgFfiAtomType.INT8, column.getAtomType(context));
        }
        {
            var column = columns.get(i++);
            assertEquals("zzz", column.getName(context));
            assertEquals(TgFfiAtomType.CHARACTER, column.getAtomType(context));
        }
    }

    @Test
    void argError() {
        var manager = getFfiObjectManager();
        try (var explainResult = getExplain(DIRECT, "select * from test"); //
                var context = TgFfiContext.create(manager)) {
            get_format_id_argError(context, explainResult);
            get_format_version_argError(context, explainResult);
            get_contents_argError(context, explainResult);
            get_columns_size_argError(context, explainResult);
            get_columns_value_argError(context, explainResult);
        }
    }

    private void get_format_id_argError(TgFfiContext context, TgFfiSqlExplainResult executeResult) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_explain_result_get_format_id(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = executeResult.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_explain_result_get_format_id(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_format_version_argError(TgFfiContext context, TgFfiSqlExplainResult metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateLongOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_explain_result_get_format_version(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_explain_result_get_format_version(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_contents_argError(TgFfiContext context, TgFfiSqlExplainResult metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_explain_result_get_contents(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_explain_result_get_contents(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_columns_size_argError(TgFfiContext context, TgFfiSqlExplainResult metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_explain_result_get_columns_size(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_explain_result_get_columns_size(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_columns_value_argError(TgFfiContext context, TgFfiSqlExplainResult executeResult) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            int index = 0;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_explain_result_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = executeResult.handle();
            int index = -1;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_explain_result_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = executeResult.handle();
            int index = 0;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_explain_result_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    private TgFfiSqlExplainResult getExplain(String pattern, String sql) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        var connectionOption = TgFfiConnectionOption.create(context);
        connectionOption.setEndpointUrl(context, getEndpoint());

        try (var session = TgFfiSession.connect(context, connectionOption); //
                var client = session.makeSqlClient(context)) {
            switch (pattern) {
            case DIRECT:
                return client.explain(context, sql);
            case DIRECT_FOR:
                return client.explainFor(context, sql, Duration.ofSeconds(5));
            default:
                try (var job = client.explainAsync(context, sql)) {
                    return jobTake(job, pattern);
                }
            }
        }
    }

    private TgFfiSqlExplainResult getPreparedExplain(String pattern, String sql) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        var connectionOption = TgFfiConnectionOption.create(context);
        connectionOption.setEndpointUrl(context, getEndpoint());

        try (var session = TgFfiSession.connect(context, connectionOption); //
                var client = session.makeSqlClient(context); //
                var ps = client.prepare(context, sql, List.of())) {
            switch (pattern) {
            case DIRECT:
                return client.preparedExplain(context, ps, List.of());
            case DIRECT_FOR:
                return client.preparedExplainFor(context, ps, List.of(), Duration.ofSeconds(5));
            default:
                try (var job = client.preparedExplainAsync(context, ps, List.of())) {
                    return jobTake(job, pattern);
                }
            }
        }
    }
}
