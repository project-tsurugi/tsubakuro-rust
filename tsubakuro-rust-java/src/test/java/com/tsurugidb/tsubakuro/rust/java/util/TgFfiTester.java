package com.tsurugidb.tsubakuro.rust.java.util;

import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.fail;
import static org.junit.jupiter.api.Assumptions.assumeFalse;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.net.URI;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.function.Consumer;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;

import com.tsurugidb.iceaxe.TsurugiConnector;
import com.tsurugidb.iceaxe.session.TgSessionOption;
import com.tsurugidb.iceaxe.sql.type.IceaxeObjectFactory;
import com.tsurugidb.tsubakuro.channel.common.connection.Credential;
import com.tsurugidb.tsubakuro.channel.common.connection.FileCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.RememberMeCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.UsernamePasswordCredential;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiJob;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.system.TgFfiSystemClient;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiCredential;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiLobTransferType;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionType;

public class TgFfiTester {

    static {
        TgFfiInitializer.loadFfiLibrary();
        TgFfiInitializer.initFfiEnvLogger("tsubakuro_rust_core=warn", null);
    }

    private static final String SYSPROP_DBTEST_ENDPOINT = "tsurugi.dbtest.endpoint";
    private static final String SYSPROP_DBTEST_ENDPOINT_JAVA = "tsurugi.dbtest.endpoint.java";
    private static final String SYSPROP_DBTEST_USER = "tsurugi.dbtest.user";
    private static final String SYSPROP_DBTEST_PASSWORD = "tsurugi.dbtest.password";
    private static final String SYSPROP_DBTEST_AUTH_TOKEN = "tsurugi.dbtest.auth-token";
    private static final String SYSPROP_DBTEST_CREDENTIALS = "tsurugi.dbtest.credentials";
    private static final String SYSPROP_DBTEST_LOB_SEND_PATH_MAPPING = "tsurugi.dbtest.lob-send-path-mapping";
    private static final String SYSPROP_DBTEST_LOB_RECV_PATH_MAPPING = "tsurugi.dbtest.lob-recv-path-mapping";
    private static final String SYSPROP_DBTEST_BLOB_RELAY_SERVICE_ENDPOINT = "tsurugi.dbtest.blob-relay-service-endpoint";
    private static final String SYSPROP_DBTEST_BLOB_RELAY_SERVICE_CA_CERT_PEM_FILE = "tsurugi.dbtest.blob-relay-service-ca-cert-pem-file";

    private static String staticEndpoint;
    private static String staticEndpointJava;
    private static Credential staticCredentialJava;

    protected static String getEndpoint() {
        if (staticEndpoint == null) {
            staticEndpoint = System.getProperty(SYSPROP_DBTEST_ENDPOINT, "tcp://localhost:12345");
        }
        return staticEndpoint;
    }

    protected static String getEndpointJava() {
        if (staticEndpointJava == null) {
            String endpoint = System.getProperty(SYSPROP_DBTEST_ENDPOINT_JAVA);
            if (endpoint == null || endpoint.isEmpty()) {
                endpoint = getEndpoint();
            }
            staticEndpointJava = endpoint;
        }
        return staticEndpointJava;
    }

    protected static boolean isIpc(String endpoint) {
        var uri = URI.create(endpoint);
        String scheme = uri.getScheme();
        return "ipc".equals(scheme);
    }

    public static TgFfiCredential getCredential(TgFfiContext context) {
        String user = getUser();
        if (user != null) {
            String password = getPassword();
            return TgFfiCredential.fromUserPassword(context, user, password);
        }

        String authToken = getAuthToken();
        if (authToken != null) {
            return TgFfiCredential.fromAuthToken(context, authToken);
        }

        String credentials = getCredentials();
        if (credentials != null) {
            return TgFfiCredential.load(context, credentials);
        }

//      return TgFfiCredential.nullCredential(context);
        return TgFfiCredential.fromUserPassword(context, "tsurugi", "password");
    }

    public static Credential getCredentialJava() {
        if (staticCredentialJava == null) {
            staticCredentialJava = createCredentialJava();
        }
        return staticCredentialJava;
    }

    private static Credential createCredentialJava() {
        String user = getUser();
        if (user != null) {
            String password = getPassword();
            return new UsernamePasswordCredential(user, password);
        }

        String authToken = getAuthToken();
        if (authToken != null) {
            return new RememberMeCredential(authToken);
        }

        String credentials = getCredentials();
        if (credentials != null) {
            try {
                return FileCredential.load(Path.of(credentials));
            } catch (IOException e) {
                throw new UncheckedIOException(e.getMessage(), e);
            }
        }

//      return NullCredential.INSTANCE;
        return new UsernamePasswordCredential("tsurugi", "password");
    }

    public static String getUser() {
        return getSystemProperty(SYSPROP_DBTEST_USER);
    }

    public static String getPassword() {
        return getSystemProperty(SYSPROP_DBTEST_PASSWORD);
    }

    public static String getAuthToken() {
        return getSystemProperty(SYSPROP_DBTEST_AUTH_TOKEN);
    }

    public static String getCredentials() {
        return getSystemProperty(SYSPROP_DBTEST_CREDENTIALS);
    }

    public static TgFfiPathMappingEntry getLobSendPathMapping() {
        String s = getSystemProperty(SYSPROP_DBTEST_LOB_SEND_PATH_MAPPING);
        if (s == null) {
            return null;
        }
        return TgFfiPathMappingEntry.parse(s);
    }

    protected Path createLobFilePath(String fileName) {
        try {
            Path dir;
            var mapping = getLobSendPathMapping();
            if (mapping != null) {
                dir = mapping.clientPath();
                Files.createDirectories(dir);
            } else {
                dir = Path.of(System.getProperty("java.io.tmpdir"));
            }

            Path path = Files.createTempFile(dir, "tsubakuro-rust-ffi.test", fileName);
            path.toFile().deleteOnExit();
            return path;
        } catch (IOException e) {
            throw new UncheckedIOException(e.getMessage(), e);
        }
    }

    public static TgFfiPathMappingEntry getLobRecvPathMapping() {
        String s = getSystemProperty(SYSPROP_DBTEST_LOB_RECV_PATH_MAPPING);
        if (s == null) {
            return null;
        }
        return TgFfiPathMappingEntry.parse(s);
    }

    public static String getBlobRelayServiceEndpoint() {
        return getSystemProperty(SYSPROP_DBTEST_BLOB_RELAY_SERVICE_ENDPOINT);
    }

    public static String getBlobRelayServiceCaCertPemFile() {
        return getSystemProperty(SYSPROP_DBTEST_BLOB_RELAY_SERVICE_CA_CERT_PEM_FILE);
    }

    private static String getSystemProperty(String key) {
        String value = System.getProperty(key);
        if (value != null && value.isEmpty()) {
            return null;
        }
        return value;
    }

    protected static TsurugiConnector getTsurugiConnector() {
        return getTsurugiConnector(null);
    }

    protected static TsurugiConnector getTsurugiConnector(Consumer<TgSessionOption> customizer) {
        var sessionOption = TgSessionOption.of();
        var sendMapping = getLobSendPathMapping();
        if (sendMapping != null) {
            sessionOption = sessionOption.addLargeObjectPathMappingOnSend(sendMapping.clientPath(), sendMapping.serverPath());
            IceaxeObjectFactory.getDefaultInstance().setTempDirectory(sendMapping.clientPath());
        }
        var recvMapping = getLobRecvPathMapping();
        if (recvMapping != null) {
            sessionOption = sessionOption.addLargeObjectPathMappingOnReceive(recvMapping.serverPath(), recvMapping.clientPath());
        }
        var blobRelayEndpoint = getBlobRelayServiceEndpoint();
        if (blobRelayEndpoint != null) {
            String s = blobRelayEndpoint;
            if (s.startsWith("http://") || s.startsWith("https://")) {
                s = s.substring(s.indexOf("://") + 3);
            }
            var endpoint = URI.create(s);
            sessionOption = sessionOption.setBlobRelayServiceEndpoint(endpoint);
        }

        if (customizer != null) {
            customizer.accept(sessionOption);
        }

        return TsurugiConnector.of(getEndpointJava(), getCredentialJava(), sessionOption);
    }

    private TgFfiObjectManager manager;

    @BeforeEach
    void beforeEach() {
        this.manager = TgFfiObjectManager.create();
    }

    protected TgFfiObjectManager getFfiObjectManager() {
        return this.manager;
    }

    @AfterEach
    void afterEach() {
        try (var _ = this.manager) {
        } finally {
            this.manager = null;
        }
    }

    protected static void dropAndCreateTable(String tableName, String createSql) {
        dropIfExists(tableName);
        executeSql(createSql);
    }

    protected static void dropIfExists(String tableName) {
        executeSql("drop table if exists " + tableName);
    }

    protected static void executeSql(String sql) {
        try (var manager = TgFfiObjectManager.create(); //
                var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());
            connectionOption.setCredential(context, getCredential(context));
            connectionOption.setApplicationName(context, "tsubakuro-rust-java/test");
            connectionOption.setSessionLabel(context, "tsubakuro-rust-java/test.session");

            try (var session = TgFfiSession.connect(context, connectionOption); //
                    var client = session.makeSqlClient(context); //
                    var transactionOption = TgFfiTransactionOption.create(context)) {
                transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);
                transactionOption.setTransactionLabel(context, "tsubakuro-rust-java/execute()");

                try (var transaction = client.startTransaction(context, transactionOption)) {
                    try (var _ = client.execute(context, transaction, sql)) {
                    }
                    try (var commitOption = TgFfiCommitOption.create(context)) {
                        client.commit(context, transaction, commitOption);
                    }
                    transaction.close(context);
                }
            }
        }
    }

    protected TgFfiSession createSession() {
        return createSession(null);
    }

    protected TgFfiSession createSession(Consumer<TgFfiConnectionOption> customizer) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, getEndpoint());
            connectionOption.setCredential(context, getCredential(context));
            var sendMapping = getLobSendPathMapping();
            if (sendMapping != null) {
                connectionOption.addLargeObjectPathMappingOnSend(context, sendMapping.clientPath(), sendMapping.serverPath());
            }
            var recvMapping = getLobRecvPathMapping();
            if (recvMapping != null) {
                connectionOption.addLargeObjectPathMappingOnRecv(context, recvMapping.serverPath(), recvMapping.clientPath());
            }
            var blobRelayServiceEndpoint = getBlobRelayServiceEndpoint();
            if (blobRelayServiceEndpoint != null) {
                connectionOption.setBlobRelayServiceEndpoint(context, blobRelayServiceEndpoint);
            }
            var blobRelayServiceCaCertPemFile = getBlobRelayServiceCaCertPemFile();
            if (blobRelayServiceCaCertPemFile != null) {
                connectionOption.setBlobRelayServiceCaCertPemFile(context, blobRelayServiceCaCertPemFile);
            }

            if (customizer != null) {
                customizer.accept(connectionOption);
            }

            var session = TgFfiSession.connect(context, connectionOption);
            return session;
        }
    }

    protected TgFfiSqlClient createSqlClient() {
        var manager = getFfiObjectManager();
        var session = createSession();

        try (var context = TgFfiContext.create(manager)) {
            var client = session.makeSqlClient(context);
            return client;
        }
    }

    protected TgFfiSqlClient createSqlClient(TgFfiLobTransferType lobTransferType) {
        var manager = getFfiObjectManager();
        try (var context = TgFfiContext.create(manager)) {
            var session = createSession(option -> {
                option.setLobTransferType(context, lobTransferType);
            });

            var client = session.makeSqlClient(context);
            return client;
        }
    }

    protected TgFfiSystemClient createSystemClient() {
        var manager = getFfiObjectManager();
        var session = createSession();

        try (var context = TgFfiContext.create(manager)) {
            var client = session.makeSystemClient(context);
            return client;
        }
    }

    protected TgFfiTransaction startOcc(TgFfiSqlClient client) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var transactionOption = TgFfiTransactionOption.create(context)) {
            transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);

            var transaction = client.startTransaction(context, transactionOption);
            return transaction;
        }
    }

    protected void commit(TgFfiSqlClient client, TgFfiTransaction transaction) {
        var manager = getFfiObjectManager();

        try (var context = TgFfiContext.create(manager); //
                var commitOption = TgFfiCommitOption.create(context)) {
            client.commit(context, transaction, commitOption);
        }
    }

    public static record Entry(String name, Object value) {
    }

    public static class Row {
        private final List<Entry> values = new ArrayList<>();

        public Row add(String name, Object value) {
            values.add(new Entry(name, value));
            return this;
        }

        @Override
        public int hashCode() {
            return Objects.hash(values);
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null)
                return false;
            if (getClass() != obj.getClass())
                return false;
            Row other = (Row) obj;
            return Objects.equals(values, other.values);
        }
    }

    protected List<Row> select(TgFfiSqlQueryResult qr) {
        var rows = new ArrayList<Row>();

        try (var context = TgFfiContext.create(manager); //
                var metadata = qr.getMetadata(context)) {
            var columns = metadata.getColumns(context);
            while (qr.nextRow(context)) {
                var row = new Row();
                for (int i = 0; qr.nextColumn(context); i++) {
                    var column = columns.get(i);
                    String name = column.getName(context);

                    Object value;
                    if (qr.isNull(context)) {
                        value = null;
                    } else {
                        var type = column.getAtomType(context);
                        value = switch (type) {
                        case INT4 -> qr.fetchInt4(context);
                        case INT8 -> qr.fetchInt8(context);
                        case CHARACTER -> qr.fetchCharacter(context);
                        default -> throw new AssertionError("unsupported type " + type);
                        };
                    }

                    row.add(name, value);
                }

                rows.add(row);
            }
        }

        return rows;
    }

    public static final String DIRECT = "DIRECT";
    public static final String DIRECT_FOR = "DIRECT_FOR";
    public static final String TAKE = "TAKE";
    public static final String TAKE_FOR = "TAKE_FOR";
    public static final String TAKE_IF_READY = "TAKE_IF_READY";

    protected <T> T jobTake(TgFfiJob<T> job, String pattern) {
        try (var context = TgFfiContext.create(getFfiObjectManager())) {
            switch (pattern) {
            case TAKE:
                return job.take(context);
            case TAKE_FOR:
                return job.takeFor(context, Duration.ofSeconds(5));
            case TAKE_IF_READY:
                if (job.wait(context, Duration.ofSeconds(5)) == false) {
                    fail("TAKE_IF_READY: wait() timeout");
                }
                var valueOpt = job.takeIfReady(context);
                assertNotNull(valueOpt);
                return valueOpt.orElse(null);
            default:
                throw new AssertionError("unsupported pattern=" + pattern);
            }
        }
    }

    protected void commitAndClose(TgFfiSqlClient client, TgFfiTransaction transaction, String pattern) {
        var manager = getFfiObjectManager();

        RuntimeException re = null;
        try (var context = TgFfiContext.create(manager)) {
            try (var commitOption = TgFfiCommitOption.create(context)) {
                switch (pattern) {
                case DIRECT:
                    client.commit(context, transaction, commitOption);
                    break;
                case DIRECT_FOR:
                    client.commitFor(context, transaction, commitOption, Duration.ofSeconds(5));
                    break;
                default:
                    jobTake(client.commitAsync(context, transaction, commitOption), pattern);
                    break;
                }
            } catch (RuntimeException e) {
                re = e;
                throw e;
            } finally {
                try {
                    switch (pattern) {
                    case DIRECT:
                    default:
                        transaction.close(context);
                        break;
                    case DIRECT_FOR:
                        transaction.closeFor(context, Duration.ofSeconds(5));
                        break;
                    }
                } catch (RuntimeException e) {
                    if (re != null) {
                        re.addSuppressed(e);
                    } else {
                        throw e;
                    }
                }
            }
        }
    }

    protected static void assumeLobTest(String lobTransferType) {
        disabledIfEnvironmentVariable("TgFfiLobTest_" + lobTransferType);
    }

    protected static void disabledIfEnvironmentVariable(String value) {
        String env = System.getenv("FFI_DBTEST_DISABLE");
        if (env == null) {
            return;
        }

        boolean matched = env.contains(value);
        assumeFalse(matched, () -> "FFI_DBTEST_DISABLE contains " + value);
    }
}
