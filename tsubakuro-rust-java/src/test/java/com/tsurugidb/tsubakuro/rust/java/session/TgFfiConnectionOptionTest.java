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

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_create(ctx, out);
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

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = TgFfiEndpoint.parse(context, "tcp://localhost:12345").handle();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var arg = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint(ctx, handle, arg);
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

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = manager.allocateString("tcp://localhost:12345");
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var arg = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_endpoint_url(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_endpoint_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_endpoint(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_endpoint(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void set_application_name() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			assertNull(target.getEndpoint(context));

			target.setApplicationName(context, "tsubakuro-rust-java/test");

			String s = target.getApplicationName(context);
			assertEquals("tsubakuro-rust-java/test", s);
		}
	}

	@Test
	void set_application_name_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = manager.allocateString("tsubakuro-rust-java/test");
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_application_name(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var arg = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_application_name(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_application_name_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_application_name(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_application_name(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void set_label() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			assertNull(target.getEndpoint(context));

			target.setSessionLabel(context, "tsubakuro-rust-java/test");

			String s = target.getSessionLabel(context);
			assertEquals("tsubakuro-rust-java/test", s);
		}
	}

	@Test
	void set_session_label_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = manager.allocateString("tsubakuro-rust-java/test");
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_session_label(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var arg = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_set_session_label(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_session_label_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_session_label(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiConnectionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_connection_option_get_session_label(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}
}
