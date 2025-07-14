package com.tsurugidb.tsubakuro.rust.odbc.stmt;

public enum StatementAttribute {
    SQL_ATTR_QUERY_TIMEOUT(0, Type.SQLULEN), //

    ;

    public enum Type {
        SQLULEN
    }

    private final int value;
    private final Type type;

    private StatementAttribute(int value, Type type) {
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
