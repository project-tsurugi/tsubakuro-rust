package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertIterableEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameters;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.parameter.TgParameterMapping;
import com.tsurugidb.iceaxe.sql.result.TsurugiResultEntity;
import com.tsurugidb.iceaxe.transaction.option.TgTxOption;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.api.CDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.ExpectedColumn;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcBindParameter;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcGetDataArgument;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcColAttributeArgument.FieldIdentifier;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

public abstract class TgOdbcTypeTester<T> extends TgOdbcTester {
    protected final Logger LOG = LoggerFactory.getLogger(getClass());

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

    protected String expectedSqlType() {
        return sqlType();
    }

    protected abstract SqlDataType dataType();

    private void tableColumns(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.columns("test", wideChar);

            {
                assertTrue(stmt.fetch());

                new ExpectedColumn(1, "pk").initialize("INT").notNull() //
                        .test(stmt, wideChar);
            }
            {
                assertTrue(stmt.fetch());

                var expected = new ExpectedColumn(2, "value").initialize(expectedSqlType());
                expected.test(stmt, wideChar);
            }
            assertFalse(stmt.fetch());
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void javaToOdbc(boolean wideChar) throws Exception {
        createTable();
        tableColumns(wideChar);

        var values = values();
        insertJava(values);
        var actual = selectOdbc(wideChar);

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

    private List<T> selectOdbc(boolean wideChar) {
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.execDirect("select * from test order by pk", wideChar);

            describeColumn(stmt, wideChar);
            columnAttribute(stmt, wideChar);

            var actual = new ArrayList<T>();
            int rowIndex = 0;
            while (stmt.fetch()) {
                int pk = stmt.getDataInt(1);
                assertEquals(rowIndex, pk, "pk");

                var arg = getDataArgument(manager, wideChar);
                T value = stmt.getData(2, arg);
                actual.add(value);

                rowIndex++;
            }

            return actual;
        }
    }

    private void describeColumn(TgOdbcStmtHandle stmt, boolean wideChar) {
        int numberOfColumns = stmt.numResultCols();
        assertEquals(2, numberOfColumns);

        {
            var desc = stmt.describeCol(1, wideChar);
            assertEquals("pk", desc.columnName());
            assertEquals(SqlDataType.SQL_INTEGER, desc.dataType());
        }
        {
            var desc = stmt.describeCol(2, wideChar);
            assertEquals("value", desc.columnName());
            assertEquals(dataType(), desc.dataType());
        }
    }

    private void columnAttribute(TgOdbcStmtHandle stmt, boolean wideChar) {
        long numberOfColumns = stmt.colAttributeNumeric(0, FieldIdentifier.SQL_DESC_COUNT, wideChar);
        assertEquals(2, numberOfColumns);

        {
            final int i = 1;
            String name = stmt.colAttributeString(i, FieldIdentifier.SQL_DESC_NAME, wideChar);
            assertEquals("pk", name);

            long dataType = stmt.colAttributeNumeric(i, FieldIdentifier.SQL_DESC_CONCISE_TYPE, wideChar);
            assertEquals(SqlDataType.SQL_INTEGER, SqlDataType.fromValue((int) dataType));
        }
        {
            final int i = 2;
            String name = stmt.colAttributeString(i, FieldIdentifier.SQL_DESC_NAME, wideChar);
            assertEquals("value", name);

            long dataType = stmt.colAttributeNumeric(i, FieldIdentifier.SQL_DESC_CONCISE_TYPE, wideChar);
            assertEquals(dataType(), SqlDataType.fromValue((int) dataType));
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void odbcToJava(boolean wideChar) throws Exception {
        createTable();

        var values = values();
        insertOdbc(values, wideChar);
        var actual = selectJava();

        assertValueList(values, actual);
    }

    private void insertOdbc(List<T> values, boolean wideChar) {
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            var sql = "insert into test values(?, ?)";
            stmt.prepare(sql, wideChar);

            int pk = 0;
            for (T value : values) {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, pk));
                stmt.bindParameter(2, bindParameter(manager, value, wideChar));
                stmt.execute();

                pk++;
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

    protected abstract TgOdbcGetDataArgument<T> getDataArgument(TgOdbcManager manager, boolean wideChar);

    protected abstract TgOdbcBindParameter bindParameter(TgOdbcManager manager, T value, boolean wideChar);

    //

    private static final List<CDataType> VALUE_TYPES = List.of( //
            CDataType.SQL_C_BIT, //
            CDataType.SQL_C_TINYINT, CDataType.SQL_C_STINYINT, CDataType.SQL_C_UTINYINT, //
            CDataType.SQL_C_SHORT, CDataType.SQL_C_SSHORT, CDataType.SQL_C_USHORT, //
            CDataType.SQL_C_LONG, CDataType.SQL_C_SLONG, CDataType.SQL_C_ULONG, //
            CDataType.SQL_C_SBIGINT, CDataType.SQL_C_UBIGINT, //
            CDataType.SQL_C_FLOAT, CDataType.SQL_C_DOUBLE, //
            CDataType.SQL_C_NUMERIC, //
            CDataType.SQL_C_CHAR, CDataType.SQL_C_WCHAR, //
            CDataType.SQL_C_BINARY, //
            CDataType.SQL_C_DATE, CDataType.SQL_C_TYPE_DATE, //
            CDataType.SQL_C_TIME, CDataType.SQL_C_TYPE_TIME, //
            CDataType.SQL_C_TIMESTAMP, CDataType.SQL_C_TYPE_TIMESTAMP, //
            CDataType.SQL_C_DEFAULT);

    protected void testBindParameterCombination(SqlDataType parameterType) {
        for (var valueType : VALUE_TYPES) {
            try {
                testBindParameter(valueType, parameterType);
            } catch (AssertionError e) {
                var message = "parameterType=%s, valueType=%s assertion. %s".formatted(parameterType, valueType, e.getMessage());
                throw new AssertionError(message, e);
            } catch (Throwable e) {
                var message = "parameterType=%s, valueType=%s error. %s".formatted(parameterType, valueType, e.getMessage());
                throw new RuntimeException(message, e);
            }
        }
    }

    private void testBindParameter(CDataType valueType, SqlDataType parameterType) throws IOException, InterruptedException {
        LOG.info("testBindParameter(): parameterType={}, valueType={}", parameterType, valueType);

        createTable();

        var values = values();
        var expectedList = new ArrayList<T>(values.size());
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            var sql = "insert into test values(?, ?)";
            stmt.prepare(sql, false);

            int pk = 0;
            for (T value : values) {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, pk));

                TgOdbcBindParameter parameter;
                boolean validParameter = true;
                T expectedValue = null;
                boolean supported = true;
                if (value != null) {
                    var expected = bindValue(manager, valueType, value);
                    if (expected != null) {
                        parameter = expected.parameter;
                        if (parameter == null) {
                            continue;
                        }
                        validParameter = expected.validParameter;
                        expectedValue = expected.expectedValue;
                    } else {
                        parameter = TgOdbcBindParameter.ofBinary(manager, new byte[256]);
                        supported = false;
                    }
                } else {
                    parameter = new TgOdbcBindParameter().nullValue();
                    expectedValue = null;
                }
                parameter.valueType(valueType);
                parameter.parameterType(parameterType);
                stmt.bindParameter(2, parameter);

                if (supported) {
                    if (validParameter) {
                        try {
                            stmt.execute();
                        } catch (Throwable e) {
                            LOG.error("{}\nvalue={}, expectedValue={}\nparameter={}", e.getMessage(), value, expectedValue, parameter);
                            throw e;
                        }
                    } else {
                        var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                            stmt.execute();
                        });
                        assertFalse(e.getMessage().contains("Unsupported"), e.getMessage());
                        continue;
                    }
                    expectedList.add(expectedValue);
                    pk++;
                } else {
                    var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                        stmt.execute();
                    });
                    assertTrue(e.getMessage().contains("Unsupported"), () -> e.getMessage());
                }
            }
        }

        {
            var actual = selectJava();
            assertValueList(expectedList, actual);
        }
        {
            var actual = selectOdbc(false);
            assertValueList(expectedList, actual);
        }
    }

    protected class ExpectedBindValue {
        private final TgOdbcBindParameter parameter;
        private final boolean validParameter;
        private final T expectedValue;

        public ExpectedBindValue(TgOdbcBindParameter parameter, T expectedValue) {
            this(parameter, true, expectedValue);
        }

        public ExpectedBindValue(TgOdbcBindParameter parameter, boolean validParameter, T expectedValue) {
            this.parameter = parameter;
            this.validParameter = validParameter;
            this.expectedValue = expectedValue;
        }
    }

    protected abstract ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, T value);

    //

    protected void testGetDataCombination() throws Exception {
        createTable();

        var values = values();
        insertJava(values);

        var manager = getOdbcManager();
        testGetData(values, TgOdbcGetDataArgument.ofBoolean(manager).targetType(CDataType.SQL_C_BIT));
        testGetData(values, TgOdbcGetDataArgument.ofByte(manager).targetType(CDataType.SQL_C_TINYINT));
        testGetData(values, TgOdbcGetDataArgument.ofByte(manager).targetType(CDataType.SQL_C_STINYINT));
        testGetData(values, TgOdbcGetDataArgument.ofByte(manager).targetType(CDataType.SQL_C_UTINYINT));
        testGetData(values, TgOdbcGetDataArgument.ofShort(manager).targetType(CDataType.SQL_C_SHORT));
        testGetData(values, TgOdbcGetDataArgument.ofShort(manager).targetType(CDataType.SQL_C_SSHORT));
        testGetData(values, TgOdbcGetDataArgument.ofShort(manager).targetType(CDataType.SQL_C_USHORT));
        testGetData(values, TgOdbcGetDataArgument.ofInt(manager).targetType(CDataType.SQL_C_LONG));
        testGetData(values, TgOdbcGetDataArgument.ofInt(manager).targetType(CDataType.SQL_C_SLONG));
        testGetData(values, TgOdbcGetDataArgument.ofInt(manager).targetType(CDataType.SQL_C_ULONG));
        testGetData(values, TgOdbcGetDataArgument.ofLong(manager).targetType(CDataType.SQL_C_SBIGINT));
        testGetData(values, TgOdbcGetDataArgument.ofLong(manager).targetType(CDataType.SQL_C_UBIGINT));
        testGetData(values, TgOdbcGetDataArgument.ofFloat(manager).targetType(CDataType.SQL_C_FLOAT));
        testGetData(values, TgOdbcGetDataArgument.ofDouble(manager).targetType(CDataType.SQL_C_DOUBLE));
        testGetData(values, TgOdbcGetDataArgument.ofDecimal(manager).targetType(CDataType.SQL_C_NUMERIC));
        testGetData(values, TgOdbcGetDataArgument.ofString(manager, 1024, false).targetType(CDataType.SQL_C_CHAR));
        testGetData(values, TgOdbcGetDataArgument.ofString(manager, 1024, true).targetType(CDataType.SQL_C_WCHAR));
        testGetData(values, TgOdbcGetDataArgument.ofBinary(manager, 1024).targetType(CDataType.SQL_C_BINARY));
        testGetData(values, TgOdbcGetDataArgument.ofDate(manager).targetType(CDataType.SQL_C_DATE));
        testGetData(values, TgOdbcGetDataArgument.ofDate(manager).targetType(CDataType.SQL_C_TYPE_DATE));
        testGetData(values, TgOdbcGetDataArgument.ofTime(manager).targetType(CDataType.SQL_C_TIME));
        testGetData(values, TgOdbcGetDataArgument.ofTime(manager).targetType(CDataType.SQL_C_TYPE_TIME));
        testGetData(values, TgOdbcGetDataArgument.ofTimestamp(manager).targetType(CDataType.SQL_C_TIMESTAMP));
        testGetData(values, TgOdbcGetDataArgument.ofTimestamp(manager).targetType(CDataType.SQL_C_TYPE_TIMESTAMP));
    }

    private <C> void testGetData(List<T> values, TgOdbcGetDataArgument<C> arg) {
        LOG.info("testGetData(): targetType={}", arg.targetType());

        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by pk", false);

            int rowIndex = 0;
            while (stmt.fetch()) {
                int pk = stmt.getDataInt(1);
                assertEquals(rowIndex, pk, "pk");

                T value = values.get(rowIndex);
                C actual;
                try {
                    actual = stmt.getData(2, arg);
                } catch (Throwable e) {
                    if (e instanceof TgOdbcRuntimeException re) {
                        try {
                            assertGetData(value, arg.targetType(), null, re);
                        } catch (Throwable e2) {
                            LOG.error("{}\n{}\npk={}, targetType={}\nvalue ={}", e.getMessage(), e2.getMessage(), pk, arg.targetType(), value);
                            throw e2;
                        }
                        rowIndex++;
                        continue;
                    }
                    LOG.error("{}\npk={}, targetType={}\nvalue ={}", e.getMessage(), pk, arg.targetType(), value);
                    throw e;
                }
                if (value == null) {
                    assertNull(actual);
                } else {
                    try {
                        assertGetData(value, arg.targetType(), actual, null);
                    } catch (Throwable e) {
                        LOG.error("{}\npk={}, targetType={}\nvalue ={}\nactual={}", e.getMessage(), pk, arg.targetType(), value, actual);
                        throw e;
                    }
                }

                rowIndex++;
            }
            assertEquals(values.size(), rowIndex);
        }
    }

    protected abstract void assertGetData(T value, CDataType targetType, Object actual, TgOdbcRuntimeException e);

    //

    protected void assertValueList(List<T> expected, List<T> actual) {
        try {
            assertIterableEquals(expected, actual);
        } catch (Throwable e) {
            LOG.error("{}\nexpected={}\nactual=  {}", e.getMessage(), expected, actual);
            throw e;
        }
    }
}
