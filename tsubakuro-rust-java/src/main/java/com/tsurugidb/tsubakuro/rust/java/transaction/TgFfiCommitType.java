package com.tsurugidb.tsubakuro.rust.java.transaction;

public enum TgFfiCommitType {

    /**
     * <pre>
     * the default commit status (rely on the database settings).
     * </pre>
     *
     * <code>UNSPECIFIED = 0;</code>
     */
    UNSPECIFIED(0),
    /**
     * <pre>
     * commit operation has accepted, and the transaction will never abort except system errors.
     * </pre>
     *
     * <code>ACCEPTED = 10;</code>
     */
    ACCEPTED(10),
    /**
     * <pre>
     * commit data has been visible for others.
     * </pre>
     *
     * <code>AVAILABLE = 20;</code>
     */
    AVAILABLE(20),
    /**
     * <pre>
     * commit data has been saved on the local disk.
     * </pre>
     *
     * <code>STORED = 30;</code>
     */
    STORED(30),
    /**
     * <pre>
     * commit data has been propagated to the all suitable nodes.
     * </pre>
     *
     * <code>PROPAGATED = 40;</code>
     */
    PROPAGATED(40),

    //
    ;

    private final int value;

    TgFfiCommitType(int value) {
        this.value = value;
    }

    public int value() {
        return this.value;
    }

    public static TgFfiCommitType forNumber(int value) {
        switch (value) {
        case 0:
            return UNSPECIFIED;
        case 10:
            return ACCEPTED;
        case 20:
            return AVAILABLE;
        case 30:
            return STORED;
        case 40:
            return PROPAGATED;
        default:
            return null;
        }
    }
}
