package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import static org.junit.jupiter.api.Assumptions.assumeFalse;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLConnectTest extends TgOdbcTester {

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void connect(boolean wideChar) {
        try (var dbc = createDbc()) {
            String dsn = getDsn();
            assumeFalse(dsn.isEmpty(), "DSN not specified");

            try (var _ = dbc.connect(dsn, wideChar)) {
            }
        }
    }
}
