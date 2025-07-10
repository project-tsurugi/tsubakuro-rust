package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
import java.time.OffsetDateTime;
import java.time.ZoneOffset;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeFormatterBuilder;
import java.time.temporal.ChronoField;
import java.time.temporal.ChronoUnit;
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

class TgOdbcTypeTimestampTzTest extends TgOdbcTester {
    protected final Logger LOG = LoggerFactory.getLogger(getClass());

    @BeforeAll
    static void createTable() throws Exception {
        String sql = """
                create table test (
                  pk int primary key,
                  value timestamp with time zone
                )
                """;
        dropAndCreateTable("test", sql);
        insertJava();
    }

    static void insertJava() throws IOException, InterruptedException {
        var sql = "insert into test values(:pk, :value)";
        var mapping = TgParameterMapping.of(TgBindVariable.ofInt("pk"), TgBindVariable.ofOffsetDateTime("value"));

        var connector = TsurugiConnector.of(getEndpointJava());
        try (var session = connector.createSession(); //
                var ps = session.createStatement(sql, mapping)) {
            var manager = session.createTransactionManager(TgTxOption.ofOCC());

            manager.execute(transaction -> {
                var values = values();
                int pk = 0;
                for (var value : values) {
                    var parameter = TgBindParameters.of(TgBindParameter.of("pk", pk++), TgBindParameter.of("value", value));
                    transaction.executeAndGetCountDetail(ps, parameter);
                }
            });
        }
    }

    private static final OffsetDateTime NOW = OffsetDateTime.now();

    private static List<OffsetDateTime> values() {
        var list = new ArrayList<OffsetDateTime>();
        list.add(NOW);
        for (var offset : List.of(ZoneOffset.UTC, ZoneOffset.ofHours(9))) {
            list.add(OffsetDateTime.of(1969, 12, 31, 23, 59, 59, 999_999_999, offset));
            list.add(OffsetDateTime.of(1970, 1, 1, 0, 0, 0, 0, offset));
            list.add(OffsetDateTime.of(2025, 2, 7, 12, 30, 59, 123456789, offset));
            list.add(OffsetDateTime.of(9999, 12, 31, 23, 59, 59, 999_999_999, offset));
            list.add(OffsetDateTime.of(-1, 1, 1, 0, 0, 0, 0, offset));
            list.add(OffsetDateTime.of(0, 1, 1, 0, 0, 0, 0, offset));
        }
        list.add(null);
        return list;
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

                var expected = new ExpectedColumn(2, "value").initialize("TIMESTAMP WITH TIME ZONE");
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

            stmt.execDirect("select * from test order by pk", wideChar);

            describeColumn(stmt, wideChar);

            var actual = new ArrayList<OffsetDateTime>();
            int rowIndex = 0;
            while (stmt.fetch()) {
                int pk = stmt.getDataInt(1);
                assertEquals(rowIndex, pk, "pk");

                var arg = getDataArgument(manager, wideChar);
                OffsetDateTime value = stmt.getData(2, arg);
                actual.add(value);

                rowIndex++;
            }

            var expected = values();
            assertValueList(expected, actual);
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
            assertEquals(SqlDataType.SQL_UNKNOWN_TYPE, desc.dataType());
        }
    }

    private TgOdbcGetDataArgument<OffsetDateTime> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofTimestampTz(manager);
    }

    @Test
    void getDataCombination() throws Exception {
        var values = values();

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

    private <C> void testGetData(List<OffsetDateTime> values, TgOdbcGetDataArgument<C> arg) {
        LOG.info("testGetData(): targetType={}", arg.targetType());

        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by pk", false);

            int rowIndex = 0;
            while (stmt.fetch()) {
                int pk = stmt.getDataInt(1);
                assertEquals(rowIndex, pk, "pk");

                OffsetDateTime value = values.get(rowIndex);
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

    protected void assertGetData(OffsetDateTime value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
        switch (targetType) {
        case SQL_C_BIT:
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
        case SQL_C_UTINYINT:
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
        case SQL_C_USHORT:
        case SQL_C_LONG:
        case SQL_C_SLONG:
        case SQL_C_ULONG:
        case SQL_C_SBIGINT:
        case SQL_C_UBIGINT:
        case SQL_C_FLOAT:
        case SQL_C_DOUBLE:
        case SQL_C_NUMERIC:
        case SQL_C_BINARY:
            assertTrue(re.getMessage().contains("Unsupported"), () -> re.getMessage());
            break;
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
            assertEquals(toExpectedString(value), (String) actual);
            break;
        case SQL_C_DATE:
        case SQL_C_TYPE_DATE:
            assertEquals(expectedTimestamp(value).toLocalDate(), (LocalDate) actual);
            break;
        case SQL_C_TIME:
        case SQL_C_TYPE_TIME:
            assertEquals(expectedTimestamp(value).toLocalTime().truncatedTo(ChronoUnit.SECONDS), (LocalTime) actual);
            break;
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            assertEquals(expectedTimestamp(value).toLocalDateTime(), (LocalDateTime) actual);
            break;
        default:
            throw new InternalError("Not yet implements targetType " + targetType);
        }
    }

    private static final DateTimeFormatter FORMATTER = new DateTimeFormatterBuilder() //
            .appendPattern("uuuu-MM-dd HH:mm:ss") //
            .appendFraction(ChronoField.NANO_OF_SECOND, 1, 9, true) // 最小1桁, 最大9桁, 常に小数点あり
            .appendOffset("+HH:MM", "+00:00") //
            .toFormatter();

    private static String toExpectedString(OffsetDateTime value) {
        OffsetDateTime utc = value.withOffsetSameInstant(ZoneOffset.UTC);
        return utc.format(FORMATTER);
    }

    protected void assertValueList(List<OffsetDateTime> expected, List<OffsetDateTime> actual) {
        try {
            assertEquals(expected.size(), actual.size());
            for (int i = 0; i < actual.size(); i++) {
                assertEquals(expectedTimestamp(expected.get(i)), actual.get(i));
            }
        } catch (Throwable e) {
            LOG.error("{}\nexpected={}\nactual=  {}", e.getMessage(), expected, actual);
            throw e;
        }
    }

    private static OffsetDateTime expectedTimestamp(OffsetDateTime value) {
        if (value == null) {
            return null;
        }
        return value.withOffsetSameInstant(ZoneOffset.UTC);
    }
}
