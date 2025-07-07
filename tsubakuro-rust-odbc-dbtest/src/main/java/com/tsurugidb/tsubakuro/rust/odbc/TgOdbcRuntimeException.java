package com.tsurugidb.tsubakuro.rust.odbc;

import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDiagRec;

@SuppressWarnings("serial")
public class TgOdbcRuntimeException extends RuntimeException {

    private final String functionName;
    private final int result;
    private final TgOdbcDiagRec diagRec;

    public TgOdbcRuntimeException(String functionName, int result) {
        this(functionName, result, null);
    }

    public TgOdbcRuntimeException(String functionName, int result, TgOdbcDiagRec diagRec) {
        super(message(functionName, result, diagRec));
        this.functionName = functionName;
        this.result = result;
        this.diagRec = diagRec;
    }

    private static String message(String functionName, int result, TgOdbcDiagRec diagRec) {
        if (diagRec == null) {
            return String.format("%s error. result=%d", functionName, result);
        }
        return String.format("%s error. result=%d, diagRec=%s", functionName, result, diagRec);
    }

    public String functionName() {
        return this.functionName;
    }

    public short result() {
        return (short) result;
    }

    public TgOdbcDiagRec diagRec() {
        return this.diagRec;
    }
}
