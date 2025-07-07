package com.tsurugidb.tsubakuro.rust.odbc.api;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcRuntimeException;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDiagRec;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcHandle;

public class SqlReturn {

    public static final int SQL_SUCCESS = 0;
    public static final int SQL_SUCCESS_WITH_INFO = 1;
    public static final int SQL_NO_DATA = 100;
    public static final int SQL_ERROR = -1;

    public static boolean isSuccess(short result) {
        return (result == SQL_SUCCESS) || (result == SQL_SUCCESS_WITH_INFO);
    }

    public static void check(String functionName, short result) {
        if (isSuccess(result)) {
            return;
        }
        throw new TgOdbcRuntimeException(functionName, result);
    }

    public static void check(String functionName, short result, TgOdbcHandle handle) {
        if (isSuccess(result)) {
            return;
        }

        TgOdbcDiagRec diagRec;
        try {
            diagRec = handle.getDiagRec(1);
        } catch (Throwable t) {
            var e = new TgOdbcRuntimeException(functionName, result);
            e.addSuppressed(t);
            throw e;
        }
        throw new TgOdbcRuntimeException(functionName, result, diagRec);
    }
}
