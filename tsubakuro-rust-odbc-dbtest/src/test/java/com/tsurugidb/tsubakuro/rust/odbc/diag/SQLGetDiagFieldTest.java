package com.tsurugidb.tsubakuro.rust.odbc.diag;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDiagFieldArgument.DiagIdentifier;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

public class SQLGetDiagFieldTest extends TgOdbcTester {

    @BeforeAll
    static void beforeAll() {
        dropIfExists("test");
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getDiagField(boolean wideChar) {
        try (var stmt = createStmt()) {
            short rc = stmt.execDirect0("select * from test", OdbcConst.SQL_NTS, wideChar);
            assertEquals(SqlReturn.SQL_ERROR, rc);

            int rows = stmt.getDiagFieldInteger(0, DiagIdentifier.SQL_DIAG_NUMBER, wideChar);
            assertEquals(1, rows);

            {
                var rec1 = stmt.getDiagRec(1, wideChar);
                assertNotNull(rec1);

                var rec2 = stmt.getDiagRec(2, wideChar);
                assertNull(rec2);
            }
        }
    }
}
