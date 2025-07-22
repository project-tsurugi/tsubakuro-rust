package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
import java.time.OffsetTime;
import java.time.ZoneOffset;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeFormatterBuilder;
import java.time.temporal.ChronoField;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Disabled;
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

class TgOdbcTypeTimeTzTest extends TgOdbcTypeTester<OffsetTime> {

    @Override
    protected String sqlType() {
        return "time with time zone";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_TYPE_TIME;
    }

    private static final OffsetTime NOW = OffsetTime.now();

    @Override
    protected List<OffsetTime> values() {
        var offset = ZoneOffset.ofHours(9);

        var list = new ArrayList<OffsetTime>();
        list.add(NOW);
        list.add(OffsetTime.of(0, 0, 0, 0, ZoneOffset.UTC));
        list.add(OffsetTime.of(1, 30, 59, 123456789, ZoneOffset.UTC));
        list.add(OffsetTime.of(23, 59, 59, 999_999_999, ZoneOffset.UTC));
        list.add(OffsetTime.of(0, 0, 0, 0, offset));
        list.add(OffsetTime.of(1, 30, 59, 123456789, offset));
        list.add(OffsetTime.of(23, 59, 59, 999_999_999, offset));
        list.add(OffsetTime.of(17, 12, 31, 450_000_000, offset));
        list.add(OffsetTime.of(17, 12, 31, 450_000, offset));
        list.add(null);
        return list;
    }

    @Override
    protected CDataType insertOdbc(List<OffsetTime> values, boolean wideChar) {
        assumeTrue(false, "Unsupported time_with_time_zone literal"); // TODO remove assume
        try (var stmt = createStmt()) {

            int pk = 0;
            for (OffsetTime value : values) {
                String sql;
                if (value != null) {
                    String s = value.format(FORMATTER);
                    sql = "insert into test values(%d, time with time zone'%s')".formatted(pk, s);
                } else {
                    sql = "insert into test values(%d, null)".formatted(pk);
                }
                stmt.execDirect(sql, wideChar);

                pk++;
            }
        }

        return null;
    }

    @Override
    protected TgBindVariable<OffsetTime> bindVariable(String name) {
        return TgBindVariable.ofOffsetTime(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, OffsetTime value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected OffsetTime get(TsurugiResultEntity entity, String name) {
        return entity.getOffsetTime(name);
    }

    @Override
    protected TgOdbcGetDataArgument<OffsetTime> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofTimeTz(manager);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, OffsetTime value, boolean wideChar) {
        return TgOdbcBindParameter.ofTimeTz(manager, value);
    }

    @Test
    @Disabled // SQLBindParameterはTIME WITH TIME ZONEに対応していない
    void bindParameterCombination() {
        testBindParameterCombination(SqlDataType.SQL_TYPE_TIME);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, OffsetTime value) {
        TgOdbcBindParameter parameter;
        OffsetTime expected;
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
            parameter = TgOdbcBindParameter.ofTimeTz(manager, value);
            expected = value;
            break;
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            parameter = TgOdbcBindParameter.ofTimestamp(manager, LocalDateTime.of(LocalDate.EPOCH, value.toLocalTime()));
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
    protected void assertGetData(OffsetTime value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
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
            assertEquals(expectedTime(value).toLocalTime(), (LocalTime) actual);
            break;
        default:
            throw new InternalError("Not yet implements targetType " + targetType);
        }
    }

    private static final DateTimeFormatter FORMATTER = new DateTimeFormatterBuilder() //
            .appendPattern("HH:mm:ss") //
            .appendFraction(ChronoField.NANO_OF_SECOND, 1, 9, true) // 最小1桁, 最大9桁, 常に小数点あり
            .appendOffset("+HH:MM", "+00:00") //
            .toFormatter();

    private static String toExpectedString(OffsetTime value) {
        OffsetTime utc = value.withOffsetSameInstant(ZoneOffset.UTC);
        return utc.format(FORMATTER);
    }

    @Override
    protected void assertValueList(List<OffsetTime> expected, List<OffsetTime> actual) {
        try {
            assertEquals(expected.size(), actual.size());
            for (int i = 0; i < actual.size(); i++) {
                assertEquals(expectedTime(expected.get(i)), actual.get(i));
            }
        } catch (Throwable e) {
            LOG.error("{}\nexpected={}\nactual=  {}", e.getMessage(), expected, actual);
            throw e;
        }
    }

    private static OffsetTime expectedTime(OffsetTime value) {
        if (value == null) {
            return null;
        }
        OffsetTime utc = value.withOffsetSameInstant(ZoneOffset.UTC);
        return utc.truncatedTo(ChronoUnit.SECONDS);
    }
}
