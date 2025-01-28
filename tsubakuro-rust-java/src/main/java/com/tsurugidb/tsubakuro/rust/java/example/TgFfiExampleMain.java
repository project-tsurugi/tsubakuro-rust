package com.tsurugidb.tsubakuro.rust.java.example;

import java.util.List;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
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
			connectionOption.setLabel(context, "TgFfiExampleMain.session");

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
		}
	}

	static void insert(TgFfiSqlClient client, TgFfiContext context) {
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
		}
	}

	static void select(TgFfiSqlClient client, TgFfiContext context) {
		try (var transaction = startOcc(client, context)) {
			try (var rs = client.query(context, transaction, "select * from test order by foo"); //
					var metadata = rs.getMetadata(context)) {
				var columns = metadata.getColumns(context);
				for (int row = 0; rs.nextRow(context); row++) {
					for (int i = 0; rs.nextColumn(context); i++) {
						var column = columns.get(i);
						String name = column.getName(context);

						Object value;
						if (rs.isNull(context)) {
							value = null;
						} else {
							var type = column.getAtomType(context);
							value = switch (type) {
							case INT4 -> rs.fetchInt4(context);
							case INT8 -> rs.fetchInt8(context);
							case CHARACTER -> rs.fetchCharacter(context);
							default -> throw new AssertionError("unsupported type " + type);
							};
						}

						System.out.printf("%d.%s=%s%n", row, name, value);
					}
				}
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
		}
	}

	static void insertPrepared(TgFfiSqlClient client, TgFfiContext context) {
		var sql = "insert into test values(:foo, :bar, :zzz)";
		var placeholders = List.of( //
				TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4), //
				TgFfiSqlPlaceholder.ofAtomType(context, "bar", TgFfiAtomType.INT8), //
				TgFfiSqlPlaceholder.ofAtomType(context, "zzz", TgFfiAtomType.CHARACTER) //
		);
		try (var ps = client.prepare(context, sql, placeholders)) {
			// TODO prepared_execute()

			ps.close(context);
		}
	}
}
