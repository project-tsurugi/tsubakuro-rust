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
    void connectAttr_CONNECTION_TIMEOUT(boolean wideChar) {
        try (var dbc = createDbc(); var _ = createConnection(dbc)) {
            dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_CONNECTION_TIMEOUT, 30, wideChar);

            int value = (Integer) dbc.getConnectAttr(ConnectionAttribute.SQL_ATTR_CONNECTION_TIMEOUT, wideChar);
            assertEquals(30, value);
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void connectAttr_LOGIN_TIMEOUT(boolean wideChar) {
        try (var dbc = createDbc(); var _ = createConnection(dbc)) {
            dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_LOGIN_TIMEOUT, 20, wideChar);

            int value = (Integer) dbc.getConnectAttr(ConnectionAttribute.SQL_ATTR_LOGIN_TIMEOUT, wideChar);
            assertEquals(20, value);
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void connectAttr_AUTOCOMMIT(boolean wideChar) {
        try (var dbc = createDbc(); var _ = createConnection(dbc)) {
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
