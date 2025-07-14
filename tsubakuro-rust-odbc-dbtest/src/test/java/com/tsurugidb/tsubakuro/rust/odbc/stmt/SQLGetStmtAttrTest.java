package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLGetStmtAttrTest extends TgOdbcTester {

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_ATTR_QUERY_TIMEOUT(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.setStmtAttr(StatementAttribute.SQL_ATTR_QUERY_TIMEOUT, 30L, wideChar);

            long actual = stmt.getStmtAttrLong(StatementAttribute.SQL_ATTR_QUERY_TIMEOUT, wideChar);
            assertEquals(30, actual);
        }
    }
}
