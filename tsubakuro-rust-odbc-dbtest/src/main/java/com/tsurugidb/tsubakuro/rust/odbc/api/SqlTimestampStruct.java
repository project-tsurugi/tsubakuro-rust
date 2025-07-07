package com.tsurugidb.tsubakuro.rust.odbc.api;

import static java.lang.foreign.ValueLayout.JAVA_INT;
import static java.lang.foreign.ValueLayout.JAVA_SHORT;

import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemoryLayout.PathElement;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.StructLayout;
import java.time.LocalDateTime;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;

public class SqlTimestampStruct {

    private static final StructLayout LAYOUT = MemoryLayout.structLayout( //
            JAVA_SHORT.withName("year"), // 2 byte
            JAVA_SHORT.withName("month"), // 2 byte
            JAVA_SHORT.withName("day"), // 2 byte
            JAVA_SHORT.withName("hour"), // 2 byte
            JAVA_SHORT.withName("minute"), // 2 byte
            JAVA_SHORT.withName("second"), // 2 byte
            JAVA_INT.withName("fraction") // 4 byte
    );
    private static final long YEAR_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("year"));
    private static final long MONTH_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("month"));
    private static final long DAY_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("day"));
    private static final long HOUR_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("hour"));
    private static final long MINUTE_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("minute"));
    private static final long SECOND_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("second"));
    private static final long FRACTION_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("fraction"));

    public static SqlTimestampStruct allocate(TgOdbcManager manager) {
        var struct = manager.arena().allocate(LAYOUT);
        return new SqlTimestampStruct(struct);
    }

    public static SqlTimestampStruct of(TgOdbcManager manager, LocalDateTime value) {
        var struct = allocate(manager);

        struct.setYear(value.getYear());
        struct.setMonth(value.getMonthValue());
        struct.setDay(value.getDayOfMonth());
        struct.setHour(value.getHour());
        struct.setMinute(value.getMinute());
        struct.setSecond(value.getSecond());
        struct.setFraction(value.getNano());

        return struct;
    }

    private final MemorySegment struct;

    public SqlTimestampStruct(MemorySegment struct) {
        this.struct = struct;
    }

    public MemorySegment address() {
        return struct;
    }

    public long byteSize() {
        return LAYOUT.byteSize();
    }

    public short getYear() {
        return struct.get(JAVA_SHORT, YEAR_OFFSET);
    }

    private void setYear(int year) {
        struct.set(JAVA_SHORT, YEAR_OFFSET, (short) year);
    }

    public short getMonth() {
        return struct.get(JAVA_SHORT, MONTH_OFFSET);
    }

    private void setMonth(int month) {
        struct.set(JAVA_SHORT, MONTH_OFFSET, (short) month);
    }

    public short getDay() {
        return struct.get(JAVA_SHORT, DAY_OFFSET);
    }

    private void setDay(int day) {
        struct.set(JAVA_SHORT, DAY_OFFSET, (short) day);
    }

    public short getHour() {
        return struct.get(JAVA_SHORT, HOUR_OFFSET);
    }

    private void setHour(int hour) {
        struct.set(JAVA_SHORT, HOUR_OFFSET, (short) hour);
    }

    public short getMinute() {
        return struct.get(JAVA_SHORT, MINUTE_OFFSET);
    }

    private void setMinute(int minute) {
        struct.set(JAVA_SHORT, MINUTE_OFFSET, (short) minute);
    }

    public short getSecond() {
        return struct.get(JAVA_SHORT, SECOND_OFFSET);
    }

    private void setSecond(int second) {
        struct.set(JAVA_SHORT, SECOND_OFFSET, (short) second);
    }

    public int getFraction() {
        return struct.get(JAVA_INT, FRACTION_OFFSET);
    }

    private void setFraction(int fraction) {
        struct.set(JAVA_INT, FRACTION_OFFSET, fraction);
    }

    public LocalDateTime toLocalDateTime() {
        return LocalDateTime.of(getYear(), getMonth(), getDay(), getHour(), getMinute(), getSecond(), getFraction());
    }
}
