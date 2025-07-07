package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.Arrays;
import java.util.Map;
import java.util.stream.Collectors;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;

public class TgOdbcDriverConnectArgument {
    private final TgOdbcManager manager;
    private final boolean wideChar;

    private MemorySegment windowHandle = MemorySegment.NULL;
    private MemorySegment inConnectionStringPtr;
    private short inConnectionStringLength;
    private short bufferLength;
    private MemorySegment outConnectionStringPtr;
    private MemorySegment outConnectionStringLengthPtr;
    private short driverCompletion = 0;

    public TgOdbcDriverConnectArgument(TgOdbcManager manager, boolean wideChar) {
        this.manager = manager;
        this.wideChar = wideChar;
        this.outConnectionStringLengthPtr = manager.allocateShort();
    }

    public TgOdbcDriverConnectArgument inConnectionString(String inConnectionString) {
        return inConnectionString(inConnectionString, OdbcConst.SQL_NTS);
    }

    public TgOdbcDriverConnectArgument inConnectionString(String inConnectionString, int length) {
        if (wideChar) {
            this.inConnectionStringPtr = manager.allocateUtf16(inConnectionString);
        } else {
            this.inConnectionStringPtr = manager.allocateUtf8(inConnectionString);
        }
        this.inConnectionStringLength = (short) length;
        return this;
    }

    public TgOdbcDriverConnectArgument bufferLength(int bufferLength) {
        this.bufferLength = (short) bufferLength;
        if (wideChar) {
            this.outConnectionStringPtr = manager.allocateBytes(bufferLength * 2);
        } else {
            this.outConnectionStringPtr = manager.allocateBytes(bufferLength);
        }
        return this;
    }

    public boolean wideChar() {
        return this.wideChar;
    }

    public MemorySegment windowHandle() {
        return this.windowHandle;
    }

    public MemorySegment inConnectionStringPtr() {
        return this.inConnectionStringPtr;
    }

    public short inConnectionStringLength() {
        return this.inConnectionStringLength;
    }

    public short bufferLength() {
        return this.bufferLength;
    }

    public MemorySegment outConnectionStringPtr() {
        return this.outConnectionStringPtr;
    }

    public MemorySegment outConnectionStringLengthPtr() {
        return this.outConnectionStringLengthPtr;
    }

    public short driverCompletion() {
        return this.driverCompletion;
    }

    public String outConnectionString() {
        if (wideChar) {
            return TgOdbcManager.stringFromUtf16(outConnectionStringPtr, outConnectionStringLengthPtr);
        } else {
            return TgOdbcManager.stringFromUtf8(outConnectionStringPtr, outConnectionStringLengthPtr);
        }
    }

    public Map<String, String> outConnectionMap() {
        String[] ss = outConnectionString().split(";");
        return Arrays.stream(ss).map(s -> s.split("=", 2)).collect(Collectors.toMap(a -> a[0], a -> a[1]));
    }

    public short outConnectionStringLength() {
        return outConnectionStringLengthPtr.get(ValueLayout.JAVA_SHORT, 0);
    }
}
