package com.tsurugidb.tsubakuro.rust.java.util;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;

@SuppressWarnings("serial")
public class TgFfiRuntimeException extends RuntimeException {

	private final int rc;

	public TgFfiRuntimeException(int rc) {
		this(rc, null);
	}

	public TgFfiRuntimeException(int rc, TgFfiContext context) {
		super(message(rc, context));
		this.rc = rc;
	}

	private static String message(int rc, TgFfiContext context) {
		String rcName = String.format("%s[%08x]", TgFfiRcUtil.toName(rc), rc);
		if (context != null) {
			String errorMessage = context.getErrorMessage();
			if (errorMessage != null) {
				return rcName + ": " + errorMessage;
			}
		}
		return rcName;
	}

	public int getReturnCode() {
		return this.rc;
	}
}
