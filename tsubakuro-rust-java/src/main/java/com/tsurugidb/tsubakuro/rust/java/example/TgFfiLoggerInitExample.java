package com.tsurugidb.tsubakuro.rust.java.example;

import java.util.List;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiCredential;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiInitializer;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

public class TgFfiLoggerInitExample {

    public static void main(String[] args) throws Exception {
        TgFfiInitializer.loadFfiLibrary();

        int pattern;
        try {
            pattern = Integer.parseInt(args[0]);
        } catch (Exception e) {
            System.err.println("args: logger_init_pattern");
            throw e;
        }

        int rc = initEnvLogger(pattern);
        System.out.printf("env_logger_init rc=%x%n", rc);

        // 2回目以降のLogger初期化の呼び出しは無視される
        TgFfiInitializer.initFfiEnvLogger();
        TgFfiInitializer.initFfiEnvLogger("tsubakuro_rust_core=debug", null);

        String endpoint = "tcp://localhost:12345";
        try (var manager = TgFfiObjectManager.create()) {
            try (var context = TgFfiContext.create(manager)) {
                var example = new TgFfiLoggerInitExample(manager, context);
                example.execute(endpoint);
            }
        }
    }

    private static int initEnvLogger(int pattern) {
        switch (pattern) {
        default:
            return TgFfiInitializer.initFfiEnvLogger();
        case 1:
            return TgFfiInitializer.initFfiEnvLogger(null, null);
        case 2:
            return TgFfiInitializer.initFfiEnvLogger("tsubakuro_rust_ffi=trace", null);
        case 3:
            return TgFfiInitializer.initFfiEnvLogger("tsubakuro_rust_ffi=trace", "/tmp/TgFfiLoggerInitExample.log");
        }
    }

    private final TgFfiContext context;

    private TgFfiSqlClient client;

    TgFfiLoggerInitExample(TgFfiObjectManager manager, TgFfiContext context) {
        this.context = context;
    }

    void execute(String endpoint) {
        try (var session = connect(endpoint); //
                var client = session.makeSqlClient(context); //
                var commitOption = TgFfiCommitOption.create(context)) {
            this.client = client;

            listTables();
        } finally {
            this.client = null;
        }
    }

    TgFfiSession connect(String endpoint) {
        try (var connectionOption = TgFfiConnectionOption.create(context); //
                var credential = TgFfiCredential.fromUserPassword(context, "tsurugi", "password")) {
            connectionOption.setEndpointUrl(context, endpoint);
            connectionOption.setCredential(context, credential);
            connectionOption.setApplicationName(context, "tsubakuro-rust-java.TgFfiLoggerInitExample");
            connectionOption.setSessionLabel(context, "TgFfiLoggerInitExample.session");

            return TgFfiSession.connect(context, connectionOption);
        }
    }

    void listTables() {
        try (var tableList = client.listTables(context)) {
            List<String> tableNames = tableList.getTableNames(context);
            System.out.println("SqlClient.listTables().tableNames=" + tableNames);
        }
    }
}
