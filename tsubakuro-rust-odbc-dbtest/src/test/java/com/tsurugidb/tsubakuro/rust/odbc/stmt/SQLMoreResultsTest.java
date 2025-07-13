package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLMoreResultsTest extends TgOdbcTester {

    @BeforeAll
    static void beforeAll() {
        dropAndCreateTable("test", """
                create table test (
                  foo int primary key,
                  bar bigint,
                  zzz varchar(10)
                )
                """);
    }

    @Test
    void moreResults() {
        final boolean wideChar = false;
        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by foo", wideChar);

//          assertFalse(stmt.moreResults());　// fetch前には呼べない
            while (stmt.fetch()) {
            }
            assertFalse(stmt.moreResults());
        }
    }

    @Test
    void moreResults2() {
        final boolean wideChar = false;
        try (var stmt = createStmt()) {
            var e = assertThrows(TgOdbcRuntimeException.class, () -> {
                stmt.execDirect("""
                        select * from test order by foo;
                        select * from test order by foo
                        """, wideChar);
            });
            assertTrue(e.getMessage().contains("UNSUPPORTED_RUNTIME_FEATURE_EXCEPTION"), () -> e.getMessage());

//            // assertFalse(stmt.moreResults());　// fetch前には呼べない
//            while (stmt.fetch()) {
//            }
//            assertTrue(stmt.moreResults());
//
//            while (stmt.fetch()) {
//            }
//            assertFalse(stmt.moreResults());
        }
    }
}
