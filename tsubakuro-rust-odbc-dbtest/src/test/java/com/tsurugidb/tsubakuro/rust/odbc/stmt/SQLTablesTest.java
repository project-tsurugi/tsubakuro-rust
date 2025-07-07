package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLTablesTest extends TgOdbcTester {

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void tables(boolean wideChar) {
        dropIfExists("test");
        testTables(false, wideChar);

        executeSql("create table test (pk int primary key)");
        testTables(true, wideChar);
    }

    private void testTables(boolean expectedFound, boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.tables(wideChar);

            int numberOfColumns = stmt.numResultCols();
            assertEquals(5, numberOfColumns);

            testDescribeCol(stmt, wideChar);

            boolean found = false;
            while (stmt.fetch()) {
                String databaseName = stmt.getDataString(1);
                String schemaName = stmt.getDataString(2);
                String tableName = stmt.getDataString(3);
                String typeName = stmt.getDataString(4);
                String remarks = stmt.getDataString(5);

                assertNull(databaseName);
                assertNull(schemaName);
                assertNotNull(tableName);
                if (tableName.equals("test")) {
                    found = true;
                }
                assertEquals("TABLE", typeName);
                assertNull(remarks);
            }

            assertEquals(expectedFound, found);
        }
    }

    private void testDescribeCol(TgOdbcStmtHandle stmt, boolean wideChar) {
        {
            var desc = stmt.describeCol(1, wideChar);
            ExpectedDescribeCol.ofVarchar("TABLE_CAT") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(2, wideChar);
            ExpectedDescribeCol.ofVarchar("TABLE_SCHEM") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(3, wideChar);
            ExpectedDescribeCol.ofVarchar("TABLE_NAME").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(4, wideChar);
            ExpectedDescribeCol.ofVarchar("TABLE_TYPE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(5, wideChar);
            ExpectedDescribeCol.ofVarchar("REMARKS") //
                    .test(desc);
        }
    }
}