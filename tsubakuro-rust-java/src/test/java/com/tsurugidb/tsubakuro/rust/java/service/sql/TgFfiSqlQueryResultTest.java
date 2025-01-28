package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.Closeable;
import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
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
		final TgFfiSqlClient client;
		final TgFfiTransaction transaction;
		final TgFfiSqlQueryResult queryResult;

		public TestResource() {
			var manager = getFfiObjectManager();
			try (var context = TgFfiContext.create(manager)) {
				this.client = createSqlClient();
				this.transaction = startOcc(client);
				this.queryResult = client.query(context, transaction, "select * from test order by foo");
			}
		}

		@Override
		public void close() {
			try (client; transaction) {
				try (queryResult) {
				}
				commit(client, transaction);
			}
		}
	}

	@Test
	void query() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		try (var transaction = startOcc(client)) {
			var sql = "select * from test order by foo";
			try (var rs = client.query(context, transaction, sql)) {
				assertTrue(rs.nextRow(context));
				assertTrue(rs.nextColumn(context));
				assertFalse(rs.isNull(context));
				assertEquals(1, rs.fetchInt4(context));
				assertTrue(rs.nextColumn(context));
				assertFalse(rs.isNull(context));
				assertEquals(11L, rs.fetchInt8(context));
				assertTrue(rs.nextColumn(context));
				assertFalse(rs.isNull(context));
				assertEquals("aaa", rs.fetchCharacter(context));
				assertFalse(rs.nextColumn(context));

				assertTrue(rs.nextRow(context));
				assertTrue(rs.nextColumn(context));
				assertFalse(rs.isNull(context));
				assertEquals(2, rs.fetchInt4(context));
				assertTrue(rs.nextColumn(context));
				assertFalse(rs.isNull(context));
				assertEquals(22L, rs.fetchInt8(context));
				assertTrue(rs.nextColumn(context));
				assertFalse(rs.isNull(context));
				assertEquals("bbb", rs.fetchCharacter(context));
				assertFalse(rs.nextColumn(context));

				assertTrue(rs.nextRow(context));
				assertTrue(rs.nextColumn(context));
				assertFalse(rs.isNull(context));
				assertEquals(3, rs.fetchInt4(context));
				assertTrue(rs.nextColumn(context));
				assertFalse(rs.isNull(context));
				assertEquals(33L, rs.fetchInt8(context));
				assertTrue(rs.nextColumn(context));
				assertTrue(rs.isNull(context));
				assertFalse(rs.nextColumn(context));

				assertFalse(rs.nextRow(context));
			}

			commit(client, transaction);
		}
	}

	@Test
	void get_metadata_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_metadata(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_get_metadata(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void next_row_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_row(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void next_column_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_next_column(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void is_null_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_is_null(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_is_null(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void fetch_int4_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int4(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int4(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void fetch_int8_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int8(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_int8(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void fetch_float4_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float4(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float4(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void fetch_float8_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float8(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_float8(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void fetch_character_argError() {
		var manager = getFfiObjectManager();
		var context = TgFfiContext.create(manager);

		{
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_character(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var resource = new TestResource()) {
			var ctx = context.handle();
			var handle = resource.queryResult.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_query_result_fetch_character(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}
}
