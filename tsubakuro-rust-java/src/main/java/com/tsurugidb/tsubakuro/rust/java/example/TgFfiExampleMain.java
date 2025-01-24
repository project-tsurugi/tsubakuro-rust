package com.tsurugidb.tsubakuro.rust.java.example;

import java.util.List;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiInitializer;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiExampleMain {

	public static void main(String[] args) throws Exception {
		TgFfiInitializer.loadFfiLibrary();

		int rc = TgFfiInitializer.initFfiEnvLogger();
		System.out.println("tsurugi_ffi_env_logger_init() rc=" + rc);

		try (var manager = TgFfiObjectManager.create()) {
			var context = TgFfiContext.create(manager);

			var connectionOption = TgFfiConnectionOption.create(context);
			connectionOption.setEndpointUrl(context, "tcp://localhost:12345");
			connectionOption.setApplicationName(context, "tsubakuro-rust-java.FfiExample");
			connectionOption.setLabel(context, "TgFfiExampleMain.session");

			try (var session = TgFfiSession.connect(context, connectionOption);
					var client = session.makeSqlClient(context)) {
				listTables(client, context);
			}
		}
	}

	static void listTables(TgFfiSqlClient client, TgFfiContext context) {
		try (var tableList = client.listTables(context)) {
			List<String> tableNames = tableList.tableNames(context);
			System.out.println("SqlClient.listTables().tableNames=" + tableNames);
		}
	}
}
