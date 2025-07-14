package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLGetInfoTest extends TgOdbcTester {

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_DRIVER_NAME(boolean wideChar) {
        var dbc = getConnection().dbc();

        String actual = dbc.getInfoTypeString(InfoType.SQL_DRIVER_NAME, 64, wideChar);
        assertEquals("tsubakuro_rust_odbc.dll", actual);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_DRIVER_VER(boolean wideChar) {
        var dbc = getConnection().dbc();

        String actual = dbc.getInfoTypeString(InfoType.SQL_DRIVER_VER, 32, wideChar);
        assertNotNull(actual);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_CURSOR_COMMIT_BEHAVIOR(boolean wideChar) {
        var dbc = getConnection().dbc();

        int actual = dbc.getInfoTypeInt(InfoType.SQL_CURSOR_COMMIT_BEHAVIOR, wideChar);
        assertEquals(0, actual);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_CURSOR_ROLLBACK_BEHAVIOR(boolean wideChar) {
        var dbc = getConnection().dbc();

        int actual = dbc.getInfoTypeInt(InfoType.SQL_CURSOR_ROLLBACK_BEHAVIOR, wideChar);
        assertEquals(0, actual);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_DRIVER_ODBC_VER(boolean wideChar) {
        var dbc = getConnection().dbc();

        String actual = dbc.getInfoTypeString(InfoType.SQL_DRIVER_ODBC_VER, 32, wideChar);
        assertEquals("03.51", actual);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_GETDATA_EXTENSIONS(boolean wideChar) {
        var dbc = getConnection().dbc();

        int actual = dbc.getInfoTypeInt(InfoType.SQL_GETDATA_EXTENSIONS, wideChar);
        assertEquals(0, actual);
    }

    @Test
    void getInfoA_length() {
        boolean wideChar = false;
        var dbc = getConnection().dbc();

        {
            var result = dbc.getInfoType(InfoType.SQL_DRIVER_ODBC_VER, 6, wideChar);
            assertEquals(SqlReturn.SQL_SUCCESS, result.rc());
            assertEquals("03.51", result.infoValue());
        }
        {
            var result = dbc.getInfoType(InfoType.SQL_DRIVER_ODBC_VER, 5, wideChar);
            assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, result.rc());

            var rec = dbc.getDiagRec(1);
            assertEquals("01004", rec.sqlState());
        }
    }

    @Test
    void getInfoW_length() {
        boolean wideChar = true;
        var dbc = getConnection().dbc();

        {
            var result = dbc.getInfoType(InfoType.SQL_DRIVER_ODBC_VER, 6 * 2, wideChar);
            assertEquals(SqlReturn.SQL_SUCCESS, result.rc());
            assertEquals("03.51", result.infoValue());
        }
        {
            var result = dbc.getInfoType(InfoType.SQL_DRIVER_ODBC_VER, 5 * 2, wideChar);
            assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, result.rc());

            var rec = dbc.getDiagRec(1);
            assertEquals("01004", rec.sqlState());
        }
    }
}
