package com.tsurugidb.tsubakuro.rust.java.type;

import com.tsurugidb.tsubakuro.rust.java.session.TgFfiLobTransferType;

class TgFfiTypeBlobRelayTest extends TgFfiTypeBlobTestBase {

    @Override
    protected TgFfiLobTransferType getLobTransferType() {
        return TgFfiLobTransferType.RELAY;
    }
}
