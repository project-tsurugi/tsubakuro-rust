package com.tsurugidb.tsubakuro.rust.odbc.api;

import static java.lang.foreign.ValueLayout.JAVA_SHORT;

import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemoryLayout.PathElement;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.StructLayout;
import java.time.LocalTime;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;

public class SqlTimeStruct {

    private static final StructLayout LAYOUT = MemoryLayout.structLayout( //
            JAVA_SHORT.withName("hour"), // 2 byte
            JAVA_SHORT.withName("minute"), // 2 byte
            JAVA_SHORT.withName("second") // 2 byte
    );
    private static final long HOUR_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("hour"));
    private static final long MINUTE_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("minute"));
    private static final long SECOND_OFFSET = LAYOUT.byteOffset(PathElement.groupElement("second"));

    public static SqlTimeStruct allocate(TgOdbcManager manager) {
        var struct = manager.arena().allocate(LAYOUT);
        return new SqlTimeStruct(struct);
    }

    public static SqlTimeStruct of(TgOdbcManager manager, LocalTime value) {
        var struct = allocate(manager);

        struct.setHour(value.getHour());
        struct.setMinute(value.getMinute());
        struct.setSecond(value.getSecond());

        return struct;
    }

    private final MemorySegment struct;

    public SqlTimeStruct(MemorySegment struct) {
        this.struct = struct;
    }

    public MemorySegment address() {
        return struct;
    }

    public long byteSize() {
        return LAYOUT.byteSize();
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

    public LocalTime toLocalTime() {
        return LocalTime.of(getHour(), getMinute(), getSecond());
    }
}
