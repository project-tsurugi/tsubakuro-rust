package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeFalse;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.exception.TsurugiIOException;
import com.tsurugidb.tsubakuro.channel.common.connection.NullCredential;
import com.tsurugidb.tsubakuro.exception.CoreServiceCode;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLConnectTest extends TgOdbcTester {

    private static boolean noAuth = false;

    @BeforeAll
    static void beforeAll() throws Exception {
        var credential = NullCredential.INSTANCE;
        var connector = TsurugiConnector.of(getEndpointJava(), credential);
        try (var session = connector.createSession()) {
            session.getLowSession();
            noAuth = true;
        } catch (TsurugiIOException e) {
            if (e.getDiagnosticCode() != CoreServiceCode.AUTHENTICATION_ERROR) {
                throw e;
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void connect(boolean wideChar) {
        String dsn = getDsn();
        assumeFalse(dsn.isEmpty(), "DSN not specified");

        try (var dbc = createDbc()) {
            try (var _ = dbc.connect(dsn, null, null, wideChar)) {
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void connect_with_UserPassword(boolean wideChar) {
        String dsn = getDsn();
        assumeFalse(dsn.isEmpty(), "DSN not specified");
        String user = getUser();
        assumeFalse(user == null, "user not specified");
        String password = getPassword();

        try (var dbc = createDbc()) {
            try (var _ = dbc.connect(dsn, user, password, wideChar)) {
            }
        }

        if (!noAuth) {
            String invalidPassword = (password != null) ? "" : "empty";
            try (var dbc = createDbc()) {
                var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                    try (var _ = dbc.connect(dsn, user, invalidPassword, wideChar)) {
                    }
                });
                assertTrue(e.getMessage().contains("AUTHENTICATION_ERROR"), e.getMessage());
            }
        }
    }
}
