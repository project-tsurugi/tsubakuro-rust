package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

import java.util.Map;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDiagRec;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLDriverConnectTest extends TgOdbcTester {

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void driverConnect(boolean wideChar) {
        try (var dbc = createDbc()) {
            var manager = dbc.manager();

            String inConnectionString = getConnectionString();
            int length = inConnectionString.length();
            var arg = new TgOdbcDriverConnectArgument(manager, wideChar) //
                    .inConnectionString(inConnectionString) //
                    .bufferLength(length * 2);

            short rc = dbc.driverConnect(arg);
            try {
                assertEquals(SqlReturn.SQL_SUCCESS, rc);

                Map<String, String> map = arg.outConnectionMap();
                assertEquals("{Tsurugi Driver}", map.get("driver"));
                assertEquals(getEndpoint(), map.get("endpoint"));

                assertEquals(inConnectionString.length(), arg.outConnectionStringLength());
            } finally {
                dbc.disconnect();
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void inConnectionString_length(boolean wideChar) {
        try (var dbc = createDbc()) {
            var manager = dbc.manager();

            String inConnectionString = getConnectionString();
            int length = inConnectionString.length();
            var arg = new TgOdbcDriverConnectArgument(manager, wideChar) //
                    .inConnectionString(inConnectionString, length) //
                    .bufferLength(length * 2);

            short rc = dbc.driverConnect(arg);
            try {
                assertEquals(SqlReturn.SQL_SUCCESS, rc);
            } finally {
                dbc.disconnect();
            }
        }
        try (var dbc = createDbc()) {
            var manager = dbc.manager();

            String ex = "zzz=zzz;";
            String inConnectionString = getConnectionString() + ex;
            int length = inConnectionString.length() - ex.length();
            var arg = new TgOdbcDriverConnectArgument(manager, wideChar) //
                    .inConnectionString(inConnectionString, length) //
                    .bufferLength(length * 2);

            short rc = dbc.driverConnect(arg);
            try {
                assertEquals(SqlReturn.SQL_SUCCESS, rc);

                Map<String, String> map = arg.outConnectionMap();
                assertFalse(map.containsKey("zzz"), () -> map.toString());
            } finally {
                dbc.disconnect();
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void outConnectionString_length(boolean wideChar) {
        try (var dbc = createDbc()) {
            var manager = dbc.manager();

            String inConnectionString = getConnectionString();
            int length = inConnectionString.length();
            var arg = new TgOdbcDriverConnectArgument(manager, wideChar) //
                    .inConnectionString(inConnectionString) //
                    .bufferLength(length + 1);

            short rc = dbc.driverConnect(arg);
            try {
                assertEquals(SqlReturn.SQL_SUCCESS, rc);
            } finally {
                dbc.disconnect();
            }
        }
        try (var dbc = createDbc()) {
            var manager = dbc.manager();

            String inConnectionString = getConnectionString();
            int length = inConnectionString.length();
            var arg = new TgOdbcDriverConnectArgument(manager, wideChar) //
                    .inConnectionString(inConnectionString) //
                    .bufferLength(length);

            short rc = dbc.driverConnect(arg);
            try {
                assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, rc);

                TgOdbcDiagRec rec = dbc.getDiagRec(1);
                assertEquals("01004", rec.sqlState());
            } finally {
                dbc.disconnect();
            }
        }
    }
}
