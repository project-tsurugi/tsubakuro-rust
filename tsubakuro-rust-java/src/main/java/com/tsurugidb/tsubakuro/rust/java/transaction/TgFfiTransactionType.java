package com.tsurugidb.tsubakuro.rust.java.transaction;

public enum TgFfiTransactionType {

	/**
	 * <pre>
	 * use default transaction type.
	 * </pre>
	 *
	 * <code>TRANSACTION_TYPE_UNSPECIFIED = 0;</code>
	 */
	TRANSACTION_TYPE_UNSPECIFIED(0),
	/**
	 * <pre>
	 * short transactions (optimistic concurrency control).
	 * </pre>
	 *
	 * <code>SHORT = 1;</code>
	 */
	SHORT(1),
	/**
	 * <pre>
	 * long transactions (pessimistic concurrency control).
	 * </pre>
	 *
	 * <code>LONG = 2;</code>
	 */
	LONG(2),
	/**
	 * <pre>
	 * read only transactions (may be abort-free).
	 * </pre>
	 *
	 * <code>READ_ONLY = 3;</code>
	 */
	READ_ONLY(3),

	//
	;

	private final int value;

	TgFfiTransactionType(int value) {
		this.value = value;
	}

	public int value() {
		return this.value;
	}

	public static TgFfiTransactionType forNumber(int value) {
		switch (value) {
		case 0:
			return TRANSACTION_TYPE_UNSPECIFIED;
		case 1:
			return SHORT;
		case 2:
			return LONG;
		case 3:
			return READ_ONLY;
		default:
			return null;
		}
	}
}
