package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLGetTypeInfoTest extends TgOdbcTester {

    private static final short SQL_ALL_TYPES = 0;

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_allTypes(boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.getTypeInfo(SQL_ALL_TYPES, wideChar);

            int numberOfColumns = stmt.numResultCols();
            assertEquals(19, numberOfColumns);

            testDescribeCol(stmt, wideChar);

            var list = new ArrayList<SqlDataType>();
            while (stmt.fetch()) {
                var dataType = SqlDataType.fromValue(stmt.getDataShort(2));
                list.add(dataType);
            }

            var expected = List.of( //
                    SqlDataType.SQL_INTEGER, //
                    SqlDataType.SQL_BIGINT, //
                    SqlDataType.SQL_REAL, //
                    SqlDataType.SQL_DOUBLE, //
                    SqlDataType.SQL_DECIMAL, //
                    SqlDataType.SQL_CHAR, //
                    SqlDataType.SQL_VARCHAR, //
                    SqlDataType.SQL_BINARY, //
                    SqlDataType.SQL_VARBINARY //
            );
            assertEquals(expected, list);
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_INTEGER(boolean wideChar) {
        var dataType = SqlDataType.SQL_INTEGER;
        var expected = expectedInt(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_BIGINT(boolean wideChar) {
        var dataType = SqlDataType.SQL_BIGINT;
        var expected = expectedBigint(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_REAL(boolean wideChar) {
        var dataType = SqlDataType.SQL_REAL;
        var expected = expectedReal(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_FLOAT(boolean wideChar) {
        var dataType = SqlDataType.SQL_FLOAT;
        var expected = expectedReal(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_DOUBLE(boolean wideChar) {
        var dataType = SqlDataType.SQL_DOUBLE;
        var expected = expectedDouble(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_DECIMAL(boolean wideChar) {
        var dataType = SqlDataType.SQL_DECIMAL;
        var expected = expectedDecimal(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_NUMERIC(boolean wideChar) {
        var dataType = SqlDataType.SQL_NUMERIC;
        var expected = expectedDecimal(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_CHAR(boolean wideChar) {
        var dataType = SqlDataType.SQL_CHAR;
        var expected = expectedChar(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_WCHAR(boolean wideChar) {
        var dataType = SqlDataType.SQL_WCHAR;
        var expected = expectedChar(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_VARCHAR(boolean wideChar) {
        var dataType = SqlDataType.SQL_VARCHAR;
        var expected = expectedVarchar(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_WVARCHAR(boolean wideChar) {
        var dataType = SqlDataType.SQL_WVARCHAR;
        var expected = expectedVarchar(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_BINARY(boolean wideChar) {
        var dataType = SqlDataType.SQL_BINARY;
        var expected = expectedBinary(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void getTypeInfo_VARBINARY(boolean wideChar) {
        var dataType = SqlDataType.SQL_VARBINARY;
        var expected = expectedVarbinary(dataType);
        testGetTypeInfo(dataType, expected, wideChar);
    }

    private static ExpectedTypeInfo expectedInt(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "INT");
        expected.columnSize(32).numPrecRadix(2);
        expected.unsignedAttribute(false);
        expected.autoUniqueValue(false);
        return expected;
    }

    private static ExpectedTypeInfo expectedBigint(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "BIGINT");
        expected.columnSize(64).numPrecRadix(2);
        expected.unsignedAttribute(false);
        expected.autoUniqueValue(false);
        return expected;
    }

    private static ExpectedTypeInfo expectedReal(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "REAL");
        expected.columnSize(38).numPrecRadix(10);
        expected.unsignedAttribute(false);
        return expected;
    }

    private static ExpectedTypeInfo expectedDouble(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "DOUBLE");
        expected.columnSize(308).numPrecRadix(10);
        expected.unsignedAttribute(false);
        return expected;
    }

    private static ExpectedTypeInfo expectedDecimal(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "DECIMAL");
        expected.columnSize(38).numPrecRadix(10);
        expected.createParams("precision,scale");
        expected.unsignedAttribute(false);
        return expected;
    }

    private static ExpectedTypeInfo expectedChar(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "CHAR");
        expected.columnSize(2097132);
        expected.literalPrefix("'").literalSuffix("'");
        expected.createParams("length");
        expected.caseSensitive(true);
        expected.searchable(OdbcConst.SQL_PRED_CHAR);
        return expected;
    }

    private static ExpectedTypeInfo expectedVarchar(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "VARCHAR");
        expected.columnSize(2097132);
        expected.literalPrefix("'").literalSuffix("'");
        expected.createParams("length");
        expected.caseSensitive(true);
        expected.searchable(OdbcConst.SQL_PRED_CHAR);
        return expected;
    }

    private static ExpectedTypeInfo expectedBinary(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "BINARY");
        expected.columnSize(2097132);
        expected.literalPrefix("X'").literalSuffix("'");
        expected.createParams("length");
        return expected;
    }

    private static ExpectedTypeInfo expectedVarbinary(SqlDataType dataType) {
        var expected = new ExpectedTypeInfo(dataType, "VARBINARY");
        expected.columnSize(2097132);
        expected.literalPrefix("X'").literalSuffix("'");
        expected.createParams("length");
        return expected;
    }

    private void testGetTypeInfo(SqlDataType dataType, ExpectedTypeInfo expected, boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.getTypeInfo(dataType, wideChar);

            int numberOfColumns = stmt.numResultCols();
            assertEquals(19, numberOfColumns);

            testDescribeCol(stmt, wideChar);

            assertTrue(stmt.fetch());
            expected.test(stmt);
            assertFalse(stmt.fetch());
        }
    }

    private void testDescribeCol(TgOdbcStmtHandle stmt, boolean wideChar) {
        {
            var desc = stmt.describeCol(1, wideChar);
            ExpectedDescribeCol.ofVarchar("TYPE_NAME").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(2, wideChar);
            ExpectedDescribeCol.ofSmallInt("DATA_TYPE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(3, wideChar);
            ExpectedDescribeCol.ofInteger("COLUMN_SIZE") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(4, wideChar);
            ExpectedDescribeCol.ofVarchar("LITERAL_PREFIX") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(5, wideChar);
            ExpectedDescribeCol.ofVarchar("LITERAL_SUFFIX") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(6, wideChar);
            ExpectedDescribeCol.ofVarchar("CREATE_PARAMS") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(7, wideChar);
            ExpectedDescribeCol.ofSmallInt("NULLABLE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(8, wideChar);
            ExpectedDescribeCol.ofSmallInt("CASE_SENSITIVE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(9, wideChar);
            ExpectedDescribeCol.ofSmallInt("SEARCHABLE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(10, wideChar);
            ExpectedDescribeCol.ofSmallInt("UNSIGNED_ATTRIBUTE") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(11, wideChar);
            ExpectedDescribeCol.ofSmallInt("FIXED_PREC_SCALE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(12, wideChar);
            ExpectedDescribeCol.ofSmallInt("AUTO_UNIQUE_VALUE") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(13, wideChar);
            ExpectedDescribeCol.ofVarchar("LOCAL_TYPE_NAME") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(14, wideChar);
            ExpectedDescribeCol.ofSmallInt("MINIMUM_SCALE") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(15, wideChar);
            ExpectedDescribeCol.ofSmallInt("MAXIMUM_SCALE") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(16, wideChar);
            ExpectedDescribeCol.ofSmallInt("SQL_DATA_TYPE").noNulls() //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(17, wideChar);
            ExpectedDescribeCol.ofSmallInt("SQL_DATETIME_SUB") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(18, wideChar);
            ExpectedDescribeCol.ofInteger("NUM_PREC_RADIX") //
                    .test(desc);
        }
        {
            var desc = stmt.describeCol(19, wideChar);
            ExpectedDescribeCol.ofSmallInt("INTERVAL_PRECISION") //
                    .test(desc);
        }
    }
}
