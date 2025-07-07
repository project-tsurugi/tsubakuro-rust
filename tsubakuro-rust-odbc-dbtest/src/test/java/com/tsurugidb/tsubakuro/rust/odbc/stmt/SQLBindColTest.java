package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcEnvHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLBindColTest extends TgOdbcTester {

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

    @Test
    void bindCol() {
        final boolean wideChar = false;
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            var foo = TgOdbcGetDataArgument.ofInt(manager);
            var bar = TgOdbcGetDataArgument.ofLong(manager);
            var zzz = TgOdbcGetDataArgument.ofString(manager, 1024, wideChar);

            stmt.bindCol(1, foo);
            stmt.bindCol(2, bar);
            stmt.bindCol(3, zzz);

            stmt.execDirect("select * from test order by foo", wideChar);

            {
                assertTrue(stmt.fetch());
                assertEquals(1, foo.getData());
                assertEquals(111L, bar.getData());
                assertEquals("aaa", zzz.getData());
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(2, foo.getData());
                assertEquals(222L, bar.getData());
                assertEquals("bbb", zzz.getData());
            }
            {
                assertTrue(stmt.fetch());
                assertEquals(3, foo.getData());
                assertNull(bar.getDataOrNull());
                assertEquals("ccc", zzz.getData());
            }
            assertFalse(stmt.fetch());
        }
    }
}
