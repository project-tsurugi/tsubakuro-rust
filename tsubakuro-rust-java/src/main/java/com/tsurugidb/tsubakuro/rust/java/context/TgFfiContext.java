package com.tsurugidb.tsubakuro.rust.java.context;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.Objects;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRcUtil;

public class TgFfiContext extends TgFfiObject {

	public static TgFfiContext create(TgFfiObjectManager manager) {
		Objects.requireNonNull(manager, "manager must not be null");

		var handleRef = manager.allocateHandleRef();
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_create(handleRef);
		TgFfiRcUtil.throwIfNg(rc);

		var handle = handleRef.get(ValueLayout.ADDRESS, 0);
		return new TgFfiContext(manager, handle);
	}

	TgFfiContext(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_context_dispose(handle);
	}
}
