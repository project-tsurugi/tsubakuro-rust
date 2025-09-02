package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assumptions.assumeFalse;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

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

            short rc = dbc.driverConnect0(arg);
            try {
                assertEquals(SqlReturn.SQL_SUCCESS, rc);

                Map<String, String> map = arg.outConnectionMap();
                String driver = map.get("driver");
                if (driver != null) {
                    assertEquals("{Tsurugi Driver}", map.get("driver"));
                    assertEquals(getEndpoint(), map.get("endpoint"));

                    assertEquals(inConnectionString.length(), arg.outConnectionStringLength());
                }
            } finally {
                dbc.disconnect();
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void dsn(boolean wideChar) {
        String dsn = getDsn();
        assumeFalse(dsn.isEmpty(), "DSN not specified");

        try (var dbc = createDbc()) {
            String inConnectionString = "DSN=%s;".formatted(dsn);

            try (var _ = dbc.driverConnect(inConnectionString, wideChar)) {
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

            short rc = dbc.driverConnect0(arg);
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

            short rc = dbc.driverConnect0(arg);
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
            assumeTrue(inConnectionString.contains("Driver"));
            int length = inConnectionString.length();
            var arg = new TgOdbcDriverConnectArgument(manager, wideChar) //
                    .inConnectionString(inConnectionString) //
                    .bufferLength(length + 1);

            short rc = dbc.driverConnect0(arg);
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

            short rc = dbc.driverConnect0(arg);
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
