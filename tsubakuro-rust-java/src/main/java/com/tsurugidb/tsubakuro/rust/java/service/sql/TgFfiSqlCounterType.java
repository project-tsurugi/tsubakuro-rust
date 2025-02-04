package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.util.HashMap;
import java.util.Map;

public enum TgFfiSqlCounterType {

    /**
     * <pre>
     * the un-categorized counter type.
     * </pre>
     *
     * <code>UNSPECIFIED = 0;</code>
     */
    UNSPECIFIED(0),
    /**
     * <pre>
     * The number of rows inserted in the execution.
     * </pre>
     *
     * <code>INSERTED_ROWS = 10;</code>
     */
    INSERTED_ROWS(10),
    /**
     * <pre>
     * The number of rows updated in the execution.
     * </pre>
     *
     * <code>UPDATED_ROWS = 20;</code>
     */
    UPDATED_ROWS(20),
    /**
     * <pre>
     * The number of rows merged in the execution.
     * </pre>
     *
     * <code>MERGED_ROWS = 30;</code>
     */
    MERGED_ROWS(30),
    /**
     * <pre>
     * The number of rows deleted in the execution.
     * </pre>
     *
     * <code>DELETED_ROWS = 40;</code>
     */
    DELETED_ROWS(40),

    //
    ;

    private static final Map<Integer, TgFfiSqlCounterType> VALUE_MAP;
    static {
        var map = new HashMap<Integer, TgFfiSqlCounterType>();
        for (var value : values()) {
            map.put(value.value, value);
        }
        VALUE_MAP = map;
    }

    private final int value;

    TgFfiSqlCounterType(int value) {
        this.value = value;
    }

    public static TgFfiSqlCounterType forNumber(int value) {
        var type = VALUE_MAP.get(value);
        if (type != null) {
            return type;
        }
        return null;
    }
}
