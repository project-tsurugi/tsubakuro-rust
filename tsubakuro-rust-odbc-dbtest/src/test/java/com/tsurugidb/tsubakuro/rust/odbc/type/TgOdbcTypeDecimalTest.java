package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.math.BigDecimal;
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

class TgOdbcTypeDecimalTest extends TgOdbcTypeTester<BigDecimal> {

    @Override
    protected String sqlType() {
        return "decimal(38, 1)";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_DECIMAL;
    }

    private static final BigDecimal MIN_DECIMAL = new BigDecimal("-" + "9".repeat(37) + ".9");
    private static final BigDecimal MAX_DECIMAL = new BigDecimal("9".repeat(37) + ".9");

    @Override
    protected List<BigDecimal> values() {
        var list = new ArrayList<BigDecimal>();
        list.add(MIN_DECIMAL);
        list.add(BigDecimal.valueOf(-1));
        list.add(BigDecimal.valueOf(0));
        list.add(BigDecimal.valueOf(1));
        list.add(new BigDecimal("123.4"));
        list.add(new BigDecimal("-123.4"));
        list.add(MAX_DECIMAL);
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<BigDecimal> bindVariable(String name) {
        return TgBindVariable.ofDecimal(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, BigDecimal value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected BigDecimal get(TsurugiResultEntity entity, String name) {
        return entity.getDecimal(name);
    }

    @Override
    protected TgOdbcGetDataArgument<BigDecimal> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofDecimal(manager);
    }

    @Override
    protected void assertValueList(List<BigDecimal> expected, List<BigDecimal> actual) {
        assertEquals(expected.size(), actual.size());
        for (int i = 0; i < actual.size(); i++) {
            var e = expected.get(i);
            var a = actual.get(i);

            if (e == null) {
                assertNull(a);
                continue;
            }

            assertEquals(e.setScale(1), a);
        }
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, BigDecimal value, boolean wideChar) {
        return TgOdbcBindParameter.ofDecimal(manager, value);
    }

    @Test
    void bindParameterCombination_DECIMAL() {
        testBindParameterCombination(SqlDataType.SQL_DECIMAL);
    }

    @Test
    void bindParameterCombination_NUMERIC() {
        testBindParameterCombination(SqlDataType.SQL_NUMERIC);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, BigDecimal value) {
        TgOdbcBindParameter parameter;
        boolean valid = true;
        BigDecimal expected;
        switch (valueType) {
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
            parameter = TgOdbcBindParameter.ofByte(manager, value.byteValue());
            expected = BigDecimal.valueOf(value.byteValue());
            break;
        case SQL_C_UTINYINT:
            parameter = TgOdbcBindParameter.ofByte(manager, value.byteValue());
            expected = BigDecimal.valueOf(value.byteValue() & 0xff);
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
            parameter = TgOdbcBindParameter.ofShort(manager, value.shortValue());
            expected = BigDecimal.valueOf(value.shortValue());
            break;
        case SQL_C_USHORT:
            parameter = TgOdbcBindParameter.ofShort(manager, value.shortValue());
            expected = BigDecimal.valueOf(value.shortValue() & 0xffff);
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
            parameter = TgOdbcBindParameter.ofInt(manager, value.intValue());
            expected = BigDecimal.valueOf(value.intValue());
            break;
        case SQL_C_ULONG:
            parameter = TgOdbcBindParameter.ofInt(manager, value.intValue());
            expected = BigDecimal.valueOf(value.longValue() & 0xffff_ffffL);
            break;
        case SQL_C_SBIGINT:
            parameter = TgOdbcBindParameter.ofLong(manager, value.longValue());
            expected = BigDecimal.valueOf(value.longValue());
            break;
        case SQL_C_UBIGINT:
            parameter = TgOdbcBindParameter.ofLong(manager, value.longValue());
            expected = new BigDecimal(Long.toUnsignedString(value.longValue()));
            break;
        case SQL_C_FLOAT:
            float floatValue = value.floatValue();
            if (floatValue <= -1e37f || floatValue >= 1e37f) {
                valid = false;
            }
            parameter = TgOdbcBindParameter.ofFloat(manager, floatValue);
            expected = value;
            break;
        case SQL_C_DOUBLE:
            double doubleValue = value.doubleValue();
            if (doubleValue <= -1e37f || doubleValue >= 1e37f) {
                valid = false;
            }
            parameter = TgOdbcBindParameter.ofDouble(manager, doubleValue);
            expected = value;
            break;
        case SQL_C_NUMERIC:
            parameter = TgOdbcBindParameter.ofDecimal(manager, value);
            expected = value;
            break;
        case SQL_C_CHAR:
            parameter = TgOdbcBindParameter.ofStringUtf8(manager, value.toPlainString());
            expected = value;
            break;
        case SQL_C_WCHAR:
            parameter = TgOdbcBindParameter.ofStringUtf16(manager, value.toPlainString());
            expected = value;
            break;
        default:
            return null;
        }
        return new ExpectedBindValue(parameter, valid, expected);
    }

    @Test
    void getDataCombination() throws Exception {
        testGetDataCombination();
    }

    @Override
    protected void assertGetData(BigDecimal value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
        switch (targetType) {
        case SQL_C_BIT:
            assertEquals(value.compareTo(BigDecimal.ZERO) != 0, (boolean) actual);
            break;
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
        case SQL_C_UTINYINT:
            assertEquals(value.byteValue(), (byte) actual);
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
        case SQL_C_USHORT:
            assertEquals(value.shortValue(), (short) actual);
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
        case SQL_C_ULONG:
            assertEquals(value.intValue(), (int) actual);
            break;
        case SQL_C_SBIGINT:
        case SQL_C_UBIGINT:
            assertEquals(value.longValue(), (long) actual);
            break;
        case SQL_C_FLOAT:
            assertEquals(value.floatValue(), (float) actual);
            break;
        case SQL_C_DOUBLE:
            assertEquals(expectedDouble(value), (double) actual);
            break;
        case SQL_C_NUMERIC:
            assertEquals(value.setScale(1), (BigDecimal) actual);
            break;
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
            assertEquals(value.setScale(1).toPlainString(), (String) actual);
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

    private static double expectedDouble(BigDecimal value) {
        if (value.equals(MIN_DECIMAL)) {
            return -1.0000000000000001e37;
        }
        if (value.equals(MAX_DECIMAL)) {
            return 1.0000000000000001e37;
        }
        return value.doubleValue();
    }
}
