package com.tsurugidb.tsubakuro.rust.odbc.type;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
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

class TgOdbcTypeDateTest extends TgOdbcTypeTester<LocalDate> {

    @Override
    protected String sqlType() {
        return "date";
    }

    @Override
    protected SqlDataType dataType() {
        return SqlDataType.SQL_TYPE_DATE;
    }

    @Override
    protected List<LocalDate> values() {
        var list = new ArrayList<LocalDate>();
        list.add(LocalDate.now());
        list.add(LocalDate.of(1970, 1, 1));
        list.add(LocalDate.of(-1, 1, 1));
        list.add(LocalDate.of(0, 1, 1));
        list.add(LocalDate.of(1, 1, 1));
        list.add(LocalDate.of(9999, 12, 31));
        list.add(null);
        return list;
    }

    @Override
    protected TgBindVariable<LocalDate> bindVariable(String name) {
        return TgBindVariable.ofDate(name);
    }

    @Override
    protected TgBindParameter bindParameter(String name, LocalDate value) {
        return TgBindParameter.of(name, value);
    }

    @Override
    protected LocalDate get(TsurugiResultEntity entity, String name) {
        return entity.getDate(name);
    }

    @Override
    protected TgOdbcGetDataArgument<LocalDate> getDataArgument(TgOdbcManager manager, boolean wideChar) {
        return TgOdbcGetDataArgument.ofDate(manager);
    }

    @Override
    protected TgOdbcBindParameter bindParameter(TgOdbcManager manager, LocalDate value, boolean wideChar) {
        return TgOdbcBindParameter.ofDate(manager, value);
    }

    @Test
    void bindParameterCombination() {
        testBindParameterCombination(SqlDataType.SQL_TYPE_DATE);
    }

    @Override
    protected ExpectedBindValue bindValue(TgOdbcManager manager, CDataType valueType, LocalDate value) {
        TgOdbcBindParameter parameter;
        LocalDate expected;
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
            parameter = TgOdbcBindParameter.ofDate(manager, value);
            expected = value;
            break;
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            parameter = TgOdbcBindParameter.ofTimestamp(manager, LocalDateTime.of(value, LocalTime.MIN));
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
    protected void assertGetData(LocalDate value, CDataType targetType, Object actual, TgOdbcRuntimeException re) {
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
        case SQL_C_TIME:
        case SQL_C_TYPE_TIME:
            assertTrue(re.getMessage().contains("Unsupported"), () -> re.getMessage());
            break;
        case SQL_C_CHAR:
        case SQL_C_WCHAR:
            assertEquals(value.toString(), (String) actual);
            break;
        case SQL_C_DATE:
        case SQL_C_TYPE_DATE:
            assertEquals(value, (LocalDate) actual);
            break;
        case SQL_C_TIMESTAMP:
        case SQL_C_TYPE_TIMESTAMP:
            assertEquals(LocalDateTime.of(value, LocalTime.MIN), (LocalDateTime) actual);
            break;
        default:
            throw new InternalError("Not yet implements targetType " + targetType);
        }
    }
}
