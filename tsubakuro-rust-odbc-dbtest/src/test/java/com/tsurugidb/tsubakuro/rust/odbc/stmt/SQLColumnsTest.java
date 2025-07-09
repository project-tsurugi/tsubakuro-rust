package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLColumnsTest extends TgOdbcTester {

    @BeforeAll
    static void beforeAll() {
        dropAndCreateTable("test", """
                /** table description*/
                create table test (
                  /** int value */
                  foo int primary key,
                  /** long value */
                  bar bigint not null,
                  /** string value */
                  zzz varchar(10),
                  /** decimal value */
                  num decimal(15, 3)
                )
                """);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void columns(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.columns("test", wideChar);

            int numberOfColumns = stmt.numResultCols();
            assertEquals(18, numberOfColumns);

            testDescribeCol(stmt, wideChar);

            assertTrue(stmt.fetch());
            new ExpectedColumn(1, "foo").initialize("INT").notNull().remarks("int value") //
                    .test(stmt);
            assertTrue(stmt.fetch());
            new ExpectedColumn(2, "bar").initialize("BIGINT").notNull().remarks("long value") //
                    .test(stmt);
            assertTrue(stmt.fetch());
            new ExpectedColumn(3, "zzz").initialize("VARCHAR(10)").remarks("string value") //
                    .test(stmt);
            assertTrue(stmt.fetch());
            new ExpectedColumn(4, "num").initialize("DECIMAL(15, 3)").remarks("decimal value") //
                    .test(stmt);
            assertFalse(stmt.fetch());
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
            ExpectedDescribeCol.ofSmallInt("DATA_TYPE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(6, wideChar);
            ExpectedDescribeCol.ofVarchar("TYPE_NAME").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(7, wideChar);
            ExpectedDescribeCol.ofInteger("COLUMN_SIZE") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(8, wideChar);
            ExpectedDescribeCol.ofInteger("BUFFER_LENGTH") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(9, wideChar);
            ExpectedDescribeCol.ofSmallInt("DECIMAL_DIGITS") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(10, wideChar);
            ExpectedDescribeCol.ofSmallInt("NUM_PREC_RADIX") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(11, wideChar);
            ExpectedDescribeCol.ofSmallInt("NULLABLE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(12, wideChar);
            ExpectedDescribeCol.ofVarchar("REMARKS") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(13, wideChar);
            ExpectedDescribeCol.ofVarchar("COLUMN_DEF") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(14, wideChar);
            ExpectedDescribeCol.ofSmallInt("SQL_DATA_TYPE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(15, wideChar);
            ExpectedDescribeCol.ofSmallInt("SQL_DATETIME_SUB") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(16, wideChar);
            ExpectedDescribeCol.ofInteger("CHAR_OCTET_LENGTH") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(17, wideChar);
            ExpectedDescribeCol.ofInteger("ORDINAL_POSITION").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(18, wideChar);
            ExpectedDescribeCol.ofVarchar("IS_NULLABLE") //
                    .test(desc);
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void tableNameLength(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.columns("test", 4, wideChar);

            assertTrue(stmt.fetch());
        }

        try (var stmt = createStmt()) {
            stmt.columns("testz", 4, wideChar);

            assertTrue(stmt.fetch());
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void notFound(boolean wideChar) {
        dropIfExists("not_found_test");

        try (var stmt = createStmt()) {
            stmt.columns("not_found_test", wideChar);

            assertFalse(stmt.fetch());
        }
    }
}
