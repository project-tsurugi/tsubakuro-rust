package com.tsurugidb.tsubakuro.rust.odbc.example;

import java.util.Arrays;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;

public class TgOdbcExampleMain extends TgOdbcExample {

    public static void main(String[] args) {
        int exitCode = new TgOdbcExampleMain().main0(args);
        if (exitCode != 0) {
            System.exit(exitCode);
        }
    }

    private String connectionString;

    private int main0(String[] args) {
        LOG.info("start. args={}", Arrays.toString(args));

        if (args.length < 1) {
            System.err.println("args: endpoint-url");
            return 1;
        }

        String endpoint = args[0];
        this.connectionString = "DRIVER=Tsurugi Driver;Endpoint=" + endpoint;

        return super.exampleMain();
    }

    @Override
    protected TgOdbcConnection connect(TgOdbcDbcHandle hdbc) {
        LOG.info("connectionString={}", connectionString);

        return hdbc.driverConnect(connectionString, wideChar);
    }
}
