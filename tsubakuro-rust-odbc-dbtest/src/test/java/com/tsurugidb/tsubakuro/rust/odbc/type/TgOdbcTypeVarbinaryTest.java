package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

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

class TgOdbcTypeVarbinaryTest extends TgOdbcTypeTester<byte[]> {

    @Override
    protected String sqlType() {
        return "varbinary";
    }

    @Override
    protected String expectedSqlType() {
        return "VARBINARY(*)";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_VARBINARY;
    }

    @Override
    protected List<byte[]> values() {
        var list = new ArrayList<byte[]>();
        list.add(new byte[0]);
        list.add(new byte[] { 1, 2, 3, 100, (byte) 0xff });
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<byte[]> bindVariable(String name) {
        return TgBindVariable.ofBytes(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, byte[] value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected byte[] get(TsurugiResultEntity entity, String name) {
        return entity.getBytes(name);
    }

    @Override
    protected TgOdbcGetDataArgument<byte[]> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofBinary(manager, 16);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, byte[] value, boolean wideChar) {
        return TgOdbcBindParameter.ofBinary(manager, value);
    }

    @Test
    void bindParameterCombination_BINARY() {
        testBindParameterCombination(SqlDataType.SQL_BINARY);
    }

    @Test
    void bindParameterCombination_VARBINARY() {
        testBindParameterCombination(SqlDataType.SQL_VARBINARY);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, byte[] value) {
        TgOdbcBindParameter parameter;
        byte[] expected;
        switch (valueType) {
        case SQL_C_BINARY:
            parameter = TgOdbcBindParameter.ofBinary(manager, value);
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
    protected void assertGetData(byte[] value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
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
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
        case SQL_C_DATE:
        case SQL_C_TYPE_DATE:
        case SQL_C_TIME:
        case SQL_C_TYPE_TIME:
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            assertTrue(re.getMessage().contains("Unsupported"), () -> re.getMessage());
            break;
        case SQL_C_BINARY:
            assertArrayEquals(value, (byte[]) actual);
            break;
        default:
            throw new InternalError("Not yet implements targetType " + targetType);
        }
    }

    @Override
    protected void assertValueList(List<byte[]> expected, List<byte[]> actual) {
        try {
            assertEquals(expected.size(), actual.size());
            for (int i = 0; i < actual.size(); i++) {
                assertArrayEquals(expected.get(i), actual.get(i));
            }
        } catch (Throwable e) {
            LOG.error("{}\nexpected={}\nactual=  {}", e.getMessage(), toString(expected), toString(actual));
            throw e;
        }
    }

    private static String toString(List<byte[]> list) {
        return list.stream().map(Arrays::toString).collect(Collectors.joining(", ", "[", "]"));

    }
}
