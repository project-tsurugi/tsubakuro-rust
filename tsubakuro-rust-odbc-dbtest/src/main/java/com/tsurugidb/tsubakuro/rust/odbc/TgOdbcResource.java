package com.tsurugidb.tsubakuro.rust.odbc;

public abstract class TgOdbcResource implements AutoCloseable {

    protected final TgOdbcManager manager;

    public TgOdbcResource(TgOdbcManager manager) {
        this.manager = manager;
    }

    public TgOdbcManager manager() {
        return this.manager;
    }
}
