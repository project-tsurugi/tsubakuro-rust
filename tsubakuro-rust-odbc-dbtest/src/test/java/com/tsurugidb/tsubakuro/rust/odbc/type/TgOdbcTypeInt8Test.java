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

class TgOdbcTypeInt8Test extends TgOdbcTypeTester<Long> {

    @Override
    protected String sqlType() {
        return "bigint";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_BIGINT;
    }

    @Override
    protected List<Long> values() {
        var list = new ArrayList<Long>();
        list.add(Long.MIN_VALUE);
        list.add(-1L);
        list.add(0L);
        list.add(1L);
        list.add(123L);
        list.add(Long.MAX_VALUE);
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<Long> bindVariable(String name) {
        return TgBindVariable.ofLong(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, Long value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected Long get(TsurugiResultEntity entity, String name) {
        return entity.getLong(name);
    }

    @Override
    protected TgOdbcGetDataArgument<Long> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofLong(manager);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, Long value, boolean wideChar) {
        return TgOdbcBindParameter.ofLong(manager, value);
    }

    @Test
    void bindParameterCombination() {
        testBindParameterCombination(SqlDataType.SQL_BIGINT);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, Long value) {
        TgOdbcBindParameter parameter;
        Long expected;
        switch (valueType) {
        case SQL_C_TINYINT:
        case SQL_C_STINYINT:
            parameter = TgOdbcBindParameter.ofByte(manager, value.byteValue());
            expected = (long) value.shortValue();
            break;
        case SQL_C_UTINYINT:
            parameter = TgOdbcBindParameter.ofByte(manager, value.byteValue());
            expected = value & 0xff;
            break;
        case SQL_C_SHORT:
        case SQL_C_SSHORT:
            parameter = TgOdbcBindParameter.ofShort(manager, value.shortValue());
            expected = (long) value.shortValue();
            break;
        case SQL_C_USHORT:
            parameter = TgOdbcBindParameter.ofShort(manager, value.shortValue());
            expected = value & 0xffff;
            break;
        case SQL_C_LONG:
        case SQL_C_SLONG:
            parameter = TgOdbcBindParameter.ofInt(manager, value.intValue());
            expected = (long) value.intValue();
            break;
        case SQL_C_ULONG:
            parameter = TgOdbcBindParameter.ofInt(manager, value.intValue());
            expected = value & 0xffff_ffffL;
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
            parameter = TgOdbcBindParameter.ofStringUtf8(manager, Long.toString(value));
            expected = value;
            break;
        case SQL_C_WCHAR:
            parameter = TgOdbcBindParameter.ofStringUtf16(manager, Long.toString(value));
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
    protected void assertGetData(Long value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
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
            assertEquals(value.intValue(), (int) actual);
            break;
        case SQL_C_SBIGINT:
        case SQL_C_UBIGINT:
            assertEquals(value, (long) actual);
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
            assertEquals(Long.toString(value), (String) actual);
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
