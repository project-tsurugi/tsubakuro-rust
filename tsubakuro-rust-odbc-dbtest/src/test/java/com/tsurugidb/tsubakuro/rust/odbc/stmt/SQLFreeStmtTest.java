package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcEnvHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLFreeStmtTest extends TgOdbcTester {
    private static final Logger LOG = LoggerFactory.getLogger(SQLFreeStmtTest.class);

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
        }

        connection.commit();
    }

    @Test
    void SQL_CLOSE() {
        final boolean wideChar = false;
        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by foo", wideChar);
            assertTrue(stmt.fetch());

            stmt.freeStmt(FreeStmtOption.SQL_CLOSE);

            assertThrows(TgOdbcRuntimeException.class, () -> {
                stmt.fetch();
            });
        }
    }

    @Test
    void SQL_DROP() {
        final boolean wideChar = false;

        boolean closed = false;
        var stmt = createStmt();
        try {
            stmt.execDirect("select * from test order by foo", wideChar);
            assertTrue(stmt.fetch());

            // ODBCドライバーマネージャーがSQLFreeHandleを呼ぶ模様
            stmt.freeStmt(FreeStmtOption.SQL_DROP);
            closed = true;
        } finally {
            if (!closed) {
                try {
                    stmt.close();
                } catch (Throwable e) {
                    LOG.error("stmt.close() error", e);
                }
            }
        }
    }

    @Test
    void SQL_UNBIND() {
        final boolean wideChar = false;
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.execDirect("select * from test order by foo", wideChar);

            var foo = TgOdbcGetDataArgument.ofInt(manager);
            var bar = TgOdbcGetDataArgument.ofLong(manager);
            var zzz = TgOdbcGetDataArgument.ofString(manager, 1024, wideChar);

            stmt.bindCol(1, foo);
            stmt.bindCol(2, bar);
            stmt.bindCol(3, zzz);

            {
                assertTrue(stmt.fetch());
                assertEquals(1, foo.getData());
                assertEquals(111L, bar.getData());
                assertEquals("aaa", zzz.getData());

                // SQLFetch内でバインド領域にデータを取得済みなので、SQLGetDataは不可
                assertThrows(TgOdbcRuntimeException.class, () -> {
                    stmt.getDataInt(1);
                });
            }

            stmt.freeStmt(FreeStmtOption.SQL_UNBIND);

            {
                assertTrue(stmt.fetch());

                assertEquals(2, stmt.getDataInt(1));
                assertEquals(222L, stmt.getDataLong(2));
                assertEquals("bbb", stmt.getDataString(3));

                // バインドしていた領域には前のデータが残ったまま
                assertEquals(1, foo.getData());
                assertEquals(111L, bar.getData());
                assertEquals("aaa", zzz.getData());
            }
        }
    }

    @Test
    void SQL_RESET_PARAMS() {
        final boolean wideChar = false;
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 2));

            stmt.execDirect("select * from test where foo = ?", wideChar);
            {
                assertTrue(stmt.fetch());
                assertEquals(2, stmt.getDataInt(1));
                assertEquals(222L, stmt.getDataLong(2));
                assertEquals("bbb", stmt.getDataString(3));
                assertFalse(stmt.fetch());
            }

            stmt.execDirect("select * from test where foo = ?", wideChar);
            {
                assertTrue(stmt.fetch());
                assertEquals(2, stmt.getDataInt(1));
                assertEquals(222L, stmt.getDataLong(2));
                assertEquals("bbb", stmt.getDataString(3));
                assertFalse(stmt.fetch());
            }

            stmt.freeStmt(FreeStmtOption.SQL_RESET_PARAMS);

            var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                stmt.execDirect("select * from test where foo = ?", wideChar);
            });
            assertTrue(e.getMessage().contains("SYMBOL_ANALYZE_EXCEPTION"), () -> e.getMessage());
        }
    }
}
