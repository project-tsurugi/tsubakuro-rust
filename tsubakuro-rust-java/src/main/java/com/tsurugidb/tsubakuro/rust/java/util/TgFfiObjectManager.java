package com.tsurugidb.tsubakuro.rust.java.util;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

public class TgFfiObjectManager implements AutoCloseable {

	public static TgFfiObjectManager create() {
		return new TgFfiObjectManager();
	}

	private final Arena arena;
	private final Set<TgFfiObject> objectSet = ConcurrentHashMap.newKeySet();

	public TgFfiObjectManager() {
		this.arena = Arena.ofConfined();
	}

	public MemorySegment allocatePtr() {
		return arena.allocate(ValueLayout.ADDRESS);
	}

	public MemorySegment allocateString(String s) {
		return arena.allocateFrom(s);
	}

	public void add(TgFfiObject object) {
		objectSet.add(object);
	}

	public void remove(TgFfiObject object) {
		objectSet.remove(object);
	}

	@Override
	public void close() {
		List<RuntimeException> list = null;
		for (var i = objectSet.iterator(); i.hasNext();) {
			var object = i.next();
			i.remove();
			try {
				object.dispose();
			} catch (RuntimeException e) {
				if (list == null) {
					list = new ArrayList<>();
				}
				list.add(e);
			}
		}

		try {
			arena.close();
		} catch (RuntimeException e) {
			if (list != null) {
				for (var e0 : list) {
					e.addSuppressed(e0);
				}
			}
			throw e;
		}

		if (list != null) {
			RuntimeException e = null;
			for (var e0 : list) {
				if (e == null) {
					e = e0;
				} else {
					e.addSuppressed(e0);
				}
			}
			throw e;
		}
	}
}
