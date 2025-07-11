package com.tsurugidb.tsubakuro.rust.odbc;

import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDiagRec;

@SuppressWarnings("serial")
public class TgOdbcRuntimeException extends RuntimeException {

    private final String functionName;
    private final int retrunCode;
    private final TgOdbcDiagRec diagRec;

    public TgOdbcRuntimeException(String functionName, int rc) {
        this(functionName, rc, null);
    }

    public TgOdbcRuntimeException(String functionName, int rc, TgOdbcDiagRec diagRec) {
        super(message(functionName, rc, diagRec));
        this.functionName = functionName;
        this.retrunCode = rc;
        this.diagRec = diagRec;
    }

    private static String message(String functionName, int rc, TgOdbcDiagRec diagRec) {
        if (diagRec == null) {
            return String.format("%s error. rc=%d", functionName, rc);
        }
        return String.format("%s error. rc=%d, diagRec=%s", functionName, rc, diagRec);
    }

    public String functionName() {
        return this.functionName;
    }

    public short returnCode() {
        return (short) retrunCode;
    }

    public TgOdbcDiagRec diagRec() {
        return this.diagRec;
    }
}
