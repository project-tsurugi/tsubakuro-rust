package com.tsurugidb.tsubakuro.rust.odbc.diag;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;

import java.net.URI;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDiagFieldArgument;
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
                String actual = stmt.getDiagFieldString(1, DiagIdentifier.SQL_DIAG_CONNECTION_NAME, 1024, wideChar);
                String endpoint = getEndpoint();
                assertEquals(endpoint, actual);
            }
            {
                String actual = stmt.getDiagFieldString(1, DiagIdentifier.SQL_DIAG_SERVER_NAME, 1024, wideChar);
                String endpoint = getEndpoint();
                String host = URI.create(endpoint).getHost();
                assertEquals(host, actual);
            }

            {
                var rec1 = stmt.getDiagRec(1, wideChar);
                assertNotNull(rec1);

                var rec2 = stmt.getDiagRec(2, wideChar);
                assertNull(rec2);
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void length(boolean wideChar) {
        final int charByte = wideChar ? 2 : 1;

        try (var stmt = createStmt()) {
            var manager = stmt.manager();

            short rc = stmt.execDirect0("select * from test", OdbcConst.SQL_NTS, wideChar);
            assertEquals(SqlReturn.SQL_ERROR, rc);

            String endpoint = getEndpoint();

            {
                int bufferLength = (endpoint.length() + 1) * charByte;
                var arg = TgOdbcDiagFieldArgument.ofString(manager, DiagIdentifier.SQL_DIAG_CONNECTION_NAME, bufferLength);
                rc = stmt.getDiagField0(1, arg, wideChar);
                assertEquals(SqlReturn.SQL_SUCCESS, rc);
                assertEquals(endpoint.length() * charByte, arg.stringLength());
            }
            {
                int bufferLength = (endpoint.length() + 0) * charByte;
                var arg = TgOdbcDiagFieldArgument.ofString(manager, DiagIdentifier.SQL_DIAG_CONNECTION_NAME, bufferLength);
                rc = stmt.getDiagField0(1, arg, wideChar);
                assertEquals(SqlReturn.SQL_SUCCESS_WITH_INFO, rc);
//              assertEquals(endpoint.length() * charByte, arg.stringLength()); // odbc32の返し方がおかしい
            }
        }
    }
}
