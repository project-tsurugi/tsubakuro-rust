package com.tsurugidb.tsubakuro.rust.java.util;

import java.io.Closeable;
import java.lang.foreign.MemorySegment;
import java.lang.ref.Cleaner;
import java.util.Objects;

public abstract class TgFfiObject implements Closeable {
	private static final Cleaner cleaner = Cleaner.create();

	private final TgFfiObjectManager manager;
	private MemorySegment handle;

	@SuppressWarnings("unused")
	private final Cleaner.Cleanable cleanable;

	public TgFfiObject(TgFfiObjectManager manager, MemorySegment handle) {
		this.manager = Objects.requireNonNull(manager, "manager must not be null");
		this.handle = Objects.requireNonNull(handle, "handle must not be null");
		this.cleanable = cleaner.register(this, this::close);

		manager.add(this);
	}

	public final TgFfiObjectManager manager() {
		return this.manager;
	}

	// use in synchronized(this)
	public final MemorySegment handle() {
		var handle = this.handle;
		if (handle == null) {
			throw new IllegalStateException("handle already closed");
		}

		return handle;
	}

	@Override
	public final int hashCode() {
		return super.hashCode(); // System.identityHashCode(this)
	}

	@Override
	public final boolean equals(Object obj) {
		return super.equals(obj); // this == obj
	}

	@Override
	public void close() {
		try {
			dispose();
		} finally {
			manager.remove(this);
		}
	}

	public void dispose() {
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
