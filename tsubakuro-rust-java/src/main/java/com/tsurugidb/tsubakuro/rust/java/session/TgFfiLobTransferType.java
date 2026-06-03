package com.tsurugidb.tsubakuro.rust.java.session;

public enum TgFfiLobTransferType {

    /** Indicates the default transfer policy. */
    DEFAULT(0),

    /** Does not use transfer type. */
    NOT_USE(1),

    /** Privileged transfer type. */
    PRIVILEGED(2),

    /** Blob Relay transfer type. */
    RELAY(3),

    //
    ;

    private final int value;

    TgFfiLobTransferType(int value) {
        this.value = value;
    }

    public int value() {
        return this.value;
    }

    public static TgFfiLobTransferType forNumber(int value) {
        switch (value) {
        case 0:
            return DEFAULT;
        case 1:
            return NOT_USE;
        case 2:
            return PRIVILEGED;
        case 3:
            return RELAY;
        default:
            return null;
        }
    }
}
