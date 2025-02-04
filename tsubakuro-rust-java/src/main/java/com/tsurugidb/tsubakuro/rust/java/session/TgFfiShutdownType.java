package com.tsurugidb.tsubakuro.rust.java.session;

public enum TgFfiShutdownType {

    /**
     * <pre>
     * The default shutdown type.
     * </pre>
     *
     * <code>NOT_SET = 0;</code>
     */
    NOT_SET(0),
    /**
     * <pre>
     * Waits for the ongoing requests and safely shutdown the session.
     * </pre>
     *
     * <code>GRACEFUL = 1;</code>
     */
    GRACEFUL(1),
    /**
     * <pre>
     * Cancelling the ongoing requests and safely shutdown the session.
     * </pre>
     *
     * <code>FORCEFUL = 2;</code>
     */
    FORCEFUL(2),

    //
    ;

    private final int value;

    TgFfiShutdownType(int value) {
        this.value = value;
    }

    public int value() {
        return this.value;
    }

    public static TgFfiShutdownType forNumber(int value) {
        switch (value) {
        case 0:
            return NOT_SET;
        case 1:
            return GRACEFUL;
        case 2:
            return FORCEFUL;
        default:
            return null;
        }
    }
}
