package com.tsurugidb.tsubakuro.rust.java.transaction;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcUtil;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiTransactionOption extends TgFfiObject {

	public static TgFfiTransactionOption create(TgFfiContext context) {
		Objects.requireNonNull(context, "context must not be null");
		return create(context.manager(), context);
	}

	public static TgFfiTransactionOption create(TgFfiObjectManager manager) {
		return create(manager, null);
	}

	public static TgFfiTransactionOption create(TgFfiObjectManager manager, TgFfiContext context) {
		Objects.requireNonNull(manager, "manager must not be null");

		if (context != null) {
			synchronized (context) {
				return createMain(manager, context);
			}
		} else {
			return createMain(manager, null);
		}
	}

	private static TgFfiTransactionOption createMain(TgFfiObjectManager manager, TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var out = manager.allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_create(ctx, out);
		TgFfiRcUtil.throwIfError(rc, context);

		var outHandle = outToHandle(out);
		return new TgFfiTransactionOption(manager, outHandle);
	}

	TgFfiTransactionOption(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized void setTransactionType(TgFfiContext context, TgFfiTransactionType type) {
		Objects.requireNonNull(type, "type must not be null");

		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = type.value();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_type(ctx, handle, arg);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	public synchronized TgFfiTransactionType getTransactionType(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_type(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		int outInt = outToInt(out);
		return TgFfiTransactionType.forNumber(outInt);
	}

	public synchronized void setTransactionLabel(TgFfiContext context, String label) {
		Objects.requireNonNull(label, "label must not be null");

		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = allocateString(label);
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_label(ctx, handle, arg);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	public synchronized String getTransactionLabel(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_label(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToString(out);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_dispose(handle);
	}
}
