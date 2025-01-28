package com.tsurugidb.tsubakuro.rust.java.util;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionType;

public class TgFfiTester {

	static {
		TgFfiInitializer.loadFfiLibrary();
	}

	private static final String SYSPROP_DBTEST_ENDPOINT = "tsurugi.dbtest.endpoint";
	private static String staticEndpoint;

	protected static String getEndpoint() {
		if (staticEndpoint == null) {
			staticEndpoint = System.getProperty(SYSPROP_DBTEST_ENDPOINT, "tcp://localhost:12345");
		}
		return staticEndpoint;
	}

	private TgFfiObjectManager manager;

	@BeforeEach
	void beforeEach() {
		this.manager = TgFfiObjectManager.create();
	}

	protected TgFfiObjectManager getFfiObjectManager() {
		return this.manager;
	}

	@AfterEach
	void afterEach() {
		try (var _ = this.manager) {
		} finally {
			this.manager = null;
		}
	}

	protected static void dropAndCreateTable(String tableName, String createSql) {
		dropIfExists(tableName);
		executeSql(createSql);
	}

	protected static void dropIfExists(String tableName) {
		executeSql("drop table if exists " + tableName);
	}

	protected static void executeSql(String sql) {
		try (var manager = TgFfiObjectManager.create(); //
				var context = TgFfiContext.create(manager); //
				var connectionOption = TgFfiConnectionOption.create(context)) {
			connectionOption.setEndpointUrl(context, getEndpoint());
			connectionOption.setApplicationName(context, "tsubakuro-rust-java/test");
			connectionOption.setLabel(context, "tsubakuro-rust-java/test.session");

			try (var session = TgFfiSession.connect(context, connectionOption); //
					var client = session.makeSqlClient(context); //
					var transactionOption = TgFfiTransactionOption.create(context)) {
				transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);
				transactionOption.setTransactionLabel(context, "tsubakuro-rust-java/execute()");

				try (var transaction = client.startTransaction(context, transactionOption)) {
					try (var executeResult = client.execute(context, transaction, sql)) {
					}
					try (var commitOption = TgFfiCommitOption.create(context)) {
						client.commit(context, transaction, commitOption);
					}
				}
			}
		}
	}

	protected TgFfiSqlClient createSqlClient() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var connectionOption = TgFfiConnectionOption.create(context)) {
			connectionOption.setEndpointUrl(context, getEndpoint());

			var session = TgFfiSession.connect(context, connectionOption);
			var client = session.makeSqlClient(context);
			return client;
		}
	}

	protected TgFfiTransaction startOcc(TgFfiSqlClient client) {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var transactionOption = TgFfiTransactionOption.create(context)) {
			transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);

			var transaction = client.startTransaction(context, transactionOption);
			return transaction;
		}
	}

	protected void commit(TgFfiSqlClient client, TgFfiTransaction transaction) {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var commitOption = TgFfiCommitOption.create(context)) {
			client.commit(context, transaction, commitOption);
		}
	}
}
