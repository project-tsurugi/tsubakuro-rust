package com.tsurugidb.tsubakuro.rust.odbc.util;

import java.net.URI;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;

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
    private static String staticConnectionString;
    private static String staticDsn;
    private static String staticEndpoint;
    private static String staticEndpointJava;

    protected static String getConnectionString() {
        if (staticConnectionString == null) {
            staticConnectionString = System.getProperty(SYSPROP_DBTEST_CONNECTION_STRING);
            if (staticConnectionString == null || staticConnectionString.isEmpty()) {
                String endpoint = getEndpoint();
                staticConnectionString = "Driver={Tsurugi Driver};Endpoint=%s;".formatted(endpoint);
            }
        }
        return staticConnectionString;
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
