package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
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

class TgOdbcTypeVarcharTest extends TgOdbcTypeTester<String> {

    @Override
    protected String sqlType() {
        return "varchar(32)";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_VARCHAR;
    }

    @Override
    protected List<String> values() {
        var list = new ArrayList<String>();
        list.add("");
        list.add("abc");
        list.add("true");
        list.add("false");
        list.add(Byte.toString(Byte.MIN_VALUE));
        list.add(Byte.toString(Byte.MAX_VALUE));
        list.add(Short.toString(Short.MIN_VALUE));
        list.add(Short.toString(Short.MAX_VALUE));
        list.add(Integer.toString(Integer.MIN_VALUE));
        list.add(Integer.toString(Integer.MAX_VALUE));
        list.add(Long.toString(Long.MIN_VALUE));
        list.add(Long.toString(Long.MAX_VALUE));
        list.add("-9223372036854775809"); // Long.MIN_VALUE - 1
        list.add("9223372036854775808"); // Long.MAX_VALUE + 1
        list.add("123.4");
        list.add("12345e-1");
        list.add("NaN");
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<String> bindVariable(String name) {
        return TgBindVariable.ofString(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, String value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected String get(TsurugiResultEntity entity, String name) {
        return entity.getString(name);
    }

    @Override
    protected TgOdbcGetDataArgument<String> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofString(manager, 64, wideChar);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, String value, boolean wideChar) {
        if (wideChar) {
            return TgOdbcBindParameter.ofStringUtf8(manager, value);
        } else {
            return TgOdbcBindParameter.ofStringUtf16(manager, value);
        }
    }

    @Test
    void bindParameterCombination_CHAR() {
        testBindParameterCombination(SqlDataType.SQL_CHAR);
    }

    @Test
    void bindParameterCombination_VARCHAR() {
        testBindParameterCombination(SqlDataType.SQL_VARCHAR);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, String value) {
        TgOdbcBindParameter parameter;
        String expected;
        switch (valueType) {
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
        case SQL_C_UTINYINT:
            try {
                var v = (byte) Long.parseLong(value);
                parameter = TgOdbcBindParameter.ofByte(manager, v);
                expected = "%d".formatted(valueType == CDataType.SQL_C_UTINYINT ? Byte.toUnsignedInt(v) : v);
            } catch (NumberFormatException e) {
                try {
                    var v = (byte) Double.parseDouble(value);
                    parameter = TgOdbcBindParameter.ofByte(manager, v);
                    expected = "%d".formatted(valueType == CDataType.SQL_C_UTINYINT ? Byte.toUnsignedInt(v) : v);
                } catch (NumberFormatException e2) {
                    parameter = null;
                    expected = value;
                }
            }
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
        case SQL_C_USHORT:
            try {
                var v = (short) Long.parseLong(value);
                parameter = TgOdbcBindParameter.ofShort(manager, v);
                expected = "%d".formatted(valueType == CDataType.SQL_C_USHORT ? Short.toUnsignedInt(v) : v);
            } catch (NumberFormatException e) {
                try {
                    var v = (short) Double.parseDouble(value);
                    parameter = TgOdbcBindParameter.ofShort(manager, v);
                    expected = "%d".formatted(valueType == CDataType.SQL_C_USHORT ? Short.toUnsignedInt(v) : v);
                } catch (NumberFormatException e2) {
                    parameter = null;
                    expected = value;
                }
            }
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
        case SQL_C_ULONG:
            try {
                var v = (int) Long.parseLong(value);
                parameter = TgOdbcBindParameter.ofInt(manager, v);
                expected = (valueType == CDataType.SQL_C_ULONG) ? Integer.toUnsignedString(v) : Integer.toString(v);
            } catch (NumberFormatException e) {
                try {
                    var v = (int) Double.parseDouble(value);
                    parameter = TgOdbcBindParameter.ofInt(manager, v);
                    expected = (valueType == CDataType.SQL_C_ULONG) ? Integer.toUnsignedString(v) : Integer.toString(v);
                } catch (NumberFormatException e2) {
                    parameter = null;
                    expected = value;
                }
            }
            break;
        case SQL_C_SBIGINT:
        case SQL_C_UBIGINT:
            try {
                long v = Long.parseLong(value);
                parameter = TgOdbcBindParameter.ofLong(manager, v);
                expected = (valueType == CDataType.SQL_C_UBIGINT) ? Long.toUnsignedString(v) : Long.toString(v);
            } catch (NumberFormatException e) {
                try {
                    var v = (long) Double.parseDouble(value);
                    parameter = TgOdbcBindParameter.ofLong(manager, v);
                    expected = (valueType == CDataType.SQL_C_UBIGINT) ? Long.toUnsignedString(v) : Long.toString(v);
                } catch (NumberFormatException e2) {
                    parameter = null;
                    expected = value;
                }
            }
            break;
        case SQL_C_FLOAT:
            try {
                float v = Float.parseFloat(value);
                parameter = TgOdbcBindParameter.ofFloat(manager, v);
                expected = toPlainString(Float.toString(v));
            } catch (NumberFormatException e) {
                parameter = null;
                expected = value;
            }
            break;
        case SQL_C_DOUBLE:
            try {
                double v = Double.parseDouble(value);
                parameter = TgOdbcBindParameter.ofDouble(manager, v);
                expected = toPlainString(Double.toString(v));
            } catch (NumberFormatException e) {
                parameter = null;
                expected = value;
            }
            break;
        case SQL_C_NUMERIC:
            try {
                var v = new BigDecimal(value);
                parameter = TgOdbcBindParameter.ofDecimal(manager, v);
                expected = v.toPlainString();
            } catch (NumberFormatException e) {
                parameter = null;
                expected = value;
            }
            break;
        case SQL_C_CHAR:
            parameter = TgOdbcBindParameter.ofStringUtf8(manager, value);
            expected = value;
            break;
        case SQL_C_WCHAR:
            parameter = TgOdbcBindParameter.ofStringUtf16(manager, value);
            expected = value;
            break;
        default:
            return null;
        }
        return new ExpectedBindValue(parameter, expected);
    }

    private static String toPlainString(String s) {
        int n = s.indexOf('E');
        if (n < 0) {
            return s;
        }

        return new BigDecimal(s).setScale(1).toPlainString();
    }

    @Test
    void getDataCombination() throws Exception {
        testGetDataCombination();
    }

    @Override
    protected void assertGetData(String value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
        switch (targetType) {
        case SQL_C_BIT:
            switch (value) {
            case "true":
                assertTrue((boolean) actual);
                break;
            case "false":
                assertFalse((boolean) actual);
                break;
            default:
                assertTrue(re.getMessage().contains("convert error"), () -> re.getMessage());
                break;
            }
            break;
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
        case SQL_C_UTINYINT:
            try {
                assertEquals(new BigDecimal(value).byteValue(), (byte) actual);
            } catch (NumberFormatException e) {
                assertTrue(re.getMessage().contains("convert error"), () -> re.getMessage());
            }
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
        case SQL_C_USHORT:
            try {
                assertEquals(new BigDecimal(value).shortValue(), (short) actual);
            } catch (NumberFormatException e) {
                assertTrue(re.getMessage().contains("convert error"), () -> re.getMessage());
            }
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
        case SQL_C_ULONG:
            try {
                assertEquals(new BigDecimal(value).intValue(), (int) actual);
            } catch (NumberFormatException e) {
                assertTrue(re.getMessage().contains("convert error"), () -> re.getMessage());
            }
            break;
        case SQL_C_SBIGINT:
        case SQL_C_UBIGINT:
            try {
                assertEquals(new BigDecimal(value).longValue(), (long) actual);
            } catch (NumberFormatException e) {
                assertTrue(re.getMessage().contains("convert error"), () -> re.getMessage());
            }
            break;
        case SQL_C_FLOAT:
            try {
                assertEquals(Float.parseFloat(value), (float) actual);
            } catch (NumberFormatException e) {
                assertTrue(re.getMessage().contains("convert error"), () -> re.getMessage());
            }
            break;
        case SQL_C_DOUBLE:
            try {
                assertEquals(Double.parseDouble(value), (double) actual);
            } catch (NumberFormatException e) {
                assertTrue(re.getMessage().contains("convert error"), () -> re.getMessage());
            }
            break;
        case SQL_C_NUMERIC:
            try {
                assertEquals(new BigDecimal(value), (BigDecimal) actual);
            } catch (NumberFormatException e) {
                assertTrue(re.getMessage().contains("convert error"), () -> re.getMessage());
            }
            break;
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
            assertEquals(value, (String) actual);
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
