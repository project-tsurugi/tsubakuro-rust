package com.tsurugidb.tsubakuro.rust.java.transaction;

public enum TgFfiTransactionPriority {

	/**
	 * <pre>
	 * use default transaction priority.
	 * </pre>
	 *
	 * <code>UNSPECIFIED = 0;</code>
	 */
	UNSPECIFIED(0),
	/**
	 * <pre>
	 * halts the running transactions immediately.
	 * </pre>
	 *
	 * <code>INTERRUPT = 1;</code>
	 */
	INTERRUPT(1),
	/**
	 * <pre>
	 * prevents new transactions and waits for the running transactions will end.
	 * </pre>
	 *
	 * <code>WAIT = 2;</code>
	 */
	WAIT(2),
	/**
	 * <pre>
	 * halts the running transactions immediately, and keep lock-out until its end.
	 * </pre>
	 *
	 * <code>INTERRUPT_EXCLUDE = 3;</code>
	 */
	INTERRUPT_EXCLUDE(3),
	/**
	 * <pre>
	 * prevents new transactions and waits for the running transactions will end, and keep lock-out until its end.
	 * </pre>
	 *
	 * <code>WAIT_EXCLUDE = 4;</code>
	 */
	WAIT_EXCLUDE(4),

	//
	;

	private final int value;

	TgFfiTransactionPriority(int value) {
		this.value = value;
	}

	public int value() {
		return this.value;
	}

	public static TgFfiTransactionPriority forNumber(int value) {
		switch (value) {
		case 0:
			return UNSPECIFIED;
		case 1:
			return INTERRUPT;
		case 2:
			return WAIT;
		case 3:
			return INTERRUPT_EXCLUDE;
		case 4:
			return WAIT_EXCLUDE;
		default:
			return null;
		}
	}
}
