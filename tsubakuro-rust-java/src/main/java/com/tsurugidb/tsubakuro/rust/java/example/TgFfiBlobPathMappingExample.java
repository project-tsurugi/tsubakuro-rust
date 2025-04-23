package com.tsurugidb.tsubakuro.rust.java.example;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.Arrays;
import java.util.List;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiAtomType;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlClient;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlExecuteResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.TgFfiSqlQueryResult;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlParameter;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPlaceholder;
import com.tsurugidb.tsubakuro.rust.java.service.sql.prepare.TgFfiSqlPreparedStatement;
import com.tsurugidb.tsubakuro.rust.java.service.sql.type.TgFfiBlobReference;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionType;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiInitializer;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;

// docker run -d -p 12345:12345 --name tsurugi -v D:/tmp/client:/mnt/client -v D:/tmp/tsurugi:/opt/tsurugi/var/data/log ghcr.io/project-tsurugi/tsurugidb:latest
public class TgFfiBlobPathMappingExample {

    public static void main(String[] args) throws Exception {
        TgFfiInitializer.loadFfiLibrary();

        int rc = TgFfiInitializer.initFfiEnvLogger();
        System.out.printf("tsurugi_ffi_env_logger_init() rc=%x%n", rc);

        String endpoint = "tcp://localhost:12345";

        System.out.println("start");
        try (var manager = TgFfiObjectManager.create(); //
                var context = TgFfiContext.create(manager)) {
            var example = new TgFfiBlobPathMappingExample(manager, context);
            example.execute(endpoint);
        }
        System.out.println("end");
    }

    private final TgFfiContext context;

    private Duration timeout = Duration.ofSeconds(5);
    private TgFfiSqlClient client;
    private TgFfiCommitOption commitOption;

    TgFfiBlobPathMappingExample(TgFfiObjectManager manager, TgFfiContext context) {
        this.context = context;
    }

    void execute(String endpoint) {
        try (var session = connect(endpoint); //
                var client = session.makeSqlClient(context); //
                var commitOption = TgFfiCommitOption.create(context)) {
            this.client = client;
            this.commitOption = commitOption;

            {
                var sql = "drop table if exists blob_example";
                executeOcc(sql);
            }
            {
                var sql = """
                        create table blob_example (
                          pk int primary key,
                          value blob
                        )""";
                executeOcc(sql);
            }

            insertPrepared();
            select();
        } finally {
            this.client = null;
        }
    }

    TgFfiSession connect(String endpoint) {
        try (var connectionOption = TgFfiConnectionOption.create(context)) {
            connectionOption.setEndpointUrl(context, endpoint);
            connectionOption.setApplicationName(context, "tsubakuro-rust-java.FfiExample");
            connectionOption.setSessionLabel(context, "TgFfiExampleMain.session");
            connectionOption.setDefaultTimeout(context, timeout);

            connectionOption.addLargeObjectPathMappingOnSend(context, Path.of("D:/tmp/client"), "/mnt/client");
            connectionOption.addLargeObjectPathMappingOnRecv(context, "/opt/tsurugi/var/data/log", Path.of("D:/tmp/tsurugi"));

            return TgFfiSession.connect(context, connectionOption);
        }
    }

    void executeOcc(String sql) {
        System.out.println("execute(): " + sql);
        try (var transaction = startOcc()) {
            try (var er = doExecute(transaction, sql)) {
                long rows = er.getRows(context);
                System.out.println("rows=" + rows);
            }

            doCommit(transaction);
            doClose(transaction);
        }
    }

    private TgFfiSqlExecuteResult doExecute(TgFfiTransaction transaction, String sql) {
        return client.execute(context, transaction, sql);
    }

    private void doCommit(TgFfiTransaction transaction) {
        client.commit(context, transaction, commitOption);
    }

    private void doClose(TgFfiTransaction transaction) {
        transaction.close(context);
    }

    TgFfiTransaction startOcc() {
        try (var transactionOption = TgFfiTransactionOption.create(context)) {
            transactionOption.setTransactionType(context, TgFfiTransactionType.SHORT);
            transactionOption.setTransactionLabel(context, "tsubakuro-rust-java.transaction");

            return client.startTransaction(context, transactionOption);
        }
    }

    void insertPrepared() {
        System.out.println("SqlClient.prepared_execute() start");

        var sql = "insert into blob_example values(:pk, :value)";
        var placeholders = List.of( //
                TgFfiSqlPlaceholder.ofAtomType(context, "pk", TgFfiAtomType.INT4), //
                TgFfiSqlPlaceholder.ofAtomType(context, "value", TgFfiAtomType.BLOB) //
        );
        try (var ps = client.prepare(context, sql, placeholders)) {
            try (var transaction = startOcc()) {
                insertPrepared(transaction, ps, 1, "ABC");

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

    private void insertPrepared(TgFfiTransaction transaction, TgFfiSqlPreparedStatement ps, int pk, String value) {
        var blobPath = Path.of("D:/tmp/client/ffi-blob.dat");
        try {
            Files.writeString(blobPath, value);
        } catch (IOException e) {
            throw new UncheckedIOException(e.getMessage(), e);
        }

        var parameters = List.of( //
                TgFfiSqlParameter.ofInt4(context, "pk", pk), //
                TgFfiSqlParameter.ofBlob(context, "value", blobPath) //
        );
        try (var er = doPreparedExecute(transaction, ps, parameters)) {
            System.out.println("inserted_rows=" + er.getInsertedRows(context));
        }

        for (var parameter : parameters) {
            parameter.close();
        }
    }

    private TgFfiSqlExecuteResult doPreparedExecute(TgFfiTransaction transaction, TgFfiSqlPreparedStatement ps, List<TgFfiSqlParameter> parameters) {
        return client.preparedExecute(context, transaction, ps, parameters);
    }

    void select() {
        System.out.println("SqlClient.query() start");

        try (var transaction = startOcc()) {
            try (var qr = doQuery(transaction, "select * from blob_example order by pk")) {
                printQueryResult(transaction, qr);
            }

            doCommit(transaction);
            doClose(transaction);
        }

        System.out.println("SqlClient.query() end");
    }

    private TgFfiSqlQueryResult doQuery(TgFfiTransaction transaction, String sql) {
        return client.query(context, transaction, sql);
    }

    private void printQueryResult(TgFfiTransaction transaction, TgFfiSqlQueryResult qr) {
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
                        case BLOB -> {
                            TgFfiBlobReference blob = qr.fetchBlob(context);
                            byte[] bytes = client.readBlob(context, transaction, blob);
                            yield Arrays.toString(bytes);
                        }
                        default -> throw new AssertionError("unsupported type " + type);
                        };
                    }

                    System.out.printf("%d.%s=%s%n", row, name, value);
                }
            }
        }
    }
}
