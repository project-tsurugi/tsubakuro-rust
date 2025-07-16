package com.tsurugidb.tsubakuro.rust.odbc.example;

import java.util.ArrayList;
import java.util.List;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.dbc.InfoType;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcEnvHandle;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcBindParameter;

public abstract class TgOdbcExample {
    protected final Logger LOG = LoggerFactory.getLogger(getClass());

    protected static final boolean wideChar = true;

    protected int exampleMain() {
        try (var manager = new TgOdbcManager()) {
            try (var henv = TgOdbcEnvHandle.allocEnvHandle(manager)) {
                LOG.info("henv={}", henv);
                henv.setEnvAttr(OdbcAttrConst.SQL_ATTR_ODBC_VERSION, OdbcAttrConst.SQL_OV_ODBC3);
                try (var hdbc = TgOdbcDbcHandle.allocDbcHandle(henv)) {
                    LOG.info("hdbc={}", hdbc);

                    try (var connection = connect(hdbc)) {
                        System.out.println("connected. " + connection);

                        info(hdbc);

                        dropAndCreateTable(connection);

                        tables(connection);
                        columns(connection, "test");
                        primaryKeys(connection, "test");

                        select(connection);
                        insert(connection);

                        manualCommit(henv, connection);

                        preparedExecute(connection);
                        preparedQuery(connection);
                    }
                }
            }
        }

        LOG.info("end");
        return 0;
    }

    protected abstract TgOdbcConnection connect(TgOdbcDbcHandle hdbc);

    void info(TgOdbcDbcHandle hdbc) {
        LOG.info("SQLGetInfo() start");

        var list = List.of( //
                InfoType.SQL_DRIVER_NAME, //
                InfoType.SQL_DRIVER_VER, //
                InfoType.SQL_DRIVER_ODBC_VER, //
                InfoType.SQL_DBMS_NAME, //
                InfoType.SQL_DBMS_VER //
        );
        for (var infoType : list) {
            String value = hdbc.getInfoString(infoType, 1024, wideChar);
            System.out.println(value);
        }

        LOG.info("SQLGetInfo() end");
    }

    void dropAndCreateTable(TgOdbcConnection connection) {
        connection.execute("drop table if exists test", wideChar);
        connection.execute("""
                create table test (
                  foo int primary key,
                  bar bigint,
                  zzz varchar(10)
                )
                """, wideChar);
    }

    void tables(TgOdbcConnection connection) {
        LOG.info("SQLTables() start");

        var tableList = doTables(connection);

        for (var table : tableList) {
            System.out.println(table);
        }

        LOG.info("SQLTables() end");
    }

    public record TableName(String databaseName, String schemaName, String tableName, String typeName, String remarks) {
    }

    private List<TableName> doTables(TgOdbcConnection connection) {
        try (var stmt = connection.createStmt()) {
            stmt.tables(wideChar);

            var list = new ArrayList<TableName>();
            while (stmt.fetch()) {
                String databaseName = stmt.getDataString(1);
                String schemaName = stmt.getDataString(2);
                String tableName = stmt.getDataString(3);
                String typeName = stmt.getDataString(4);
                String remarks = stmt.getDataString(5);
                list.add(new TableName(databaseName, schemaName, tableName, typeName, remarks));
            }
            return list;
        }
    }

    void columns(TgOdbcConnection connection, String tableName) {
        LOG.info("SQLColumns() start");

        var columns = doColumns(connection, tableName);

        for (var column : columns) {
            System.out.println(column);
        }

        LOG.info("SQLColumns() end");
    }

    public record ColumnInfo(String databaseName, String schemaName, String tableName, String columnName, String typeName, String remarks) {

    }

    private List<ColumnInfo> doColumns(TgOdbcConnection connection, String tableName) {
        try (var stmt = connection.createStmt()) {
            stmt.columns(tableName, wideChar);

            var list = new ArrayList<ColumnInfo>();
            while (stmt.fetch()) {
                String databaseName = stmt.getDataString(1);
                String schemaName = stmt.getDataString(2);
                String tableName1 = stmt.getDataString(3);
                String colmnName = stmt.getDataString(4);
                String typeName = stmt.getDataString(6);
                String remarks = stmt.getDataString(12);
                list.add(new ColumnInfo(databaseName, schemaName, tableName1, colmnName, typeName, remarks));
            }
            return list;
        }
    }

    void primaryKeys(TgOdbcConnection connection, String tableName) {
        LOG.info("SQLPrimaryKeys() start");

        var keys = doPrimaryKeys(connection, tableName);

        for (var key : keys) {
            System.out.println(key);
        }

        LOG.info("SQLPrimaryKeys() end");
    }

    public record PrimaryKey(String databaseName, String schemaName, String tableName, String columnName, int keySeq, String pkName) {

    }

    private List<PrimaryKey> doPrimaryKeys(TgOdbcConnection connection, String tableName) {
        try (var stmt = connection.createStmt()) {
            stmt.primaryKeys(tableName, wideChar);

            var list = new ArrayList<PrimaryKey>();
            while (stmt.fetch()) {
                String databaseName = stmt.getDataString(1);
                String schemaName = stmt.getDataString(2);
                String tableName1 = stmt.getDataString(3);
                String colmnName = stmt.getDataString(4);
                int keySeq = stmt.getDataInt(5);
                String pkName = stmt.getDataString(6);
                list.add(new PrimaryKey(databaseName, schemaName, tableName1, colmnName, keySeq, pkName));
            }
            return list;
        }
    }

    void select(TgOdbcConnection connection) {
        LOG.info("SQLExecDirect() start");

        try (var stmt = connection.createStmt()) {
            stmt.execDirect("select * from test order by foo", wideChar);

            int columnCount = stmt.numResultCols();
            if (columnCount != 3) {
                System.err.printf("columnCount=%d\n", columnCount);
            }

            for (int i = 1; i <= columnCount; i++) {
                var desc = stmt.describeCol(i, wideChar);
                System.out.println(i + "=" + desc);
            }

            while (stmt.fetch()) {
                int foo = stmt.getDataInt(1);
                Long bar = stmt.getDataLong(2);
                String zzz = stmt.getDataString(3);
                System.out.printf("[%d, %d, %s]\n", foo, bar, zzz);
            }
        }

        LOG.info("SQLExecDirect() end");
    }

    void insert(TgOdbcConnection connection) {
        LOG.info("SQLExecDirect() start");

        try (var stmt = connection.createStmt()) {
            stmt.execDirect("insert or replace into test values(9, 99, 'zzz')", wideChar);

            long rowCount = stmt.rowCount();
            System.out.printf("rowCount=%d\n", rowCount);
        }

        LOG.info("SQLExecDirect() end");
    }

    void manualCommit(TgOdbcEnvHandle env, TgOdbcConnection connection) {
        LOG.info("manualCommit start");

        try (var stmt = connection.createStmt()) {
            stmt.execDirect("delete from test where foo=9", wideChar);
        }
        System.out.println("deleted foo=9");
        assertSelect9(connection, 0);

        try (var dbc2 = env.allocDbcHandle(); //
                var connection2 = dbc2.driverConnect(connection.connectionString(), wideChar)) {
            connection2.setAutoCommit(false, wideChar);

            try (var stmt = connection2.createStmt()) {
                stmt.execDirect("insert into test values(9, 99, 'zzz')", wideChar);
            }
            System.out.println("inserted foo=9");
            assertSelect9(connection, 0); // after insert, before commit

            connection2.commit();
            System.out.println("committed");

            assertSelect9(connection, 1); // after commit
        }

        LOG.info("manualCommit end");
    }

    private void assertSelect9(TgOdbcConnection connection, int expected) {
        try (var stmt = connection.createStmt()) {
            stmt.execDirect("select * from test where foo=9", wideChar);

            int rowCount = 0;
            while (stmt.fetch()) {
                rowCount++;
            }

            if (rowCount == expected) {
                System.out.println("select OK");
            } else {
                String message = "select NG. rowCount=%d, expected=%d".formatted(rowCount, expected);
                System.out.println(message);
                throw new RuntimeException(message);
            }
        }
    }

    void preparedExecute(TgOdbcConnection connection) {
        LOG.info("preparedExecute start");

        try (var stmt = connection.createStmt()) {
            var manager = stmt.manager();

            stmt.prepare("insert into test values(?, ?, ?)", wideChar);
            stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 10));
            stmt.bindParameter(2, TgOdbcBindParameter.ofLong(manager, 100));
            stmt.bindParameter(3, TgOdbcBindParameter.ofStringUtf8(manager, "prepared"));

            stmt.execute();

            long rowCount = stmt.rowCount();
            System.out.printf("rowCount=%d\n", rowCount);
        }

        LOG.info("preparedExecute end");
    }

    void preparedQuery(TgOdbcConnection connection) {
        LOG.info("preparedQuery start");

        try (var stmt = connection.createStmt()) {
            var manager = stmt.manager();

            stmt.prepare("select * from test where foo=?", wideChar);
            stmt.bindParameter(1, TgOdbcBindParameter.ofInt(manager, 10));

            stmt.execute();

            int columnCount = stmt.numResultCols();
            if (columnCount != 3) {
                System.err.printf("columnCount=%d\n", columnCount);
            }
            while (stmt.fetch()) {
                int foo = stmt.getDataInt(1);
                Long bar = stmt.getDataLong(2);
                String zzz = stmt.getDataString(3);
                System.out.printf("[%d, %d, %s]\n", foo, bar, zzz);
            }
        }

        LOG.info("preparedQuery end");
    }
}
