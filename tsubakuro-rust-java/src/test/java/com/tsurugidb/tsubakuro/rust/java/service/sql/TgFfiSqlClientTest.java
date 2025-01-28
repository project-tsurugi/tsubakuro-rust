package com.tsurugidb.tsubakuro.rust.java.service.sql;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.foreign.MemorySegment;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.ffi.tsubakuro_rust_ffi_h;
import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
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

	@Test
	void make_sql_client() {
		var manager = getFfiObjectManager();

		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var session = TgFfiSession.connect(context, connectionOption); //
				var client = session.makeSqlClient(context)) {
		}
	}

	@Test
	void make_sql_client_argError() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = MemorySegment.NULL;
			var out = manager.allocatePtr();
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_make_sql_client(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG1_ERROR(), rc);
		}
		try (var context = TgFfiContext.create(manager)) {
			var ctx = context.handle();
			var handle = manager.allocateString("tcp://localhost:12345");
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_session_make_sql_client(ctx, handle, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG2_ERROR(), rc);
		}
	}

	@Test
	void list_tables() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		try (var tableList = client.listTables(context)) {
		}
	}

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
	void start_transaction() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		var transactionOption = TgFfiTransactionOption.create(context);
		transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);

		try (var transaction = client.startTransaction(context, transactionOption)) {
		}
	}

	@Test
	void start_transaction_argError() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(manager)) {
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
				var transactionOption = TgFfiTransactionOption.create(manager)) {
			var ctx = context.handle();
			var handle = client.handle();
			var arg = transactionOption.handle();
			var out = MemorySegment.NULL;
			var rc = tsubakuro_rust_ffi_h.tsurugi_ffi_sql_client_start_transaction(ctx, handle, arg, out);
			assertEquals(tsubakuro_rust_ffi_h.TSURUGI_FFI_RC_FFI_ARG3_ERROR(), rc);
		}
	}

	@Test
	void execute() {
		var manager = getFfiObjectManager();
		var client = createSqlClient();

		var context = TgFfiContext.create(manager);

		try (var transaction = startOcc(client)) {
			var sql = "insert into test values(1, 1, 'a')";
			try (var er = client.execute(context, transaction, sql)) {
				assertEquals(1, er.getRows(context));
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
		}
		try (var transaction = startOcc(client)) {
			var sql = "delete from test";
			try (var er = client.execute(context, transaction, sql)) {
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
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
			try (var rs = client.query(context, transaction, sql)) {
				assertTrue(rs.nextRow(context));
			}
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
