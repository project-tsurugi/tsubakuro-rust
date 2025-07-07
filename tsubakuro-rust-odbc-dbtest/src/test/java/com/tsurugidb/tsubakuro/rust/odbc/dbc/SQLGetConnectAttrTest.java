package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle.ConnectionAttribute;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLGetConnectAttrTest extends TgOdbcTester {

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void connectAttr_AUTOCOMMIT(boolean wideChar) {
        try (var dbc = createDbc()) {
            { // default value
                int autoCommit = (Integer) dbc.getConnectAttr(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, wideChar);
                assertEquals(OdbcAttrConst.SQL_AUTOCOMMIT_ON, autoCommit);
            }
            {
                dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, OdbcAttrConst.SQL_AUTOCOMMIT_OFF, wideChar);

                int autoCommit = (Integer) dbc.getConnectAttr(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, wideChar);
                assertEquals(OdbcAttrConst.SQL_AUTOCOMMIT_OFF, autoCommit);
            }
            {
                dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, OdbcAttrConst.SQL_AUTOCOMMIT_ON, wideChar);

                int autoCommit = (Integer) dbc.getConnectAttr(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, wideChar);
                assertEquals(OdbcAttrConst.SQL_AUTOCOMMIT_ON, autoCommit);
            }
        }
    }
}
