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

        if (args.length < 3) {
            System.err.println("args: endpoint-url user password");
            return 1;
        }

        String endpoint = args[0];
        String user = args[1];
        String password = args[2];
        this.connectionString = "DRIVER=Tsurugi Driver;Endpoint=%s;User=%s;Password=%s;".formatted(endpoint, user, password);

        return super.exampleMain();
    }

    @Override
    protected TgOdbcConnection connect(TgOdbcDbcHandle hdbc) {
        LOG.info("connectionString={}", connectionString);

        return hdbc.driverConnect(connectionString, wideChar);
    }
}
