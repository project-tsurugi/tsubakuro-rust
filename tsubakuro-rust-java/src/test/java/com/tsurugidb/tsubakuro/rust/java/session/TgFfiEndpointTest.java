package com.tsurugidb.tsubakuro.rust.java.session;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiEndpointTest extends TgFfiTester {

	@Test
	void parse() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var endpoint = TgFfiEndpoint.parse(context, "tcp://localhost:12345")) {
		}

		try (var endpoint = TgFfiEndpoint.parse(manager, "tcp://localhost:12345")) {
		}
	}

	@Test
	void parse_argError() {
		var manager = getFfiObjectManager();

		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var endpoint = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_endpoint_parse(context, endpoint, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var endpoint = manager.allocateString("tcp://localhost:12345");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_endpoint_parse(context, endpoint, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}
}
