package com.tsurugidb.tsubakuro.rust.odbc.example;

import java.util.Arrays;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;

public class TgOdbcConnectExampleMain extends TgOdbcExample {

    public static void main(String[] args) {
        int exitCode = new TgOdbcConnectExampleMain().main0(args);
        if (exitCode != 0) {
            System.exit(exitCode);
        }
    }

    private String dsn;

    private int main0(String[] args) {
        LOG.info("start. args={}", Arrays.toString(args));

        if (args.length < 1) {
            System.err.println("args: dsn");
            return 1;
        }

        this.dsn = args[0];

        return super.exampleMain();
    }

    @Override
    protected TgOdbcConnection connect(TgOdbcDbcHandle hdbc) {
        LOG.info("DSN={}", dsn);

        return hdbc.connect(dsn, null, null, wideChar);
    }
}
