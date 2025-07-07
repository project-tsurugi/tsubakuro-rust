package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLPrimaryKeysTest extends TgOdbcTester {

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void primaryKey_0(boolean wideChar) {
        dropAndCreateTable("test", "create table test (value int)");
        testPrimaryKeys(List.of(), wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void primaryKey_1(boolean wideChar) {
        dropAndCreateTable("test", "create table test (pk int primary key)");
        testPrimaryKeys(List.of("pk"), wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void primaryKey_2(boolean wideChar) {
        dropAndCreateTable("test", "create table test (key1 int, key2 int, primary key(key1, key2))");
        testPrimaryKeys(List.of("key1", "key2"), wideChar);
    }

    private void testPrimaryKeys(List<String> expected, boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.primaryKeys("test", wideChar);

            int numberOfColumns = stmt.numResultCols();
            assertEquals(6, numberOfColumns);

            testDescribeCol(stmt, wideChar);

            var keyList = new ArrayList<String>();
            int seq = 1;
            while (stmt.fetch()) {
                String databaseName = stmt.getDataString(1);
                String schemaName = stmt.getDataString(2);
                String tableName = stmt.getDataString(3);
                String columnName = stmt.getDataString(4);
                short keySeq = stmt.getDataShort(5);
                String pkName = stmt.getDataString(6);

                assertEquals("", databaseName);
                assertEquals("", schemaName);
                assertEquals("test", tableName);
                keyList.add(columnName);
                assertEquals(seq++, keySeq);
                assertNull(pkName);
            }

            assertEquals(expected, keyList);
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
            ExpectedDescribeCol.ofVarchar("COLUMN_NAME").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(5, wideChar);
            ExpectedDescribeCol.ofSmallInt("KEY_SEQ").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(6, wideChar);
            ExpectedDescribeCol.ofVarchar("PK_NAME") //
                    .test(desc);
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void tableNameLength(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.primaryKeys("test", 4, wideChar);

            assertTrue(stmt.fetch());
        }

        try (var stmt = createStmt()) {
            stmt.primaryKeys("testz", 4, wideChar);

            assertTrue(stmt.fetch());
        }
    }
}