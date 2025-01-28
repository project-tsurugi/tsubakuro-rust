package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlParameterTest extends TgFfiTester {

	@Test
	void of_null() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		try (var placeholder = TgFfiSqlParameter.ofNull(context, "test")) {
			assertEquals("test", placeholder.getName(context));
		}
	}

	@Test
	void of_null_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_null(ctx, arg1, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = manager.allocateString("test");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_null(ctx, arg1, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void of_int4() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		try (var placeholder = TgFfiSqlParameter.ofInt4(context, "test", 123)) {
			assertEquals("test", placeholder.getName(context));
		}
	}

	@Test
	void of_int4_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = MemorySegment.NULL;
			var arg2 = 123;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int4(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = manager.allocateString("test");
			var arg2 = 123;
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int4(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void of_int8() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		try (var placeholder = TgFfiSqlParameter.ofInt8(context, "test", 123)) {
			assertEquals("test", placeholder.getName(context));
		}
	}

	@Test
	void of_int8_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = MemorySegment.NULL;
			var arg2 = 123;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int8(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = manager.allocateString("test");
			var arg2 = 123;
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_int8(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void of_float4() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		try (var placeholder = TgFfiSqlParameter.ofFloat4(context, "test", 123)) {
			assertEquals("test", placeholder.getName(context));
		}
	}

	@Test
	void of_float4_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = MemorySegment.NULL;
			var arg2 = 123;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float4(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = manager.allocateString("test");
			var arg2 = 123;
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float4(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void of_float8() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		try (var placeholder = TgFfiSqlParameter.ofFloat8(context, "test", 123)) {
			assertEquals("test", placeholder.getName(context));
		}
	}

	@Test
	void of_float8_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = MemorySegment.NULL;
			var arg2 = 123;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float8(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = manager.allocateString("test");
			var arg2 = 123;
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_float8(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void of_character() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		try (var placeholder = TgFfiSqlParameter.ofCharacter(context, "test", "abc")) {
			assertEquals("test", placeholder.getName(context));
		}
	}

	@Test
	void of_character_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = MemorySegment.NULL;
			var arg2 = manager.allocateString("abc");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_character(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = manager.allocateString("test");
			var arg2 = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_character(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg1 = manager.allocateString("test");
			var arg2 = manager.allocateString("abc");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_of_character(ctx, arg1, arg2, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void get_name_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_get_name(ctx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiSqlParameter.ofNull(context, "test")) {
			var ctx = context.handle();
			var arg = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_parameter_get_name(ctx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}
}
