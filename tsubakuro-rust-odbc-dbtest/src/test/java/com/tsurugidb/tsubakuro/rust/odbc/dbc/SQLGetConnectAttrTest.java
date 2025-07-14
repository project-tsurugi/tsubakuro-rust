package com.tsurugidb.tsubakuro.rust.odbc.dbc;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.util.TgOdbcTester;

class SQLGetConnectAttrTest extends TgOdbcTester {

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_ATTR_CONNECTION_TIMEOUT(boolean wideChar) {
        try (var dbc = createDbc(); var _ = createConnection(dbc)) {
            dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_CONNECTION_TIMEOUT, 30, wideChar);

            int value = dbc.getConnectAttrInt(ConnectionAttribute.SQL_ATTR_CONNECTION_TIMEOUT, wideChar);
            assertEquals(30, value);
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_ATTR_LOGIN_TIMEOUT(boolean wideChar) {
        try (var dbc = createDbc(); var _ = createConnection(dbc)) {
            dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_LOGIN_TIMEOUT, 20, wideChar);

            int value = dbc.getConnectAttrInt(ConnectionAttribute.SQL_ATTR_LOGIN_TIMEOUT, wideChar);
            assertEquals(20, value);
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_ATTR_AUTOCOMMIT(boolean wideChar) {
        try (var dbc = createDbc(); var _ = createConnection(dbc)) {
            { // default value
                int autoCommit = dbc.getConnectAttrInt(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, wideChar);
                assertEquals(OdbcAttrConst.SQL_AUTOCOMMIT_ON, autoCommit);
            }
            {
                dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, OdbcAttrConst.SQL_AUTOCOMMIT_OFF, wideChar);

                int autoCommit = dbc.getConnectAttrInt(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, wideChar);
                assertEquals(OdbcAttrConst.SQL_AUTOCOMMIT_OFF, autoCommit);
            }
            {
                dbc.setConnectAttr(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, OdbcAttrConst.SQL_AUTOCOMMIT_ON, wideChar);

                int autoCommit = dbc.getConnectAttrInt(ConnectionAttribute.SQL_ATTR_AUTOCOMMIT, wideChar);
                assertEquals(OdbcAttrConst.SQL_AUTOCOMMIT_ON, autoCommit);
            }
        }
    }

    @ParameterizedTest
    @ValueSource(booleans = { false, true })
    void SQL_ATTR_CONNECTION_DEAD(boolean wideChar) {
        try (var dbc = createDbc(); var _ = createConnection(dbc)) {
            int value = dbc.getConnectAttrInt(ConnectionAttribute.SQL_ATTR_CONNECTION_DEAD, wideChar);
            assertEquals(0 /* SQL_CD_FALSE */, value);
        }
    }
}
