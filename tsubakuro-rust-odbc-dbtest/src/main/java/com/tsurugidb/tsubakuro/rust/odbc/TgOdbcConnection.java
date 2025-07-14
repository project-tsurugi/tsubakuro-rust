package com.tsurugidb.tsubakuro.rust.odbc;

import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.dbc.ConnectionAttribute;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcStmtHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcHandle.CompletionType;

public class TgOdbcConnection extends TgOdbcResource {

    public static TgOdbcConnection connect(TgOdbcDbcHandle dbc, String connectionString) {
        return dbc.driverConnect(connectionString, false);
    }

    private final TgOdbcDbcHandle dbc;
    private final String connectionString;

    public TgOdbcConnection(TgOdbcDbcHandle dbc, String connectionString) {
        super(dbc.manager());
        this.dbc = dbc;
        this.connectionString = connectionString;
    }

    public TgOdbcDbcHandle dbc() {
        return this.dbc;
    }

    public String connectionString() {
        return this.connectionString;
    }

    public TgOdbcStmtHandle createStmt() {
        return TgOdbcStmtHandle.allocStmtHandle(dbc);
    }

    public int execute(String sql, boolean wideChar) {
        try (var stmt = createStmt()) {
            stmt.execDirect(sql, wideChar);

            return (int) stmt.rowCount();
        }
    }

    public void setAutoCommit(boolean autoCommit, boolean wideChar) {
        int value = autoCommit ? OdbcAttrConst.SQL_AUTOCOMMIT_ON : OdbcAttrConst.SQL_AUTOCOMMIT_OFF;
        dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, value, wideChar);
    }

    public boolean autoCommit(boolean wideChar) {
        int value = dbc.getConnectAttrInt(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, wideChar);
        return value != OdbcAttrConst.SQL_AUTOCOMMIT_OFF;
    }

    public void commit() {
        dbc.endTran(CompletionType.SQL_COMMIT);
    }

    public void rollback() {
        dbc.endTran(CompletionType.SQL_ROLLBACK);
    }

    @Override
    public void close() {
        dbc.disconnect();
    }

    @Override
    public String toString() {
        return "OdbcConnection(" + connectionString + ")";
    }
}
