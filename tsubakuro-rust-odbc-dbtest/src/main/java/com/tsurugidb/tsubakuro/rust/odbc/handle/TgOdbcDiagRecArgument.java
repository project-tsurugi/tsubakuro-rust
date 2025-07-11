package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;

public record TgOdbcDiagRecArgument(MemorySegment sqlStatePtr, MemorySegment nativeErrorPtr, MemorySegment messageTextPtr, short bufferLength, MemorySegment textLengthPtr,boolean wideChar) {

    public static TgOdbcDiagRecArgument of(TgOdbcManager manager, int bufferLength, boolean wideChar) {
        MemorySegment sqlStatePtr = manager.allocateBytes(wideChar ? 6 * 2 : 6);
        MemorySegment nativeErrorPtr = manager.allocateInt();
        MemorySegment messageTextPtr = manager.allocateBytes(wideChar ? bufferLength * 2 : bufferLength);
        MemorySegment textLengthPtr = manager.allocateShort();
        return new TgOdbcDiagRecArgument(sqlStatePtr, nativeErrorPtr, messageTextPtr, (short) bufferLength, textLengthPtr, wideChar);
    }

    public String sqlState() {
        if (wideChar) {
            return TgOdbcManager.stringFromUtf16(sqlStatePtr);
        } else {
            return TgOdbcManager.stringFromUtf8(sqlStatePtr);
        }
    }

    public int nativeError() {
        return nativeErrorPtr.get(ValueLayout.JAVA_INT, 0);
    }

    public String messageText() {
        if (wideChar) {
            return TgOdbcManager.stringFromUtf16(messageTextPtr, textLengthPtr);
        } else {
            return TgOdbcManager.stringFromUtf8(messageTextPtr, textLengthPtr);
        }
    }
}
