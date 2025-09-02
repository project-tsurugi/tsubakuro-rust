package com.tsurugidb.tsubakuro.rust.odbc.util;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.net.URI;
import java.nio.file.Path;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;

import com.tsurugidb.tsubakuro.channel.common.connection.Credential;
import com.tsurugidb.tsubakuro.channel.common.connection.FileCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.RememberMeCredential;
import com.tsurugidb.tsubakuro.channel.common.connection.UsernamePasswordCredential;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcEnvHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;

public class TgOdbcTester {

    private static final String SYSPROP_DBTEST_CONNECTION_STRING = "tsurugi.dbtest.connection.string";
    private static final String SYSPROP_DBTEST_DSN = "tsurugi.dbtest.dsn";
    private static final String SYSPROP_DBTEST_ENDPOINT = "tsurugi.dbtest.endpoint";
    private static final String SYSPROP_DBTEST_ENDPOINT_JAVA = "tsurugi.dbtest.endpoint.java";
    private static final String SYSPROP_DBTEST_USER = "tsurugi.dbtest.user";
    private static final String SYSPROP_DBTEST_PASSWORD = "tsurugi.dbtest.password";
    private static final String SYSPROP_DBTEST_AUTH_TOKEN = "tsurugi.dbtest.auth-token";
    private static final String SYSPROP_DBTEST_CREDENTIALS = "tsurugi.dbtest.credentials";

    private static String staticConnectionString;
    private static String staticDsn;
    private static String staticEndpoint;
    private static String staticEndpointJava;
    private static Credential staticCredentialJava;

    protected static final String CONNECTION_STRING_DRIVER = "Driver={Tsurugi Driver};";

    protected static String getConnectionString() {
        if (staticConnectionString == null) {
            staticConnectionString = System.getProperty(SYSPROP_DBTEST_CONNECTION_STRING);
            if (staticConnectionString == null || staticConnectionString.isEmpty()) {
                var sb = new StringBuilder(CONNECTION_STRING_DRIVER);
                appendTo(sb, "Endpoint", getEndpoint());

                String user = getUser();
                if (user != null) {
                    appendTo(sb, "User", user);
                    appendTo(sb, "Password", getPassword());
                } else {
                    String token = getAuthToken();
                    if (token != null) {
                        appendTo(sb, "AuthToken", token);
                    } else {
                        String path = getCredentials();
                        if (path != null) {
                            appendTo(sb, "Credentials", path);
                        } else {
                            appendTo(sb, "User", "tsurugi");
                            appendTo(sb, "Password", "password");
                        }
                    }
                }

                staticConnectionString = sb.toString();
            }
        }
        return staticConnectionString;
    }

    protected static void appendTo(StringBuilder sb, String key, String value) {
        if (value != null) {
            sb.append(key);
            sb.append("=");
            sb.append(value);
            sb.append(";");
        }
    }

    protected static String getDsn() {
        if (staticDsn == null) {
            staticDsn = System.getProperty(SYSPROP_DBTEST_DSN, "");
        }
        return staticDsn;
    }

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

    private static String getSystemProperty(String key) {
        String value = System.getProperty(key);
        if (value != null && value.isEmpty()) {
            return null;
        }
        return value;
    }

    private TgOdbcManager manager;
    private TgOdbcEnvHandle env;
    private TgOdbcDbcHandle dbc;
    private TgOdbcConnection connection;

    @BeforeEach
    void beforeEach() {
        this.manager = new TgOdbcManager();
    }

    protected TgOdbcManager getOdbcManager() {
        return this.manager;
    }

    protected TgOdbcEnvHandle getEnv() {
        if (this.env == null) {
            this.env = createEnv();
        }
        return this.env;
    }

    protected TgOdbcEnvHandle createEnv() {
        var env = TgOdbcEnvHandle.allocEnvHandle(manager);
        env.setEnvAttr(OdbcAttrConst.SQL_ATTR_ODBC_VERSION, OdbcAttrConst.SQL_OV_ODBC3);
        return env;
    }

    protected TgOdbcDbcHandle getDbc() {
        if (this.dbc == null) {
            this.dbc = createDbc();
        }
        return this.dbc;
    }

    protected TgOdbcDbcHandle createDbc() {
        var env = getEnv();
        return TgOdbcDbcHandle.allocDbcHandle(env);
    }

    protected TgOdbcConnection getConnection() {
        if (this.connection == null) {
            var dbc = getDbc();
            this.connection = createConnection(dbc);
        }
        return this.connection;
    }

    protected TgOdbcConnection createConnection(TgOdbcDbcHandle dbc) {
        String connectionString = getConnectionString();
        return dbc.driverConnect(connectionString, true);
    }

    protected TgOdbcStmtHandle createStmt() {
        var connection = getConnection();
        return connection.createStmt();
    }

    @AfterEach
    void afterEach() {
        try (var _ = this.manager; var _ = this.env; var _ = this.dbc; var _ = this.connection) {
        } finally {
            this.manager = null;
            this.env = null;
            this.dbc = null;
            this.connection = null;
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
        try (var manager = new TgOdbcManager()) {
            try (var henv = TgOdbcEnvHandle.allocEnvHandle(manager)) {
                henv.setEnvAttr(OdbcAttrConst.SQL_ATTR_ODBC_VERSION, OdbcAttrConst.SQL_OV_ODBC3);
                try (var hdbc = TgOdbcDbcHandle.allocDbcHandle(henv)) {
                    String connectionString = getConnectionString();
                    try (var connection = hdbc.driverConnect(connectionString, true)) {
                        connection.execute(sql, false);
                    }
                }
            }
        }
    }
}
