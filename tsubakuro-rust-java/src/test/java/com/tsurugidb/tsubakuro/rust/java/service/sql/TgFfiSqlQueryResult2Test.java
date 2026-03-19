package com.tsurugidb.tsubakuro.rust.java.service.sql;

import java.util.ArrayList;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.RepeatedTest;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiConnectionOption;
import com.tsurugidb.tsubakuro.rust.java.session.TgFfiSession;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiCommitOption;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransaction;
import com.tsurugidb.tsubakuro.rust.java.transaction.TgFfiTransactionOption;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiObjectManager;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiRuntimeException;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSqlQueryResult2Test extends TgFfiTester {

    @BeforeEach
    void before() {
        dropAndCreateTable("test", """
                create table test (
                  foo bigint primary key generated always as identity,
                  bar int,
                  zzz varchar(10)
                )""");
        executeSql("insert into test(bar, zzz) values(11, 'aaa')");
        executeSql("insert into test(bar, zzz) values(22, 'bbb')");
        executeSql("insert into test(bar, zzz) values(33, null)");
    }

    @RepeatedTest(10)
    void dispose() {
        try (var executor = Executors.newFixedThreadPool(50 + 4)) {
            var threadList = new ArrayList<Future<Void>>();
            for (int i = 0; i < 50; i++) {
                var future = executor.submit(new InsertThread());
                threadList.add(future);
            }
            for (int i = 0; i < 4; i++) {
                var future = executor.submit(new SelectThread());
                threadList.add(future);
            }

            RuntimeException re = null;
            for (var future : threadList) {
                try {
                    future.get();
                } catch (Exception e) {
                    if (re == null) {
                        re = new RuntimeException("TgFfiSqlQueryResult2Test.dispose() error");
                    }
                    re.addSuppressed(e);
                }
            }
            if (re != null) {
                throw re;
            }
        }
    }

    private class InsertThread implements Callable<Void> {
        @Override
        public Void call() throws Exception {
            run();
            return null;
        }

        public void run() {
            try (var manager = TgFfiObjectManager.create(); //
                    var context = TgFfiContext.create(manager); //
                    var connectionOption = TgFfiConnectionOption.create(context)) {
                connectionOption.setEndpointUrl(context, getEndpoint());
//              connectionOption.setCredential(context, getCredential(context));
                try (var session = TgFfiSession.connect(context, connectionOption); //
                        var client = session.makeSqlClient(context); //
                        var commitOption = TgFfiCommitOption.create(context);) {
                    for (int i = 0; i < 100; i++) {
                        try (var tx = createTransaction(client, context)) {
                            try (var er = client.execute(context, tx, "insert into test(bar, zzz) values(0, '')")) {
                                er.getInsertedRows(context);
                            }
                            client.commit(context, tx, commitOption);
                        } catch (TgFfiRuntimeException e) {
                            if (e.getReturnCode() == 0xc0300fa0) { // CC_EXCEPTION
                                i--;
                                continue;
                            }
                            throw e;
                        }
                    }
                }
            }
        }

    }

    private class SelectThread implements Callable<Void> {
        @Override
        public Void call() throws Exception {
            run();
            return null;
        }

        public void run() {
            try (var manager = TgFfiObjectManager.create(); //
                    var context = TgFfiContext.create(manager); //
                    var connectionOption = TgFfiConnectionOption.create(context)) {
                connectionOption.setEndpointUrl(context, getEndpoint());
//              connectionOption.setCredential(context, getCredential(context));
                try (var session = TgFfiSession.connect(context, connectionOption); //
                        var client = session.makeSqlClient(context); //
                        var commitOption = TgFfiCommitOption.create(context);) {
                    for (int i = 0; i < 100; i++) {
                        try (var tx1 = createTransaction(client, context)) {
                            try (var rs = client.query(context, tx1, "select * from test limit 5")) {
                                while (rs.nextRow(context)) {
                                }
                            }
                            client.commit(context, tx1, commitOption);
                        } catch (TgFfiRuntimeException e) {
                            if (e.getReturnCode() == 0xc0300fa0) { // CC_EXCEPTION
                                i--;
                                continue;
                            }
                            throw e;
                        }
                        try (var tx2 = createTransaction(client, context)) {
                            try (var rs = client.query(context, tx2, "select * from test limit 5")) {
                                while (rs.nextRow(context)) {
                                }
                            }
                            client.commit(context, tx2, commitOption);
                        } catch (TgFfiRuntimeException e) {
                            if (e.getReturnCode() == 0xc0300fa0) { // CC_EXCEPTION
                                i--;
                                continue;
                            }
                            throw e;
                        }
                        try (var tx3 = createTransaction(client, context)) {
                            try (var er = client.execute(context, tx3, "insert into test(bar, zzz) values(0, '')")) {
                                er.getInsertedRows(context);
                            }
                            client.commit(context, tx3, commitOption);
                        } catch (TgFfiRuntimeException e) {
                            if (e.getReturnCode() == 0xc0300fa0) { // CC_EXCEPTION
                                i--;
                                continue;
                            }
                            throw e;
                        }
                    }
                }
            }
        }
    }

    private TgFfiTransaction createTransaction(TgFfiSqlClient client, TgFfiContext context) {
        try (var transactionOption = TgFfiTransactionOption.create(context)) {
            return client.startTransaction(context, transactionOption);
        }
    }
}
