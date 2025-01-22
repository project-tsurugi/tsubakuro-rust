package com.tsurugidb.tsubakuro.rust.java.util;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;

@SuppressWarnings("serial")
public class TgFfiRuntimeException extends RuntimeException {

	private final int rc;

	public TgFfiRuntimeException(int rc) {
		this(rc, null);
	}

	public TgFfiRuntimeException(int rc, TgFfiContext context) {
		super(message(context));
		this.rc = rc;
	}

	private static String message(TgFfiContext context) {
		return (context != null) ? context.getErrorMessage() : null;
	}

	public int getReturnCode() {
		return this.rc;
	}
}
