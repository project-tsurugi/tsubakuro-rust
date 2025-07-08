package com.tsurugidb.tsubakuro.rust.odbc.stmt;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;
import java.time.OffsetDateTime;
import java.time.OffsetTime;
import java.time.ZoneOffset;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.CDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDateStruct;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlNumericStruct;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlTimeStruct;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlTimestampStruct;

public abstract class TgOdbcGetDataArgument<T> {

    public static TgOdbcGetDataArgument<Boolean> ofBoolean(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_BIT;
        var valuePtr = manager.allocateByte();
        long bufferSize = 1;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public Boolean getData() {
                byte value = valuePtr.get(ValueLayout.JAVA_BYTE, 0);
                return value != 0;
            }
        };
    }

    public static TgOdbcGetDataArgument<Byte> ofByte(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_TINYINT;
        var valuePtr = manager.allocateByte();
        long bufferSize = 1;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public Byte getData() {
                return valuePtr.get(ValueLayout.JAVA_BYTE, 0);
            }
        };
    }

    public static TgOdbcGetDataArgument<Short> ofShort(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_SHORT;
        var valuePtr = manager.allocateShort();
        long bufferSize = 2;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public Short getData() {
                return valuePtr.get(ValueLayout.JAVA_SHORT, 0);
            }
        };
    }

    public static TgOdbcGetDataArgument<Integer> ofInt(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_LONG;
        var valuePtr = manager.allocateInt();
        long bufferSize = 4;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public Integer getData() {
                return valuePtr.get(ValueLayout.JAVA_INT, 0);
            }
        };
    }

    public static TgOdbcGetDataArgument<Long> ofLong(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_SBIGINT;
        var valuePtr = manager.allocateLong();
        long bufferSize = 8;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public Long getData() {
                return valuePtr.get(ValueLayout.JAVA_LONG, 0);
            }
        };
    }

    public static TgOdbcGetDataArgument<Float> ofFloat(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_FLOAT;
        var valuePtr = manager.allocateFloat();
        long bufferSize = 4;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public Float getData() {
                return valuePtr.get(ValueLayout.JAVA_FLOAT, 0);
            }
        };
    }

    public static TgOdbcGetDataArgument<Double> ofDouble(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_DOUBLE;
        var valuePtr = manager.allocateDouble();
        long bufferSize = 8;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public Double getData() {
                return valuePtr.get(ValueLayout.JAVA_DOUBLE, 0);
            }
        };
    }

    public static TgOdbcGetDataArgument<BigDecimal> ofDecimal(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_NUMERIC;
        var numeric = SqlNumericStruct.allocate(manager);
        var valuePtr = numeric.address();
        long bufferSize = numeric.byteSize();
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public BigDecimal getData() {
                return numeric.toDecimal();
            }
        };
    }

    public static TgOdbcGetDataArgument<String> ofString(TgOdbcManager manager, int length, boolean wideChar) {
        var targetType = wideChar ? CDataType.SQL_C_WCHAR : CDataType.SQL_C_CHAR;
        var valuePtr = manager.allocateBytes(length);
        long bufferSize = length;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public String getData() {
                if (wideChar) {
                    return TgOdbcManager.stringFromUtf16Long(valuePtr, lengthOrInd());
                } else {
                    return TgOdbcManager.stringFromUtf8(valuePtr, lengthOrInd());
                }
            }
        };
    }

    public static TgOdbcGetDataArgument<byte[]> ofBinary(TgOdbcManager manager, int length) {
        var targetType = CDataType.SQL_C_BINARY;
        var valuePtr = manager.allocateBytes(length);
        long bufferSize = length;
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public byte[] getData() {
                int length = (int) lengthOrInd();
                int size = Math.min((int) bufferSize, length);
                var buf = new byte[size];
                for (int i = 0; i < size; i++) {
                    buf[i] = valuePtr.get(ValueLayout.JAVA_BYTE, i);
                }
                return buf;
            }
        };
    }

    public static TgOdbcGetDataArgument<LocalDate> ofDate(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_TYPE_DATE;
        var date = SqlDateStruct.allocate(manager);
        var valuePtr = date.address();
        long bufferSize = date.byteSize();
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public LocalDate getData() {
                return date.toLocalDate();
            }
        };
    }

    public static TgOdbcGetDataArgument<LocalTime> ofTime(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_TYPE_TIME;
        var time = SqlTimeStruct.allocate(manager);
        var valuePtr = time.address();
        long bufferSize = time.byteSize();
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public LocalTime getData() {
                return time.toLocalTime();
            }
        };
    }

    public static TgOdbcGetDataArgument<LocalDateTime> ofTimestamp(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_TYPE_TIMESTAMP;
        var timestamp = SqlTimestampStruct.allocate(manager);
        var valuePtr = timestamp.address();
        long bufferSize = timestamp.byteSize();
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public LocalDateTime getData() {
                return timestamp.toLocalDateTime();
            }
        };
    }

    public static TgOdbcGetDataArgument<OffsetTime> ofTimeTz(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_TYPE_TIME;
        var time = SqlTimeStruct.allocate(manager);
        var valuePtr = time.address();
        long bufferSize = time.byteSize();
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public OffsetTime getData() {
                return time.toLocalTime().atOffset(ZoneOffset.UTC);
            }
        };
    }

    public static TgOdbcGetDataArgument<OffsetDateTime> ofTimestampTz(TgOdbcManager manager) {
        var targetType = CDataType.SQL_C_TYPE_TIMESTAMP;
        var timestamp = SqlTimestampStruct.allocate(manager);
        var valuePtr = timestamp.address();
        long bufferSize = timestamp.byteSize();
        return new TgOdbcGetDataArgument<>(manager, targetType, valuePtr, bufferSize) {
            @Override
            public OffsetDateTime getData() {
                return timestamp.toLocalDateTime().atOffset(ZoneOffset.UTC);
            }
        };
    }

    private CDataType targetType;
    protected final MemorySegment valuePtr;
    private final long bufferSize;
    private final MemorySegment lengthOrIndPtr;

    public TgOdbcGetDataArgument(TgOdbcManager manager, CDataType targetType, MemorySegment valuePtr, long bufferSize) {
        this.targetType = targetType;
        this.valuePtr = valuePtr;
        this.bufferSize = bufferSize;
        this.lengthOrIndPtr = manager.allocateLong();
    }

    public TgOdbcGetDataArgument<T> targetType(CDataType targetType) {
        this.targetType = targetType;
        return this;
    }

    public CDataType targetType() {
        return this.targetType;
    }

    public MemorySegment valuePtr() {
        return this.valuePtr;
    }

    public long bufferSize() {
        return this.bufferSize;
    }

    public MemorySegment lengthOrIndPtr() {
        return this.lengthOrIndPtr;
    }

    public long lengthOrInd() {
        return this.lengthOrIndPtr.get(ValueLayout.JAVA_LONG, 0);
    }

    public boolean isDataNull() {
        return lengthOrInd() == OdbcConst.SQL_NULL_DATA;
    }

    public abstract T getData();

    public T getDataOrNull() {
        if (isDataNull()) {
            return null;
        }
        return getData();
    }
}
