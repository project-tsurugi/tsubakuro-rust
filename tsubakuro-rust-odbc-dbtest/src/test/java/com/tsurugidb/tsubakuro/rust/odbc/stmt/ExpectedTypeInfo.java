package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import static com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst.SQL_FALSE;
import static com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst.SQL_NULLABLE;
import static com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst.SQL_PRED_BASIC;
import static com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst.SQL_TRUE;
import static org.junit.jupiter.api.Assertions.assertEquals;

import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataTypeSubCode;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;

public class ExpectedTypeInfo {
    private final SqlDataType dataType;
    private final String typeName;

    private Integer columnSize;
    private String literalPrefix;
    private String literalSuffix;
    private String createParams;
    private short nullable = SQL_NULLABLE;
    private short caseSensitive = SQL_FALSE;
    private short searchable = SQL_PRED_BASIC;
    private Short unsignedAttribute;
    private short fixedPrecScale = SQL_FALSE;
    private Short autoUniqueValue;
    private String localTypeName;
    private Short minimumScale;
    private Short maximumScale;
    private SqlDataType sqlDataType;
    private Short sqlDatetimeSub;
    private Integer numPrecRadix;
    private Short intervalPrecision;

    public ExpectedTypeInfo(SqlDataType dataType, String typeName) {
        this.dataType = dataType;
        this.typeName = typeName;

        this.sqlDataType = dataType;
    }

    public ExpectedTypeInfo columnSize(Integer columnSize) {
        this.columnSize = columnSize;
        return this;
    }

    public ExpectedTypeInfo literalPrefix(String literalPrefix) {
        this.literalPrefix = literalPrefix;
        return this;
    }

    public ExpectedTypeInfo literalSuffix(String literalSuffix) {
        this.literalSuffix = literalSuffix;
        return this;
    }

    public ExpectedTypeInfo createParams(String createParams) {
        this.createParams = createParams;
        return this;
    }

    public ExpectedTypeInfo nullable(short nullable) {
        this.nullable = nullable;
        return this;
    }

    public ExpectedTypeInfo caseSensitive(short caseSensitive) {
        this.caseSensitive = caseSensitive;
        return this;
    }

    public ExpectedTypeInfo caseSensitive(boolean caseSensitive) {
        return caseSensitive(caseSensitive ? SQL_TRUE : SQL_FALSE);
    }

    public ExpectedTypeInfo searchable(short searchable) {
        this.searchable = searchable;
        return this;
    }

    public ExpectedTypeInfo unsignedAttribute(Short unsignedAttribute) {
        this.unsignedAttribute = unsignedAttribute;
        return this;
    }

    public ExpectedTypeInfo unsignedAttribute(boolean unsignedAttribute) {
        return unsignedAttribute(unsignedAttribute ? SQL_TRUE : SQL_FALSE);
    }

    public ExpectedTypeInfo fixedPrecScale(short fixedPrecScale) {
        this.fixedPrecScale = fixedPrecScale;
        return this;
    }

    public ExpectedTypeInfo fixedPrecScale(boolean fixedPrecScale) {
        return fixedPrecScale(fixedPrecScale ? SQL_TRUE : SQL_FALSE);
    }

    public ExpectedTypeInfo autoUniqueValue(Short autoUniqueValue) {
        this.autoUniqueValue = autoUniqueValue;
        return this;
    }

    public ExpectedTypeInfo autoUniqueValue(boolean autoUniqueValue) {
        return autoUniqueValue(autoUniqueValue ? SQL_TRUE : SQL_FALSE);
    }

    public ExpectedTypeInfo localTypeName(String localTypeName) {
        this.localTypeName = localTypeName;
        return this;
    }

    public ExpectedTypeInfo minimumScale(Short minimumScale) {
        this.minimumScale = minimumScale;
        return this;
    }

    public ExpectedTypeInfo minimumScale(int minimumScale) {
        return minimumScale((Short) (short) minimumScale);
    }

    public ExpectedTypeInfo maximumScale(Short maximumScale) {
        this.maximumScale = maximumScale;
        return this;
    }

    public ExpectedTypeInfo maximumScale(int maximumScale) {
        return maximumScale((Short) (short) maximumScale);
    }

    public ExpectedTypeInfo sqlDataType(SqlDataType sqlDataType) {
        this.sqlDataType = sqlDataType;
        return this;
    }

    public ExpectedTypeInfo sqlDatetimeSub(Short sqlDatetimeSub) {
        this.sqlDatetimeSub = sqlDatetimeSub;
        return this;
    }

    public ExpectedTypeInfo sqlDatetimeSub(int sqlDatetimeSub) {
        return sqlDatetimeSub((Short) (short) sqlDatetimeSub);
    }

    public ExpectedTypeInfo sqlDatetimeSub(SqlDataTypeSubCode sqlDatetimeSub) {
        return sqlDatetimeSub(sqlDatetimeSub.value());
    }

    public ExpectedTypeInfo numPrecRadix(Integer numPrecRadix) {
        this.numPrecRadix = numPrecRadix;
        return this;
    }

    public ExpectedTypeInfo intervalPrecision(Short intervalPrecision) {
        this.intervalPrecision = intervalPrecision;
        return this;
    }

    public ExpectedTypeInfo intervalPrecision(int intervalPrecision) {
        return intervalPrecision((Short) (short) intervalPrecision);
    }

    public void test(TgOdbcStmtHandle stmt) {
        String typeName = stmt.getDataString(1);
        assertEquals(this.typeName, typeName);

        var dataType = SqlDataType.fromValue(stmt.getDataShort(2));
        assertEquals(this.dataType, dataType);

        Integer columnSize = stmt.getDataInt(3);
        assertEquals(this.columnSize, columnSize);

        String literalPrefix = stmt.getDataString(4);
        assertEquals(this.literalPrefix, literalPrefix);

        String literalSuffix = stmt.getDataString(5);
        assertEquals(this.literalSuffix, literalSuffix);

        String createParams = stmt.getDataString(6);
        assertEquals(this.createParams, createParams);

        short nullable = stmt.getDataShort(7);
        assertEquals(this.nullable, nullable);

        short caseSensitive = stmt.getDataShort(8);
        assertEquals(this.caseSensitive, caseSensitive);

        short seachable = stmt.getDataShort(9);
        assertEquals(this.searchable, seachable);

        Short unsignedAttribute = stmt.getDataShort(10);
        assertEquals(this.unsignedAttribute, unsignedAttribute);

        short fixedPrecScale = stmt.getDataShort(11);
        assertEquals(this.fixedPrecScale, fixedPrecScale);

        Short autoUniqueValue = stmt.getDataShort(12);
        assertEquals(this.autoUniqueValue, autoUniqueValue);

        String localTypeName = stmt.getDataString(13);
        assertEquals(this.localTypeName, localTypeName);

        Short minimumScale = stmt.getDataShort(14);
        assertEquals(this.minimumScale, minimumScale);

        Short maximumScale = stmt.getDataShort(15);
        assertEquals(this.maximumScale, maximumScale);

        var sqlDataType = SqlDataType.fromValue(stmt.getDataShort(16));
        assertEquals(this.sqlDataType, sqlDataType);

        Short sqlDatetimeSub = stmt.getDataShort(17);
        assertEquals(this.sqlDatetimeSub, sqlDatetimeSub);

        Integer numPrecRadix = stmt.getDataInt(18);
        assertEquals(this.numPrecRadix, numPrecRadix);

        Short intervalPrecision = stmt.getDataShort(19);
        assertEquals(this.intervalPrecision, intervalPrecision);
    }
}
