package com.tsurugidb.tsubakuro.rust.java.transaction;

import java.util.HashMap;
import java.util.Map;

public enum TgFfiTransactionStatus {

    /**
     * <pre>
     * the transaction status unknown or not provided.
     * </pre>
     *
     * <code>TRANSACTION_STATUS_UNSPECIFIED = 0;</code>
     */
    UNSPECIFIED(0),
    /**
     * <pre>
     * the transaction is started and running.
     * </pre>
     *
     * <code>RUNNING = 10;</code>
     */
    RUNNING(10),
    /**
     * <pre>
     * the transaction is in the process of committing.
     * </pre>
     *
     * <code>COMMITTING = 20;</code>
     */
    COMMITTING(20),
    /**
     * <pre>
     * the transaction has been committed and visible for others.
     * </pre>
     *
     * <code>AVAILABLE = 30;</code>
     */
    AVAILABLE(30),
    /**
     * <pre>
     * the transaction has been committed and saved on the local disk.
     * </pre>
     *
     * <code>STORED = 40;</code>
     */
    STORED(40),
    /**
     * <pre>
     * the transaction has been committed and propagated to all the suitable nodes.
     * </pre>
     *
     * <code>PROPAGATED = 50;</code>
     */
    PROPAGATED(50),
    /**
     * <pre>
     * the transaction is in the process of aborting.
     * </pre>
     *
     * <code>ABORTING = 60;</code>
     */
    ABORTING(60),
    /**
     * <pre>
     * the transaction has been aborted.
     * </pre>
     *
     * <code>ABORTED = 70;</code>
     */
    ABORTED(70),

    //
    ;

    private static final Map<Integer, TgFfiTransactionStatus> NUMBER_MAP;
    static {
        var map = new HashMap<Integer, TgFfiTransactionStatus>();
        for (var type : values()) {
            map.put(type.value, type);
        }
        NUMBER_MAP = map;
    }

    private final int value;

    TgFfiTransactionStatus(int value) {
        this.value = value;
    }

    public int value() {
        return this.value;
    }

    public static TgFfiTransactionStatus forNumber(int value) {
        return NUMBER_MAP.get(value);
    }
}
