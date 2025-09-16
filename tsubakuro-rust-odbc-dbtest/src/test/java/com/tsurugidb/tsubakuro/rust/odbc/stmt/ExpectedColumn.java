package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataTypeSubCode;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;

public class ExpectedColumn {
    private final int ordinalPosition;
    private final String columnName;

    private String tableName = "test";
    private SqlDataType dataType;
    private String typeName;
    private String typeBaseName;
    private Integer columnSize;
    private Integer bufferLength;
    private Short decimalDigits;
    private Short numPrecRadix;
    private boolean nullable = true;
    private String remarks;
    private Short datetimeSub;

    private long descLength;

    public ExpectedColumn(int ordinalPosition, String columnName) {
        this.ordinalPosition = ordinalPosition;
        this.columnName = columnName;
    }

    public ExpectedColumn initialize(String typeName) {
        this.typeName = typeName.toUpperCase();

        String baseName;
        String arg1 = null;
        Integer size1 = null;
        String arg2 = null;
        Integer size2 = null;
        {
            int n = typeName.indexOf('(');
            if (n >= 0) {
                baseName = typeName.substring(0, n).trim().toUpperCase();
                int c = typeName.indexOf(',');
                if (c >= 0) {
                    arg1 = typeName.substring(n + 1, c).trim();
                    size1 = Integer.parseInt(arg1);
                    int m = typeName.indexOf(')');
                    arg2 = typeName.substring(c + 1, m).trim();
                    size2 = Integer.parseInt(arg2);
                } else {
                    int m = typeName.indexOf(')');
                    arg1 = typeName.substring(n + 1, m).trim();
                    try {
                        size1 = Integer.parseInt(arg1);
                    } catch (NumberFormatException ignore) {
                    }
                }
            } else {
                baseName = typeName.trim().toUpperCase();
            }
        }
        this.typeBaseName = baseName;

        switch (baseName) {
        case "BOOLEAN":
            this.dataType = SqlDataType.SQL_BIT;
            this.columnSize = 1;
            this.numPrecRadix = 2;
            this.bufferLength = 1;
            this.descLength = 1;
            break;
        case "INT":
            this.dataType = SqlDataType.SQL_INTEGER;
            this.columnSize = 32;
            this.numPrecRadix = 2;
            this.bufferLength = 4;
            this.descLength = 10;
            break;
        case "BIGINT":
            this.dataType = SqlDataType.SQL_BIGINT;
            this.columnSize = 64;
            this.numPrecRadix = 2;
            this.bufferLength = 8;
            this.descLength = 19;
            break;
        case "REAL":
            this.dataType = SqlDataType.SQL_REAL;
            this.columnSize = 38;
            this.numPrecRadix = 10;
            this.bufferLength = 4;
            this.descLength = 7;
            break;
        case "DOUBLE":
            this.dataType = SqlDataType.SQL_DOUBLE;
            this.columnSize = 308;
            this.numPrecRadix = 10;
            this.bufferLength = 8;
            this.descLength = 15;
            break;
        case "DECIMAL":
            this.dataType = SqlDataType.SQL_DECIMAL;
            this.bufferLength = 19;
            this.decimalDigits = size2.shortValue();
            this.numPrecRadix = 10;
            this.descLength = size1;
            break;
        case "CHAR":
            this.dataType = SqlDataType.SQL_CHAR;
            this.columnSize = size1;
            this.bufferLength = size1;
            this.descLength = size1;
            break;
        case "VARCHAR":
            this.dataType = SqlDataType.SQL_VARCHAR;
            this.columnSize = size1;
            this.bufferLength = size1;
            this.descLength = size1;
            break;
        case "BINARY":
            this.dataType = SqlDataType.SQL_BINARY;
            this.columnSize = size1;
            this.bufferLength = size1;
            this.descLength = size1;
            break;
        case "VARBINARY":
            this.dataType = SqlDataType.SQL_VARBINARY;
            if ("*".equals(arg1)) {
                this.columnSize = 2097132;
                this.bufferLength = 2097132;
            } else {
                this.columnSize = size1;
                this.bufferLength = size1;
            }
            this.descLength = 2097132;
            break;
        case "DATE":
            this.dataType = SqlDataType.SQL_TYPE_DATE;
            this.columnSize = 10;
            this.descLength = this.columnSize;
            this.datetimeSub = SqlDataTypeSubCode.SQL_CODE_DATE.value();
            break;
        case "TIME":
            this.dataType = SqlDataType.SQL_TYPE_TIME;
            this.columnSize = 18;
            this.decimalDigits = 9;
            this.descLength = this.columnSize;
            this.datetimeSub = SqlDataTypeSubCode.SQL_CODE_TIME.value();
            break;
        case "TIMESTAMP":
            this.dataType = SqlDataType.SQL_TYPE_TIMESTAMP;
            this.columnSize = 10 + 1 + 18;
            this.decimalDigits = 9;
            this.descLength = this.columnSize;
            this.datetimeSub = SqlDataTypeSubCode.SQL_CODE_TIMESTAMP.value();
            break;
        case "TIME WITH TIME ZONE":
            this.dataType = SqlDataType.SQL_TYPE_TIME;
            this.columnSize = 18 + 6;
            this.decimalDigits = 9;
            this.descLength = this.columnSize;
            this.datetimeSub = SqlDataTypeSubCode.SQL_CODE_TIME.value();
            break;
        case "TIMESTAMP WITH TIME ZONE":
            this.dataType = SqlDataType.SQL_TYPE_TIMESTAMP;
            this.columnSize = (10 + 1 + 18) + 6;
            this.decimalDigits = 9;
            this.descLength = this.columnSize;
            this.datetimeSub = SqlDataTypeSubCode.SQL_CODE_TIMESTAMP.value();
            break;
        default:
            throw new AssertionError("not yet implements. type=" + typeName);
        }

        return this;
    }

    public ExpectedColumn tableName(String tableName) {
        this.tableName = tableName;
        return this;
    }

    public ExpectedColumn dataType(SqlDataType dataType) {
        this.dataType = dataType;
        return this;
    }

    public ExpectedColumn typeName(String typeName) {
        this.typeName = typeName;
        return this;
    }

    public ExpectedColumn columnSize(Integer columnSize) {
        this.columnSize = columnSize;
        return this;
    }

    public ExpectedColumn bufferLength(Integer bufferLength) {
        this.bufferLength = bufferLength;
        return this;
    }

    public ExpectedColumn decimalDigits(Short decimalDigits) {
        this.decimalDigits = decimalDigits;
        return this;
    }

    public ExpectedColumn decimalDigits(int decimalDigits) {
        return decimalDigits((Short) (short) decimalDigits);
    }

    public ExpectedColumn numPrecRadix(Short numPrecRadix) {
        this.numPrecRadix = numPrecRadix;
        return this;
    }

    public ExpectedColumn numPrecRadix(int numPrecRadix) {
        return numPrecRadix((Short) (short) numPrecRadix);
    }

    public ExpectedColumn nullable(boolean nullable) {
        this.nullable = nullable;
        return this;
    }

    public ExpectedColumn notNull() {
        return nullable(false);
    }

    public ExpectedColumn remarks(String remarks) {
        this.remarks = remarks;
        return this;
    }

    public void test(TgOdbcStmtHandle stmt, boolean wideChar) {
        String databaseName = stmt.getDataString(1);
        assertEquals("", databaseName);

        String schemaName = stmt.getDataString(2);
        assertEquals("", schemaName);

        String tableName = stmt.getDataString(3);
        assertEquals(this.tableName, tableName);

        String columnName = stmt.getDataString(4);
        assertEquals(this.columnName, columnName);

        var dataType = SqlDataType.fromValue(stmt.getDataShort(5));
        assertEquals(this.dataType, dataType);

        String typeName = stmt.getDataString(6);
        assertEquals(this.typeName, typeName);

        Integer columnSize = stmt.getDataInt(7);
        assertEquals(this.columnSize, columnSize);

        Integer bufferLength = stmt.getDataInt(8);
        assertEquals(this.bufferLength, bufferLength);

        Short decimalDigits = stmt.getDataShort(9);
        assertEquals(this.decimalDigits, decimalDigits);

        Short numPrecRadix = stmt.getDataShort(10);
        assertEquals(this.numPrecRadix, numPrecRadix);

        short nullable = stmt.getDataShort(11);
        if (this.nullable) {
            assertEquals(OdbcConst.SQL_NULLABLE, nullable);
        } else {
            assertEquals(OdbcConst.SQL_NO_NULLS, nullable);
        }

        String remarks = stmt.getDataString(12);
        assertEquals(this.remarks, remarks);

        String columnDef = stmt.getDataString(13);
        assertNull(columnDef);

        var sqlDataType = SqlDataType.fromValue(stmt.getDataShort(14));
        assertEquals(this.dataType, sqlDataType);

        Short sqlDatetimeSub = stmt.getDataShort(15);
        assertEquals(this.datetimeSub, sqlDatetimeSub);

        Integer charOctetLength = stmt.getDataInt(16);
        switch (this.dataType) {
        case SQL_CHAR:
        case SQL_VARCHAR:
        case SQL_BINARY:
        case SQL_VARBINARY:
            assertEquals(this.columnSize, charOctetLength);
            break;
        default:
            assertNull(charOctetLength);
            break;
        }

        int ordinalPosition = stmt.getDataInt(17);
        assertEquals(this.ordinalPosition, ordinalPosition);

        String isNullable = stmt.getDataString(18);
        if (this.nullable) {
            assertEquals("YES", isNullable);
        } else {
            assertEquals("NO", isNullable);
        }
    }

    public String typeBaseName() {
        return this.typeBaseName;
    }

    public long descLength() {
        return this.descLength;
    }
}
