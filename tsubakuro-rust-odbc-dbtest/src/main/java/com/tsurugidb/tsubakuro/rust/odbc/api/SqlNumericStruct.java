package com.tsurugidb.tsubakuro.rust.odbc.api;

import static java.lang.foreign.ValueLayout.JAVA_BYTE;

import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemoryLayout.PathElement;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.StructLayout;
import java.math.BigDecimal;
import java.math.BigInteger;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;

public class SqlNumericStruct {
    private static final int SQL_MAX_NUMERIC_LEN = 16;

    private static final StructLayout LAYOUT = MemoryLayout.structLayout( //
            JAVA_BYTE.withName("precision"), // 1 byte
            JAVA_BYTE.withName("scale"), // 1 byte (signed)
            JAVA_BYTE.withName("sign"), // 1 byte (1=positive, 0=negative)
            MemoryLayout.sequenceLayout(SQL_MAX_NUMERIC_LEN, JAVA_BYTE).withName("val") // 16 bytes
    );
    private static final long PRECISION_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("precision"));
    private static final long SCALE_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("scale"));
    private static final long SIGN_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("sign"));
    private static final long VAL_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("val"));

    public static SqlNumericStruct allocate(TgOdbcManager manager) {
        var struct = manager.arena().allocate(LAYOUT);
        return new SqlNumericStruct(struct);
    }

    public static SqlNumericStruct of(TgOdbcManager manager, BigDecimal value) {
        var struct = allocate(manager);

        byte sign;
        if (value.signum() >= 0) {
            sign = 1;
        } else {
            sign = 0;
            value = value.negate();
        }

        byte presition = (byte) value.precision();
        byte scale = (byte) value.scale();
        byte[] be = value.unscaledValue().toByteArray();

        struct.setPrecision(presition);
        struct.setScale(scale);
        struct.setSign(sign);
        struct.setValue(be);

        return struct;
    }

    private final MemorySegment struct;

    public SqlNumericStruct(MemorySegment struct) {
        this.struct = struct;
    }

    public MemorySegment address() {
        return struct;
    }

    public long byteSize() {
        return LAYOUT.byteSize();
    }

    public byte getPrecision() {
        return struct.get(JAVA_BYTE, PRECISION_OFFSET);
    }

    private void setPrecision(byte precision) {
        struct.set(JAVA_BYTE, PRECISION_OFFSET, precision);
    }

    public byte getScale() {
        return struct.get(JAVA_BYTE, SCALE_OFFSET);
    }

    private void setScale(byte scale) {
        struct.set(JAVA_BYTE, SCALE_OFFSET, scale);
    }

    public byte getSign() {
        return struct.get(JAVA_BYTE, SIGN_OFFSET);
    }

    private void setSign(byte sign) {
        struct.set(JAVA_BYTE, SIGN_OFFSET, sign);
    }

    public byte[] getValue() {
        MemorySegment valSegment = struct.asSlice(VAL_OFFSET, SQL_MAX_NUMERIC_LEN);
        return valSegment.toArray(JAVA_BYTE);
    }

    private void setValue(byte[] beValue) {
        assert beValue.length <= SQL_MAX_NUMERIC_LEN;

        long offset = VAL_OFFSET;
        int n = 0;
        for (int i = beValue.length - 1; i >= 0; i--, n++) {
            struct.set(JAVA_BYTE, offset++, beValue[i]);
        }
        while (n++ < SQL_MAX_NUMERIC_LEN) {
            struct.set(JAVA_BYTE, offset++, (byte) 0);
        }
    }

    public BigDecimal toDecimal() {
        byte[] le = getValue();

        // little-endian to big-endian
        var be = new byte[le.length];
        for (int i = 0; i < le.length; i++) {
            be[be.length - 1 - i] = le[i];
        }

        boolean positive = (getSign() != 0);
        var unscaledValue = positive ? new BigInteger(1, be) : new BigInteger(-1, be);
        int scale = getScale();
        return new BigDecimal(unscaledValue, scale);
    }
}
