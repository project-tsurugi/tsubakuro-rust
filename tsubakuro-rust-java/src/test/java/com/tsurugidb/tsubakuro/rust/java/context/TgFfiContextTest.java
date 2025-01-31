package com.tsurugidb.tsubakuro.rust.java.context;

import static org.junit.jupiter.api.Assertions.*;

import java.lang.foreign.MemorySegment;
import java.util.List;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.rc.TgFfiRcType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiContextTest extends TgFfiTester {

	@Test
	void create() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			int rc = context.getReturnCode();
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_OK(), rc);

			String name = context.getErrorName();
			assertEquals("OK", name);

			var type = context.getErrorType();
			assertEquals(TgFfiRcType.OK, type);

			String message = context.getErrorMessage();
			assertNull(message);

			{
				var e = assertThrows(TgFfiRuntimeException.class, () -> {
					context.getServerErrorCategoryNumber();
				});
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND(), e.getReturnCode());
			}
			{
				var e = assertThrows(TgFfiRuntimeException.class, () -> {
					context.getServerErrorCategoryStr();
				});
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND(), e.getReturnCode());
			}
			{
				var e = assertThrows(TgFfiRuntimeException.class, () -> {
					context.getServerErrorCodeNumber();
				});
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND(), e.getReturnCode());
			}
			{
				var e = assertThrows(TgFfiRuntimeException.class, () -> {
					context.getServerErrorStructuredCode();
				});
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_DIAGNOSTIC_CODE_NOT_FOUND(), e.getReturnCode());
			}
		}
	}

	@Test
	void hasError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager)) {
			try (var ps = client.prepare(context, "not sql", List.of())) {
				fail("client.prepare() success");
			} catch (TgFfiRuntimeException ignore) {
				// fall through
			}

			int rc = context.getReturnCode();
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_CORE_SERVER_ERROR() | (3 << 20) | 3001, rc);

			String name = context.getErrorName();
			assertEquals("SYNTAX_EXCEPTION", name);

			var type = context.getErrorType();
			assertEquals(TgFfiRcType.CORE_SERVER_ERROR, type);

			String message = context.getErrorMessage();
			assertNotNull(message);

			assertEquals(3, context.getServerErrorCategoryNumber());
			assertEquals("SQL", context.getServerErrorCategoryStr());
			assertEquals(3001, context.getServerErrorCodeNumber());
			assertEquals("SQL-03001", context.getServerErrorStructuredCode());
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
	void get_error_name_argError() {
		var manager = getFfiObjectManager();

		{
			var context = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_name(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_error_name(context, out);
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

	@Test
	void get_server_error_category_number_argError() {
		var manager = getFfiObjectManager();

		{
			var context = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_category_number(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_category_number(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void get_server_error_category_str_argError() {
		var manager = getFfiObjectManager();

		{
			var context = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_category_str(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_category_str(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void get_server_error_code_number_argError() {
		var manager = getFfiObjectManager();

		{
			var context = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_code_number(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_code_number(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void get_server_error_structured_code_argError() {
		var manager = getFfiObjectManager();

		{
			var context = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_structured_code(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG0_ERROR(), rc);
		}
		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_context_get_server_error_structured_code(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}
}
