package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.math.BigDecimal;
import java.math.BigInteger;
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

class TgOdbcTypeFloat8Test extends TgOdbcTypeTester<Double> {

    @Override
    protected String sqlType() {
        return "double";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_DOUBLE;
    }

    @Override
    protected List<Double> values() {
        var list = new ArrayList<Double>();
        list.add(Double.MIN_VALUE);
        list.add(-1d);
        list.add(0d);
        list.add(1d);
        list.add(123.4d);
        list.add(Double.MAX_VALUE);
        list.add(null);
        list.add(Double.NEGATIVE_INFINITY);
        list.add(Double.POSITIVE_INFINITY);
        list.add(Double.NaN);
        return list;
    }

    @Override
    protected TgBindVariable<Double> bindVariable(String name) {
        return TgBindVariable.ofDouble(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, Double value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected Double get(TsurugiResultEntity entity, String name) {
        return entity.getDouble(name);
    }

    @Override
    protected TgOdbcGetDataArgument<Double> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofDouble(manager);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, Double value, boolean wideChar) {
        return TgOdbcBindParameter.ofDouble(manager, value);
    }

    @Test
    void bindParameterCombination() {
        testBindParameterCombination(SqlDataType.SQL_DOUBLE);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, Double value) {
        TgOdbcBindParameter parameter;
        Double expected;
        switch (valueType) {
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
            parameter = TgOdbcBindParameter.ofByte(manager, value.byteValue());
            expected = (double) value.byteValue();
            break;
        case SQL_C_UTINYINT:
            parameter = TgOdbcBindParameter.ofByte(manager, value.byteValue());
            expected = (double) (value.byteValue() & 0xff);
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
            parameter = TgOdbcBindParameter.ofShort(manager, value.shortValue());
            expected = (double) value.shortValue();
            break;
        case SQL_C_USHORT:
            parameter = TgOdbcBindParameter.ofShort(manager, value.shortValue());
            expected = (double) (value.shortValue() & 0xffff);
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
            parameter = TgOdbcBindParameter.ofInt(manager, value.intValue());
            expected = (double) value.intValue();
            break;
        case SQL_C_ULONG:
            parameter = TgOdbcBindParameter.ofInt(manager, value.intValue());
            expected = (double) (value.intValue() & 0xffff_ffffL);
            break;
        case SQL_C_SBIGINT:
            parameter = TgOdbcBindParameter.ofLong(manager, value.longValue());
            expected = (double) value.longValue();
            break;
        case SQL_C_UBIGINT:
            parameter = TgOdbcBindParameter.ofLong(manager, value.longValue());
            expected = new BigInteger(Long.toUnsignedString(value.longValue())).doubleValue();
            break;
        case SQL_C_FLOAT:
            parameter = TgOdbcBindParameter.ofFloat(manager, value.floatValue());
            expected = (double) value.floatValue();
            break;
        case SQL_C_DOUBLE:
            parameter = TgOdbcBindParameter.ofDouble(manager, value);
            expected = value;
            break;
        case SQL_C_NUMERIC:
            if (value.isInfinite() || value.isNaN()) {
                parameter = null;
            } else {
                parameter = TgOdbcBindParameter.ofDecimal(manager, BigDecimal.valueOf(value));
            }
            if (value == Double.MIN_VALUE) {
                expected = 4.9e-68;
            } else if (value == Double.MAX_VALUE) {
                expected = 1.7976931348623156e52;
            } else {
                expected = value;
            }
            break;
        case SQL_C_CHAR:
            parameter = TgOdbcBindParameter.ofStringUtf8(manager, Double.toString(value));
            expected = value;
            break;
        case SQL_C_WCHAR:
            parameter = TgOdbcBindParameter.ofStringUtf16(manager, Double.toString(value));
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
    protected void assertGetData(Double value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
        switch (targetType) {
        case SQL_C_BIT:
            assertEquals(value != 0, (boolean) actual);
            break;
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
            if (value.isNaN()) {
                assertEquals(0, (byte) actual);
            } else if (value > Byte.MAX_VALUE) {
                assertEquals(Byte.MAX_VALUE, (byte) actual);
            } else if (value < Byte.MIN_VALUE) {
                assertEquals(Byte.MIN_VALUE, (byte) actual);
            } else {
                assertEquals(value.byteValue(), (byte) actual);
            }
            break;
        case SQL_C_UTINYINT:
            if (value.isNaN()) {
                assertEquals(0, (byte) actual);
            } else if (value > 0xff) {
                assertEquals((byte) 0xff, (byte) actual);
            } else if (value < 0) {
                assertEquals(0, (byte) actual);
            } else {
                assertEquals(value.byteValue(), (byte) actual);
            }
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
            if (value.isNaN()) {
                assertEquals(0, (short) actual);
            } else if (value > Short.MAX_VALUE) {
                assertEquals(Short.MAX_VALUE, (short) actual);
            } else if (value < Short.MIN_VALUE) {
                assertEquals(Short.MIN_VALUE, (short) actual);
            } else {
                assertEquals(value.shortValue(), (short) actual);
            }
            break;
        case SQL_C_USHORT:
            if (value.isNaN()) {
                assertEquals(0, (short) actual);
            } else if (value > 0xffff) {
                assertEquals((short) 0xffff, (short) actual);
            } else if (value < 0) {
                assertEquals(0, (short) actual);
            } else {
                assertEquals(value.shortValue(), (short) actual);
            }
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
            if (value.isNaN()) {
                assertEquals(0, (int) actual);
            } else if (value > Integer.MAX_VALUE) {
                assertEquals(Integer.MAX_VALUE, (int) actual);
            } else if (value < Integer.MIN_VALUE) {
                assertEquals(Integer.MIN_VALUE, (int) actual);
            } else {
                assertEquals(value.intValue(), (int) actual);
            }
            break;
        case SQL_C_ULONG:
            if (value.isNaN()) {
                assertEquals(0, (int) actual);
            } else if (value > 0xffff_ffffL) {
                assertEquals((int) 0xffff_ffffL, (int) actual);
            } else if (value < 0) {
                assertEquals(0, (int) actual);
            } else {
                assertEquals(value.intValue(), (int) actual);
            }
            break;
        case SQL_C_SBIGINT:
            if (value.isNaN()) {
                assertEquals(0, (long) actual);
            } else if (value > Long.MAX_VALUE) {
                assertEquals(Long.MAX_VALUE, (long) actual);
            } else if (value < Long.MIN_VALUE) {
                assertEquals(Long.MIN_VALUE, (long) actual);
            } else {
                assertEquals(value.longValue(), (long) actual);
            }
            break;
        case SQL_C_UBIGINT:
            if (value.isNaN()) {
                assertEquals(0, (long) actual);
            } else if (value > Long.MAX_VALUE) { // 本当は0xffff_ffff_ffff_ffff
                assertEquals(-1L, (long) actual);
            } else if (value < 0) {
                assertEquals(0, (long) actual);
            } else {
                assertEquals(value.longValue(), (long) actual);
            }
            break;
        case SQL_C_FLOAT:
            assertEquals(value.floatValue(), (float) actual);
            break;
        case SQL_C_DOUBLE:
            assertEquals(value, (double) actual);
            break;
        case SQL_C_NUMERIC:
            if (value.isNaN() || value.isInfinite() || value >= Double.MAX_VALUE) {
                assertTrue(re.getMessage().contains("Decimal convert error"), () -> re.getMessage());
            } else if (value == Double.MIN_VALUE) {
                assertEquals(BigDecimal.ZERO, (BigDecimal) actual);
            } else if (value == 123.4d) {
                assertEquals(new BigDecimal("123.4"), (BigDecimal) actual);
            } else {
                var expected = BigDecimal.valueOf(value);
                try {
                    assertTrue(expected.compareTo((BigDecimal) actual) == 0);
                } catch (AssertionError ignore) {
                    assertEquals(expected, (BigDecimal) actual);
                }
            }
            break;
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
            assertEquals(expectedString(value), (String) actual);
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

    private static String expectedString(double value) {
        if (Double.isNaN(value)) {
            return "NaN";
        }
        if (value == Double.POSITIVE_INFINITY) {
            return "inf";
        }
        if (value == Double.NEGATIVE_INFINITY) {
            return "-inf";
        }
        if (value == Double.MIN_VALUE) {
            return "5e-324";
        }
        return Double.toString(value).toLowerCase();
    }
}
