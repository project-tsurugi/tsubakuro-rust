package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
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

class TgOdbcTypeInt4Test extends TgOdbcTypeTester<Integer> {

    @Override
    protected String sqlType() {
        return "int";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_INTEGER;
    }

    @Override
    protected List<Integer> values() {
        var list = new ArrayList<Integer>();
        list.add(Integer.MIN_VALUE);
        list.add(-1);
        list.add(0);
        list.add(1);
        list.add(123);
        list.add(Integer.MAX_VALUE);
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<Integer> bindVariable(String name) {
        return TgBindVariable.ofInt(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, Integer value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected Integer get(TsurugiResultEntity entity, String name) {
        return entity.getInt(name);
    }

    @Override
    protected TgOdbcGetDataArgument<Integer> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofInt(manager);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, Integer value, boolean wideChar) {
        return TgOdbcBindParameter.ofInt(manager, value);
    }

    @Test
    void bindParameterCombination() {
        testBindParameterCombination(SqlDataType.SQL_INTEGER);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, Integer value) {
        TgOdbcBindParameter parameter;
        Integer expected;
        switch (valueType) {
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
            parameter = TgOdbcBindParameter.ofByte(manager, value.byteValue());
            expected = (int) value.shortValue();
            break;
        case SQL_C_UTINYINT:
            parameter = TgOdbcBindParameter.ofByte(manager, value.byteValue());
            expected = value & 0xff;
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
            parameter = TgOdbcBindParameter.ofShort(manager, value.shortValue());
            expected = (int) value.shortValue();
            break;
        case SQL_C_USHORT:
            parameter = TgOdbcBindParameter.ofShort(manager, value.shortValue());
            expected = value & 0xffff;
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
        case SQL_C_ULONG:
            parameter = TgOdbcBindParameter.ofInt(manager, value);
            expected = value;
            break;
        case SQL_C_SBIGINT:
        case SQL_C_UBIGINT:
            parameter = TgOdbcBindParameter.ofLong(manager, value);
            expected = value;
            break;
        case SQL_C_FLOAT:
            parameter = TgOdbcBindParameter.ofFloat(manager, value);
            expected = value;
            break;
        case SQL_C_DOUBLE:
            parameter = TgOdbcBindParameter.ofDouble(manager, value);
            expected = value;
            break;
        case SQL_C_NUMERIC:
            parameter = TgOdbcBindParameter.ofDecimal(manager, BigDecimal.valueOf(value));
            expected = value;
            break;
        case SQL_C_CHAR:
            parameter = TgOdbcBindParameter.ofStringUtf8(manager, Integer.toString(value));
            expected = value;
            break;
        case SQL_C_WCHAR:
            parameter = TgOdbcBindParameter.ofStringUtf16(manager, Integer.toString(value));
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
    protected void assertGetData(Integer value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
        switch (targetType) {
        case SQL_C_BIT:
            assertEquals(value != 0, (boolean) actual);
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
            assertEquals(value, (int) actual);
            break;
        case SQL_C_SBIGINT:
        case SQL_C_UBIGINT:
            assertEquals(value.longValue(), (long) actual);
            break;
        case SQL_C_FLOAT:
            assertEquals(value.floatValue(), (float) actual);
            break;
        case SQL_C_DOUBLE:
            assertEquals(value.doubleValue(), (double) actual);
            break;
        case SQL_C_NUMERIC:
            assertEquals(BigDecimal.valueOf(value.longValue()), (BigDecimal) actual);
            break;
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
            assertEquals(Integer.toString(value), (String) actual);
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
