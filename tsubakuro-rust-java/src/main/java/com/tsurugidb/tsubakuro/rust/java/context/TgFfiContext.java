package com.tsurugidb.tsubakuro.rust.java.context;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.TgFfiObject;
import com.tsurugidb.tsubakuro.rust.java.TgFfiRcUtil;

public class TgFfiContext extends TgFfiObject {

	public static TgFfiContext create(Arena arena) {
		var handle = arena.allocate(ValueLayout.ADDRESS);
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_create(handle);
		TgFfiRcUtil.throwIfNg(rc);
		return new TgFfiContext(handle);
	}

	TgFfiContext(MemorySegment handle) {
		super(handle);
	}

	@Override
	protected void dispose(MemorySegment handle) {
		tsubakuro_rust_ffi_h.tsurugi_ffi_context_dispose(handle);
	}
}
