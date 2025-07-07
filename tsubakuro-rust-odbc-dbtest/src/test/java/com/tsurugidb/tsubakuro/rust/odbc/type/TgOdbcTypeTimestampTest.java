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

class TgOdbcTypeTimestampTest extends TgOdbcTypeTester<LocalDateTime> {

    @Override
    protected String sqlType() {
        return "timestamp";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_TYPE_TIMESTAMP;
    }

    @Override
    protected List<LocalDateTime> values() {
        var list = new ArrayList<LocalDateTime>();
        list.add(LocalDateTime.now());
        list.add(LocalDateTime.of(1969, 12, 31, 23, 59, 59, 999_999_999));
        list.add(LocalDateTime.of(1970, 1, 1, 0, 0, 0));
        list.add(LocalDateTime.of(2025, 2, 7, 12, 30, 59, 123456789));
        list.add(LocalDateTime.of(9999, 12, 31, 23, 59, 59, 999_999_999));
        list.add(LocalDateTime.of(-1, 1, 1, 0, 0, 0));
        list.add(LocalDateTime.of(0, 1, 1, 0, 0, 0));
        list.add(LocalDateTime.of(2025, 7, 3, 8, 17, 19, 210_000_000));
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<LocalDateTime> bindVariable(String name) {
        return TgBindVariable.ofDateTime(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, LocalDateTime value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected LocalDateTime get(TsurugiResultEntity entity, String name) {
        return entity.getDateTime(name);
    }

    @Override
    protected TgOdbcGetDataArgument<LocalDateTime> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofTimestamp(manager);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, LocalDateTime value, boolean wideChar) {
        return TgOdbcBindParameter.ofTimestamp(manager, value);
    }

    @Test
    void bindParameterCombination() {
        testBindParameterCombination(SqlDataType.SQL_TYPE_TIMESTAMP);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, LocalDateTime value) {
        TgOdbcBindParameter parameter;
        LocalDateTime expected;
        switch (valueType) {
        case SQL_C_CHAR:
            parameter = TgOdbcBindParameter.ofStringUtf8(manager, value.toString());
            expected = value;
            break;
        case SQL_C_WCHAR:
            parameter = TgOdbcBindParameter.ofStringUtf16(manager, value.toString());
            expected = value;
            break;
        case SQL_C_DATE:
        case SQL_C_TYPE_DATE:
            parameter = TgOdbcBindParameter.ofDate(manager, value.toLocalDate());
            expected = value.truncatedTo(ChronoUnit.DAYS);
            break;
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            parameter = TgOdbcBindParameter.ofTimestamp(manager, value);
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
    protected void assertGetData(LocalDateTime value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
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
            assertEquals(value.toLocalDate(), (LocalDate) actual);
            break;
        case SQL_C_TIME:
        case SQL_C_TYPE_TIME:
            assertEquals(value.toLocalTime().truncatedTo(ChronoUnit.SECONDS), (LocalTime) actual);
            break;
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            assertEquals(value, (LocalDateTime) actual);
            break;
        default:
            throw new InternalError("Not yet implements targetType " + targetType);
        }
    }

    private static final DateTimeFormatter FORMATTER = new DateTimeFormatterBuilder() //
            .appendPattern("uuuu-MM-dd HH:mm:ss") //
            .appendFraction(ChronoField.NANO_OF_SECOND, 1, 9, true) // 最小1桁, 最大9桁, 常に小数点あり
            .toFormatter();

    private static String toExpectedString(LocalDateTime value) {
        return value.format(FORMATTER);
    }

    @Override
    protected void assertValueList(List<LocalDateTime> expected, List<LocalDateTime> actual) {
        try {
            assertEquals(expected.size(), actual.size());
            for (int i = 0; i < actual.size(); i++) {
                assertEquals(expected.get(i), actual.get(i));
            }
        } catch (Throwable e) {
            LOG.error("{}\nexpected={}\nactual=  {}", e.getMessage(), expected, actual);
            throw e;
        }
    }
}
