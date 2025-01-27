package com.tsurugidb.tsubakuro.rust.java.service.sql;

public enum TgFfiSqlCounterType {

	/**
	 * <pre>
	 * the un-categorized counter type.
	 * </pre>
	 *
	 * <code>COUNTER_TYPE_UNSPECIFIED = 0;</code>
	 */
	COUNTER_TYPE_UNSPECIFIED(0),
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

	TgFfiSqlCounterType(int value) {
	}
}
