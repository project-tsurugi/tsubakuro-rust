package com.tsurugidb.tsubakuro.rust.java.transaction;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiTransactionTest extends TgFfiTester {

	@ParameterizedTest
	@ValueSource(strings = { DIRECT, TAKE, TAKE_FOR, TAKE_IF_READY })
	void get_transaction_id(String pattern) {
		getTransactionId(pattern, true);
		getTransactionId(pattern, false);
	}

	private void getTransactionId(String pattern, boolean close) {
		var manager = getFfiObjectManager();
		try (var context = TgFfiContext.create(manager); //
				var transaction = createTransaction(pattern)) {
			String transactionId = transaction.getTransactionId(context);
			assertTrue(transactionId.startsWith("TID-"));

			if (close) {
				transaction.close(context);
			}
		}
	}

	@Test
	void get_transaction_id_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_get_transaction_id(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var target = createTransaction(DIRECT)) {
			var ctx = context.handle();
			var handle = target.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_get_transaction_id(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void close_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_transaction_close(ctx, handle);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	private TgFfiTransaction createTransaction(String pattern) {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var connectionOption = TgFfiConnectionOption.create(context)) {
			connectionOption.setEndpointUrl(context, getEndpoint());

			var session = TgFfiSession.connect(context, connectionOption);
			var client = session.makeSqlClient(context);

			try (var transactionOption = TgFfiTransactionOption.create(context)) {
				if (pattern.equals(DIRECT)) {
					return client.startTransaction(context, transactionOption);
				} else {
					try (var job = client.startTransactionAsync(context, transactionOption)) {
						return jobTake(job, pattern);
					}
				}
			}
		}
	}
}
