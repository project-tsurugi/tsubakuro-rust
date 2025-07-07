package com.tsurugidb.tsubakuro.rust.odbc.handle;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcHandle.CompletionType;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLEndTranTest extends TgOdbcTester {

    private static final int SIZE = 1;

    @BeforeEach
    void before() {
        dropAndCreateTable("test", """
                create table test (
                  foo int primary key,
                  bar bigint,
                  zzz varchar(10)
                )
                """);

        insert(getConnection(), 1, 11, "aaa");
    }

    private void insert(TgOdbcConnection connection, int foo, long bar, String zzz) {
        try (var stmt = connection.createStmt()) {
            var sql = "insert into test values(%d, %d, '%s')".formatted(foo, bar, zzz);
            stmt.execDirect(sql, false);
        }
    }

    private int selectCount(TgOdbcConnection connection) {
        try (var stmt = connection.createStmt()) {
            var sql = "select count(*) from test";
            stmt.execDirect(sql, false);

            assertTrue(stmt.fetch());
            int count = stmt.getDataInt(1);
            assertFalse(stmt.fetch());

            return count;
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void commit(boolean wideChar) {
        var connection = getConnection();
        { // before insert
            int count = selectCount(connection);
            assertEquals(SIZE, count);
        }

        try (var dbc2 = createDbc(); //
                var connection2 = createConnection(dbc2)) {
            connection2.setAutoCommit(false, wideChar);

            try (var stmt = connection2.createStmt()) {
                var sql = "insert into test values(2, 22, 'bbb')";
                stmt.execDirect(sql, wideChar);
            }
            { // after insert, before commit
                int count = selectCount(connection);
                assertEquals(SIZE, count);
            }

            dbc2.endTran(CompletionType.SQL_COMMIT);

            { // after commit
                int count = selectCount(connection);
                assertEquals(SIZE + 1, count);
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void rollback(boolean wideChar) {
        var connection = getConnection();
        { // before insert
            int count = selectCount(connection);
            assertEquals(SIZE, count);
        }

        try (var dbc2 = createDbc(); //
                var connection2 = createConnection(dbc2)) {
            connection2.setAutoCommit(false, wideChar);

            try (var stmt = connection2.createStmt()) {
                var sql = "insert into test values(2, 22, 'bbb')";
                stmt.execDirect(sql, wideChar);
            }
            { // after insert, before rollback
                int count = selectCount(connection);
                assertEquals(SIZE, count);

                int count2 = selectCount(connection2);
                assertEquals(SIZE + 1, count2);
            }

            dbc2.endTran(CompletionType.SQL_ROLLBACK);

            { // after rollback
                int count = selectCount(connection);
                assertEquals(SIZE, count);
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void commit_when_autocommit(boolean wideChar) {
        try (var dbc = createDbc(); //
                var connection = createConnection(dbc)) {
            connection.setAutoCommit(true, wideChar);

            try (var stmt = connection.createStmt()) {
                var sql = "insert into test values(2, 22, 'bbb')";
                stmt.execDirect(sql, wideChar);
            }

            // autocommit時は、ドライバーマネージャーからODBCドライバーのSQLEndTranは呼ばれない模様
            dbc.endTran(CompletionType.SQL_COMMIT);
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void env_commit(boolean wideChar) {
        testEnvEndTran(CompletionType.SQL_COMMIT, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void env_rollback(boolean wideChar) {
        testEnvEndTran(CompletionType.SQL_ROLLBACK, wideChar);
    }

    private void testEnvEndTran(CompletionType completionType, boolean wideChar) {
        try (var env = createEnv()) {
            String connectionString = getConnectionString();

            int size = 3;
            var dbcList = new ArrayList<TgOdbcDbcHandle>();
            var connectionList = new ArrayList<TgOdbcConnection>();
            try {
                for (int i = 0; i < size; i++) {
                    var dbc = env.allocDbcHandle();
                    dbcList.add(dbc);

                    var connection = dbc.driverConnect(connectionString, wideChar);
                    connectionList.add(connection);
                    connection.setAutoCommit(false, wideChar);

                    try (var stmt = connection.createStmt()) {
                        var sql = "insert into test values(%d, %d, '%d')".formatted(i + 10, i, i);
                        stmt.execDirect(sql, wideChar);
                    }
                }

                { // after insert, before end_tran
                    int count = selectCount(getConnection());
                    assertEquals(SIZE, count);
                }

                env.endTran(completionType);

                { // after end_tran
                    int count = selectCount(getConnection());
                    if (completionType == CompletionType.SQL_COMMIT) {
                        assertEquals(SIZE + size, count);
                    } else {
                        assertEquals(SIZE, count);
                    }
                }
            } finally {
                RuntimeException re = null;
                for (var connection : connectionList) {
                    try {
                        connection.close();
                    } catch (Throwable e) {
                        if (re == null) {
                            re = new RuntimeException("close error");
                        }
                        re.addSuppressed(e);
                    }
                }
                for (var dbc : dbcList) {
                    try {
                        dbc.close();
                    } catch (Throwable e) {
                        if (re == null) {
                            re = new RuntimeException("close error");
                        }
                        re.addSuppressed(e);
                    }
                }
            }
        }
    }
}
