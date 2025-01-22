package com.tsurugidb.tsubakuro.rust.java.context;

import static org.junit.jupiter.api.Assertions.*;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiContextTest extends TgFfiTester {

	@Test
	void create() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			int rc = context.getReturnCode();
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_OK(), rc);

			var type = context.getErrorType();
			assertEquals(TgFfiRcType.OK, type);

			String message = context.getErrorMessage();
			assertNull(message);
		}
	}

	@Test
	void create_argError() {
		var out = MemorySegment.NULL;
		var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_create(out);
		assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
	}

	@Test
	void get_return_code_argError() {
		var manager = getFfiObjectManager();

		{
			var context = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_return_code(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_return_code(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void get_error_type_argError() {
		var manager = getFfiObjectManager();

		{
			var context = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_type(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_type(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void get_error_message_argError() {
		var manager = getFfiObjectManager();

		{
			var context = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_message(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_message(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}
}
