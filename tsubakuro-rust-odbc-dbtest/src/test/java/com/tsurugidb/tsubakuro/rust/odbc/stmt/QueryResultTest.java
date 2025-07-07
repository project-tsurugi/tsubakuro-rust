package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcEnvHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class QueryResultTest extends TgOdbcTester {

    @BeforeAll
    static void beforeAll() {
        dropAndCreateTable("test", """
                create table test (
                  foo int primary key,
                  bar bigint,
                  zzz varchar(10)
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

            var sql = "insert into test values(?, ?, ?)";
            stmt.prepare(sql, wideChar);

            {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 1));
                stmt.bindParameter(2, TgOdbcBindParameter.ofLong(manager, 111));
                stmt.bindParameter(3, TgOdbcBindParameter.ofStringUtf8(manager, "aaa"));
                stmt.execute();
            }
            {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 2));
                stmt.bindParameter(2, TgOdbcBindParameter.ofLong(manager, 222));
                stmt.bindParameter(3, TgOdbcBindParameter.ofStringUtf8(manager, "bbb"));
                stmt.execute();
            }
            {
                stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 3));
                stmt.bindParameter(2, TgOdbcBindParameter.ofLong(manager, null));
                stmt.bindParameter(3, TgOdbcBindParameter.ofStringUtf8(manager, "ccc"));
                stmt.execute();
            }
        }

        connection.commit();
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void normal(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by foo", wideChar);

            {
                assertTrue(stmt.fetch());
                assertEquals(1, stmt.getDataInt(1));
                assertEquals(111L, stmt.getDataLong(2));
                assertEquals("aaa", stmt.getDataString(3, 32));
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(2, stmt.getDataInt(1));
                assertEquals(222L, stmt.getDataLong(2));
                assertEquals("bbb", stmt.getDataString(3, 32));
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(3, stmt.getDataInt(1));
                assertNull(stmt.getDataLong(2));
                assertEquals("ccc", stmt.getDataString(3, 32));
            }
            assertFalse(stmt.fetch());
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void skip(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by foo", wideChar);

            {
                assertTrue(stmt.fetch());
//              assertEquals(1, stmt.getDataInt(1));
                assertEquals(111L, stmt.getDataLong(2));
                assertEquals("aaa", stmt.getDataString(3, 32));
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(2, stmt.getDataInt(1));
//              assertEquals(222L, stmt.getDataLong(2));
                assertEquals("bbb", stmt.getDataString(3, 32));
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(3, stmt.getDataInt(1));
                assertNull(stmt.getDataLong(2));
//              assertEquals("ccc", stmt.getDataString(3, 32));
            }
            assertFalse(stmt.fetch());
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void alreadyFetched(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by foo", wideChar);

            {
                assertTrue(stmt.fetch());
                assertEquals(111L, stmt.getDataLong(2));

                var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                    stmt.getDataInt(1);
                });
                assertTrue(e.getMessage().contains("Already fetched"), () -> e.getMessage());

                assertEquals("aaa", stmt.getDataString(3, 32));
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(2, stmt.getDataInt(1));

                var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                    stmt.getDataInt(1);
                });
                assertTrue(e.getMessage().contains("Already fetched"), () -> e.getMessage());

                assertEquals(222L, stmt.getDataLong(2));
                assertEquals("bbb", stmt.getDataString(3, 32));
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void indexOutOfBounds(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by foo", wideChar);

            {
                assertTrue(stmt.fetch());
                assertEquals(1, stmt.getDataInt(1));
                assertEquals(111L, stmt.getDataLong(2));
                assertEquals("aaa", stmt.getDataString(3, 32));

                var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                    stmt.getDataString(4, 32);
                });
                assertTrue(e.getMessage().contains("column_number must be between 1 and 3"), () -> e.getMessage());
            }
            {
                assertTrue(stmt.fetch());

                var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                    stmt.getDataString(4, 32);
                });
                assertTrue(e.getMessage().contains("column_number must be between 1 and 3"), () -> e.getMessage());
            }
        }
    }
}
