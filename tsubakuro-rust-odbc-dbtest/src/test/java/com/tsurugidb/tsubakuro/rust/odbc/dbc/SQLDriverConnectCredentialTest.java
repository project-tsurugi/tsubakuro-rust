package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeFalse;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Optional;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.exception.TsurugiIOException;
import com.tsurugidb.tsubakuro.channel.common.connection.Credential;
import com.tsurugidb.tsubakuro.channel.common.connection.FileCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.NullCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.RememberMeCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.UsernamePasswordCredential;
import com.tsurugidb.tsubakuro.exception.CoreServiceCode;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLDriverConnectCredentialTest extends TgOdbcTester {

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
    void nullCredential(boolean wideChar) throws Exception {
        try (var dbc = createDbc()) {
            var sb = new StringBuilder(CONNECTION_STRING_DRIVER);
            appendTo(sb, "Endpoint", getEndpoint());
            String inConnectionString = sb.toString();

            if (noAuth) {
                Optional<String> expectedUser = getExpectedUser(NullCredential.INSTANCE);

                try {
                    dbc.driverConnect(inConnectionString, wideChar);
                    assertUser(expectedUser, dbc, wideChar);
                } finally {
                    dbc.disconnect();
                }
            } else {
                var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                    dbc.driverConnect(inConnectionString, wideChar);
                });
                assertTrue(e.getMessage().contains("AUTHENTICATION_ERROR"), e.getMessage());
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void userPassword(boolean wideChar) throws Exception {
        String user = getUser();
        if (noAuth) {
            if (user == null) {
                user = "user";
            }
        } else {
            assumeFalse(user == null, "user not specified");
        }
        String password = getPassword();

        Optional<String> expectedUser = getExpectedUser(new UsernamePasswordCredential(user, password));

        try (var dbc = createDbc()) {
            var sb = new StringBuilder(CONNECTION_STRING_DRIVER);
            appendTo(sb, "Endpoint", getEndpoint());
            appendTo(sb, "User", user);
            appendTo(sb, "Password", password);
            String inConnectionString = sb.toString();

            try {
                dbc.driverConnect(inConnectionString, wideChar);
                assertUser(expectedUser, dbc, wideChar);
            } finally {
                dbc.disconnect();
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void authToken(boolean wideChar) throws Exception {
        String token = getAuthToken();
        if (noAuth) {
            if (token == null) {
                token = "token";
            }
        } else {
            assumeFalse(token == null, "auth-token not specified");
        }

        Optional<String> expectedUser = getExpectedUser(new RememberMeCredential(token));

        try (var dbc = createDbc()) {
            var sb = new StringBuilder(CONNECTION_STRING_DRIVER);
            appendTo(sb, "Endpoint", getEndpoint());
            appendTo(sb, "AuthToken", token);
            String inConnectionString = sb.toString();

            try {
                dbc.driverConnect(inConnectionString, wideChar);
                assertUser(expectedUser, dbc, wideChar);
            } finally {
                dbc.disconnect();
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void fileCredential(boolean wideChar) throws Exception {
        Path tempFile = null;
        try {
            String path = getCredentials();
            if (noAuth) {
                if (path == null) {
                    tempFile = Files.createTempFile("tsubakuro-rust-odbc-dbtest.credential", "key");
                    Files.writeString(tempFile, "test");
                    path = tempFile.toString();
                }
            } else {
                assumeFalse(path == null, "auth-token not specified");
            }

            Optional<String> expectedUser = getExpectedUser(FileCredential.load(Path.of(path)));

            try (var dbc = createDbc()) {
                var sb = new StringBuilder(CONNECTION_STRING_DRIVER);
                appendTo(sb, "Endpoint", getEndpoint());
                appendTo(sb, "Credentials", path);
                String inConnectionString = sb.toString();

                try {
                    dbc.driverConnect(inConnectionString, wideChar);
                    assertUser(expectedUser, dbc, wideChar);
                } finally {
                    dbc.disconnect();
                }
            }
        } finally {
            if (tempFile != null) {
                Files.delete(tempFile);
            }
        }
    }

    private Optional<String> getExpectedUser(Credential credential) throws IOException, InterruptedException {
        var connector = TsurugiConnector.of(getEndpointJava(), credential);
        try (var session = connector.createSession()) {
            return session.getUserName();
        }
    }

    private static void assertUser(Optional<String> expectedUser, TgOdbcDbcHandle dbc, boolean wideChar) {
        String user = dbc.getInfoString(InfoType.SQL_USER_NAME, 32, wideChar);
        if (user != null && user.isEmpty()) {
            assertEquals(expectedUser, Optional.empty());
        } else {
            assertEquals(expectedUser, Optional.ofNullable(user));
        }
    }
}
