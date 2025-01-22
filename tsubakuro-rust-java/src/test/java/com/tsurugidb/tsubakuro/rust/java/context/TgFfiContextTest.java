package com.tsurugidb.tsubakuro.rust.java.context;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiContextTest extends TgFfiTester {

	@Test
	void create() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			int rc = context.getReturnCode();
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_OK(), rc);

			String message = context.getErrorMessage();
			assertNull(message);
		}
	}
}
