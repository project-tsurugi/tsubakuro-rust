package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcColAttributeArgument.FieldIdentifier;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLColAttributeTest extends TgOdbcTester {

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

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void colAttribute(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.execDirect("select * from test order by foo", wideChar);

            {
                long actual = stmt.colAttributeNumeric(0, FieldIdentifier.SQL_DESC_COUNT, wideChar);
                assertEquals(3, actual);
            }

            test1(stmt, wideChar);
            test2(stmt, wideChar);
            test3(stmt, wideChar);
        }
    }

    private void test1(TgOdbcStmtHandle stmt, boolean wideChar) {
        final int i = 1;
        {
            String actual = stmt.colAttributeString(i, FieldIdentifier.SQL_DESC_NAME, wideChar);
            assertEquals("foo", actual);
        }
        {
            long actual = stmt.colAttributeNumeric(i, FieldIdentifier.SQL_DESC_CONCISE_TYPE, wideChar);
            assertEquals(SqlDataType.SQL_INTEGER.value(), actual);
        }
        {
            long actual = stmt.colAttributeNumeric(i, FieldIdentifier.SQL_DESC_UNSIGNED, wideChar);
            assertEquals(OdbcConst.SQL_FALSE, actual);
        }
    }

    private void test2(TgOdbcStmtHandle stmt, boolean wideChar) {
        final int i = 2;
        {
            String actual = stmt.colAttributeString(i, FieldIdentifier.SQL_DESC_NAME, wideChar);
            assertEquals("bar", actual);
        }
        {
            long actual = stmt.colAttributeNumeric(i, FieldIdentifier.SQL_DESC_CONCISE_TYPE, wideChar);
            assertEquals(SqlDataType.SQL_BIGINT.value(), actual);
        }
        {
            long actual = stmt.colAttributeNumeric(i, FieldIdentifier.SQL_DESC_UNSIGNED, wideChar);
            assertEquals(OdbcConst.SQL_FALSE, actual);
        }
    }

    private void test3(TgOdbcStmtHandle stmt, boolean wideChar) {
        final int i = 3;
        {
            String actual = stmt.colAttributeString(i, FieldIdentifier.SQL_DESC_NAME, wideChar);
            assertEquals("zzz", actual);
        }
        {
            long actual = stmt.colAttributeNumeric(i, FieldIdentifier.SQL_DESC_CONCISE_TYPE, wideChar);
            assertEquals(SqlDataType.SQL_CHAR.value(), actual); // TODO SQL_VARCHAR
        }
        {
            long actual = stmt.colAttributeNumeric(i, FieldIdentifier.SQL_DESC_UNSIGNED, wideChar);
            assertEquals(OdbcConst.SQL_TRUE, actual);
        }
    }
}
