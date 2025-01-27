package com.tsurugidb.tsubakuro.rust.java.transaction;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiCommitOptionTest extends TgFfiTester {

	@Test
	void create() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiCommitOption.create(context)) {
		}

		try (var target = TgFfiCommitOption.create(manager)) {
		}
	}

	@Test
	void create_argError() {
		var manager = getFfiObjectManager();

		try (var contextObject = TgFfiContext.create(manager)) {
			var context = contextObject.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_create(context, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void set_commit_type() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiCommitOption.create(context)) {
			assertEquals(TgFfiCommitType.UNSPECIFIED, target.getCommitType(context));

			target.setCommitType(context, TgFfiCommitType.AVAILABLE);

			var type = target.getCommitType(context);
			assertEquals(TgFfiCommitType.AVAILABLE, type);
		}
	}

	@Test
	void set_commit_type_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = TgFfiCommitType.AVAILABLE.value();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_set_commit_type(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiCommitOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var arg = -1;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_set_commit_type(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_commit_type_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_get_commit_type(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = TgFfiCommitOption.create(context)) {
			var ctx = context.handle();
			var handle = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_commit_option_get_commit_type(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}
}
