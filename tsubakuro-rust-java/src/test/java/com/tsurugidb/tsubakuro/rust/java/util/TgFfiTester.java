package com.tsurugidb.tsubakuro.rust.java.util;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;

public class TgFfiTester {

	static {
		TgFfiInitializer.loadFfiLibrary();
	}

	private static final String SYSPROP_DBTEST_ENDPOINT = "tsurugi.dbtest.endpoint";
	private static String staticEndpoint;

	protected static String getEndpoint() {
		if (staticEndpoint == null) {
			staticEndpoint = System.getProperty(SYSPROP_DBTEST_ENDPOINT, "tcp://localhost:12345");
		}
		return staticEndpoint;
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
