package com.tsurugidb.tsubakuro.rust.odbc.handle;

public record TgOdbcDiagRec(short recNumber, String sqlState, int nativeError, String message) {

}
