package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst.SQL_NO_NULLS;
import static com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst.SQL_NULLABLE;
import static org.junit.jupiter.api.Assertions.assertEquals;

import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle.DescribeColumn;

public class ExpectedDescribeCol {
    public static ExpectedDescribeCol ofVarchar(String columnName) {
        return new ExpectedDescribeCol(columnName, SqlDataType.SQL_VARCHAR);
    }

    public static ExpectedDescribeCol ofSmallInt(String columnName) {
        return new ExpectedDescribeCol(columnName, SqlDataType.SQL_SMALLINT);
    }

    public static ExpectedDescribeCol ofInteger(String columnName) {
        return new ExpectedDescribeCol(columnName, SqlDataType.SQL_INTEGER);
    }

    private final String columnName;
    private final SqlDataType dataType;
    private long columnSize;
    private short decimalDigits;
    private short nullable = SQL_NULLABLE;

    public ExpectedDescribeCol(String columnName, SqlDataType dataType) {
        this.columnName = columnName;
        this.dataType = dataType;
    }

    public ExpectedDescribeCol columnSize(long columnSize) {
        this.columnSize = columnSize;
        return this;
    }

    public ExpectedDescribeCol decimalDigits(int decimalDigits) {
        this.decimalDigits = (short) decimalDigits;
        return this;
    }

    public ExpectedDescribeCol nullable(short nullable) {
        this.nullable = nullable;
        return this;
    }

    public ExpectedDescribeCol nullable() {
        return nullable(SQL_NULLABLE);
    }

    public ExpectedDescribeCol noNulls() {
        return nullable(SQL_NO_NULLS);
    }

    public void test(DescribeColumn desc) {
        assertEquals(this.columnName, desc.columnName());
        assertEquals(this.dataType, desc.dataType());
        assertEquals(this.columnSize, desc.columnSize());
        assertEquals(this.decimalDigits, desc.decimalDigits());
        assertEquals(this.nullable, desc.nullabple());
    }
}
