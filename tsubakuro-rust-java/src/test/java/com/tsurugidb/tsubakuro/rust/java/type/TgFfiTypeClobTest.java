package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
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
import com.tsurugidb.iceaxe.sql.type.TgClob;
import com.tsurugidb.iceaxe.transaction.option.TgTxOption;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResultMetadata;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTypeClobTest extends TgFfiTester {

    @BeforeAll
    static void createTable() throws Exception {
        String sql = """
                create table test (
                  pk int primary key,
                  value clob
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
        var mapping = TgParameterMapping.of(TgBindVariable.ofInt("pk"), TgBindVariable.ofClob("value"));

        var connector = TsurugiConnector.of(getEndpointJava());
        try (var session = connector.createSession(); //
                var ps = session.createStatement(sql, mapping)) {
            var manager = session.createTransactionManager(TgTxOption.ofOCC());

            manager.execute(transaction -> {
                {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", 0), TgBindParameter.of("value", (TgClob) null));
                    transaction.executeAndGetCountDetail(ps, parameter);
                }
                try (var value = IceaxeObjectFactory.getDefaultInstance().createClob("abc", true)) {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", 1), TgBindParameter.of("value", value));
                    transaction.executeAndGetCountDetail(ps, parameter);
                }
                try (var value = IceaxeObjectFactory.getDefaultInstance().createClob("def", true)) {
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
                    assertEquals(TgFfiAtomType.CLOB, c.getAtomType(context));
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
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.CLOB));
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
                    TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.CLOB));
            try (var ps = client.prepare(context, sql, placeholders)) {
                var parameters = List.of( //
                        TgFfiSqlParameter.ofInt4(context, "pk", 4), //
                        TgFfiSqlParameter.ofClobContents(context, "value", "ghi"));
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
                                try (var value = qr.fetchClob(context)) {
                                }
                                break;
                            case DIRECT_FOR:
                                try (var value = qr.fetchForClob(context, Duration.ofSeconds(5))) {
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
                assertEquals(TgFfiAtomType.CLOB, c.getAtomType(context));
            }
        }
    }
}
