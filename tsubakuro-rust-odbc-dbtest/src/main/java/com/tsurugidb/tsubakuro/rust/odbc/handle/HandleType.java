package com.tsurugidb.tsubakuro.rust.odbc.handle;

public enum HandleType {

    /** env */
    SQL_HANDLE_ENV(1),
    /** dbc */
    SQL_HANDLE_DBC(2),
    /** stmt */
    SQL_HANDLE_STMT(3),
    /** desc */
    SQL_HANDLE_DESC(4);

    private final short value;

    private HandleType(int value) {
        this.value = (short) value;
    }

    public short value() {
        return value;
    }
}
