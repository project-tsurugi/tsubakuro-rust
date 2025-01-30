package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertIterableEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.util.List;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlClientTest extends TgFfiTester {

	@BeforeAll
	static void beforeAll() {
		dropAndCreateTable("test", """
				create table test (
				  foo int primary key,
				  bar bigint,
				  zzz varchar(10)
				)""");
	}

	// list_tables() → TgFfiTableListTest

	@Test
	void list_tables_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	// list_tables_async() → TgFfiTableListTest

	@Test
	void list_tables_async_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables_async(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_list_tables_async(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void get_table_metadata_found() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);
		try (var tableMetadata = client.getTableMetadata(context, "test")) {
		}
	}

	@Test
	void get_table_metadata_notFound() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		dropIfExists("test2");

		var context = TgFfiContext.create(manager);
		var e = assertThrows(TgFfiRuntimeException.class, () -> {
			try (var tableMetadata = client.getTableMetadata(context, "test2")) {
			}
		});

		assertEquals("SERVER_ERROR", e.getReturnCodeName());
		String message = e.getMessage();
		assertTrue(message.contains("TARGET_NOT_FOUND_EXCEPTION")); // TODO e.getServerErrorName()
	}

	@Test
	void get_table_metadata_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = manager.allocateString("test");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = manager.allocateString("test");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void get_table_metadata_async_found() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);
		try (var tableMetadataJob = client.getTableMetadataAsync(context, "test"); //
				var tableMetadata = tableMetadataJob.take(context)) {
		}
	}

	@Test
	void get_table_metadata_async_notFound() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		dropIfExists("test2");

		var context = TgFfiContext.create(manager);
		try (var tableMetadataJob = client.getTableMetadataAsync(context, "test2")) {
			var e = assertThrows(TgFfiRuntimeException.class, () -> {
				try (var tableMetadata = tableMetadataJob.take(context)) {
				}
			});

			assertEquals("SERVER_ERROR", e.getReturnCodeName());
			String message = e.getMessage();
			assertTrue(message.contains("TARGET_NOT_FOUND_EXCEPTION")); // TODO e.getServerErrorName()
		}
	}

	@Test
	void get_table_metadata_async_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = manager.allocateString("test");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata_async(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata_async(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = manager.allocateString("test");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_get_table_metadata_async(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void prepare() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var sql = "insert into test values(:foo, :bar, :zzz)";
		var placeholders = List.of( //
				TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4), //
				TgFfiSqlPlaceholder.ofAtomType(context, "bar", TgFfiAtomType.INT8), //
				TgFfiSqlPlaceholder.ofAtomType(context, "zzz", TgFfiAtomType.CHARACTER) //
		);
		try (var ps = client.prepare(context, sql, placeholders)) {
			ps.close(context);
		}
	}

	@Test
	void prepare_empty() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var sql = "select * from test";
		var placeholders = List.<TgFfiSqlPlaceholder>of();
		try (var ps = client.prepare(context, sql, placeholders)) {
			ps.close(context);
		}
	}

	@Test
	void prepare_null() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var sql = "select * from test";
		List<TgFfiSqlPlaceholder> placeholders = null;
		try (var ps = client.prepare(context, sql, placeholders)) {
			ps.close(context);
		}
	}

	@Test
	void prepare_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager)) {
			var sql = "select * from test where foo=:foo";
			var placeholders = List.of(TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4));

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var arg1 = manager.allocateString(sql);
				var arg2 = manager.allocateArray(placeholders);
				int size = placeholders.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = MemorySegment.NULL;
				var arg2 = manager.allocateArray(placeholders);
				int size = placeholders.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = manager.allocateString(sql);
				var arg2 = MemorySegment.NULL;
				int size = placeholders.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = manager.allocateString(sql);
				int size = 1;
				var arg2 = manager.arena().allocate(ValueLayout.ADDRESS, size);
				arg2.setAtIndex(ValueLayout.ADDRESS, 0, MemorySegment.NULL);
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
			{ // size==0のときは、arg2==NULLでもエラーにならない
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = manager.allocateString("select * from test");
				var arg2 = MemorySegment.NULL;
				int size = 0;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_OK(), rc);

				var outHandle = out.get(ValueLayout.ADDRESS, 0);
				tsubakuro_rust_ffi_h.tsurugi_ffi_sql_prepared_statement_dispose(outHandle);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = manager.allocateString(sql);
				var arg2 = manager.allocateArray(placeholders);
				int size = placeholders.size();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), rc);
			}
		}
	}

	@Test
	void prepare_async() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var sql = "insert into test values(:foo, :bar, :zzz)";
		var placeholders = List.of( //
				TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4), //
				TgFfiSqlPlaceholder.ofAtomType(context, "bar", TgFfiAtomType.INT8), //
				TgFfiSqlPlaceholder.ofAtomType(context, "zzz", TgFfiAtomType.CHARACTER) //
		);
		try (var psJob = client.prepareAsync(context, sql, placeholders); //
				var ps = psJob.take(context)) {
			ps.close(context);
		}
	}

	@Test
	void prepare_async_empty() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var sql = "select * from test";
		var placeholders = List.<TgFfiSqlPlaceholder>of();
		try (var psJob = client.prepareAsync(context, sql, placeholders); //
				var ps = psJob.take(context)) {
			ps.close(context);
		}
	}

	@Test
	void prepare_async_null() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var sql = "select * from test";
		List<TgFfiSqlPlaceholder> placeholders = null;
		try (var psJob = client.prepareAsync(context, sql, placeholders); //
				var ps = psJob.take(context)) {
			ps.close(context);
		}
	}

	@Test
	void prepare_async_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager)) {
			var sql = "select * from test where foo=:foo";
			var placeholders = List.of(TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4));

			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var arg1 = manager.allocateString(sql);
				var arg2 = manager.allocateArray(placeholders);
				int size = placeholders.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_async(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = MemorySegment.NULL;
				var arg2 = manager.allocateArray(placeholders);
				int size = placeholders.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_async(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = manager.allocateString(sql);
				var arg2 = MemorySegment.NULL;
				int size = placeholders.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_async(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = manager.allocateString(sql);
				int size = 1;
				var arg2 = manager.arena().allocate(ValueLayout.ADDRESS, size);
				arg2.setAtIndex(ValueLayout.ADDRESS, 0, MemorySegment.NULL);
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_async(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
			{ // size==0のときは、arg2==NULLでもエラーにならない
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = manager.allocateString("select * from test");
				var arg2 = MemorySegment.NULL;
				int size = 0;
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_async(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_OK(), rc);

				var outHandle = out.get(ValueLayout.ADDRESS, 0);
				tsubakuro_rust_ffi_h.tsurugi_ffi_job_dispose(outHandle);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var arg1 = manager.allocateString(sql);
				var arg2 = manager.allocateArray(placeholders);
				int size = placeholders.size();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepare_async(ctx, handle, arg1, arg2, size, out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG5_ERROR(), rc);
			}
		}
	}

	@Test
	void start_transaction() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var transactionOption = TgFfiTransactionOption.create(context);
		transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);

		try (var transaction = client.startTransaction(context, transactionOption)) {
			transaction.close(context);
		}
	}

	@Test
	void start_transaction_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = transactionOption.handle();
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = transactionOption.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void start_transaction_async() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var transactionOption = TgFfiTransactionOption.create(context);
		transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);

		try (var transactionJob = client.startTransactionAsync(context, transactionOption); //
				var transaction = transactionJob.take(context)) {
			transaction.close(context);
		}
	}

	@Test
	void start_transaction_async_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var arg = transactionOption.handle();
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction_async(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction_async(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = transactionOption.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction_async(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void execute() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		try (var transaction = startOcc(client)) {
			var sql = "insert or replace into test values(1, 1, 'a')";
			try (var er = client.execute(context, transaction, sql)) {
				assertEquals(1, er.getRows(context));
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}
		try (var transaction = startOcc(client)) {
			var sql = "delete from test";
			try (var er = client.execute(context, transaction, sql)) {
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}
	}

	@Test
	void execute_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var tx = transaction.handle();
			var arg = manager.allocateString("drop table if exists test");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = MemorySegment.NULL;
			var arg = manager.allocateString("drop table if exists test");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = transaction.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = transaction.handle();
			var arg = manager.allocateString("drop table if exists test");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
		}
	}

	@Test
	void execute_async() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		try (var transaction = startOcc(client)) {
			var sql = "insert or replace into test values(1, 1, 'a')";
			try (var erJob = client.executeAsync(context, transaction, sql); //
					var er = erJob.take(context)) {
				assertEquals(1, er.getRows(context));
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}
		try (var transaction = startOcc(client)) {
			var sql = "delete from test";
			try (var erJob = client.executeAsync(context, transaction, sql); //
					var er = erJob.take(context)) {
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}
	}

	@Test
	void execute_async_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var tx = transaction.handle();
			var arg = manager.allocateString("drop table if exists test");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute_async(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = MemorySegment.NULL;
			var arg = manager.allocateString("drop table if exists test");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute_async(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = transaction.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute_async(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = transaction.handle();
			var arg = manager.allocateString("drop table if exists test");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_execute_async(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
		}
	}

	@Test
	void prepared_execute() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		{
			var sql = "insert into test values(:foo, :bar, :zzz)";
			var placeholders = List.of( //
					TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4), //
					TgFfiSqlPlaceholder.ofAtomType(context, "bar", TgFfiAtomType.INT8), //
					TgFfiSqlPlaceholder.ofAtomType(context, "zzz", TgFfiAtomType.CHARACTER) //
			);
			try (var ps = client.prepare(context, sql, placeholders)) {
				try (var transaction = startOcc(client)) {
					var parameters = List.of( //
							TgFfiSqlParameter.ofInt4(context, "foo", 4), //
							TgFfiSqlParameter.ofInt8(context, "bar", 44), //
							TgFfiSqlParameter.ofCharacter(context, "zzz", "ddd") //
					);
					try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
						assertEquals(1, er.getRows(context));
					}

					try (var commitOption = TgFfiCommitOption.create(context)) {
						client.commit(context, transaction, commitOption);
					}
					transaction.close(context);
				}

				ps.close(context);
			}
		}
		{
			var sql = "delete from test";
			var placeholders = List.<TgFfiSqlPlaceholder>of();
			try (var ps = client.prepare(context, sql, placeholders)) {
				try (var transaction = startOcc(client)) {
					var parameters = List.<TgFfiSqlParameter>of();
					try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
					}

					try (var commitOption = TgFfiCommitOption.create(context)) {
						client.commit(context, transaction, commitOption);
					}
					transaction.close(context);
				}

				ps.close(context);
			}
		}
	}

	@Test
	void prepared_execute_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var sql = "delete from test where foo=:foo";
		var placeholders = List.of(TgFfiSqlPlaceholder.ofAtomType(manager, "foo", TgFfiAtomType.INT4));
		var parameters = List.of(TgFfiSqlParameter.ofInt4(manager, "foo", 9));

		try (var context = TgFfiContext.create(manager); //
				var preparedStatement = client.prepare(context, sql, placeholders); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var tx = transaction.handle();
				var ps = preparedStatement.handle();
				var arg = manager.allocateArray(parameters);
				int size = parameters.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = MemorySegment.NULL;
				var ps = preparedStatement.handle();
				var arg = manager.allocateArray(parameters);
				int size = parameters.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = transaction.handle();
				var ps = MemorySegment.NULL;
				var arg = manager.allocateArray(parameters);
				int size = parameters.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = transaction.handle();
				var ps = preparedStatement.handle();
				var arg = MemorySegment.NULL;
				int size = parameters.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = transaction.handle();
				var ps = preparedStatement.handle();
				int size = 1;
				var arg = manager.arena().allocate(ValueLayout.ADDRESS, size);
				arg.setAtIndex(ValueLayout.ADDRESS, 0, MemorySegment.NULL);
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = transaction.handle();
				var ps = preparedStatement.handle();
				var arg = manager.allocateArray(parameters);
				int size = parameters.size();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_execute(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG6_ERROR(), rc);
			}
		}
	}

	@Test
	void query() {
		executeSql("delete from test");
		executeSql("""
				insert into test values
				(1, 11, 'aaa'),
				(2, 22, null),
				(3, 33, 'ccc')""");

		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		try (var transaction = startOcc(client)) {
			var sql = "select * from test order by foo";
			try (var qr = client.query(context, transaction, sql)) {
				var actual = select(qr);
				var expected = List.of( //
						new Row().add("foo", 1).add("bar", 11L).add("zzz", "aaa"), //
						new Row().add("foo", 2).add("bar", 22L).add("zzz", null), //
						new Row().add("foo", 3).add("bar", 33L).add("zzz", "ccc"));
				assertIterableEquals(expected, actual);
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}
	}

	@Test
	void query_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var tx = transaction.handle();
			var arg = manager.allocateString("select * from test");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_query(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = MemorySegment.NULL;
			var arg = manager.allocateString("select * from test");
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_query(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = transaction.handle();
			var arg = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_query(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = transaction.handle();
			var arg = manager.allocateString("select * from test");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_query(ctx, handle, tx, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
		}
	}

	@Test
	void prepared_query() {
		executeSql("delete from test");
		executeSql("""
				insert into test values
				(1, 11, 'aaa'),
				(2, 22, null),
				(3, 33, 'ccc')""");

		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		{
			var sql = "select * from test where foo=:foo";
			var placeholders = List.of(TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4));
			try (var ps = client.prepare(context, sql, placeholders)) {
				try (var transaction = startOcc(client)) {
					var parameters = List.of(TgFfiSqlParameter.ofInt4(context, "foo", 2));
					try (var qr = client.preparedQuery(context, transaction, ps, parameters)) {
						assertTrue(qr.nextRow(context));
						assertTrue(qr.nextColumn(context));
						assertEquals(2, qr.fetchInt4(context));
						assertTrue(qr.nextColumn(context));
						assertEquals(22L, qr.fetchInt8(context));
						assertTrue(qr.nextColumn(context));
						assertTrue(qr.isNull(context));

						assertFalse(qr.nextColumn(context));
						assertFalse(qr.nextRow(context));
					}

					try (var commitOption = TgFfiCommitOption.create(context)) {
						client.commit(context, transaction, commitOption);
					}
					transaction.close(context);
				}

				ps.close(context);
			}
		}
		{
			var sql = "select * from test order by foo";
			var placeholders = List.of(TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4));
			try (var ps = client.prepare(context, sql, placeholders)) {
				try (var transaction = startOcc(client)) {
					var parameters = List.of(TgFfiSqlParameter.ofInt4(context, "foo", 2));
					try (var qr = client.preparedQuery(context, transaction, ps, parameters)) {
						var actual = select(qr);
						var expected = List.of( //
								new Row().add("foo", 1).add("bar", 11L).add("zzz", "aaa"), //
								new Row().add("foo", 2).add("bar", 22L).add("zzz", null), //
								new Row().add("foo", 3).add("bar", 33L).add("zzz", "ccc"));
						assertIterableEquals(expected, actual);
					}

					try (var commitOption = TgFfiCommitOption.create(context)) {
						client.commit(context, transaction, commitOption);
					}
					transaction.close(context);
				}

				ps.close(context);
			}
		}
	}

	@Test
	void prepared_query_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var sql = "select * from test where foo=:foo";
		var placeholders = List.of(TgFfiSqlPlaceholder.ofAtomType(manager, "foo", TgFfiAtomType.INT4));
		var parameters = List.of(TgFfiSqlParameter.ofInt4(manager, "foo", 9));

		try (var context = TgFfiContext.create(manager); //
				var preparedStatement = client.prepare(context, sql, placeholders); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			{
				var ctx = context.handle();
				var handle = MemorySegment.NULL;
				var tx = transaction.handle();
				var ps = preparedStatement.handle();
				var arg = manager.allocateArray(parameters);
				int size = parameters.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = MemorySegment.NULL;
				var ps = preparedStatement.handle();
				var arg = manager.allocateArray(parameters);
				int size = parameters.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = transaction.handle();
				var ps = MemorySegment.NULL;
				var arg = manager.allocateArray(parameters);
				int size = parameters.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = transaction.handle();
				var ps = preparedStatement.handle();
				var arg = MemorySegment.NULL;
				int size = parameters.size();
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = transaction.handle();
				var ps = preparedStatement.handle();
				int size = 1;
				var arg = manager.arena().allocate(ValueLayout.ADDRESS, size);
				arg.setAtIndex(ValueLayout.ADDRESS, 0, MemorySegment.NULL);
				var out = manager.allocatePtr();
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG4_ERROR(), rc);
			}
			{
				var ctx = context.handle();
				var handle = client.handle();
				var tx = transaction.handle();
				var ps = preparedStatement.handle();
				var arg = manager.allocateArray(parameters);
				int size = parameters.size();
				var out = MemorySegment.NULL;
				var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_prepared_query(ctx, handle, tx, ps, arg, size,
						out);
				assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG6_ERROR(), rc);
			}
		}
	}

	@Test
	void commit() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		try (var transaction = startOcc(client); //
				var commitOption = TgFfiCommitOption.create(context)) {
			client.commit(context, transaction, commitOption);
		}
	}

	@Test
	void commit_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption); //
				var commitOption = TgFfiCommitOption.create(context)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var tx = transaction.handle();
			var arg = commitOption.handle();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_commit(ctx, handle, tx, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption); //
				var commitOption = TgFfiCommitOption.create(context)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = MemorySegment.NULL;
			var arg = commitOption.handle();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_commit(ctx, handle, tx, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption); //
				var commitOption = TgFfiCommitOption.create(context)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = transaction.handle();
			var arg = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_commit(ctx, handle, tx, arg);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void rollback() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		try (var transaction = startOcc(client)) {
			client.rollback(context, transaction);
		}
	}

	@Test
	void rollback_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var tx = transaction.handle();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_rollback(ctx, handle, tx);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context); //
				var transaction = client.startTransaction(context, transactionOption)) {
			var ctx = context.handle();
			var handle = client.handle();
			var tx = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_rollback(ctx, handle, tx);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}
}
