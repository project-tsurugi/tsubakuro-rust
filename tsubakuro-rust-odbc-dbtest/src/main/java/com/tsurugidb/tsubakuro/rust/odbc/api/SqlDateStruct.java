package com.tsurugidb.tsubakuro.rust.odbc.api;

import static java.lang.foreign.ValueLayout.JAVA_SHORT;

import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemoryLayout.PathElement;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.StructLayout;
import java.time.LocalDate;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;

public class SqlDateStruct {

    private static final StructLayout LAYOUT = MemoryLayout.structLayout( //
            JAVA_SHORT.withName("year"), // 2 byte
            JAVA_SHORT.withName("month"), // 2 byte
            JAVA_SHORT.withName("day") // 2 byte
    );
    private static final long YEAR_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("year"));
    private static final long MONTH_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("month"));
    private static final long DAY_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("day"));

    public static SqlDateStruct allocate(TgOdbcManager manager) {
        var struct = manager.arena().allocate(LAYOUT);
        return new SqlDateStruct(struct);
    }

    public static SqlDateStruct of(TgOdbcManager manager, LocalDate value) {
        var struct = allocate(manager);

        struct.setYear(value.getYear());
        struct.setMonth(value.getMonthValue());
        struct.setDay(value.getDayOfMonth());

        return struct;
    }

    private final MemorySegment struct;

    public SqlDateStruct(MemorySegment struct) {
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

    public LocalDate toLocalDate() {
        return LocalDate.of(getYear(), getMonth(), getDay());
    }
}
