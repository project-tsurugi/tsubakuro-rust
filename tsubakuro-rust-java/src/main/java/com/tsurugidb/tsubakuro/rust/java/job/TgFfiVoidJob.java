package com.tsurugidb.tsubakuro.rust.java.job;

import java.lang.foreign.MemorySegment;

import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiVoidJob extends TgFfiJob<Void> {

	public TgFfiVoidJob(TgFfiObjectManager manager, MemorySegment handle) {
		super(manager, handle);
	}

	@Override
	protected Void valueToFfiObject(TgFfiObjectManager manager, MemorySegment valueHandle) {
		assert valueHandle.address() == 0;
		return null;
	}
}
