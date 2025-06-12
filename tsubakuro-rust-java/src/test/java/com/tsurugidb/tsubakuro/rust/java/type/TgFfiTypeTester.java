package com.tsurugidb.tsubakuro.rust.java.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertIterableEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameters;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.parameter.TgParameterMapping;
import com.tsurugidb.iceaxe.sql.result.TsurugiResultEntity;
import com.tsurugidb.iceaxe.transaction.option.TgTxOption;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResultMetadata;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

public abstract class TgFfiTypeTester<T> extends TgFfiTester {

    protected void createTable() {
        String sql = """
                create table test (
                  pk int primary key,
                  value %s
                )
                """.formatted(sqlType());
        dropAndCreateTable("test", sql);
    }

    protected abstract String sqlType();

    private String sqlTypeName() {
        String type = sqlType();
        int n = type.indexOf('(');
        if (n < 0) {
            return type.toUpperCase();
        } else {
            return type.substring(0, n).toUpperCase();
        }
    }

    private void tableMetadata() {
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
                    assertEquals(ffiAtomType(), c.getAtomType(context));
                    assertEquals(sqlTypeName(), c.getSqlTypeName(context));
                }
            }
        }
    }

    @ParameterizedTest
    @ValueSource(strings = { DIRECT, DIRECT_FOR })
    void javaToFfi(String pattern) throws Exception {
        createTable();
        tableMetadata();

        var values = values();
        insertJava(values);
        var actual = selectFfi(pattern);

        assertValueList(values, actual);
    }

    private void insertJava(List<T> values) throws IOException, InterruptedException {
        var sql = "insert into test values(:pk, :value)";
        var mapping = TgParameterMapping.of(TgBindVariable.ofInt("pk"), bindVariable("value"));

        var connector = TsurugiConnector.of(getEndpointJava());
        try (var session = connector.createSession(); //
                var ps = session.createStatement(sql, mapping)) {
            var manager = session.createTransactionManager(TgTxOption.ofOCC());

            manager.execute(transaction -> {
                int i = 0;
                for (var value : values) {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", i), bindParameter("value", value));
                    transaction.executeAndGetCountDetail(ps, parameter);

                    i++;
                }
            });
        }
    }

    private List<T> selectFfi(String pattern) {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        var actual = new ArrayList<T>();
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
                    T value;
                    if (qr.isNull(context)) {
                        value = null;
                    } else {
                        value = fetch(context, qr, pattern);
                    }
                    actual.add(value);

                    i++;
                }
            }

            commitAndClose(client, transaction, DIRECT_FOR);
        }

        return actual;
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
                assertEquals(ffiAtomType(), c.getAtomType(context));
            }
        }
    }

    @Test
    void ffiToJava() throws Exception {
        createTable();

        var values = values();
        insertFfi(values);
        var actual = selectJava();

        assertValueList(values, actual);
    }

    private void insertFfi(List<T> values) {
        var manager = getFfiObjectManager();
        var context = TgFfiContext.create(manager);

        var sql = "insert into test values(:pk, :value)";
        var placeholders = List.of(TgFfiSqlPlaceholder.ofAtomType(context, "pk", TgFfiAtomType.INT4), TgFfiSqlPlaceholder.ofAtomType(context, "value", ffiAtomType()));

        try (var client = createSqlClient(); //
                var ps = client.prepare(context, sql, placeholders)) {
            try (var transaction = startOcc(client)) {

                int i = 0;
                for (var value : values) {
                    var parameters = List.of(TgFfiSqlParameter.ofInt4(context, "pk", i), (value != null) ? ffiParameter(context, "value", value) : TgFfiSqlParameter.ofNull(context, "value"));
                    try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
                        assertEquals(1, er.getRows(context));
                    }
                    i++;
                }

                commitAndClose(client, transaction, DIRECT_FOR);
            }
        }
    }

    private List<T> selectJava() throws IOException, InterruptedException {
        var actual = new ArrayList<T>();

        var connector = TsurugiConnector.of(getEndpointJava());
        try (var session = connector.createSession()) {
            var manager = session.createTransactionManager(TgTxOption.ofOCC());

            var list = manager.executeAndGetList("select * from test order by pk");
            int i = 0;
            for (var entity : list) {
                int pk = entity.getInt("pk");
                assertEquals(i, pk);

                T value;
                if (entity.getValueOrNull("value") == null) {
                    value = null;
                } else {
                    value = get(entity, "value");
                }
                actual.add(value);

                i++;
            }
        }

        return actual;
    }

    protected abstract List<T> values();

    protected abstract TgBindVariable<T> bindVariable(String name);

    protected abstract TgBindParameter bindParameter(String name, T value);

    protected abstract T get(TsurugiResultEntity entity, String name);

    protected abstract TgFfiAtomType ffiAtomType();

    protected abstract TgFfiSqlParameter ffiParameter(TgFfiContext context, String name, T value);

    protected T fetch(TgFfiContext context, TgFfiSqlQueryResult qr, String pattern) {
        switch (pattern) {
        case DIRECT:
            return fetch(context, qr);
        case DIRECT_FOR:
            return fetchFor(context, qr, Duration.ofSeconds(5));
        default:
            throw new AssertionError();
        }
    }

    protected abstract T fetch(TgFfiContext context, TgFfiSqlQueryResult qr);

    protected abstract T fetchFor(TgFfiContext context, TgFfiSqlQueryResult qr, Duration timeout);

    protected void assertValueList(List<T> expected, List<T> actual) {
        assertIterableEquals(expected, actual);
    }
}
