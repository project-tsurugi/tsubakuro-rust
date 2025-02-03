package com.tsurugidb.tsubakuro.rust.java.transaction;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.ArrayList;
import java.util.List;
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

	public synchronized void setModifiesDefinitions(TgFfiContext context, boolean modifiesDefinitions) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = modifiesDefinitions;
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_modifies_definitions(ctx, handle, arg);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	public synchronized boolean getModifiesDefinitions(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_modifies_definitions(ctx, handle, out);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToBoolean(out);
	}

	public synchronized void setWritePreserve(TgFfiContext context, List<String> tableNames) {
		Objects.requireNonNull(tableNames, "tableNames must not be null");

		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();
		var arg = allocateStringArray(tableNames);
		var size = tableNames.size();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_write_preserve(ctx, handle, arg, size);
		TgFfiRcUtil.throwIfError(rc, context);
	}

	public synchronized List<String> getWritePreserve(TgFfiContext context) {
		var ctx = (context != null) ? context.handle() : MemorySegment.NULL;
		var handle = handle();

		var out = allocatePtr();
		var sizeOut = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_write_preserve(ctx, handle, out, sizeOut);
		TgFfiRcUtil.throwIfError(rc, context);

		return outToStringList(out, sizeOut);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_dispose(handle);
	}
}
