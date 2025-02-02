package com.tsurugidb.tsubakuro.rust.java.util;

import static org.junit.jupiter.api.Assertions.fail;

import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiJob;
import com.tsurugidb.tsubakuro.rust.java.job.TgFfiVoidJob;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
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
			connectionOption.setSessionLabel(context, "tsubakuro-rust-java/test.session");

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
					transaction.close(context);
				}
			}
		}
	}

	protected TgFfiSession createSession() {
		var manager = getFfiObjectManager();

		try (var context = TgFfiContext.create(manager); //
				var connectionOption = TgFfiConnectionOption.create(context)) {
			connectionOption.setEndpointUrl(context, getEndpoint());

			var session = TgFfiSession.connect(context, connectionOption);
			return session;
		}
	}

	protected TgFfiSqlClient createSqlClient() {
		var manager = getFfiObjectManager();
		var session = createSession();

		try (var context = TgFfiContext.create(manager)) {
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

	public static record Entry(String name, Object value) {
	}

	public static class Row {
		private final List<Entry> values = new ArrayList<>();

		public Row add(String name, Object value) {
			values.add(new Entry(name, value));
			return this;
		}

		@Override
		public int hashCode() {
			return Objects.hash(values);
		}

		@Override
		public boolean equals(Object obj) {
			if (this == obj)
				return true;
			if (obj == null)
				return false;
			if (getClass() != obj.getClass())
				return false;
			Row other = (Row) obj;
			return Objects.equals(values, other.values);
		}
	}

	protected List<Row> select(TgFfiSqlQueryResult qr) {
		var rows = new ArrayList<Row>();

		try (var context = TgFfiContext.create(manager); //
				var metadata = qr.getMetadata(context)) {
			var columns = metadata.getColumns(context);
			while (qr.nextRow(context)) {
				var row = new Row();
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

					row.add(name, value);
				}

				rows.add(row);
			}
		}

		return rows;
	}

	public static final String DIRECT = "DIRECT";
	public static final String DIRECT_FOR = "DIRECT_FOR";
	public static final String TAKE = "TAKE";
	public static final String TAKE_FOR = "TAKE_FOR";
	public static final String TAKE_IF_READY = "TAKE_IF_READY";

	protected <T> T jobTake(TgFfiJob<T> job, String pattern) {
		try (var context = TgFfiContext.create(getFfiObjectManager())) {
			switch (pattern) {
			case TAKE:
				return job.take(context);
			case TAKE_FOR:
				return job.takeFor(context, Duration.ofSeconds(5));
			case TAKE_IF_READY:
				if (job.wait(context, Duration.ofSeconds(5)) == false) {
					fail("TAKE_IF_READY: wait() timeout");
				}
				var value = job.takeIfReady(context);
				if (value != null) {
					return value;
				}
				if (job instanceof TgFfiVoidJob) {
					return null;
				}
				fail("Job.take_if_ready() was not ready");
			default:
				throw new AssertionError("unsupported pattern=" + pattern);
			}
		}
	}

	protected void commitAndClose(TgFfiSqlClient client, TgFfiTransaction transaction, String pattern) {
		var manager = getFfiObjectManager();

		RuntimeException re = null;
		try (var context = TgFfiContext.create(manager)) {
			try (var commitOption = TgFfiCommitOption.create(context)) {
				switch (pattern) {
				case DIRECT:
					client.commit(context, transaction, commitOption);
					break;
				case DIRECT_FOR:
					client.commitFor(context, transaction, commitOption, Duration.ofSeconds(5));
					break;
				default:
					jobTake(client.commitAsync(context, transaction, commitOption), pattern);
					break;
				}
			} catch (RuntimeException e) {
				re = e;
				throw e;
			} finally {
				try {
					switch (pattern) {
					case DIRECT:
					default:
						transaction.close(context);
						break;
					case DIRECT_FOR:
						transaction.closeFor(context, Duration.ofSeconds(5));
						break;
					}
				} catch (RuntimeException e) {
					if (re != null) {
						re.addSuppressed(e);
					} else {
						throw e;
					}
				}
			}
		}
	}
}
