package com.tsurugidb.tsubakuro.rust.java.context;

import java.lang.foreign.MemorySegment;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRcUtil;

public class TgFfiContext extends TgFfiObject {

	public static TgFfiContext create(TgFfiObjectManager manager) {
		Objects.requireNonNull(manager, "manager must not be null");

		var out = manager.allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_create(out);
		TgFfiRcUtil.throwIfNg(rc);

		var handle = outToHandle(out);
		return new TgFfiContext(manager, handle);
	}

	TgFfiContext(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	public synchronized int getReturnCode() {
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_return_code(handle, out);
		TgFfiRcUtil.throwIfNg(rc);

		return outToInt(out);
	}

	public synchronized String getErrorMessage() {
		var handle = handle();
		var out = allocatePtr();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_message(handle, out);
		TgFfiRcUtil.throwIfNg(rc);

		return outToString(out);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_context_dispose(handle);
	}
}
