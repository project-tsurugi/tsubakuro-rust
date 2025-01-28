package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.util.HashMap;
import java.util.Map;

public enum TgFfiAtomType {

	/**
	 * <pre>
	 * unspecified type.
	 * </pre>
	 *
	 * <code>TYPE_UNSPECIFIED = 0;</code>
	 */
	TYPE_UNSPECIFIED(0),
	/**
	 * <pre>
	 * boolean type.
	 * </pre>
	 *
	 * <code>BOOLEAN = 1;</code>
	 */
	BOOLEAN(1),
	/**
	 * <pre>
	 * 32-bit signed integer.
	 * </pre>
	 *
	 * <code>INT4 = 4;</code>
	 */
	INT4(4),
	/**
	 * <pre>
	 * 64-bit signed integer.
	 * </pre>
	 *
	 * <code>INT8 = 5;</code>
	 */
	INT8(5),
	/**
	 * <pre>
	 * 32-bit floating point number.
	 * </pre>
	 *
	 * <code>FLOAT4 = 6;</code>
	 */
	FLOAT4(6),
	/**
	 * <pre>
	 * 64-bit floating point number.
	 * </pre>
	 *
	 * <code>FLOAT8 = 7;</code>
	 */
	FLOAT8(7),
	/**
	 * <pre>
	 * multi precision decimal number.
	 * </pre>
	 *
	 * <code>DECIMAL = 8;</code>
	 */
	DECIMAL(8),
	/**
	 * <pre>
	 * character sequence.
	 * </pre>
	 *
	 * <code>CHARACTER = 9;</code>
	 */
	CHARACTER(9),
	/**
	 * <pre>
	 * octet sequence.
	 * </pre>
	 *
	 * <code>OCTET = 11;</code>
	 */
	OCTET(11),
	/**
	 * <pre>
	 * bit sequence.
	 * </pre>
	 *
	 * <code>BIT = 13;</code>
	 */
	BIT(13),
	/**
	 * <pre>
	 * date.
	 * </pre>
	 *
	 * <code>DATE = 15;</code>
	 */
	DATE(15),
	/**
	 * <pre>
	 * time of day.
	 * </pre>
	 *
	 * <code>TIME_OF_DAY = 16;</code>
	 */
	TIME_OF_DAY(16),
	/**
	 * <pre>
	 * time point.
	 * </pre>
	 *
	 * <code>TIME_POINT = 17;</code>
	 */
	TIME_POINT(17),
	/**
	 * <pre>
	 * date-time interval.
	 * </pre>
	 *
	 * <code>DATETIME_INTERVAL = 18;</code>
	 */
	DATETIME_INTERVAL(18),
	/**
	 * <pre>
	 * time of day with time zone.
	 * </pre>
	 *
	 * <code>TIME_OF_DAY_WITH_TIME_ZONE = 19;</code>
	 */
	TIME_OF_DAY_WITH_TIME_ZONE(19),
	/**
	 * <pre>
	 * time point with time zone.
	 * </pre>
	 *
	 * <code>TIME_POINT_WITH_TIME_ZONE = 20;</code>
	 */
	TIME_POINT_WITH_TIME_ZONE(20),
	/**
	 * <pre>
	 * character large objects.
	 * </pre>
	 *
	 * <code>CLOB = 21;</code>
	 */
	CLOB(21),
	/**
	 * <pre>
	 * binary large objects.
	 * </pre>
	 *
	 * <code>BLOB = 22;</code>
	 */
	BLOB(22),
	/**
	 * <pre>
	 * unknown type.
	 * </pre>
	 *
	 * <code>UNKNOWN = 31;</code>
	 */
	UNKNOWN(31),
	//
	;

	private static final Map<Integer, TgFfiAtomType> VALUE_MAP;
	static {
		var map = new HashMap<Integer, TgFfiAtomType>();
		for (var value : values()) {
			map.put(value.value, value);
		}
		VALUE_MAP = map;
	}

	private final int value;

	TgFfiAtomType(int value) {
		this.value = value;
	}

	public int value() {
		return this.value;
	}

	public static TgFfiAtomType forNumber(int value) {
		var type = VALUE_MAP.get(value);
		if (type != null) {
			return type;
		}
		return null;
	}
}
