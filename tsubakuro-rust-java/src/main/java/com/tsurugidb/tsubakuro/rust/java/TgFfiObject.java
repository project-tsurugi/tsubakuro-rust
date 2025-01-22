package com.tsurugidb.tsubakuro.rust.java;

import java.io.Closeable;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.util.Objects;

public abstract class TgFfiObject implements Closeable {
	private static final Cleaner cleaner = Cleaner.create();

	private MemorySegment handle;

	@SuppressWarnings("unused")
	private final Cleaner.Cleanable cleanable;

	public TgFfiObject(MemorySegment handle) {
		this.handle = Objects.requireNonNull(handle);
		this.cleanable = cleaner.register(this, this::close);
	}

	// use in synchronized(this)
	protected final MemorySegment handle() {
		var handle = this.handle;
		if (handle == null) {
			throw new IllegalStateException("handle already closed");
		}

		return handle;
	}

	@Override
	public void close() {
		MemorySegment handle;
		synchronized (this) {
			handle = this.handle;
			if (handle == null) {
				return;
			}
			this.handle = null;
		}

		dispose(handle);
	}

	protected abstract void dispose(MemorySegment handle);
}
