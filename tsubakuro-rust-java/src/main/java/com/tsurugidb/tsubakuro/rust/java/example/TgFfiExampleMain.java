package com.tsurugidb.tsubakuro.rust.java.example;

import java.time.Duration;
import java.util.List;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiJob;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlExecuteResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiTableList;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiTableMetadata;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPreparedStatement;
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

		String endpoint = "tcp://localhost:12345";
		for (var execType : ExecuteType.values()) {
			main(execType, endpoint);
		}
	}

	enum ExecuteType {
		DIRECT, DIRECT_FOR, //
		JOB_TAKE, JOB_TAKE_FOR, JOB_TAKE_IF_READY,
	}

	private static void main(ExecuteType execType, String endpoint) {
		System.out.println(execType + " start");

		try (var manager = TgFfiObjectManager.create(); //
				var context = TgFfiContext.create(manager)) {
			var example = new TgFfiExampleMain(execType, manager, context);
			example.execute(endpoint);
		}

		System.out.println(execType + " end");
	}

	private final ExecuteType executeType;
	private final TgFfiContext context;

	private Duration timeout = Duration.ofSeconds(5);
	private TgFfiSqlClient client;
	private TgFfiCommitOption commitOption;

	TgFfiExampleMain(ExecuteType execType, TgFfiObjectManager manager, TgFfiContext context) {
		this.executeType = execType;
		this.context = context;
	}

	void execute(String endpoint) {
		try (var session = connect(endpoint); //
				var client = session.makeSqlClient(context); //
				var commitOption = TgFfiCommitOption.create(context)) {
			this.client = client;
			this.commitOption = commitOption;

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

			return doConnect(connectionOption);
		}
	}

	private TgFfiSession doConnect(TgFfiConnectionOption connectionOption) {
		switch (executeType) {
		case DIRECT:
			return TgFfiSession.connect(context, connectionOption);
		case DIRECT_FOR:
			return TgFfiSession.connectFor(context, connectionOption, timeout);
		default:
			return take(TgFfiSession.connectAsync(context, connectionOption));
		}
	}

	void executeOcc(String sql) {
		System.out.println("execute(): " + sql);
		try (var transaction = startOcc()) {
			try (var er = doExecute(transaction, sql)) {
			}

			doCommit(transaction);
			doClose(transaction);
		}
	}

	TgFfiTransaction startOcc() {
		try (var transactionOption = TgFfiTransactionOption.create(context)) {
			transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);
			transactionOption.setTransactionLabel(context, "tsubakuro-rust-java.transaction");

			return doStartTransaction(transactionOption);
		}
	}

	private TgFfiTransaction doStartTransaction(TgFfiTransactionOption transactionOption) {
		switch (executeType) {
		case DIRECT:
			return client.startTransaction(context, transactionOption);
		case DIRECT_FOR:
			return client.startTransactionFor(context, transactionOption, timeout);
		default:
			return take(client.startTransactionAsync(context, transactionOption));
		}
	}

	private void doCommit(TgFfiTransaction transaction) {
		switch (executeType) {
		case DIRECT:
			client.commit(context, transaction, commitOption);
			return;
		case DIRECT_FOR:
			client.commitFor(context, transaction, commitOption, timeout);
			return;
		default:
			take(client.commitAsync(context, transaction, commitOption));
			return;
		}
	}

	private void doClose(TgFfiTransaction transaction) {
		switch (executeType) {
		case DIRECT:
		default:
			transaction.close(context);
			return;
		case DIRECT_FOR:
			transaction.closeFor(context, timeout);
			return;
		}
	}

	void listTables() {
		try (var tableList = doListTables()) {
			List<String> tableNames = tableList.getTableNames(context);
			System.out.println("SqlClient.listTables().tableNames=" + tableNames);
		}
	}

	private TgFfiTableList doListTables() {
		switch (executeType) {
		case DIRECT:
			return client.listTables(context);
		case DIRECT_FOR:
			return client.listTablesFor(context, timeout);
		default:
			return take(client.listTablesAsync(context));
		}
	}

	void getTableMetadata() {
		try (var tableMetadata = doGetTableMetadata("test")) {
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

	private TgFfiTableMetadata doGetTableMetadata(String tableName) {
		switch (executeType) {
		case DIRECT:
			return client.getTableMetadata(context, tableName);
		case DIRECT_FOR:
			return client.getTableMetadataFor(context, tableName, timeout);
		default:
			return take(client.getTableMetadataAsync(context, tableName));
		}
	}

	void insert() {
		System.out.println("SqlClient.execute() start");

		try (var transaction = startOcc()) {
			insert(transaction, "insert into test values(1, 11, 'aaa')");
			insert(transaction, "insert into test values(2, 22, 'bbb')");
			insert(transaction, "insert into test values(3, 33, null)");

			doCommit(transaction);
			doClose(transaction);
		}

		System.out.println("SqlClient.execute() end");
	}

	private void insert(TgFfiTransaction transaction, String sql) {
		try (var er = doExecute(transaction, sql)) {
			System.out.println("inserted_rows=" + er.getInsertedRows(context));
		}
	}

	private TgFfiSqlExecuteResult doExecute(TgFfiTransaction transaction, String sql) {
		switch (executeType) {
		case DIRECT:
			return client.execute(context, transaction, sql);
		case DIRECT_FOR:
			return client.executeFor(context, transaction, sql, timeout);
		default:
			return take(client.executeAsync(context, transaction, sql));
		}
	}

	void select() {
		System.out.println("SqlClient.query() start");

		try (var transaction = startOcc()) {
			try (var qr = doQuery(transaction, "select * from test order by foo")) {
				printQueryResult(qr);
			}

			doCommit(transaction);
			doClose(transaction);
		}

		System.out.println("SqlClient.query() end");
	}

	private TgFfiSqlQueryResult doQuery(TgFfiTransaction transaction, String sql) {
		switch (executeType) {
		case DIRECT:
			return client.query(context, transaction, sql);
		case DIRECT_FOR:
			return client.queryFor(context, transaction, sql, timeout);
		default:
			return take(client.queryAsync(context, transaction, sql));
		}
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
				insertPrepared(transaction, ps, 4, 44L, "ddd");
				insertPrepared(transaction, ps, 5, null, "eee");

				doCommit(transaction);
				doClose(transaction);
			}

			ps.close(context);
		}

		for (var placeholder : placeholders) {
			placeholder.close();
		}

		System.out.println("SqlClient.prepared_execute() end");
	}

	private void insertPrepared(TgFfiTransaction transaction, TgFfiSqlPreparedStatement ps, int foo, Long bar,
			String zzz) {
		var parameters = List.of( //
				TgFfiSqlParameter.ofInt4(context, "foo", foo), //
				(bar != null) ? TgFfiSqlParameter.ofInt8(context, "bar", bar)
						: TgFfiSqlParameter.ofNull(context, "bar"), //
				TgFfiSqlParameter.ofCharacter(context, "zzz", zzz) //
		);
		try (var er = doPreparedExecute(transaction, ps, parameters)) {
			System.out.println("inserted_rows=" + er.getInsertedRows(context));
		}

		for (var parameter : parameters) {
			parameter.close();
		}
	}

	private TgFfiSqlExecuteResult doPreparedExecute(TgFfiTransaction transaction, TgFfiSqlPreparedStatement ps,
			List<TgFfiSqlParameter> parameters) {
		switch (executeType) {
		case DIRECT:
			return client.preparedExecute(context, transaction, ps, parameters);
		case DIRECT_FOR:
			return client.preparedExecuteFor(context, transaction, ps, parameters, timeout);
		default:
			return take(client.preparedExecuteAsync(context, transaction, ps, parameters));
		}
	}

	void selectPrepared() {
		System.out.println("SqlClient.prepared_query() start");

		var sql = "select * from test order by foo";
		var placeholders = List.<TgFfiSqlPlaceholder>of();
		try (var ps = client.prepare(context, sql, placeholders)) {
			try (var transaction = startOcc()) {
				var parameters = List.<TgFfiSqlParameter>of();
				try (var qr = doPreparedQuery(transaction, ps, parameters)) {
					printQueryResult(qr);
				}

				for (var parameter : parameters) {
					parameter.close();
				}

				doCommit(transaction);
				doClose(transaction);
			}

			ps.close(context);
		}

		for (var placeholder : placeholders) {
			placeholder.close();
		}

		System.out.println("SqlClient.prepared_query() end");
	}

	private TgFfiSqlQueryResult doPreparedQuery(TgFfiTransaction transaction, TgFfiSqlPreparedStatement ps,
			List<TgFfiSqlParameter> parameters) {
		switch (executeType) {
		case DIRECT:
			return client.preparedQuery(context, transaction, ps, parameters);
		case DIRECT_FOR:
			return client.preparedQueryFor(context, transaction, ps, parameters, timeout);
		default:
			return take(client.preparedQueryAsync(context, transaction, ps, parameters));
		}
	}

	private <T> T take(TgFfiJob<T> job) {
		try (job) {
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
					return value.orElse(null);
				}
				throw new RuntimeException("JOB_TAKE_IF_READY: take_if_ready() fail");
			default:
				throw new UnsupportedOperationException("unsupported executeType " + executeType);
			}
		}
	}
}
