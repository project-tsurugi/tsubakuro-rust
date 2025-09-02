package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;

public abstract class InfoType<T> {

    public static final InfoTypeString SQL_DATA_SOURCE_NAME = new InfoTypeString(2);
    public static final InfoTypeString SQL_DRIVER_NAME = new InfoTypeString(6);
    public static final InfoTypeString SQL_DRIVER_VER = new InfoTypeString(7);
    public static final InfoTypeString SQL_SERVER_NAME = new InfoTypeString(13);
    public static final InfoTypeString SQL_DBMS_NAME = new InfoTypeString(17);
    public static final InfoTypeString SQL_DBMS_VER = new InfoTypeString(18);
    public static final InfoTypeInteger SQL_CURSOR_COMMIT_BEHAVIOR = new InfoTypeInteger(23, 2);
    public static final InfoTypeInteger SQL_CURSOR_ROLLBACK_BEHAVIOR = new InfoTypeInteger(24, 2);
    public static final InfoTypeString SQL_USER_NAME = new InfoTypeString(47);
    public static final InfoTypeString SQL_DRIVER_ODBC_VER = new InfoTypeString(77);
    public static final InfoTypeInteger SQL_GETDATA_EXTENSIONS = new InfoTypeInteger(81, 4);

    private final short infoType;
    protected final short bufferLength;

    public InfoType(int infoType, int bufferLength) {
        this.infoType = (short) infoType;
        this.bufferLength = (short) bufferLength;
    }

    public short infoType() {
        return this.infoType;
    }

    public short bufferLength() {
        return this.bufferLength;
    }

    public abstract T infoValue(MemorySegment infoValuePtr, short stringLength, boolean wideChar);

    public static class InfoTypeInteger extends InfoType<Integer> {

        InfoTypeInteger(int infoType, int bufferLength) {
            super(infoType, bufferLength);
        }

        @Override
        public Integer infoValue(MemorySegment infoValuePtr, short stringLength, boolean wideChar) {
            switch (this.bufferLength) {
            case 2:
                return (int) infoValuePtr.get(ValueLayout.JAVA_SHORT, 0);
            case 4:
                return infoValuePtr.get(ValueLayout.JAVA_INT, 0);
            default:
                throw new UnsupportedOperationException("bufferLength=" + bufferLength);
            }
        }
    }

    public static class InfoTypeString extends InfoType<String> {

        InfoTypeString(int infoType) {
            super(infoType, -1);
        }

        @Override
        public String infoValue(MemorySegment infoValuePtr, short stringLength, boolean wideChar) {
            if (wideChar) {
                return TgOdbcManager.stringFromUtf16Bytes(infoValuePtr, stringLength);
            } else {
                return TgOdbcManager.stringFromUtf8(infoValuePtr, stringLength);
            }
        }
    }
}
