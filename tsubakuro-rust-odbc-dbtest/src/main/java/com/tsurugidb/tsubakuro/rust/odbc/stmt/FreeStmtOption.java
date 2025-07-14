package com.tsurugidb.tsubakuro.rust.odbc.stmt;

public enum FreeStmtOption {
    SQL_CLOSE(0), //
    SQL_DROP(1), //
    SQL_UNBIND(2), //
    SQL_RESET_PARAMS(3), //
    ;

    private final short value;

    private FreeStmtOption(int value) {
        this.value = (short) value;
    }

    public short value() {
        return this.value;
    }
}
