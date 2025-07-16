package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcEnvHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class QueryResultLastEmptyTest extends TgOdbcTester {

    @BeforeAll
    static void beforeAll() {
        dropAndCreateTable("test", """
                create table test (
                  pk int primary key,
                  value varchar(10)
                )
                """);

        final boolean wideChar = false;
        try (var manager = new TgOdbcManager()) {
            try (var henv = TgOdbcEnvHandle.allocEnvHandle(manager)) {
                henv.setEnvAttr(OdbcAttrConst.SQL_ATTR_ODBC_VERSION, OdbcAttrConst.SQL_OV_ODBC3);
                try (var hdbc = TgOdbcDbcHandle.allocDbcHandle(henv)) {
                    String connectionString = getConnectionString();
                    try (var connection = hdbc.driverConnect(connectionString, wideChar)) {
                        insert(connection, wideChar);
                    }
                }
            }
        }
    }

    private static void insert(TgOdbcConnection connection, boolean wideChar) {
        connection.setAutoCommit(false, wideChar);

        try (var stmt = connection.createStmt()) {
            var manager = stmt.manager();

            var sql = "insert into test values(?, ?)";
            stmt.prepare(sql, wideChar);

            {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 1));
                stmt.bindParameter(2, TgOdbcBindParameter.ofStringUtf8(manager, "Hello"));
                stmt.execute();
            }
            {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 2));
                stmt.bindParameter(2, TgOdbcBindParameter.ofStringUtf8(manager, null));
                stmt.execute();
            }
            {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 3));
                stmt.bindParameter(2, TgOdbcBindParameter.ofStringUtf8(manager, "Hi"));
                stmt.execute();
            }
            {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 4));
                stmt.bindParameter(2, TgOdbcBindParameter.ofStringUtf8(manager, ""));
                stmt.execute();
            }
        }

        connection.commit();
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void select(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by pk", wideChar);

            {
                assertTrue(stmt.fetch());
                assertEquals(1, stmt.getDataInt(1));
                assertEquals("Hello", stmt.getDataString(2, 32, wideChar));
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(2, stmt.getDataInt(1));
                assertEquals(null, stmt.getDataString(2, 32, wideChar));
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(3, stmt.getDataInt(1));
                assertEquals("Hi", stmt.getDataString(2, 32, wideChar));
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(4, stmt.getDataInt(1));
                assertEquals("", stmt.getDataString(2, 32, wideChar));
            }
            assertFalse(stmt.fetch());
        }
    }
}
