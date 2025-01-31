package com.tsurugidb.tsubakuro.rust.java.util;

import java.util.function.Supplier;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcType;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;

@SuppressWarnings("serial")
public class TgFfiRuntimeException extends RuntimeException {

	private final int rc;
	private final String errorName;
	private final TgFfiRcType errorType;
	private final ContextValue<Integer> serverErrorCategoryNumber;
	private final ContextValue<String> serverErrorCategoryStr;
	private final ContextValue<Integer> serverErrorCodeNumber;
	private final ContextValue<String> serverErrorStructuredCode;

	public TgFfiRuntimeException(int rc) {
		this(rc, null);
	}

	public TgFfiRuntimeException(int rc, TgFfiContext context) {
		super(message(rc, context));
		this.rc = rc;
		if (context != null) {
			this.errorName = context.getErrorName();
			this.errorType = context.getErrorType();
			this.serverErrorCategoryNumber = new ContextValue<>(context::getServerErrorCategoryNumber);
			this.serverErrorCategoryStr = new ContextValue<>(context::getServerErrorCategoryStr);
			this.serverErrorCodeNumber = new ContextValue<>(context::getServerErrorCodeNumber);
			this.serverErrorStructuredCode = new ContextValue<>(context::getServerErrorStructuredCode);
		} else {
			this.errorName = null;
			this.errorType = null;
			this.serverErrorCategoryNumber = null;
			this.serverErrorCategoryStr = null;
			this.serverErrorCodeNumber = null;
			this.serverErrorStructuredCode = null;
		}
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

	public String getReturnCodeName() {
		return TgFfiRcUtil.toName(this.rc);
	}

	public String getErrorName() {
		return this.errorName;
	}

	public TgFfiRcType getErrorType() {
		return this.errorType;
	}

	public int getServerErrorCategoryNumber() {
		if (this.serverErrorCategoryNumber != null) {
			return serverErrorCategoryNumber.get();
		}
		return -1;
	}

	public String getServerErrorCategoryStr() {
		if (this.serverErrorCategoryStr != null) {
			return serverErrorCategoryStr.get();
		}
		return null;
	}

	public int getServerErrorCodeNumber() {
		if (this.serverErrorCodeNumber != null) {
			return serverErrorCodeNumber.get();
		}
		return -1;
	}

	public String getServerErrorStructuredCode() {
		if (this.serverErrorStructuredCode != null) {
			return serverErrorStructuredCode.get();
		}
		return null;
	}

	private static class ContextValue<T> {
		private final T value;
		private final TgFfiRuntimeException exception;

		public ContextValue(Supplier<T> getter) {
			T value = null;
			TgFfiRuntimeException exception = null;
			try {
				value = getter.get();
			} catch (TgFfiRuntimeException e) {
				exception = e;
			}
			this.value = value;
			this.exception = exception;
		}

		public T get() {
			if (this.exception != null) {
				throw this.exception;
			}
			return this.value;
		}
	}
}
