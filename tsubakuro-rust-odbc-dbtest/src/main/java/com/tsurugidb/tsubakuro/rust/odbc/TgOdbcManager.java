package com.tsurugidb.tsubakuro.rust.odbc;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class TgOdbcManager implements AutoCloseable {
    private static final Logger LOG = LoggerFactory.getLogger(TgOdbcManager.class);

    protected final Arena arena = Arena.ofAuto();
    private final List<TgOdbcResource> resourceList = new CopyOnWriteArrayList<>();

    public Arena arena() {
        return this.arena;
    }

    public MemorySegment allocateAddress() {
        return arena.allocate(ValueLayout.ADDRESS);
    }

    public MemorySegment allocateByte() {
        return arena.allocate(ValueLayout.JAVA_BYTE);
    }

    public MemorySegment allocateShort() {
        return arena.allocate(ValueLayout.JAVA_SHORT);
    }

    public MemorySegment allocateInt() {
        return arena.allocate(ValueLayout.JAVA_INT);
    }

    public MemorySegment allocateLong() {
        return arena.allocate(ValueLayout.JAVA_LONG);
    }

    public MemorySegment allocateFloat() {
        return arena.allocate(ValueLayout.JAVA_FLOAT);
    }

    public MemorySegment allocateDouble() {
        return arena.allocate(ValueLayout.JAVA_DOUBLE);
    }

    public MemorySegment allocateUtf8(String value) {
        return arena.allocateFrom(value, StandardCharsets.UTF_8);
    }

    public MemorySegment allocateUtf16(String value) {
        return arena.allocateFrom(value, StandardCharsets.UTF_16LE);
    }

    public MemorySegment allocateByte(byte value) {
        return arena.allocateFrom(ValueLayout.JAVA_BYTE, value);
    }

    public MemorySegment allocateShort(short value) {
        return arena.allocateFrom(ValueLayout.JAVA_SHORT, value);
    }

    public MemorySegment allocateInt(int value) {
        return arena.allocateFrom(ValueLayout.JAVA_INT, value);
    }

    public MemorySegment allocateLong(long value) {
        return arena.allocateFrom(ValueLayout.JAVA_LONG, value);
    }

    public MemorySegment allocateFloat(float value) {
        return arena.allocateFrom(ValueLayout.JAVA_FLOAT, value);
    }

    public MemorySegment allocateDouble(double value) {
        return arena.allocateFrom(ValueLayout.JAVA_DOUBLE, value);
    }

    public MemorySegment allocateBytes(byte[] value) {
        return arena.allocateFrom(ValueLayout.JAVA_BYTE, value);
    }

    public MemorySegment allocateBytes(int size) {
        return arena.allocate(ValueLayout.JAVA_BYTE, size);
    }

    public MemorySegment allocateWords(int size) {
        return arena.allocate(ValueLayout.JAVA_SHORT, size);
    }

    public static String stringFromUtf8(MemorySegment ptr) {
        return ptr.getString(0, StandardCharsets.UTF_8);
    }

    public static String stringFromUtf8(MemorySegment ptr, MemorySegment lengthPtr) {
        short length = lengthPtr.get(ValueLayout.JAVA_SHORT, 0);
        return stringFromUtf8(ptr, length);
    }

    public static String stringFromUtf8Long(MemorySegment ptr, MemorySegment longLengthPtr) {
        long length = longLengthPtr.get(ValueLayout.JAVA_LONG, 0);
        return stringFromUtf8(ptr, length);
    }

    public static String stringFromUtf8(MemorySegment ptr, long length) {
        if (length == 0) {
            return "";
        }
        if (length < 0) {
            return null;
        }
        return ptr.asSlice(0, length + 1).getString(0, StandardCharsets.UTF_8);
    }

    public static String stringFromUtf16(MemorySegment ptr) {
        return ptr.getString(0, StandardCharsets.UTF_16LE);
    }

    public static String stringFromUtf16(MemorySegment ptr, MemorySegment lengthPtr) {
        short length = lengthPtr.get(ValueLayout.JAVA_SHORT, 0);
        if (length == 0) {
            return "";
        }
        if (length < 0) {
            return null;
        }
        return ptr.asSlice(0, (length + 1) * 2).getString(0, StandardCharsets.UTF_16LE);
    }

    public static String stringFromUtf16Long(MemorySegment ptr, MemorySegment longLengthPtr) {
        long length = longLengthPtr.get(ValueLayout.JAVA_LONG, 0);
        return stringFromUtf16Long(ptr, length);
    }

    public static String stringFromUtf16Long(MemorySegment ptr, long length) {
        if (length == 0) {
            return "";
        }
        if (length < 0) {
            return null;
        }
        return ptr.asSlice(0, length + 2).getString(0, StandardCharsets.UTF_16LE);
    }

    public void add(TgOdbcResource object) {
        resourceList.add(object);
    }

    @Override
    public void close() {
        for (var object : resourceList.reversed()) {
            try {
                object.close();
            } catch (Exception e) {
                LOG.error("resource close error", e);
            }
        }
    }
}
