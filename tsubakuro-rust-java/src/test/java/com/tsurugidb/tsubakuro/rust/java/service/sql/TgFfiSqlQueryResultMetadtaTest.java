package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.io.Closeable;
import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlQueryResultMetadtaTest extends TgFfiTester {

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
        final TgFfiSqlClient client;
        final TgFfiTransaction transaction;
        final TgFfiSqlQueryResult queryResult;
        final TgFfiSqlQueryResultMetadata metadata;

        public TestResource() {
            var manager = getFfiObjectManager();
            try (var context = TgFfiContext.create(manager)) {
                this.client = createSqlClient();
                this.transaction = startOcc(client);
                this.queryResult = client.query(context, transaction, "select * from test");
                this.metadata = queryResult.getMetadata(context);
            }
        }

        @Override
        public void close() {
            try (client; transaction) {
                try (queryResult; metadata) {
                }
                commit(client, transaction);
            }
        }
    }

    @Test
    void metadata() {
        var manager = getFfiObjectManager();

        try (var resource = new TestResource(); //
                var context = TgFfiContext.create(manager)) {
            var metadata = resource.metadata;

            var columns = metadata.getColumns(context);
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
    }

    @Test
    void get_columns_size_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_metadata_get_columns_size(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var resource = new TestResource(); //
                var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = resource.metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_metadata_get_columns_size(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void get_columns_value_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            int index = 0;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_metadata_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var resource = new TestResource(); //
                var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = resource.metadata.handle();
            int index = -1;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_metadata_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var resource = new TestResource(); //
                var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = resource.metadata.handle();
            int index = 0;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_metadata_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }
}
