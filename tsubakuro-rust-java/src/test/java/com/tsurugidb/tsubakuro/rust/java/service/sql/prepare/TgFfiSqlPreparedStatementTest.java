package com.tsurugidb.tsubakuro.rust.java.service.sql.prepare;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.lang.foreign.MemorySegment;
import java.time.Duration;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlPreparedStatementTest extends TgFfiTester {

	@ParameterizedTest
	@ValueSource(strings = { DIRECT, TAKE, TAKE_FOR, TAKE_IF_READY })
	void close(String pattern) {
		close(pattern, true);
		close(pattern, false);
	}

	private void close(String pattern, boolean close) {
		var manager = getFfiObjectManager();
		try (var client = createSqlClient(); //
				var context = TgFfiContext.create(manager); //
				var ps = getSqlPreparedStatement(pattern)) {

			if (close) {
				doClose(ps, pattern);
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

	@Test
	void close_for_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var client = createSqlClient()) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var t = Duration.ofSeconds(5).toNanos();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_close_for(ctx, handle, t);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	private TgFfiSqlPreparedStatement getSqlPreparedStatement(String pattern) {
		var manager = getFfiObjectManager();

		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var session = TgFfiSession.connect(context, connectionOption); //
				var client = session.makeSqlClient(context)) {
			if (pattern.equals(DIRECT)) {
				return client.prepare(context, "select * from test", null);
			} else {
				try (var job = client.prepareAsync(context, "select * from test", null)) {
					return jobTake(job, pattern);
				}
			}
		}
	}

	private void doClose(TgFfiSqlPreparedStatement ps, String pattern) {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			switch (pattern) {
			case DIRECT:
			default:
				ps.close(context);
				break;
			case DIRECT_FOR:
				ps.closeFor(context, Duration.ofSeconds(5));
				break;
			}
		}
	}
}
