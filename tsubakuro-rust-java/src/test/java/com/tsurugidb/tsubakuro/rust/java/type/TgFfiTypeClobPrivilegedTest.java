package com.tsurugidb.tsubakuro.rust.java.type;

import com.tsurugidb.tsubakuro.rust.java.session.TgFfiLobTransferType;

class TgFfiTypeClobPrivilegedTest extends TgFfiTypeClobTestBase {

    @Override
    protected TgFfiLobTransferType getLobTransferType() {
        return TgFfiLobTransferType.PRIVILEGED;
    }
}
