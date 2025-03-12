package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.lang.foreign.MemorySegment;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.List;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameters;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.parameter.TgParameterMapping;
import com.tsurugidb.iceaxe.sql.type.IceaxeObjectFactory;
import com.tsurugidb.iceaxe.sql.type.TgBlob;
import com.tsurugidb.iceaxe.transaction.option.TgTxOption;
import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResultMetadata;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiBlobReference;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTypeBlobTest extends TgFfiTester {

    @BeforeAll
    static void createTable() throws Exception {
        String sql = """
                create table test (
                  pk int primary key,
                  value blob
                )
                """;
        dropAndCreateTable("test", sql);
        insertJava();
    }

    static void insertJava() throws IOException, InterruptedException {
        if (!isIpc(getEndpointJava())) {
            return;
        }

        var sql = "insert into test values(:pk, :value)";
        var mapping = TgParameterMapping.of(TgBindVariable.ofInt("pk"), TgBindVariable.ofBlob("value"));

        var connector = TsurugiConnector.of(getEndpointJava());
        try (var session = connector.createSession(); //
                var ps = session.createStatement(sql, mapping)) {
            var manager = session.createTransactionManager(TgTxOption.ofOCC());

            manager.execute(transaction -> {
                {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", 0), TgBindParameter.of("value", (TgBlob) null));
                    transaction.executeAndGetCountDetail(ps, parameter);
                }
                try (var value = IceaxeObjectFactory.getDefaultInstance().createBlob(new byte[] { 1, 2, 3 }, true)) {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", 1), TgBindParameter.of("value", value));
                    transaction.executeAndGetCountDetail(ps, parameter);
                }
                try (var value = IceaxeObjectFactory.getDefaultInstance().createBlob(new byte[] { 0x11, 0x22, 0x33 }, true)) {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", 2), TgBindParameter.of("value", value));
                    transaction.executeAndGetCountDetail(ps, parameter);
                }
            });
        }
    }

    @Test
    void tableMetadata() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var client = createSqlClient()) {
            try (var metadata = client.getTableMetadata(context, "test")) {
                var columns = metadata.getColumns(context);
                assertEquals(2, columns.size());
                {
                    var c = columns.get(0);
                    assertEquals("pk", c.getName(context));
                    assertEquals(TgFfiAtomType.INT4, c.getAtomType(context));
                }
                {
                    var c = columns.get(1);
                    assertEquals("value", c.getName(context));
                    assertEquals(TgFfiAtomType.BLOB, c.getAtomType(context));
                }
            }
        }
    }

    @Test
    void insertFfi() throws Exception {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var client = createSqlClient(); //
                var transaction = startOcc(client)) {

            var sql = "insert into test values(:pk, :value)";
            var placeholders = List.of( //
                    TgFfiSqlPlaceholder.ofAtomType(context, "pk", TgFfiAtomType.INT4), //
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.BLOB));
            try (var ps = client.prepare(context, sql, placeholders)) {
                var parameters = List.of( //
                        TgFfiSqlParameter.ofInt4(context, "pk", 4), //
                        TgFfiSqlParameter.ofBlob(context, "value", Path.of("/path/to/file")));
                var e = assertThrows(TgFfiRuntimeException.class, () -> {
                    try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
                    }
                });
                assertEquals(TgFfiRcType.CORE_SERVER_ERROR, e.getErrorType());
            }

            client.rollback(context, transaction);
            transaction.close(context);
        }
    }

    @Test
    void insertFfi_contents() throws Exception {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var client = createSqlClient(); //
                var transaction = startOcc(client)) {

            var sql = "insert into test values(:pk, :value)";
            var placeholders = List.of( //
                    TgFfiSqlPlaceholder.ofAtomType(context, "pk", TgFfiAtomType.INT4), //
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.BLOB));
            try (var ps = client.prepare(context, sql, placeholders)) {
                var parameters = List.of( //
                        TgFfiSqlParameter.ofInt4(context, "pk", 4), //
                        TgFfiSqlParameter.ofBlobContents(context, "value", new byte[] { 4, 5, 6 }));
                var e = assertThrows(TgFfiRuntimeException.class, () -> {
                    try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
                    }
                });
                assertEquals(TgFfiRcType.CORE_SERVER_ERROR, e.getErrorType());
            }

            client.rollback(context, transaction);
            transaction.close(context);
        }
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR })
    void selectFfi(String pattern) throws Exception {
        selectFfi(false, pattern);
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR })
    void selectFfi_skip(String pattern) throws Exception {
        selectFfi(true, pattern);
    }

    private void selectFfi(boolean skip, String pattern) throws Exception {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var client = createSqlClient(); //
                var transaction = startOcc(client)) {

            try (var qr = client.query(context, transaction, "select * from test order by pk")) {
                queryResultMetadata(qr.getMetadata(context));

                int i = 0;
                while (qr.nextRow(context)) {
                    assertTrue(qr.nextColumn(context));
                    assertFalse(qr.isNull(context));
                    int pk = qr.fetchInt4(context);
                    assertEquals(i, pk);

                    assertTrue(qr.nextColumn(context));
                    if (!skip) {
                        if (qr.isNull(context)) {
                        } else {
                            switch (pattern) {
                            case DIRECT:
                                try (var value = qr.fetchBlob(context)) {
                                    new BlobTester(client, transaction, i, value).test();
                                }
                                break;
                            case DIRECT_FOR:
                                try (var value = qr.fetchForBlob(context, Duration.ofSeconds(5))) {
                                    new BlobTester(client, transaction, i, value).test();
                                }
                                break;
                            default:
                                throw new AssertionError();
                            }
                        }
                    }
                    assertFalse(qr.nextColumn(context));

                    i++;
                }
                if (isIpc(getEndpointJava())) {
                    assertEquals(3, i);
                } else {
                    assertEquals(0, i);
                }
            }

            commitAndClose(client, transaction, DIRECT_FOR);
        }
    }

    private void queryResultMetadata(TgFfiSqlQueryResultMetadata metadata) {
        var manager = getFfiObjectManager();

        try (metadata; var context = TgFfiContext.create(manager)) {
            var columns = metadata.getColumns(context);
            assertEquals(2, columns.size());
            {
                var c = columns.get(0);
                assertEquals("pk", c.getName(context));
                assertEquals(TgFfiAtomType.INT4, c.getAtomType(context));
            }
            {
                var c = columns.get(1);
                assertEquals("value", c.getName(context));
                assertEquals(TgFfiAtomType.BLOB, c.getAtomType(context));
            }
        }
    }

    class BlobTester {
        private final TgFfiSqlClient client;
        private final TgFfiTransaction transaction;
        private final int index;
        private final TgFfiBlobReference blob;

        BlobTester(TgFfiSqlClient client, TgFfiTransaction transaction, int index, TgFfiBlobReference blob) {
            this.client = client;
            this.transaction = transaction;
            this.index = index;
            this.blob = blob;
        }

        void test() throws IOException {
            for (var pattern : List.of(DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY)) {
                try {
                    copy_blob_to(pattern);
                } catch (TgFfiRuntimeException e) {
                    if (e.getServerErrorStructuredCode().equals("SCD-00404")) { // OPERATION_DENIED
                        continue;
                    }
                    throw e;
                }
            }

            copy_blob_to_argError();
            copy_blob_to_for_argError();
            copy_blob_to_async_argError();
        }

        private void copy_blob_to(String pattern) throws IOException {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var path = Path.of(System.getProperty("java.io.tmpdir")).resolve("TgFfiTypeBlobTest.copy_blob_to." + System.currentTimeMillis() + ".bin");
                try {
                    switch (pattern) {
                    case DIRECT:
                        client.copyBlobTo(context, transaction, blob, path);
                        break;
                    case DIRECT_FOR:
                        client.copyBlobToFor(context, transaction, blob, path, Duration.ofSeconds(5));
                        break;
                    default:
                        try (var job = client.copyBlobToAsync(context, transaction, blob, path)) {
                            Void value = jobTake(job, pattern);
                            assertNull(value);
                        }

                    }

                    var value = Files.readAllBytes(path);
                    switch (index) {
                    case 1:
                        assertArrayEquals(new byte[] { 1, 2, 3 }, value);
                        break;
                    case 2:
                        assertArrayEquals(new byte[] { 0x11, 0x22, 0x33 }, value);
                        break;
                    }
                } finally {
                    Files.deleteIfExists(path);
                }
            }
        }

        private void copy_blob_to_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = blob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to(ctx, handle, tx, arg1, arg2);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = blob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to(ctx, handle, tx, arg1, arg2);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var arg2 = manager.allocateString("/path/to/destination");
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to(ctx, handle, tx, arg1, arg2);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = blob.handle();
                var arg2 = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to(ctx, handle, tx, arg1, arg2);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }

        private void copy_blob_to_for_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = blob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var t = Duration.ofSeconds(5).toNanos();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_for(ctx, handle, tx, arg1, arg2, t);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = blob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var t = Duration.ofSeconds(5).toNanos();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_for(ctx, handle, tx, arg1, arg2, t);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var arg2 = manager.allocateString("/path/to/destination");
                var t = Duration.ofSeconds(5).toNanos();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_for(ctx, handle, tx, arg1, arg2, t);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = blob.handle();
                var arg2 = MemorySegment.NULL;
                var t = Duration.ofSeconds(5).toNanos();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_for(ctx, handle, tx, arg1, arg2, t);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }

        private void copy_blob_to_async_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = blob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_async(ctx, handle, tx, arg1, arg2, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = blob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_async(ctx, handle, tx, arg1, arg2, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var arg2 = manager.allocateString("/path/to/destination");
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_async(ctx, handle, tx, arg1, arg2, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = blob.handle();
                var arg2 = MemorySegment.NULL;
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_blob_to_async(ctx, handle, tx, arg1, arg2, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }
    }
}
