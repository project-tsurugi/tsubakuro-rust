package com.tsurugidb.tsubakuro.rust.java.session;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assumptions.assumeFalse;

import java.io.IOException;
import java.lang.foreign.MemorySegment;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Optional;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.exception.TsurugiIOException;
import com.tsurugidb.tsubakuro.channel.common.connection.Credential;
import com.tsurugidb.tsubakuro.channel.common.connection.FileCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.NullCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.RememberMeCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.UsernamePasswordCredential;
import com.tsurugidb.tsubakuro.exception.CoreServiceCode;
import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiCredentialTest extends TgFfiTester {

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

    @Test
    void nullCredential() throws Exception {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var credential = TgFfiCredential.nullCredential(context);

            var connectionOption = TgFfiConnectionOption.create(context);
            connectionOption.setEndpointUrl(context, getEndpoint());
            connectionOption.setCredential(context, credential);

            if (noAuth) {
                Optional<String> expectedUser = getExpectedUser(NullCredential.INSTANCE);

                try (var session = TgFfiSession.connect(context, connectionOption)) {
                    Optional<String> user = session.getUserName(context);
                    assertEquals(expectedUser, user);
                }
            } else {
                var e = assertThrows(TgFfiRuntimeException.class, () -> {
                    try (var _ = TgFfiSession.connect(context, connectionOption)) {
                    }
                });
                assertEquals("AUTHENTICATION_ERROR", e.getErrorName());
            }
        }
    }

    @Test
    void null_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_null(ctx, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
    }

    @Test
    void from_user_password() throws Exception {
        var manager = getFfiObjectManager();

        String user = getUser();
        String password = getPassword();
        if (noAuth) {
            if (user == null) {
                user = "user";
            }
        } else {
            assumeFalse(user == null, "user not specified");
        }

        Optional<String> expectedUser = getExpectedUser(new UsernamePasswordCredential(user, password));

        try (var context = TgFfiContext.create(manager)) {
            var credential = TgFfiCredential.fromUserPassword(context, user, password);

            var connectionOption = TgFfiConnectionOption.create(context);
            connectionOption.setEndpointUrl(context, getEndpoint());
            connectionOption.setCredential(context, credential);

            try (var session = TgFfiSession.connect(context, connectionOption)) {
                Optional<String> userFfi = session.getUserName(context);
                assertEquals(expectedUser, userFfi);
            }
        }

        if (!noAuth) {
            String invalidPassword = (password != null) ? "" : "empty";
            try (var context = TgFfiContext.create(manager)) {
                var credential = TgFfiCredential.fromUserPassword(context, user, invalidPassword);

                var connectionOption = TgFfiConnectionOption.create(context);
                connectionOption.setEndpointUrl(context, getEndpoint());
                connectionOption.setCredential(context, credential);

                var e = assertThrows(TgFfiRuntimeException.class, () -> {
                    try (var _ = TgFfiSession.connect(context, connectionOption)) {
                    }
                });
                assertEquals("SCD-00201", e.getServerErrorStructuredCode());
                assertEquals("AUTHENTICATION_ERROR", e.getErrorName());
            }
        }
    }

    @Test
    void from_user_password_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = MemorySegment.NULL;
            var arg2 = manager.allocateString("password");
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_from_user_password(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg1 = manager.allocateString("user");
            var arg2 = manager.allocateString("password");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_from_user_password(ctx, arg1, arg2, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
        }
    }

    @Test
    void from_auth_token() throws Exception {
        var manager = getFfiObjectManager();

        String token = getAuthToken();
        if (noAuth) {
            if (token == null) {
                token = "auth token";
            }
        } else {
            assumeFalse(token == null, "auth-token not specified");
        }

        Optional<String> expectedUser = getExpectedUser(new RememberMeCredential(token));

        try (var context = TgFfiContext.create(manager)) {
            var credential = TgFfiCredential.fromAuthToken(context, token);

            var connectionOption = TgFfiConnectionOption.create(context);
            connectionOption.setEndpointUrl(context, getEndpoint());
            connectionOption.setCredential(context, credential);

            try (var session = TgFfiSession.connect(context, connectionOption)) {
                Optional<String> userFfi = session.getUserName(context);
                assertEquals(expectedUser, userFfi);
            }
        }
    }

    @Test
    void from_auth_token_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_from_auth_token(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = manager.allocateString("token");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_from_auth_token(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    @Test
    void load() throws Exception {
        var manager = getFfiObjectManager();

        Path tempFile = null;
        try {
            Path path = getCredentials();
            if (noAuth) {
                if (path == null) {
                    tempFile = Files.createTempFile("tsubakuro-rust-java.file-credential", "key");
                    Files.writeString(path, "test");
                    path = tempFile;
                }
            } else {
                assumeFalse(path == null, "credential file path not specified");
            }

            Optional<String> expectedUser = getExpectedUser(FileCredential.load(path));

            try (var context = TgFfiContext.create(manager)) {
                var credential = TgFfiCredential.load(context, path);

                var connectionOption = TgFfiConnectionOption.create(context);
                connectionOption.setEndpointUrl(context, getEndpoint());
                connectionOption.setCredential(context, credential);

                try (var session = TgFfiSession.connect(context, connectionOption)) {
                    Optional<String> userFfi = session.getUserName(context);
                    assertEquals(expectedUser, userFfi);
                }
            }
        } finally {
            if (tempFile != null) {
                Files.delete(tempFile);
            }
        }
    }

    @Test
    void load_argError() {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = MemorySegment.NULL;
            var out = manager.allocateHandleOut();
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_load(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
        }
        try (var context = TgFfiContext.create(manager)) {
            var ctx = context.handle();
            var arg = manager.allocateString("path");
            var out = MemorySegment.NULL;
            var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_credential_load(ctx, arg, out);
            assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
        }
    }

    private Optional<String> getExpectedUser(Credential credential) throws IOException, InterruptedException {
        var connector = TsurugiConnector.of(getEndpointJava(), credential);
        try (var session = connector.createSession()) {
            return session.getUserName();
        }
    }
}
