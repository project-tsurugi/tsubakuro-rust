package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.ValueLayout;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLFetchTest extends TgOdbcTester {

    @BeforeAll
    static void beforeAll() {
        dropAndCreateTable("test", "create table test (pk int primary key)");
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void fetch(boolean wideChar) {
        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            stmt.tables(wideChar);

            var rowsFetchedPtr = manager.allocateLong();
            stmt.setStmtAttr(StatementAttribute.SQL_ATTR_ROWS_FETCHED_PTR, rowsFetchedPtr, wideChar);

            while (stmt.fetch()) {
                long rowsFetched = rowsFetchedPtr.get(ValueLayout.JAVA_LONG, 0);
                assertEquals(1, rowsFetched);
            }

            long rowsFetched = rowsFetchedPtr.get(ValueLayout.JAVA_LONG, 0);
            assertEquals(0, rowsFetched);
        }
    }
}
