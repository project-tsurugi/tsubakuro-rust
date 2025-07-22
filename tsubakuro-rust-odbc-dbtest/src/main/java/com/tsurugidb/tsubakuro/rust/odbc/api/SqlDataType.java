package com.tsurugidb.tsubakuro.rust.odbc.api;

import java.util.HashMap;
import java.util.Map;

/**
 * SQL data types enumeration.
 */
public enum SqlDataType {
    /** Unknown SQL type */
    SQL_UNKNOWN_TYPE(0),

    /** CHAR */
    SQL_CHAR(1),

    /** DECIMAL */
    SQL_NUMERIC(2),

    /** DECIMAL */
    SQL_DECIMAL(3),

    /** INT */
    SQL_INTEGER(4),

    /** SMALLINT */
    SQL_SMALLINT(5),

    /** REAL */
    SQL_FLOAT(6),

    /** REAL */
    SQL_REAL(7),

    /** BOULE */
    SQL_DOUBLE(8),

    /** DATETIME */
    SQL_DATETIME(9),

    /** VARCHAR */
    SQL_VARCHAR(12),

    /** DATE */
    SQL_TYPE_DATE(91),

    /** TIME */
    SQL_TYPE_TIME(92),

    /** TIMESTAMP */
    SQL_TYPE_TIMESTAMP(93),

    /** TIMESTAMP WITH TIME ZONE */
    SQL_TYPE_TIMESTAMP_WITH_TIMEZONE(95),

    /** Interval in years */
    SQL_INTERVAL_YEAR(101),

    /** Interval in months */
    SQL_INTERVAL_MONTH(102),

    /** Interval in days */
    SQL_INTERVAL_DAY(103),

    /** Interval in hours */
    SQL_INTERVAL_HOUR(104),

    /** Interval in minutes */
    SQL_INTERVAL_MINUTE(105),

    /** Interval in seconds */
    SQL_INTERVAL_SECOND(106),

    /** Interval in years and months */
    SQL_INTERVAL_YEAR_TO_MONTH(107),

    /** Interval in days and hours */
    SQL_INTERVAL_DAY_TO_HOUR(108),

    /** Interval in days and minutes */
    SQL_INTERVAL_DAY_TO_MINUTE(109),

    /** Interval in days and seconds */
    SQL_INTERVAL_DAY_TO_SECOND(110),

    /** Interval in hours and minutes */
    SQL_INTERVAL_HOUR_TO_MINUTE(111),

    /** Interval in hours and seconds */
    SQL_INTERVAL_HOUR_TO_SECOND(112),

    /** Interval in minutes and seconds */
    SQL_INTERVAL_MINUTE_TO_SECOND(113),

    /** CLOB */
    SQL_LONGVARCHAR(-1),

    /** BINARY */
    SQL_BINARY(-2),

    /** VARBINARY */
    SQL_VARBINARY(-3),

    /** BLOB */
    SQL_LONGVARBINARY(-4),

    /** BIGINT */
    SQL_BIGINT(-5),

    /** TINYINT */
    SQL_TINYINT(-6),

    /** BIT */
    SQL_BIT(-7),

    /** Wide CHAR */
    SQL_WCHAR(-8),

    /** Wide VARCHAR */
    SQL_WVARCHAR(-9),

    /** Wide CLOB */
    SQL_WLONGVARCHAR(-10),

    /** Globally unique identifier */
    SQL_GUID(-11);

    private final int value;

    SqlDataType(int value) {
        this.value = value;
    }

    public short value() {
        return (short) this.value;
    }

    private static Map<Integer, SqlDataType> map;

    /**
     * Gets the SQL data type from its integer value.
     * 
     * @param value integer value
     * @return corresponding SqlDataType
     * @throws IllegalArgumentException if the value is unknown
     */
    public static SqlDataType fromValue(int value) {
        if (map == null) {
            var values = values();
            var m = new HashMap<Integer, SqlDataType>(values.length);
            for (SqlDataType type : values()) {
                m.put(type.value, type);
            }
            map = m;
        }

        var type = map.get(value);
        if (type == null) {
            throw new IllegalArgumentException("Unknown SQL data type value: " + value);
        }
        return type;
    }
}
