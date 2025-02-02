package com.tsurugidb.tsubakuro.rust.java.example;

import java.time.Duration;
import java.util.List;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiJob;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiVoidJob;
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

		main(ExecuteType.DIRECT, "tcp://localhost:12345");
	}

	enum ExecuteType {
		DIRECT, DIRECT_FOR, //
		JOB_TAKE, JOB_TAKE_FOR, JOB_TAKE_IF_READY,
	}

	private static void main(ExecuteType execType, String endpoint) {
		try (var manager = TgFfiObjectManager.create()) {
			var context = TgFfiContext.create(manager);

			var main = new TgFfiExampleMain(execType, manager, context);
			main.execute(endpoint);
		}
	}

	private final ExecuteType executeType;
	private final TgFfiContext context;
	private Duration timeout;
	private TgFfiSqlClient client = null;

	TgFfiExampleMain(ExecuteType execType, TgFfiObjectManager manager, TgFfiContext context) {
		this.executeType = execType;
		this.context = context;
		this.timeout = Duration.ofSeconds(5);
	}

	void execute(String endpoint) {
		try (var session = connect(endpoint); //
				var client = session.makeSqlClient(context)) {
			this.client = client;

			{
				var sql = "drop table if exists test";
				executeOcc(sql);
			}
			{
				var sql = """
						create table test (
						  foo int primary key,
						  bar bigint,
						  zzz varchar(10)
						)""";
				executeOcc(sql);
			}

			listTables();
			getTableMetadata();

			insert();
			select();

			insertPrepared();
			selectPrepared();
		} finally {
			this.client = null;
		}
	}

	TgFfiSession connect(String endpoint) {
		try (var connectionOption = TgFfiConnectionOption.create(context)) {
			connectionOption.setEndpointUrl(context, endpoint);
			connectionOption.setApplicationName(context, "tsubakuro-rust-java.FfiExample");
			connectionOption.setSessionLabel(context, "TgFfiExampleMain.session");

			switch (executeType) {
			case DIRECT:
				return TgFfiSession.connect(context, connectionOption);
			case DIRECT_FOR:
				return TgFfiSession.connectFor(context, connectionOption, timeout);
			default:
				return take(TgFfiSession.connectAsync(context, connectionOption));
			}
		}
	}

	void executeOcc(String sql) {
		System.out.println("execute(): " + sql);
		try (var transaction = startOcc()) {
			client.execute(context, transaction, sql);

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}
	}

	TgFfiTransaction startOcc() {
		try (var transactionOption = TgFfiTransactionOption.create(context)) {
			transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);
			transactionOption.setTransactionLabel(context, "tsubakuro-rust-java.transaction");

			switch (executeType) {
			case DIRECT:
				return client.startTransaction(context, transactionOption);
			case DIRECT_FOR:
				throw new AssertionError("not yet implemented DIRECT_FOR");
			default:
				return take(client.startTransactionAsync(context, transactionOption));

			}
		}
	}

	void listTables() {
		try (var tableList = client.listTables(context)) {
			List<String> tableNames = tableList.getTableNames(context);
			System.out.println("SqlClient.listTables().tableNames=" + tableNames);
		}
	}

	void getTableMetadata() {
		try (var tableMetadata = client.getTableMetadata(context, "test")) {
			System.out.println("SqlClient.getTableMetadata().tableName=" + tableMetadata.getTableName(context));
			var columns = tableMetadata.getColumns(context);
			for (var column : columns) {
				System.out.printf("%s: %s%n", column.getName(context), column.getAtomType(context));
			}
		} catch (TgFfiRuntimeException e) {
			String name = e.getErrorName();
			if (name.equals("TARGET_NOT_FOUND_EXCEPTION")) {
				System.out.println("getTableMetadata(): " + e.getMessage());
			} else {
				throw e;
			}
		}
	}

	void insert() {
		System.out.println("SqlClient.execute() start");

		try (var transaction = startOcc()) {
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

	void select() {
		System.out.println("SqlClient.query() start");

		try (var transaction = startOcc()) {
			try (var qr = client.query(context, transaction, "select * from test order by foo")) {
				printQueryResult(qr);
			}

			try (var commitOption = TgFfiCommitOption.create(context)) {
				client.commit(context, transaction, commitOption);
			}
			transaction.close(context);
		}

		System.out.println("SqlClient.query() end");
	}

	private void printQueryResult(TgFfiSqlQueryResult qr) {
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

	void insertPrepared() {
		System.out.println("SqlClient.prepared_execute() start");

		var sql = "insert into test values(:foo, :bar, :zzz)";
		var placeholders = List.of( //
				TgFfiSqlPlaceholder.ofAtomType(context, "foo", TgFfiAtomType.INT4), //
				TgFfiSqlPlaceholder.ofAtomType(context, "bar", TgFfiAtomType.INT8), //
				TgFfiSqlPlaceholder.ofAtomType(context, "zzz", TgFfiAtomType.CHARACTER) //
		);
		try (var ps = client.prepare(context, sql, placeholders)) {
			try (var transaction = startOcc()) {
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

	void selectPrepared() {
		System.out.println("SqlClient.prepared_query() start");

		var sql = "select * from test order by foo";
		var placeholders = List.<TgFfiSqlPlaceholder>of();
		try (var ps = client.prepare(context, sql, placeholders)) {
			try (var transaction = startOcc()) {
				var parameters = List.<TgFfiSqlParameter>of();
				try (var qr = client.preparedQuery(context, transaction, ps, parameters)) {
					printQueryResult(qr);
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

	private <T> T take(TgFfiJob<T> job) {
		switch (executeType) {
		case JOB_TAKE:
			return job.take(context);
		case JOB_TAKE_FOR:
			return job.takeFor(context, timeout);
		case JOB_TAKE_IF_READY:
			if (job.wait(context, Duration.ofSeconds(5)) == false) {
				throw new RuntimeException("JOB_TAKE_IF_READY: wait() timeout");
			}
			var value = job.takeIfReady(context);
			if (value != null) {
				return value;
			}
			if (job instanceof TgFfiVoidJob) {
				return null;
			}
			throw new RuntimeException("JOB_TAKE_IF_READY: take_if_ready() fail");
		default:
			throw new UnsupportedOperationException("unsupported executeType " + executeType);
		}
	}
}
