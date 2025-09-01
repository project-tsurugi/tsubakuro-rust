package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.time.Duration;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameters;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.parameter.TgParameterMapping;
import com.tsurugidb.iceaxe.transaction.option.TgTxOption;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResultMetadata;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTypeBooleanTest extends TgFfiTester {

    @BeforeAll
    static void createTable() throws Exception {
        String sql = """
                create table test (
                  pk int primary key,
                  value int
                )
                """; // TODO boolean
        dropAndCreateTable("test", sql);
        insertJava();
    }

    static void insertJava() throws IOException, InterruptedException {
        var sql = "insert into test values(:pk, :value)";
        var mapping = TgParameterMapping.of(TgBindVariable.ofInt("pk"), TgBindVariable.ofInt("value"));

        var connector = getTsurugiConnector();
        try (var session = connector.createSession(); //
                var ps = session.createStatement(sql, mapping)) {
            var manager = session.createTransactionManager(TgTxOption.ofOCC());

            manager.execute(transaction -> {
                {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", 0), TgBindParameter.of("value", (Integer) null)); // TODO Boolean
                    transaction.executeAndGetCountDetail(ps, parameter);
                }
                {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", 1), TgBindParameter.of("value", 1)); // TODO true
                    transaction.executeAndGetCountDetail(ps, parameter);
                }
                {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", 2), TgBindParameter.of("value", 0)); // TODO false
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
                    assertEquals(TgFfiAtomType.INT4, c.getAtomType(context)); // TODO BOOLEAN
                    // TODO assertEquals("BOOLEAN", c.getSqlTypeName(context));
                    // TODO assertEquals("BOOLEAN", c.getSqlType(context));
                }
            }
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

            // TODO select * from
            try (var qr = client.query(context, transaction, "select pk, value<>0 as value from test order by pk")) {
                queryResultMetadata(qr.getMetadata(context));

                int i = 0;
                while (qr.nextRow(context)) {
                    assertTrue(qr.nextColumn(context));
                    assertFalse(qr.isNull(context));
                    int pk = qr.fetchInt4(context);
                    assertEquals(i, pk);

                    assertTrue(qr.nextColumn(context));
                    if (!skip) {
                        if (i == 0) {
                            assertTrue(qr.isNull(context));
                        } else {
                            boolean value;
                            switch (pattern) {
                            case DIRECT:
                                value = qr.fetchBoolean(context);
                                break;
                            case DIRECT_FOR:
                                value = qr.fetchForBoolean(context, Duration.ofSeconds(5));
                                break;
                            default:
                                throw new AssertionError();
                            }
                            boolean expected = (i == 1);
                            assertEquals(expected, value);
                        }
                    }
                    assertFalse(qr.nextColumn(context));

                    i++;
                }
                assertEquals(3, i);
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
                assertEquals(TgFfiAtomType.BOOLEAN, c.getAtomType(context));
            }
        }
    }
}
