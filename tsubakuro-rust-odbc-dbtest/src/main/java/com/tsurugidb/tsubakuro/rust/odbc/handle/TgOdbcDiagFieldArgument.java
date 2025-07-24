package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;

public record TgOdbcDiagFieldArgument(DiagIdentifier diagIdentifier, MemorySegment diagInfoPtr, short bufferLength, MemorySegment stringLengthPtr) {

    public static TgOdbcDiagFieldArgument ofString(TgOdbcManager manager, DiagIdentifier fieldIdentifier, int bufferLength) {
        return new TgOdbcDiagFieldArgument(fieldIdentifier, manager.allocateBytes(bufferLength), (short) bufferLength, manager.allocateShort());
    }

    // SQLINTEGER
    public static TgOdbcDiagFieldArgument ofInteger(TgOdbcManager manager, DiagIdentifier fieldIdentifier) {
        return new TgOdbcDiagFieldArgument(fieldIdentifier, manager.allocateInt(), (short) 0, MemorySegment.NULL);
    }

    // SQLLEN
    public static TgOdbcDiagFieldArgument ofLong(TgOdbcManager manager, DiagIdentifier fieldIdentifier) {
        return new TgOdbcDiagFieldArgument(fieldIdentifier, manager.allocateLong(), (short) 0, MemorySegment.NULL);
    }

    public enum DiagIdentifier {
        SQL_DIAG_NUMBER(2), //
        SQL_DIAG_CONNECTION_NAME(10), //
        SQL_DIAG_SERVER_NAME(11), //
        ;

        private final short value;

        private DiagIdentifier(int value) {
            this.value = (short) value;
        }

        public short value() {
            return value;
        }
    }

    public String diagInfoString(boolean wideChar) {
        int length = stringLength();
        if (wideChar) {
            return TgOdbcManager.stringFromUtf16Bytes(diagInfoPtr, length);
        } else {
            return TgOdbcManager.stringFromUtf8(diagInfoPtr, length);
        }
    }

    public int diagInfoInteger() {
        return diagInfoPtr.get(ValueLayout.JAVA_INT, 0);
    }

    public long diagInfoLong() {
        return diagInfoPtr.get(ValueLayout.JAVA_LONG, 0);
    }

    public short stringLength() {
        return stringLengthPtr.get(ValueLayout.JAVA_SHORT, 0);
    }
}
