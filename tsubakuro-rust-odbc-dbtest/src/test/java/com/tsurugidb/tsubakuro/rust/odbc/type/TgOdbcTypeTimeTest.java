package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeFormatterBuilder;
import java.time.temporal.ChronoField;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Test;

import com.tsurugidb.iceaxe.sql.parameter.TgBindParameter;
import com.tsurugidb.iceaxe.sql.parameter.TgBindVariable;
import com.tsurugidb.iceaxe.sql.result.TsurugiResultEntity;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.api.CDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcBindParameter;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcGetDataArgument;

class TgOdbcTypeTimeTest extends TgOdbcTypeTester<LocalTime> {

    @Override
    protected String sqlType() {
        return "time";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_TYPE_TIME;
    }

    @Override
    protected List<LocalTime> values() {
        var list = new ArrayList<LocalTime>();
        list.add(LocalTime.now());
        list.add(LocalTime.of(0, 0, 0));
        list.add(LocalTime.of(1, 2, 3, 456));
        list.add(LocalTime.of(1, 2, 3, 456_000_000));
        list.add(LocalTime.of(12, 30, 59, 123456789));
        list.add(LocalTime.of(23, 59, 59, 999_999_999));
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<LocalTime> bindVariable(String name) {
        return TgBindVariable.ofTime(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, LocalTime value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected LocalTime get(TsurugiResultEntity entity, String name) {
        return entity.getTime(name);
    }

    @Override
    protected TgOdbcGetDataArgument<LocalTime> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofTime(manager);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, LocalTime value, boolean wideChar) {
        return TgOdbcBindParameter.ofTime(manager, value);
    }

    @Test
    void bindParameterCombination() {
        testBindParameterCombination(SqlDataType.SQL_TYPE_TIME);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, LocalTime value) {
        TgOdbcBindParameter parameter;
        LocalTime expected;
        switch (valueType) {
        case SQL_C_CHAR:
            parameter = TgOdbcBindParameter.ofStringUtf8(manager, value.toString());
            expected = value;
            break;
        case SQL_C_WCHAR:
            parameter = TgOdbcBindParameter.ofStringUtf16(manager, value.toString());
            expected = value;
            break;
        case SQL_C_TIME:
        case SQL_C_TYPE_TIME:
            parameter = TgOdbcBindParameter.ofTime(manager, value);
            expected = value;
            break;
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            parameter = TgOdbcBindParameter.ofTimestamp(manager, LocalDateTime.of(LocalDate.EPOCH, value));
            expected = value;
            break;
        default:
            return null;
        }
        return new ExpectedBindValue(parameter, expected);
    }

    @Test
    void getDataCombination() throws Exception {
        testGetDataCombination();
    }

    @Override
    protected void assertGetData(LocalTime value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
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
        case SQL_C_DATE:
        case SQL_C_TYPE_DATE:
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            assertTrue(re.getMessage().contains("Unsupported"), () -> re.getMessage());
            break;
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
            assertEquals(toExpectedString(value), (String) actual);
            break;
        case SQL_C_TIME:
        case SQL_C_TYPE_TIME:
            assertEquals(expectedTime(value), (LocalTime) actual);
            break;
        default:
            throw new InternalError("Not yet implements targetType " + targetType);
        }
    }

    private static final DateTimeFormatter FORMATTER = new DateTimeFormatterBuilder() //
            .appendPattern("HH:mm:ss") //
            .appendFraction(ChronoField.NANO_OF_SECOND, 1, 9, true) // 最小1桁, 最大9桁, 常に小数点あり
            .toFormatter();

    private static String toExpectedString(LocalTime value) {
        return value.format(FORMATTER);
    }

    @Override
    protected void assertValueList(List<LocalTime> expected, List<LocalTime> actual) {
        try {
            assertEquals(expected.size(), actual.size());
            for (int i = 0; i < actual.size(); i++) {
                try {
                    assertEquals(expected.get(i), actual.get(i));
                } catch (AssertionError ignore) {
                    assertEquals(expectedTime(expected.get(i)), actual.get(i));
                }
            }
        } catch (Throwable e) {
            LOG.error("{}\nexpected={}\nactual=  {}", e.getMessage(), expected, actual);
            throw e;
        }
    }

    private static LocalTime expectedTime(LocalTime value) {
        if (value == null) {
            return null;
        }
        return value.truncatedTo(ChronoUnit.SECONDS);
    }
}
