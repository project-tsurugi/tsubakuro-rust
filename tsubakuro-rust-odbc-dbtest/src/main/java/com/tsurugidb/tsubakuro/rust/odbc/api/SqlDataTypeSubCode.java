package com.tsurugidb.tsubakuro.rust.odbc.api;

public enum SqlDataTypeSubCode {
    SQL_CODE_DATE(1), //
    SQL_CODE_TIME(2), //
    SQL_CODE_TIMESTAMP(3), //
    ;

    private final short value;

    SqlDataTypeSubCode(int value) {
        this.value = (short) value;
    }

    public short value() {
        return this.value;
    }
}
