package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.Closeable;
import java.lang.foreign.MemorySegment;
import java.time.Duration;
import java.util.List;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPreparedStatement;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlQueryResultTest extends TgFfiTester {

	@BeforeAll
	static void beforeAll() {
		dropAndCreateTable("test", """
				create table test (
				  foo int primary key,
				  bar bigint,
				  zzz varchar(10)
				)""");
		executeSql("insert into test values(1, 11, 'aaa')");
		executeSql("insert into test values(2, 22, 'bbb')");
		executeSql("insert into test values(3, 33, null)");
	}

	private class TestResource implements Closeable {
		final TgFfiContext context;
		final TgFfiSqlClient client;
		final TgFfiSqlPreparedStatement preparedStatement;
		final TgFfiTransaction transaction;
		final TgFfiSqlQueryResult queryResult;

		public TestResource() {
			this(false, DIRECT, "select * from test order by foo");
		}

		public TestResource(boolean prepare, String pattern, String sql) {
			var manager = getFfiObjectManager();
			try (var context = TgFfiContext.create(manager)) {
				this.context = TgFfiContext.create(manager);
				this.client = createSqlClient();
				this.transaction = startOcc(client);

				if (prepare) {
					this.preparedStatement = client.prepare(context, sql, List.of());
					if (pattern.equals(DIRECT)) {
						this.queryResult = client.preparedQuery(context, transaction, preparedStatement, List.of());
					} else {
						try (var job = client.preparedQueryAsync(context, transaction, preparedStatement, List.of())) {
							this.queryResult = jobTake(job, pattern);
						}
					}
				} else {
					this.preparedStatement = null;
					if (pattern.equals(DIRECT)) {
						this.queryResult = client.query(context, transaction, sql);
					} else {
						try (var job = client.queryAsync(context, transaction, sql)) {
							this.queryResult = jobTake(job, pattern);
						}
					}
				}
			}
		}

		@Override
		public void close() {
			try (context; client; preparedStatement; transaction) {
				try (queryResult) {
				}
				commit(client, transaction);
			}
		}
	}

	@Test
	void set_default_timeout() {
		try (var resource = new TestResource()) {
			var context = resource.context;
			var target = resource.queryResult;

			target.setDefaultTimeout(context, Duration.ofSeconds(5));

			var timeout = target.getDefaultTimeout(context);
			assertEquals(Duration.ofSeconds(5), timeout);
		}
	}

	@Test
	void set_default_timeout_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = Duration.ofSeconds(5).toNanos();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_set_default_timeout(ctx, handle, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
	}

	@Test
	void get_default_timeout_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;
			var target = resource.queryResult;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_default_timeout(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = target.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_default_timeout(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@ParameterizedTest
	@ValueSource(strings = { DIRECT, TAKE, TAKE_FOR, TAKE_IF_READY })
	void query(String pattern) {
		query(false, pattern);
	}

	@ParameterizedTest
	@ValueSource(strings = { DIRECT, TAKE, TAKE_FOR, TAKE_IF_READY })
	void query_fromPs(String pattern) {
		query(true, pattern);
	}

	private void query(boolean prepare, String pattern) {
		var sql = "select * from test order by foo";
		try (var resource = new TestResource(prepare, pattern, sql)) {
			var context = resource.context;

			try (var qr = resource.queryResult) {
				assertTrue(qr.nextRow(context));
				assertTrue(qr.nextColumn(context));
				assertFalse(qr.isNull(context));
				assertEquals(1, qr.fetchInt4(context));
				assertTrue(qr.nextColumn(context));
				assertFalse(qr.isNull(context));
				assertEquals(11L, qr.fetchInt8(context));
				assertTrue(qr.nextColumn(context));
				assertFalse(qr.isNull(context));
				assertEquals("aaa", qr.fetchCharacter(context));
				assertFalse(qr.nextColumn(context));

				assertTrue(qr.nextRow(context));
				assertTrue(qr.nextColumn(context));
				assertFalse(qr.isNull(context));
				assertEquals(2, qr.fetchInt4(context));
				assertTrue(qr.nextColumn(context));
				assertFalse(qr.isNull(context));
				assertEquals(22L, qr.fetchInt8(context));
				assertTrue(qr.nextColumn(context));
				assertFalse(qr.isNull(context));
				assertEquals("bbb", qr.fetchCharacter(context));
				assertFalse(qr.nextColumn(context));

				assertTrue(qr.nextRow(context));
				assertTrue(qr.nextColumn(context));
				assertFalse(qr.isNull(context));
				assertEquals(3, qr.fetchInt4(context));
				assertTrue(qr.nextColumn(context));
				assertFalse(qr.isNull(context));
				assertEquals(33L, qr.fetchInt8(context));
				assertTrue(qr.nextColumn(context));
				assertTrue(qr.isNull(context));
				assertFalse(qr.nextColumn(context));

				assertFalse(qr.nextRow(context));
			}
		}
	}

	@Test
	void get_metadata_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_metadata(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_metadata(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@Test
	void next_row_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void next_row_for_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var t = Duration.ofSeconds(5).toNanos();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row_for(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var t = Duration.ofSeconds(5).toNanos();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row_for(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
		}
	}

	@Test
	void next_column_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@Test
	void next_column_for_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var t = Duration.ofSeconds(5).toNanos();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column_for(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var t = Duration.ofSeconds(5).toNanos();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column_for(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
		}
	}

	@Test
	void is_null_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_is_null(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_is_null(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_int4_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int4(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int4(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_for_int4_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var t = Duration.ofSeconds(5).toNanos();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_int4(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var t = Duration.ofSeconds(5).toNanos();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_int4(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_int8_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int8(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int8(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_for_int8_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var t = Duration.ofSeconds(5).toNanos();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_int8(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var t = Duration.ofSeconds(5).toNanos();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_int8(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_float4_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float4(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float4(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_for_float4_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var t = Duration.ofSeconds(5).toNanos();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_float4(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var t = Duration.ofSeconds(5).toNanos();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_float4(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_float8_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float8(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float8(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_for_float8_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var t = Duration.ofSeconds(5).toNanos();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_float8(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var t = Duration.ofSeconds(5).toNanos();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_float8(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_character_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_character(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_character(ctx, handle, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
		}
	}

	@Test
	void fetch_for_character_argError() {
		var manager = getFfiObjectManager();

		try (var resource = new TestResource()) {
			var context = resource.context;

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var t = Duration.ofSeconds(5).toNanos();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_character(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = resource.queryResult.handle();
				var t = Duration.ofSeconds(5).toNanos();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_for_character(ctx, handle, t, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
		}
	}
}
