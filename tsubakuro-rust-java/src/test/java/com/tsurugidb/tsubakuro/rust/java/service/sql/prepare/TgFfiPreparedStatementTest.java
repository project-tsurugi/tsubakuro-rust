package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiPreparedStatementTest extends TgFfiTester {

	@Test
	void close() {
		close(true);
		close(false);
	}

	private void close(boolean close) {
		var manager = getFfiObjectManager();
		try (var client = createSqlClient(); //
				var context = TgFfiContext.create(manager); //
				var ps = client.prepare(context, "select * from test", null)) {

			if (close) {
				ps.close(context);
			}
		}
	}

	@Test
	void close_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var client = createSqlClient()) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_close(ctx, handle);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}
}
