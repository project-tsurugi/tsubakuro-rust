package com.tsurugidb.tsubakuro.rust.java.transaction;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.foreign.MemorySegment;
import java.util.List;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTransactionOptionTest extends TgFfiTester {

	@Test
	void create() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
		}

		try (var target = TgFfiTransactionOption.create(manager)) {
		}
	}

	@Test
	void create_argError() {
		var manager = getFfiObjectManager();

		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_create(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void set_transaction_type() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			assertEquals(TgFfiTransactionType.SHORT, target.getTransactionType(context));

			target.setTransactionType(context, TgFfiTransactionType.LONG);

			var type = target.getTransactionType(context);
			assertEquals(TgFfiTransactionType.LONG, type);
		}
	}

	@Test
	void set_transaction_type_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = TgFfiTransactionType.LONG.value();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_type(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var arg = -1;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_type(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_transaction_type_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_type(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_type(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void set_transaction_label() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			assertNull(target.getTransactionLabel(context));

			target.setTransactionLabel(context, "tsubakuro-rust-java/test");

			String s = target.getTransactionLabel(context);
			assertEquals("tsubakuro-rust-java/test", s);
		}
	}

	@Test
	void set_transaction_label_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = manager.allocateString("tsubakuro-rust-java/test");
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_label(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var arg = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_transaction_label(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_transaction_label_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_label(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_transaction_label(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void set_modifies_definitions() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			assertFalse(target.getModifiesDefinitions(context));

			target.setModifiesDefinitions(context, true);

			assertTrue(target.getModifiesDefinitions(context));
		}
	}

	@Test
	void set_modifies_definitions_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = true;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_modifies_definitions(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void get_modifies_definitions_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_modifies_definitions(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_get_modifies_definitions(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void set_write_preserve() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			assertTrue(target.getWritePreserve(context).isEmpty());

			var list = List.of("abc", "def", "ghi");
			target.setWritePreserve(context, list);

			var actual = target.getWritePreserve(context);
			assertEquals(list, actual);
		}
	}

	@Test
	void set_write_preserve_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = manager.allocateStringArray(List.of("a"));
			var size = 1;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_write_preserve(ctx, handle, arg, size);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var arg = MemorySegment.NULL;
			var size = 1;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_option_set_write_preserve(ctx, handle, arg, size);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

}
