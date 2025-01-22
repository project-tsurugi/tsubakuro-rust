package com.tsurugidb.tsubakuro.rust.java.util;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;

public class TgFfiTester {

	static {
		String ffiLibraryPath = System.getProperty("tsurugi.ffi_library_path");
		if (ffiLibraryPath == null) {
			throw new RuntimeException("-Dtsurugi.ffi_library_path is not defined");
		}
		System.load(ffiLibraryPath);
	}

	private TgFfiObjectManager manager;

	@BeforeEach
	void beforeEach() {
		this.manager = TgFfiObjectManager.create();
	}

	protected TgFfiObjectManager getFfiObjectManager() {
		return this.manager;
	}

	@AfterEach
	void afterEach() {
		try (var _ = this.manager) {
		} finally {
			this.manager = null;
		}
	}
}
