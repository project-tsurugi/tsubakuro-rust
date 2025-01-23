package com.tsurugidb.tsubakuro.rust.java.session;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiConnectionOptionTest extends TgFfiTester {

	@Test
	void create() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
		}

		try (var target = TgFfiConnectionOption.create(manager)) {
		}
	}

	@Test
	void create_argError() {
		var manager = getFfiObjectManager();

		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_create(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void set_endpoint() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			assertNull(target.getEndpoint(context));

			var endpoint = TgFfiEndpoint.parse(context, "tcp://localhost:12345");
			target.setEndpoint(context, endpoint);

			String s = target.getEndpoint(context);
			assertEquals("tcp://localhost:12345", s);
		}
	}

	@Test
	void set_endpoint_argError() {
		var manager = getFfiObjectManager();

		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var target = MemorySegment.NULL;
			var endpoint = TgFfiEndpoint.parse(contextObject, "tcp://localhost:12345").handle();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint(context, target, endpoint);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager); //
				var targetObject = TgFfiConnectionOption.create(contextObject)) {
			var context = contextObject.handle();
			var target = targetObject.handle();
			var endpoint = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint(context, target, endpoint);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void set_endpoint_url() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			assertNull(target.getEndpoint(context));

			target.setEndpointUrl(context, "tcp://localhost:12345");

			String s = target.getEndpoint(context);
			assertEquals("tcp://localhost:12345", s);
		}
	}

	@Test
	void set_endpoint_url_argError() {
		var manager = getFfiObjectManager();

		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var target = MemorySegment.NULL;
			var endpoint = manager.allocateString("tcp://localhost:12345");
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(context, target, endpoint);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager); //
				var targetObject = TgFfiConnectionOption.create(contextObject)) {
			var context = contextObject.handle();
			var target = targetObject.handle();
			var endpoint = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(context, target, endpoint);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_endpoint_argError() {
		var manager = getFfiObjectManager();

		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var target = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_endpoint(context, target, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager); //
				var targetObject = TgFfiConnectionOption.create(contextObject)) {
			var context = contextObject.handle();
			var target = targetObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_endpoint(context, target, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}
}
