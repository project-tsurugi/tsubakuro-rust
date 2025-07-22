package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import java.lang.foreign.MemorySegment;
import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
import java.time.OffsetDateTime;
import java.time.OffsetTime;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.CDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDateStruct;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlNumericStruct;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlTimeStruct;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlTimestampStruct;

public class TgOdbcBindParameter {

    public static TgOdbcBindParameter ofByte() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_TINYINT) //
                .parameterType(SqlDataType.SQL_TINYINT);
    }

    public static TgOdbcBindParameter ofByte(TgOdbcManager manager, byte value) {
        return ofByte().parameterValuePtr(manager.allocateByte(value), 1);
    }

    public static TgOdbcBindParameter ofByte(TgOdbcManager manager, Byte value) {
        if (value != null) {
            return ofByte(manager, value.byteValue());
        } else {
            return ofByte().nullValue();
        }
    }

    public static TgOdbcBindParameter ofShort() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_SHORT) //
                .parameterType(SqlDataType.SQL_SMALLINT);
    }

    public static TgOdbcBindParameter ofShort(TgOdbcManager manager, short value) {
        return ofShort().parameterValuePtr(manager.allocateShort(value), 2);
    }

    public static TgOdbcBindParameter ofShort(TgOdbcManager manager, Short value) {
        if (value != null) {
            return ofShort(manager, value.shortValue());
        } else {
            return ofShort().nullValue();
        }
    }

    public static TgOdbcBindParameter ofInt() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_LONG) //
                .parameterType(SqlDataType.SQL_INTEGER);
    }

    public static TgOdbcBindParameter ofInt(TgOdbcManager manager, int value) {
        return ofInt().parameterValuePtr(manager.allocateInt(value), 4);
    }

    public static TgOdbcBindParameter ofInt(TgOdbcManager manager, Integer value) {
        if (value != null) {
            return ofInt(manager, value.intValue());
        } else {
            return ofInt().nullValue();
        }
    }

    public static TgOdbcBindParameter ofLong() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_SBIGINT) //
                .parameterType(SqlDataType.SQL_BIGINT);
    }

    public static TgOdbcBindParameter ofLong(TgOdbcManager manager, long value) {
        return ofLong().parameterValuePtr(manager.allocateLong(value), 8);
    }

    public static TgOdbcBindParameter ofLong(TgOdbcManager manager, Long value) {
        if (value != null) {
            return ofLong(manager, value.longValue());
        } else {
            return ofLong().nullValue();
        }
    }

    public static TgOdbcBindParameter ofFloat() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_FLOAT) //
                .parameterType(SqlDataType.SQL_FLOAT);
    }

    public static TgOdbcBindParameter ofFloat(TgOdbcManager manager, float value) {
        return ofFloat().parameterValuePtr(manager.allocateFloat(value), 4);
    }

    public static TgOdbcBindParameter ofFloat(TgOdbcManager manager, Float value) {
        if (value != null) {
            return ofFloat(manager, value.floatValue());
        } else {
            return ofFloat().nullValue();
        }
    }

    public static TgOdbcBindParameter ofDouble() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_DOUBLE) //
                .parameterType(SqlDataType.SQL_DOUBLE);
    }

    public static TgOdbcBindParameter ofDouble(TgOdbcManager manager, double value) {
        return ofDouble().parameterValuePtr(manager.allocateDouble(value), 8);
    }

    public static TgOdbcBindParameter ofDouble(TgOdbcManager manager, Double value) {
        if (value != null) {
            return ofDouble(manager, value.doubleValue());
        } else {
            return ofDouble().nullValue();
        }
    }

    public static TgOdbcBindParameter ofDecimal() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_NUMERIC) //
                .parameterType(SqlDataType.SQL_DECIMAL);
    }

    public static TgOdbcBindParameter ofDecimal(TgOdbcManager manager, BigDecimal value) {
        if (value != null) {
            var numeric = SqlNumericStruct.of(manager, value);
            return ofDecimal() //
                    .parameterValuePtr(numeric.address(), numeric.byteSize()) //
                    .columnSize(value.precision()) //
                    .decimalDigits(value.scale());
        } else {
            return ofDecimal().nullValue();
        }
    }

    public static TgOdbcBindParameter ofStringUtf8() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_CHAR) //
                .parameterType(SqlDataType.SQL_CHAR);
    }

    public static TgOdbcBindParameter ofStringUtf8(TgOdbcManager manager, String value) {
        if (value != null) {
            return ofStringUtf8().parameterValuePtr(manager.allocateUtf8(value), OdbcConst.SQL_NTS);
        } else {
            return ofStringUtf8().nullValue();
        }
    }

    public static TgOdbcBindParameter ofStringUtf16() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_WCHAR) //
                .parameterType(SqlDataType.SQL_CHAR);
    }

    public static TgOdbcBindParameter ofStringUtf16(TgOdbcManager manager, String value) {
        if (value != null) {
            return ofStringUtf16().parameterValuePtr(manager.allocateUtf16(value), OdbcConst.SQL_NTS);
        } else {
            return ofStringUtf16().nullValue();
        }
    }

    public static TgOdbcBindParameter ofBinary() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_BINARY) //
                .parameterType(SqlDataType.SQL_BINARY);
    }

    public static TgOdbcBindParameter ofBinary(TgOdbcManager manager, byte[] value) {
        if (value != null) {
            return ofBinary().parameterValuePtr(manager.allocateBytes(value), value.length);
        } else {
            return ofBinary().nullValue();
        }
    }

    public static TgOdbcBindParameter ofDate() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_TYPE_DATE) //
                .parameterType(SqlDataType.SQL_TYPE_DATE);
    }

    public static TgOdbcBindParameter ofDate(TgOdbcManager manager, LocalDate value) {
        if (value != null) {
            var date = SqlDateStruct.of(manager, value);
            return ofDate().parameterValuePtr(date.address(), date.byteSize());
        } else {
            return ofDate().nullValue();
        }
    }

    public static TgOdbcBindParameter ofTime() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_TYPE_TIME) //
                .parameterType(SqlDataType.SQL_TYPE_TIME);
    }

    public static TgOdbcBindParameter ofTime(TgOdbcManager manager, LocalTime value) {
        if (value != null) {
            var date = SqlTimeStruct.of(manager, value);
            return ofTime().parameterValuePtr(date.address(), date.byteSize());
        } else {
            return ofTime().nullValue();
        }
    }

    public static TgOdbcBindParameter ofTimestamp() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_TYPE_TIMESTAMP) //
                .parameterType(SqlDataType.SQL_TYPE_TIMESTAMP);
    }

    public static TgOdbcBindParameter ofTimestamp(TgOdbcManager manager, LocalDateTime value) {
        if (value != null) {
            var date = SqlTimestampStruct.of(manager, value);
            return ofTimestamp().parameterValuePtr(date.address(), date.byteSize());
        } else {
            return ofTimestamp().nullValue();
        }
    }

    public static TgOdbcBindParameter ofTimeTz(TgOdbcManager manager, OffsetTime value) {
        if (value != null) {
            var date = SqlTimeStruct.of(manager, value.toLocalTime());
            return ofTime().parameterValuePtr(date.address(), date.byteSize());
        } else {
            return ofTime().nullValue();
        }
    }

    public static TgOdbcBindParameter ofTimestampTz() {
        return new TgOdbcBindParameter() //
                .valueType(CDataType.SQL_C_TYPE_TIMESTAMP) //
                .parameterType(SqlDataType.SQL_TYPE_TIMESTAMP_WITH_TIMEZONE);
    }

    public static TgOdbcBindParameter ofTimestampTz(TgOdbcManager manager, OffsetDateTime value) {
        if (value != null) {
            var date = SqlTimestampStruct.of(manager, value.toLocalDateTime());
            return ofTimestampTz().parameterValuePtr(date.address(), date.byteSize());
        } else {
            return ofTimestampTz().nullValue();
        }
    }

    private CDataType valueType;
    private SqlDataType parameterType;
    private long columnSize;
    private short decimalDigits;
    private MemorySegment parameterValuePtr;
    private long lengthOrInd;

    public TgOdbcBindParameter valueType(CDataType valueType) {
        this.valueType = valueType;
        return this;
    }

    public TgOdbcBindParameter parameterType(SqlDataType parameterType) {
        this.parameterType = parameterType;
        return this;
    }

    public TgOdbcBindParameter columnSize(long columnSize) {
        this.columnSize = columnSize;
        return this;
    }

    public TgOdbcBindParameter decimalDigits(int decimalDigits) {
        this.decimalDigits = (short) decimalDigits;
        return this;
    }

    public TgOdbcBindParameter parameterValuePtr(MemorySegment parameterValuePtr, long lengthOrInd) {
        this.parameterValuePtr = parameterValuePtr;
        this.lengthOrInd = lengthOrInd;
        return this;
    }

    public TgOdbcBindParameter nullValue() {
        this.parameterValuePtr = MemorySegment.NULL;
        this.lengthOrInd = OdbcConst.SQL_NULL_DATA;
        return this;
    }

    public TgOdbcBindParameter lengthOrInd(long lengthOrInd) {
        this.lengthOrInd = lengthOrInd;
        return this;
    }

    public CDataType valueType() {
        return valueType;
    }

    public SqlDataType parameterType() {
        return parameterType;
    }

    public long columnSize() {
        return columnSize;
    }

    public short decimalDigits() {
        return decimalDigits;
    }

    public MemorySegment parameterValuePtr() {
        return parameterValuePtr;
    }

    public long lengthOrInd() {
        return lengthOrInd;
    }

    @Override
    public String toString() {
        return "TgOdbcBindParameter [valueType=" + valueType + ", parameterType=" + parameterType + ", columnSize=" + columnSize + ", decimalDigits=" + decimalDigits + ", parameterValuePtr="
                + parameterValuePtr + ", lengthOrInd=" + lengthOrInd + "]";
    }
}
