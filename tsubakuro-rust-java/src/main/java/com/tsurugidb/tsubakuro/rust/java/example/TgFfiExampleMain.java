package com.tsurugidb.tsubakuro.rust.java.example;

import java.util.List;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiInitializer;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;

public class TgFfiExampleMain {

	public static void main(String[] args) throws Exception {
		TgFfiInitializer.loadFfiLibrary();

		int rc = TgFfiInitializer.initFfiEnvLogger();
		System.out.printf("tsurugi_ffi_env_logger_init() rc=%x%n", rc);

		try (var manager = TgFfiObjectManager.create()) {
			var context = TgFfiContext.create(manager);

			var connectionOption = TgFfiConnectionOption.create(context);
			connectionOption.setEndpointUrl(context, "tcp://localhost:12345");
			connectionOption.setApplicationName(context, "tsubakuro-rust-java.FfiExample");
			connectionOption.setSessionLabel(context, "TgFfiExampleMain.session");

			try (var session = TgFfiSession.connect(context, connectionOption);
					var client = session.makeSqlClient(context)) {
				{
					var sql = "drop table if exists test";
					executeOcc(client, context, sql);
				}
				{
					var sql = """
							create table test (
							  foo int primary key,
							  bar bigint,
							  zzz varchar(10)
							)""";
					executeOcc(client, context, sql);
				}

				listTables(client, context);
				getTableMetadata(client, context);

				insert(client, context);
				select(client, context);

				insertPrepared(client, context);
				selectPrepared(client, context);
			}
		}
	}

	static void listTables(TgFfiSqlClient client, TgFfiContext context) {
		try (var tableList = client.listTables(context)) {
			List<String> tableNames = tableList.getTableNames(context);
			System.out.println("SqlClient.listTables().tableNames=" + tableNames);
		}
	}

	static void getTableMetadata(TgFfiSqlClient client, TgFfiContext context) {
		try (var tableMetadata = client.getTableMetadata(context, "test")) {
			System.out.println("SqlClient.getTableMetadata().tableName=" + tableMetadata.getTableName(context));
			var columns = tableMetadata.getColumns(context);
			for (var column : columns) {
				System.out.printf("%s: %s%n", column.getName(context), column.getAtomType(context));
			}
		} catch (TgFfiRuntimeException e) {
			String message = e.getMessage();
			if (message.contains("TARGET_NOT_FOUND_EXCEPTION")) {
				System.out.println("getTableMetadata(): " + message);
			} else {
				throw e;
			}
		}
	}

	static TgFfiTransaction startOcc(TgFfiSqlClient client, TgFfiContext context) {
		try (var transactionOption = TgFfiTransactionOption.create(context)) {
			transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);
			transactionOption.setTransactionLabel(context, "tsubakuro-rust-java.transaction");

			return client.startTransaction(context, transactionOption);
		}
	}

	static void executeOcc(TgFfiSqlClient client, TgFfiContext context, String sql) {
		System.out.println("execute(): " + sql);
		try (var transaction = startOcc(client, context)) {
			client.execute(context, transaction, sql);

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}
	}

	static void insert(TgFfiSqlClient client, TgFfiContext context) {
		System.out.println("SqlClient.execute() start");

		try (var transaction = startOcc(client, context)) {
			try (var er = client.execute(context, transaction, "insert into test values(1, 11, 'aaa')")) {
			}
			try (var er = client.execute(context, transaction, "insert into test values(2, 22, 'bbb')")) {
			}
			try (var er = client.execute(context, transaction, "insert into test values(3, 33, null)")) {
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}

		System.out.println("SqlClient.execute() end");
	}

	static void select(TgFfiSqlClient client, TgFfiContext context) {
		System.out.println("SqlClient.query() start");

		try (var transaction = startOcc(client, context)) {
			try (var qr = client.query(context, transaction, "select * from test order by foo")) {
				printQueryResult(context, qr);
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}

		System.out.println("SqlClient.query() end");
	}

	private static void printQueryResult(TgFfiContext context, TgFfiSqlQueryResult qr) {
		try (var metadata = qr.getMetadata(context)) {
			var columns = metadata.getColumns(context);
			for (int row = 0; qr.nextRow(context); row++) {
				for (int i = 0; qr.nextColumn(context); i++) {
					var column = columns.get(i);
					String name = column.getName(context);

					Object value;
					if (qr.isNull(context)) {
						value = null;
					} else {
						var type = column.getAtomType(context);
						value = switch (type) {
						case INT4 -> qr.fetchInt4(context);
						case INT8 -> qr.fetchInt8(context);
						case CHARACTER -> qr.fetchCharacter(context);
						default -> throw new AssertionError("unsupported type " + type);
						};
					}

					System.out.printf("%d.%s=%s%n", row, name, value);
				}
			}
		}
	}

	static void insertPrepared(TgFfiSqlClient client, TgFfiContext context) {
		System.out.println("SqlClient.prepared_execute() start");

		var sql = "insert into test values(:foo, :bar, :zzz)";
		var placeholders = List.of( //
				TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4), //
				TgFfiSqlPlaceholder.ofAtomType(context, "bar", TgFfiAtomType.INT8), //
				TgFfiSqlPlaceholder.ofAtomType(context, "zzz", TgFfiAtomType.CHARACTER) //
		);
		try (var ps = client.prepare(context, sql, placeholders)) {
			try (var transaction = startOcc(client, context)) {
				{
					var parameters = List.of( //
							TgFfiSqlParameter.ofInt4(context, "foo", 4), //
							TgFfiSqlParameter.ofInt8(context, "bar", 44), //
							TgFfiSqlParameter.ofCharacter(context, "zzz", "ddd") //
					);
					try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
					}
				}
				{
					var parameters = List.of( //
							TgFfiSqlParameter.ofInt4(context, "foo", 5), //
							TgFfiSqlParameter.ofNull(context, "bar"), //
							TgFfiSqlParameter.ofCharacter(context, "zzz", "eee") //
					);
					try (var er = client.preparedExecute(context, transaction, ps, parameters)) {
					}
				}

				try (var commitOption = TgFfiCommitOption.create(context)) {
					client.commit(context, transaction, commitOption);
				}
				transaction.close(context);
			}

			ps.close(context);
		}

		System.out.println("SqlClient.prepared_execute() end");
	}

	static void selectPrepared(TgFfiSqlClient client, TgFfiContext context) {
		System.out.println("SqlClient.prepared_query() start");

		var sql = "select * from test order by foo";
		var placeholders = List.<TgFfiSqlPlaceholder>of();
		try (var ps = client.prepare(context, sql, placeholders)) {
			try (var transaction = startOcc(client, context)) {
				var parameters = List.<TgFfiSqlParameter>of();
				try (var qr = client.preparedQuery(context, transaction, ps, parameters)) {
					printQueryResult(context, qr);
				}

				try (var commitOption = TgFfiCommitOption.create(context)) {
					client.commit(context, transaction, commitOption);
				}
				transaction.close(context);
			}

			ps.close(context);
		}

		System.out.println("SqlClient.prepared_query() end");
	}
}
