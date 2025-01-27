package com.tsurugidb.tsubakuro.rust.java.transaction;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiCommitOption extends TgFfiObject {

	public static TgFfiCommitOption create(TgFfiContext context) {
		Objects.requireNonNull(context, "context must not be null");
		return create(context.manager(), context);
	}

	public static TgFfiCommitOption create(TgFfiObjectManager manager) {
		return create(manager, null);
	}

	public static TgFfiCommitOption create(TgFfiObjectManager manager, TgFfiContext context) {
		Objects.requireNonNull(manager, "manager must not be null");

		if (context != null) {
			synchronized (context) {
				return createMain(manager, context);
			}
		} else {
			return createMain(manager, null);
		}
	}

	private static TgFfiCommitOption createMain(TgFfiObjectManager manager, TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var out = manager.allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_create(ctx, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiCommitOption(manager, outHandle);
	}

	TgFfiCommitOption(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized void setCommitType(TgFfiContext context, TgFfiCommitType type) {
		Objects.requireNonNull(type, "type must not be null");

		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = type.value();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_set_commit_type(ctx, handle, arg);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	public synchronized TgFfiCommitType getCommitType(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_get_commit_type(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		int outInt = outToInt(out);
		return TgFfiCommitType.forNumber(outInt);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_dispose(handle);
	}
}
