package com.tsurugidb.tsubakuro.rust.odbc.dbc;

public enum ConnectionAttribute {
    SQL_ATTR_AUTOCOMMIT(102, Type.SQLUINTEGER), //
    SQL_ATTR_LOGIN_TIMEOUT(103, Type.SQLUINTEGER), //
    SQL_ATTR_CONNECTION_TIMEOUT(113, Type.SQLUINTEGER), //
    SQL_ATTR_CONNECTION_DEAD(1209, Type.SQLUINTEGER), //

    ;

    public enum Type {
        SQLUINTEGER
    }

    private final int value;
    private final Type type;

    private ConnectionAttribute(int value, Type type) {
        this.value = value;
        this.type = type;
    }

    public int value() {
        return this.value;
    }

    public Type type() {
        return this.type;
    }
}
