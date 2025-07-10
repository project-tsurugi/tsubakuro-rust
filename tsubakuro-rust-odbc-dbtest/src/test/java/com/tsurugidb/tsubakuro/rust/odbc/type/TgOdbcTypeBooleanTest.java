package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindParameters;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.parameter.TgParameterMapping;
import com.tsurugidb.iceaxe.transaction.option.TgTxOption;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.api.CDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.ExpectedColumn;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcGetDataArgument;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class TgOdbcTypeBooleanTest extends TgOdbcTester {
    protected final Logger LOG = LoggerFactory.getLogger(getClass());

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

        var connector = TsurugiConnector.of(getEndpointJava());
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

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void tableColumns(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.columns("test", wideChar);

            {
                assertTrue(stmt.fetch());

                new ExpectedColumn(1, "pk").initialize("INT").notNull() //
                        .test(stmt, wideChar);
            }
            {
                assertTrue(stmt.fetch());

                var expected = new ExpectedColumn(2, "value").initialize("INT"); // TODO BOOLEAN
                expected.test(stmt, wideChar);
            }
            assertFalse(stmt.fetch());
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void selectOdbc(boolean wideChar) {
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.execDirect("select pk, value<>0 as value from test order by pk", wideChar); // TODO select *

            describeColumn(stmt, wideChar);

            var actual = new ArrayList<Boolean>();
            int rowIndex = 0;
            while (stmt.fetch()) {
                int pk = stmt.getDataInt(1);
                assertEquals(rowIndex, pk, "pk");

                var arg = getDataArgument(manager, wideChar);
                Boolean value = stmt.getData(2, arg);
                actual.add(value);

                rowIndex++;
            }

            assertNull(actual.get(0));
            assertTrue(actual.get(1));
            assertFalse(actual.get(2));
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
            assertEquals(SqlDataType.SQL_BIT, desc.dataType());
        }
    }

    private TgOdbcGetDataArgument<Boolean> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofBoolean(manager);
    }

    @Test
    void getDataCombination() throws Exception {
        var values = new ArrayList<Boolean>();
        values.add(null);
        values.add(true);
        values.add(false);

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

    private <C> void testGetData(List<Boolean> values, TgOdbcGetDataArgument<C> arg) {
        LOG.info("testGetData(): targetType={}", arg.targetType());

        try (var stmt = createStmt()) {
            stmt.execDirect("select pk, value<>0 as value from test order by pk", false); // TODO select *

            int rowIndex = 0;
            while (stmt.fetch()) {
                int pk = stmt.getDataInt(1);
                assertEquals(rowIndex, pk, "pk");

                Boolean value = values.get(rowIndex);
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

    protected void assertGetData(Boolean value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
        switch (targetType) {
        case SQL_C_BIT:
            assertEquals(value, (boolean) actual);
            break;
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
        case SQL_C_UTINYINT:
            assertEquals(value ? 1 : 0, (byte) actual);
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
        case SQL_C_USHORT:
            assertEquals(value ? 1 : 0, (short) actual);
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
        case SQL_C_ULONG:
            assertEquals(value ? 1 : 0, (int) actual);
            break;
        case SQL_C_SBIGINT:
        case SQL_C_UBIGINT:
            assertEquals(value ? 1 : 0, (long) actual);
            break;
        case SQL_C_FLOAT:
            assertEquals(value ? 1 : 0, (float) actual);
            break;
        case SQL_C_DOUBLE:
            assertEquals(value ? 1 : 0, (double) actual);
            break;
        case SQL_C_NUMERIC:
            if (value) {
                assertTrue(((BigDecimal) actual).compareTo(BigDecimal.ONE) == 0);
            } else {
                assertTrue(((BigDecimal) actual).compareTo(BigDecimal.ZERO) == 0);
            }
            break;
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
            assertEquals(value.toString(), (String) actual);
            break;
        case SQL_C_BINARY:
        case SQL_C_DATE:
        case SQL_C_TYPE_DATE:
        case SQL_C_TIME:
        case SQL_C_TYPE_TIME:
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            assertTrue(re.getMessage().contains("Unsupported"), () -> re.getMessage());
            break;
        default:
            throw new InternalError("Not yet implements targetType " + targetType);
        }
    }
}
