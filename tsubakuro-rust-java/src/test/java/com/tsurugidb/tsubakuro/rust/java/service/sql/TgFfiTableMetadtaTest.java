package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTableMetadtaTest extends TgFfiTester {

    @BeforeAll
    static void beforeAll() {
        dropAndCreateTable("test", """
                /**
                 test table.
                 */
                create table test (
                  /** primary key */
                  foo int primary key,
                  /** long value */
                  bar bigint,
                  /** text value */
                  zzz varchar(10)
                )""");
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void metadata(String pattern) {
        var manager = getFfiObjectManager();

        try (var metadata = getTableMetadata(pattern, "test"); //
                var context = TgFfiContext.create(manager)) {
            var databaseName = metadata.getDatabaseName(context);
            assertEquals("", databaseName);
            var schemaName = metadata.getSchemaName(context);
            assertEquals("", schemaName);
            var tableName = metadata.getTableName(context);
            assertEquals("test", tableName);
            var description = metadata.getDescription(context);
            assertEquals(null, description);

            var columns = metadata.getColumns(context);
            assertEquals(3, columns.size());

            int i = 0;
            {
                var column = columns.get(i++);
                assertEquals("foo", column.getName(context));
                assertEquals(TgFfiAtomType.INT4, column.getAtomType(context));
                assertEquals(null, column.getDescription(context));
            }
            {
                var column = columns.get(i++);
                assertEquals("bar", column.getName(context));
                assertEquals(TgFfiAtomType.INT8, column.getAtomType(context));
                assertEquals(null, column.getDescription(context));
            }
            {
                var column = columns.get(i++);
                assertEquals("zzz", column.getName(context));
                assertEquals(TgFfiAtomType.CHARACTER, column.getAtomType(context));
                assertTrue(column.getVarying(context));
                assertEquals(null, column.getDescription(context));
            }
        }
    }

    @Test
    void argError() {
        var manager = getFfiObjectManager();
        try (var metadata = getTableMetadata(DIRECT, "test"); //
                var context = TgFfiContext.create(manager)) {
            get_database_name_argError(context, metadata);
            get_schema_name_argError(context, metadata);
            get_table_name_argError(context, metadata);
            get_description_argError(context, metadata);
            get_columns_size_argError(context, metadata);
            get_columns_value_argError(context, metadata);
        }
    }

    private void get_database_name_argError(TgFfiContext context, TgFfiTableMetadata metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_database_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_database_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_schema_name_argError(TgFfiContext context, TgFfiTableMetadata metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_schema_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_schema_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_table_name_argError(TgFfiContext context, TgFfiTableMetadata metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_table_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_table_name(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_description_argError(TgFfiContext context, TgFfiTableMetadata metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_description(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_description(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_columns_size_argError(TgFfiContext context, TgFfiTableMetadata metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var out = manager.allocateIntOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_size(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_size(ctx, handle, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private void get_columns_value_argError(TgFfiContext context, TgFfiTableMetadata metadata) {
        var manager = getFfiObjectManager();

        {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            int index = 0;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            int index = -1;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        {
            var ctx = context.handle();
            var handle = metadata.handle();
            int index = 0;
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_table_metadata_get_columns_value(ctx, handle, index, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    private TgFfiTableMetadata getTableMetadata(String pattern, String tableName) {
        var manager = getFfiObjectManager();

        var context = TgFfiContext.create(manager);

        var connectionOption = TgFfiConnectionOption.create(context);
        connectionOption.setEndpointUrl(context, getEndpoint());

        try (var session = TgFfiSession.connect(context, connectionOption); //
                var client = session.makeSqlClient(context)) {
            switch (pattern) {
            case DIRECT:
                return client.getTableMetadata(context, tableName);
            case DIRECT_FOR:
                return client.getTableMetadataFor(context, tableName, Duration.ofSeconds(5));
            default:
                try (var job = client.getTableMetadataAsync(context, tableName)) {
                    return jobTake(job, pattern);
                }
            }
        }
    }
}
