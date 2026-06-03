package com.tsurugidb.tsubakuro.rust.java.type;

import com.tsurugidb.tsubakuro.rust.java.session.TgFfiLobTransferType;

class TgFfiTypeClobRelayTest extends TgFfiTypeClobTestBase {

    @Override
    protected TgFfiLobTransferType getLobTransferType() {
        return TgFfiLobTransferType.RELAY;
    }
}
