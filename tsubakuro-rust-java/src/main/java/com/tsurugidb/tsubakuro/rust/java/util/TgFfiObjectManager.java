package com.tsurugidb.tsubakuro.rust.java.util;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

public class TgFfiObjectManager implements AutoCloseable {

    public static TgFfiObjectManager create() {
        return new TgFfiObjectManager();
    }

    private final Arena arena;
    private final Set<TgFfiObject> objectSet = ConcurrentHashMap.newKeySet();

    public TgFfiObjectManager() {
        this.arena = Arena.ofConfined();
    }

    public Arena arena() {
        return this.arena;
    }

    public MemorySegment allocateString(String s) {
        return arena.allocateFrom(s);
    }

    public <T extends TgFfiObject> MemorySegment allocateArray(List<T> list) {
        if (list == null) {
            return MemorySegment.NULL;
        }

        var array = arena.allocate(ValueLayout.ADDRESS, list.size());
        int i = 0;
        for (var object : list) {
            var handle = object.handle();
            array.setAtIndex(ValueLayout.ADDRESS, i++, handle);
        }

        return array;
    }

    public MemorySegment allocateStringArray(List<String> list) {
        if (list == null) {
            return MemorySegment.NULL;
        }

        var array = arena.allocate(ValueLayout.ADDRESS, list.size());
        int i = 0;
        for (String s : list) {
            var handle = allocateString(s);
            array.setAtIndex(ValueLayout.ADDRESS, i++, handle);
        }

        return array;
    }

    public MemorySegment allocateBytes(byte[] value) {
        if (value == null) {
            return MemorySegment.NULL;
        }

        // TODO バイト配列をallocateする効率の良い方法
        var array = arena.allocate(value.length);
        for (int i = 0; i < value.length; i++) {
            array.setAtIndex(ValueLayout.JAVA_BYTE, i, value[i]);
        }

        return array;
    }

    public MemorySegment allocateHandleOut() {
        return arena.allocate(ValueLayout.ADDRESS);
    }

    public MemorySegment allocatePtrOut() {
        return arena.allocate(ValueLayout.ADDRESS);
    }

    public MemorySegment allocateIntOut() {
        return arena.allocate(ValueLayout.JAVA_INT);
    }

    public MemorySegment allocateLongOut() {
        return arena.allocate(ValueLayout.JAVA_LONG);
    }

    public MemorySegment allocateFloatOut() {
        return arena.allocate(ValueLayout.JAVA_FLOAT);
    }

    public MemorySegment allocateDoubleOut() {
        return arena.allocate(ValueLayout.JAVA_DOUBLE);
    }

    public MemorySegment allocateBooleanOut() {
        return arena.allocate(ValueLayout.JAVA_BOOLEAN);
    }

    public void add(TgFfiObject object) {
        objectSet.add(object);
    }

    public void remove(TgFfiObject object) {
        objectSet.remove(object);
    }

    @Override
    public void close() {
        List<RuntimeException> list = null;
        for (var i = objectSet.iterator(); i.hasNext();) {
            var object = i.next();
            i.remove();
            try {
                object.dispose();
            } catch (RuntimeException e) {
                if (list == null) {
                    list = new ArrayList<>();
                }
                list.add(e);
            }
        }

        try {
            arena.close();
        } catch (RuntimeException e) {
            if (list != null) {
                for (var e0 : list) {
                    e.addSuppressed(e0);
                }
            }
            throw e;
        }

        if (list != null) {
            RuntimeException e = null;
            for (var e0 : list) {
                if (e == null) {
                    e = e0;
                } else {
                    e.addSuppressed(e0);
                }
            }
            throw e;
        }
    }
}
