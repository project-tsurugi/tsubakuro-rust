package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.lang.foreign.MemorySegment;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.iceaxe.session.TgLobTransferType;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameters;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.parameter.TgParameterMapping;
import com.tsurugidb.iceaxe.transaction.option.TgTxOption;
import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiLobOperation;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResultMetadata;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiClobReference;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiLargeObjectCache;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiLobTransferType;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

abstract class TgFfiTypeClobTestBase extends TgFfiTester {

    @BeforeEach
    void createTable() throws Exception {
        String sql = """
                create table test (
                  pk int primary key,
                  value clob
                )
                """;
        dropAndCreateTable("test", sql);
    }

    protected abstract TgFfiLobTransferType getLobTransferType();

    protected final TgLobTransferType getIceaxeLobTransferType() {
        return TgLobTransferType.valueOf(getLobTransferType().name());
    }

    @Override
    protected TgFfiSqlClient createSqlClient() {
        var lobTransferType = getLobTransferType();
        return createSqlClient(lobTransferType);
    }

    private static final List<String> VALUES;
    static {
        var list = new ArrayList<String>();
        list.add(null);
        list.add("");
        list.add("abc");
        list.add(createTestData(1024 * 1024));
        list.add(createTestData(1024 * 1024 - 1));
        list.add(createTestData(1024 * 1024 + 1));
        VALUES = list;
    }

    private static String createTestData(int size) {
        byte[] data = new byte[size];
        for (int i = 0; i < size; i++) {
            data[i] = (byte) ('A' + (i % ('Z' - 'A' + 1)));
        }
        return new String(data);
    }

    @Test
    void insertJava() throws Exception {
        var sql = "insert into test values(:pk, :value)";
        var mapping = TgParameterMapping.of(TgBindVariable.ofInt("pk"), TgBindVariable.ofClob("value"));

        var connector = getTsurugiConnector(option -> {
            option.setLobTransferType(getIceaxeLobTransferType());
        });
        try (var session = connector.createSession(); //
                var ps = session.createStatement(sql, mapping)) {
            var manager = session.createTransactionManager(TgTxOption.ofOCC());

            manager.execute(transaction -> {
                var lobFactory = session.getLobFactory();
                int i = 0;
                for (var value : VALUES) {
                    try (var clob = lobFactory.uploadClob(value)) {
                        var parameter = TgBindParameters.of(TgBindParameter.of("pk", i++), TgBindParameter.of("value", clob));
                        transaction.executeAndGetCountDetail(ps, parameter);
                    }
                }
            });
        }

        selectFfi(false, DIRECT);
        selectFfi(false, DIRECT_FOR);
        selectFfi(true, DIRECT);
    }

    @Test
    void insertFfi_path() throws Exception {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var client = createSqlClient(); //
                var transaction = startOcc(client)) {

            var sql = "insert into test values(:pk, :value)";
            var placeholders = List.of( //
                    TgFfiSqlPlaceholder.ofAtomType(context, "pk", TgFfiAtomType.INT4), //
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.CLOB));
            try (var ps = client.prepare(context, sql, placeholders)) {
                int i = 0;
                for (var value : VALUES) {
                    TgFfiSqlParameter parameter;
                    if (value != null) {
                        var path = createLobFilePath("clob.txt");
                        Files.write(path, value.getBytes(StandardCharsets.UTF_8));
                        parameter = TgFfiSqlParameter.ofClob(context, "value", path);
                    } else {
                        parameter = TgFfiSqlParameter.ofNull(context, "value");
                    }
                    var parameters = List.of( //
                            TgFfiSqlParameter.ofInt4(context, "pk", i++), //
                            parameter);
                    try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
                        assertEquals(1, er.getRows(context));
                    }
                }
            }

            commitAndClose(client, transaction, DIRECT);
        }

        selectFfi(false, DIRECT);
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
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.CLOB));
            try (var ps = client.prepare(context, sql, placeholders)) {
                var parameters = List.of( //
                        TgFfiSqlParameter.ofInt4(context, "pk", 4), //
                        TgFfiSqlParameter.ofClobContents(context, "value", "ABC"));
                var e = assertThrows(TgFfiRuntimeException.class, () -> {
                    try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
                        assertEquals(1, er.getRows(context));
                    }
                });
                assertEquals(TgFfiRcType.CORE_SERVER_ERROR, e.getErrorType());
            }

            client.rollback(context, transaction);
            transaction.close(context);
        }
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void insertFfi_uploadClobFile(String pattern) throws Exception {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var client = createSqlClient(); //
                var transaction = startOcc(client)) {
            assertTrue(client.allowsLobOperation(context, TgFfiLobOperation.UPLOAD_LOB_FILE));

            var sql = "insert into test values(:pk, :value)";
            var placeholders = List.of( //
                    TgFfiSqlPlaceholder.ofAtomType(context, "pk", TgFfiAtomType.INT4), //
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.CLOB));
            try (var ps = client.prepare(context, sql, placeholders)) {
                int i = 0;
                for (var value : VALUES) {
                    TgFfiSqlParameter parameter;
                    if (value != null) {
                        var path = createLobFilePath("clob.bin");
                        Files.write(path, value.getBytes(StandardCharsets.UTF_8));
                        var timeout = Duration.ofSeconds(5);
                        var clob = switch (pattern) {
                        case DIRECT -> client.uploadClobFile(context, path);
                        case DIRECT_FOR -> client.uploadClobFileFor(context, path, timeout);
                        default -> jobTake(client.uploadClobFileAsync(context, path), pattern);
                        };
                        parameter = TgFfiSqlParameter.ofClob2(context, "value", clob);
                    } else {
                        parameter = TgFfiSqlParameter.ofNull(context, "value");
                    }
                    var parameters = List.of( //
                            TgFfiSqlParameter.ofInt4(context, "pk", i++), //
                            parameter);
                    try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
                        assertEquals(1, er.getRows(context));
                    }
                }
            }

            commitAndClose(client, transaction, DIRECT);

            upload_clob_file_argError(client);
            upload_clob_file_for_argError(client);
            upload_clob_file_async_argError(client);
        }

        selectFfi(false, pattern);
    }

    private void upload_clob_file_argError(TgFfiObject client) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg1 = manager.allocateString("path");
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = manager.allocateString("path");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    private void upload_clob_file_for_argError(TgFfiObject client) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg1 = manager.allocateString("path");
            var t = Duration.ofSeconds(5).toNanos();
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file_for(ctx, handle, arg1, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = MemorySegment.NULL;
            var t = Duration.ofSeconds(5).toNanos();
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file_for(ctx, handle, arg1, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = manager.allocateString("path");
            var t = Duration.ofSeconds(5).toNanos();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file_for(ctx, handle, arg1, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
        }
    }

    private void upload_clob_file_async_argError(TgFfiObject client) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg1 = manager.allocateString("path");
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file_async(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file_async(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = manager.allocateString("path");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_file_async(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY })
    void insertFfi_uploadClob(String pattern) throws Exception {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var client = createSqlClient(); //
                var transaction = startOcc(client)) {
            if (getLobTransferType() == TgFfiLobTransferType.PRIVILEGED) {
                assertFalse(client.allowsLobOperation(context, TgFfiLobOperation.UPLOAD_LOB));
                return;
            } else {
                assertTrue(client.allowsLobOperation(context, TgFfiLobOperation.UPLOAD_LOB));
            }

            var sql = "insert into test values(:pk, :value)";
            var placeholders = List.of( //
                    TgFfiSqlPlaceholder.ofAtomType(context, "pk", TgFfiAtomType.INT4), //
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.CLOB));
            try (var ps = client.prepare(context, sql, placeholders)) {
                int i = 0;
                for (var value : VALUES) {
                    TgFfiSqlParameter parameter;
                    if (value != null) {
                        var timeout = Duration.ofSeconds(5);
                        var clob = switch (pattern) {
                        case DIRECT -> client.uploadClob(context, value);
                        case DIRECT_FOR -> client.uploadClobFor(context, value, timeout);
                        default -> jobTake(client.uploadClobAsync(context, value), pattern);
                        };
                        parameter = TgFfiSqlParameter.ofClob2(context, "value", clob);
                    } else {
                        parameter = TgFfiSqlParameter.ofNull(context, "value");
                    }
                    var parameters = List.of( //
                            TgFfiSqlParameter.ofInt4(context, "pk", i++), //
                            parameter);
                    try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
                        assertEquals(1, er.getRows(context));
                    }
                }
            }

            commitAndClose(client, transaction, DIRECT);

            upload_clob_argError(client);
            upload_clob_for_argError(client);
            upload_clob_async_argError(client);
        }

        selectFfi(false, pattern);
    }

    private void upload_clob_argError(TgFfiObject client) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg1 = manager.allocateString("abc");
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = manager.allocateString("abc");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    private void upload_clob_for_argError(TgFfiObject client) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg1 = manager.allocateString("abc");
            var t = Duration.ofSeconds(5).toNanos();
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_for(ctx, handle, arg1, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = MemorySegment.NULL;
            var t = Duration.ofSeconds(5).toNanos();
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_for(ctx, handle, arg1, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = manager.allocateString("abc");
            var t = Duration.ofSeconds(5).toNanos();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_for(ctx, handle, arg1, t, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
        }
    }

    private void upload_clob_async_argError(TgFfiObject client) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = MemorySegment.NULL;
            var arg1 = manager.allocateString("abc");
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_async(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = MemorySegment.NULL;
            var out = manager.allocatePtrOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_async(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var handle = client.handle();
            var arg1 = manager.allocateString("abc");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_upload_clob_async(ctx, handle, arg1, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void insertFfi_uploader() throws Exception {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        try (var client = createSqlClient(); //
                var transaction = startOcc(client)) {
            if (getLobTransferType() == TgFfiLobTransferType.PRIVILEGED) {
                assertFalse(client.allowsLobOperation(context, TgFfiLobOperation.CREATE_LOB_UPLOADER));
                return;
            } else {
                assertTrue(client.allowsLobOperation(context, TgFfiLobOperation.CREATE_LOB_UPLOADER));
            }

            var sql = "insert into test values(:pk, :value)";
            var placeholders = List.of( //
                    TgFfiSqlPlaceholder.ofAtomType(context, "pk", TgFfiAtomType.INT4), //
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.CLOB));
            try (var ps = client.prepare(context, sql, placeholders)) {
                int i = 0;
                for (var value : VALUES) {
                    TgFfiSqlParameter parameter;
                    if (value != null) {
                        var timeout = Duration.ofSeconds(5);
                        var chunks = split(value);
                        try (@SuppressWarnings("unused")
                        var uploader = client.createClobUploader(context)) {
                        }
                        try (var uploader = client.createClobUploader(context)) {
                            uploader.uploadChunkUtf8(context, chunks.getFirst(), timeout);
                        }
                        try (var uploader = client.createClobUploader(context)) {
                            uploader.cancel(context);
                        }
                        try (var uploader = client.createClobUploader(context)) {
                            uploader.uploadChunkUtf8(context, chunks.getFirst(), timeout);
                            uploader.cancel(context);
                        }
                        try (var uploader = client.createClobUploader(context)) {
                            for (var chunk : chunks) {
                                uploader.uploadChunkUtf8(context, chunk, timeout);
                            }
                            var clob = uploader.finish(context, timeout);
                            parameter = TgFfiSqlParameter.ofClob2(context, "value", clob);
                        }
                    } else {
                        parameter = TgFfiSqlParameter.ofNull(context, "value");
                    }
                    var parameters = List.of( //
                            TgFfiSqlParameter.ofInt4(context, "pk", i++), //
                            parameter);
                    try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
                        assertEquals(1, er.getRows(context));
                    }
                }
            }

            commitAndClose(client, transaction, DIRECT);
        }

        selectFfi(false, DIRECT);
    }

    static List<byte[]> split(String value) {
        byte[] utf8 = value.getBytes(StandardCharsets.UTF_8);
        if (utf8.length == 0) {
            return List.of(utf8);
        }

        var chunks = new ArrayList<byte[]>();

        int chunkSize = 1024 * 1024;
        for (int offset = 0; offset < utf8.length; offset += chunkSize) {
            int end = Math.min(offset + chunkSize, utf8.length);
            chunks.add(Arrays.copyOfRange(utf8, offset, end));
        }

        return chunks;
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
                        var expected = VALUES.get(i);
                        if (qr.isNull(context)) {
                            assertNull(expected);
                        } else {
                            switch (pattern) {
                            case DIRECT:
                            default:
                                try (var value = qr.fetchClob(context)) {
                                    new ClobTester(client, transaction, expected, value).test();
                                }
                                break;
                            case DIRECT_FOR:
                                try (var value = qr.fetchForClob(context, Duration.ofSeconds(5))) {
                                    new ClobTester(client, transaction, expected, value).test();
                                }
                                break;
                            }
                        }
                    }
                    assertFalse(qr.nextColumn(context));

                    i++;
                }
                assertEquals(VALUES.size(), i);
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
                assertEquals(TgFfiAtomType.CLOB, c.getAtomType(context));
            }
        }
    }

    class ClobTester {
        private final TgFfiSqlClient client;
        private final TgFfiTransaction transaction;
        private final String expected;
        private final TgFfiClobReference clob;

        ClobTester(TgFfiSqlClient client, TgFfiTransaction transaction, String expected, TgFfiClobReference clob) {
            this.client = client;
            this.transaction = transaction;
            this.expected = expected;
            this.clob = clob;
        }

        void test() throws IOException {
            for (var pattern : List.of(DIRECT, DIRECT_FOR, TAKE, TAKE_FOR, TAKE_IF_READY)) {
                read_clob(pattern);
                get_clob_cache(pattern);
                copy_clob_to(pattern);
            }
            downloader();

            read_clob_argError();
            read_clob_for_argError();
            get_clob_cache_argError();
            get_clob_cache_for_argError();
            get_clob_cache_async_argError();
            copy_clob_to_argError();
            copy_clob_to_for_argError();
            copy_clob_to_async_argError();
            create_clob_downloader_argError();
        }

        private void read_clob(String pattern) throws IOException {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                assertTrue(client.allowsLobOperation(context, TgFfiLobOperation.READ_LOB));

                String value;
                switch (pattern) {
                case DIRECT:
                    value = client.readClob(context, transaction, clob);
                    break;
                case DIRECT_FOR:
                    value = client.readClobFor(context, transaction, clob, Duration.ofSeconds(5));
                    break;
                default:
                    return;
                }
                assertValue(value);
            }
        }

        private void get_clob_cache(String pattern) throws IOException {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                assertTrue(client.allowsLobOperation(context, TgFfiLobOperation.GET_LOB_CACHE));

                TgFfiLargeObjectCache cache;
                switch (pattern) {
                case DIRECT:
                    cache = client.getClobCache(context, transaction, clob);
                    break;
                case DIRECT_FOR:
                    cache = client.getClobCacheFor(context, transaction, clob, Duration.ofSeconds(5));
                    break;
                default:
                    try (var job = client.getClobCacheAsync(context, transaction, clob)) {
                        cache = jobTake(job, pattern);
                    }
                    break;
                }

                String path = cache.getPath(context);
                switch (getLobTransferType()) {
                case PRIVILEGED:
                    assertNotNull(path);

                    var value = Files.readAllBytes(Path.of(path));
                    assertValue(value);
                    break;
                default:
                    assertNull(path);
                    break;
                }
            }
        }

        private void copy_clob_to(String pattern) throws IOException {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                assertTrue(client.allowsLobOperation(context, TgFfiLobOperation.COPY_LOB_TO));

                var path = Path.of(System.getProperty("java.io.tmpdir")).resolve("TgFfiTypeClobTest.copy_clob_to." + System.currentTimeMillis() + ".bin");
                try {
                    switch (pattern) {
                    case DIRECT:
                        client.copyClobTo(context, transaction, clob, path);
                        break;
                    case DIRECT_FOR:
                        client.copyClobToFor(context, transaction, clob, path, Duration.ofSeconds(5));
                        break;
                    default:
                        try (var job = client.copyClobToAsync(context, transaction, clob, path)) {
                            Void value = jobTake(job, pattern);
                            assertNull(value);
                        }
                    }

                    var value = Files.readAllBytes(path);
                    assertValue(value);
                } finally {
                    Files.deleteIfExists(path);
                }
            }
        }

        private void downloader() throws IOException {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                assertTrue(client.allowsLobOperation(context, TgFfiLobOperation.CREATE_LOB_DOWNLOADER));

                var timeout = Duration.ofSeconds(5);
                try (@SuppressWarnings("unused")
                var downloader = client.createClobDownloader(context, transaction, clob, timeout)) {
                }
                try (var downloader = client.createClobDownloader(context, transaction, clob, timeout); //
                        var bos = new ByteArrayOutputStream()) {
                    for (;;) {
                        byte[] chunk = downloader.downloadChunk(context, 1024 * 1024, timeout);
                        if (chunk.length == 0) {
                            break;
                        }
                        bos.write(chunk);
                    }
                    assertValue(bos.toByteArray());
                }
                try (var downloader = client.createClobDownloader(context, transaction, clob, timeout); //
                        var bos = new ByteArrayOutputStream()) {
                    byte[] buffer = new byte[1024 * 1024];
                    for (;;) {
                        long length = downloader.downloadChunkInto(context, buffer, timeout);
                        if (length == 0) {
                            break;
                        }
                        bos.write(buffer, 0, (int) length);
                    }
                    assertValue(bos.toByteArray());
                }
            }
        }

        private void assertValue(byte[] value) {
            assertValue(new String(value, StandardCharsets.UTF_8));
        }

        private void assertValue(String value) {
            assertEquals(expected, value);
        }

        private void read_clob_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var out = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }

        private void read_clob_for_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob_for(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob_for(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob_for(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_read_clob_for(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), rc);
            }
        }

        private void get_clob_cache_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var out = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }

        private void get_clob_cache_for_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache_for(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache_for(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache_for(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache_for(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), rc);
            }
        }

        private void get_clob_cache_async_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache_async(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache_async(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var out = manager.allocatePtrOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache_async(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var out = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_clob_cache_async(ctx, handle, tx, arg1, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }

        private void copy_clob_to_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to(ctx, handle, tx, arg1, arg2);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to(ctx, handle, tx, arg1, arg2);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var arg2 = manager.allocateString("/path/to/destination");
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to(ctx, handle, tx, arg1, arg2);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var arg2 = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to(ctx, handle, tx, arg1, arg2);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }

        private void copy_clob_to_for_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var t = Duration.ofSeconds(5).toNanos();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_for(ctx, handle, tx, arg1, arg2, t);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var t = Duration.ofSeconds(5).toNanos();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_for(ctx, handle, tx, arg1, arg2, t);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var arg2 = manager.allocateString("/path/to/destination");
                var t = Duration.ofSeconds(5).toNanos();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_for(ctx, handle, tx, arg1, arg2, t);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var arg2 = MemorySegment.NULL;
                var t = Duration.ofSeconds(5).toNanos();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_for(ctx, handle, tx, arg1, arg2, t);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }

        private void copy_clob_to_async_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_async(ctx, handle, tx, arg1, arg2, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var arg2 = manager.allocateString("/path/to/destination");
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_async(ctx, handle, tx, arg1, arg2, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var arg2 = manager.allocateString("/path/to/destination");
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_async(ctx, handle, tx, arg1, arg2, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var arg2 = MemorySegment.NULL;
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_copy_clob_to_async(ctx, handle, tx, arg1, arg2, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
            }
        }

        private void create_clob_downloader_argError() {
            var manager = getFfiObjectManager();

            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = MemorySegment.NULL;
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_create_clob_downloader(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = MemorySegment.NULL;
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_create_clob_downloader(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = MemorySegment.NULL;
                var t = Duration.ofSeconds(5).toNanos();
                var out = manager.allocateHandleOut();
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_create_clob_downloader(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
            }
            try (var context = TgFfiContext.create(manager)) {
                var ctx = context.handle();
                var handle = client.handle();
                var tx = transaction.handle();
                var arg1 = clob.handle();
                var t = Duration.ofSeconds(5).toNanos();
                var out = MemorySegment.NULL;
                var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_create_clob_downloader(ctx, handle, tx, arg1, t, out);
                assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), rc);
            }
        }
    }
}
