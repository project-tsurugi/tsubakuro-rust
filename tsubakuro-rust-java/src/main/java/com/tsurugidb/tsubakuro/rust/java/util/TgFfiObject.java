package com.tsurugidb.tsubakuro.rust.java.util;

import java.io.Closeable;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.ref.Cleaner;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public abstract class TgFfiObject implements Closeable {
    private static final Cleaner cleaner = Cleaner.create();

    private final TgFfiObjectManager manager;
    private MemorySegment handle;

    @SuppressWarnings("unused")
    private final Cleaner.Cleanable cleanable;

    public TgFfiObject(TgFfiObjectManager manager, MemorySegment handle) {
        this.manager = Objects.requireNonNull(manager, "manager must not be null");
        this.handle = Objects.requireNonNull(handle, "handle must not be null");
        this.cleanable = cleaner.register(this, this::close);

        manager.add(this);
    }

    public final TgFfiObjectManager manager() {
        return this.manager;
    }

    protected final static long allocateDuration(Duration duration) {
        return duration.toNanos();
    }

    protected final MemorySegment allocateString(String s) {
        return manager.allocateString(s);
    }

    protected final <T extends TgFfiObject> MemorySegment allocateArray(List<T> list) {
        return manager.allocateArray(list);
    }

    protected final MemorySegment allocateStringArray(List<String> list) {
        return manager.allocateStringArray(list);
    }

    // use in synchronized(this)
    public final MemorySegment handle() {
        var handle = this.handle;
        if (handle == null) {
            throw new IllegalStateException("handle already closed");
        }

        return handle;
    }

    protected final MemorySegment allocateHandleOut() {
        return manager.allocateHandleOut();
    }

    protected final MemorySegment allocateBooleanOut() {
        return manager.allocateBooleanOut();
    }

    protected final MemorySegment allocateIntOut() {
        return manager.allocateIntOut();
    }

    protected final MemorySegment allocateLongOut() {
        return manager.allocateLongOut();
    }

    protected final MemorySegment allocatePtrOut() {
        return manager.allocatePtrOut();
    }

    protected static MemorySegment outToHandle(MemorySegment out) {
        return out.get(ValueLayout.ADDRESS, 0);
    }

    protected static boolean outToBoolean(MemorySegment out) {
        return out.get(ValueLayout.JAVA_BOOLEAN, 0);
    }

    protected static int outToInt(MemorySegment out) {
        return out.get(ValueLayout.JAVA_INT, 0);
    }

    protected static long outToLong(MemorySegment out) {
        return out.get(ValueLayout.JAVA_LONG, 0);
    }

    protected static Duration outToDuration(MemorySegment out) {
        long value = outToLong(out);
        return Duration.ofNanos(value);
    }

    protected static String outToString(MemorySegment out) {
        var stringPtr = out.get(ValueLayout.ADDRESS, 0);
        if (stringPtr.address() == 0) {
            return null;
        }

        // stringPtr.byteSize() == 0 なので、
        // stringPtr.getString(0)だと、1バイトも取得できずにIndexOutOfBoundsExceptionが発生する。
        // そこで、バイト数を適当に増やしてからgetString()を呼ぶ。
        var ptr = stringPtr.reinterpret(Short.MAX_VALUE);
        return ptr.getString(0);
    }

    protected static List<String> outToStringList(MemorySegment out, MemorySegment sizeOut) {
        int size = outToInt(sizeOut);
        if (size == 0) {
            return List.of();
        }

        var array = outToHandle(out).reinterpret(ValueLayout.ADDRESS.byteSize() * size);

        var list = new ArrayList<String>();
        for (int i = 0; i < size; i++) {
            var ptr = array.getAtIndex(ValueLayout.ADDRESS, i).reinterpret(Short.MAX_VALUE);
            String s = ptr.getString(0);

            list.add(s);
        }
        return list;
    }

    protected static byte[] outToBytesLong(MemorySegment out, MemorySegment sizeOut) {
        long size = outToLong(sizeOut);
        if (size == 0) {
            return new byte[0];
        }

        var array = outToHandle(out).reinterpret(size);
        return array.toArray(ValueLayout.JAVA_BYTE);
    }

    protected static byte[] outToBytesInt(MemorySegment out, MemorySegment sizeOut) {
        long size = outToInt(sizeOut);
        if (size == 0) {
            return new byte[0];
        }

        var array = outToHandle(out).reinterpret(size);
        return array.toArray(ValueLayout.JAVA_BYTE);
    }

    @Override
    public final int hashCode() {
        return super.hashCode(); // System.identityHashCode(this)
    }

    @Override
    public final boolean equals(Object obj) {
        return super.equals(obj); // this == obj
    }

    @Override
    public void close() {
        try {
            dispose();
        } finally {
            manager.remove(this);
        }
    }

    void dispose() {
        MemorySegment handle;
        synchronized (this) {
            handle = this.handle;
            if (handle == null) {
                return;
            }
            this.handle = null;
        }

        dispose(handle);
    }

    protected abstract void dispose(MemorySegment handle);
}
