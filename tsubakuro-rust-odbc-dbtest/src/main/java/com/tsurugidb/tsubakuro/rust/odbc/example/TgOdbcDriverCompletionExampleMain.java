package com.tsurugidb.tsubakuro.rust.odbc.example;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.text.MessageFormat;
import java.util.Arrays;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcAttrConst;
import com.tsurugidb.tsubakuro.rust.odbc.dbc.TgOdbcDriverConnectArgument;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcDbcHandle;
import com.tsurugidb.tsubakuro.rust.odbc.handle.TgOdbcEnvHandle;

public class TgOdbcDriverCompletionExampleMain {
    protected final Logger LOG = LoggerFactory.getLogger(getClass());

    protected static final boolean wideChar = true;

    public static void main(String[] args) {
        int exitCode = new TgOdbcDriverCompletionExampleMain().main0(args);
        if (exitCode != 0) {
            System.exit(exitCode);
        }
    }

    private String connectionString;
    private int driverCompletion;

    private int main0(String[] args) {
        LOG.info("start. args={}", Arrays.toString(args));

        if (args.length < 2) {
            System.err.println("args: driverCompletion connection-string-parts");
            return 1;
        }

        this.driverCompletion = Integer.parseInt(args[0].trim());
        this.connectionString = args[1];

        return exampleMain();
    }

    protected int exampleMain() {
        try (var manager = new TgOdbcManager()) {
            try (var henv = TgOdbcEnvHandle.allocEnvHandle(manager)) {
                LOG.info("henv={}", henv);
                henv.setEnvAttr(OdbcAttrConst.SQL_ATTR_ODBC_VERSION, OdbcAttrConst.SQL_OV_ODBC3);
                try (var hdbc = TgOdbcDbcHandle.allocDbcHandle(henv)) {
                    LOG.info("hdbc={}", hdbc);

                    String outConnectionString;
                    try (var connection = connect(hdbc)) {
                        System.out.println("connected. " + connection);
                        outConnectionString = connection.connectionString();
                    }

                    try (var connection = hdbc.driverConnect(outConnectionString, wideChar)) {
                        System.out.println("connected. " + connection);
                    }
                }
            }
        }

        LOG.info("end");
        return 0;
    }

    protected TgOdbcConnection connect(TgOdbcDbcHandle hdbc) {
        LOG.info("connectionString={}", connectionString);
        LOG.info("driverCompletion={}", DriverCompletion.from(driverCompletion));

        try (Arena arena = Arena.ofConfined()) {
            var user32 = SymbolLookup.libraryLookup("user32.dll", arena);
            var descriptor = FunctionDescriptor.of(ValueLayout.ADDRESS);
            MemorySegment getDesktopWindowSymbol = user32.find("GetDesktopWindow").orElseThrow(() -> new RuntimeException("GetDesktopWindow not found"));
            MethodHandle getDesktopWindow = Linker.nativeLinker().downcallHandle(getDesktopWindowSymbol, descriptor);

            var arg = new TgOdbcDriverConnectArgument(hdbc.manager(), wideChar) //
                    .inConnectionString(connectionString) //
                    .bufferLength(1024);
            try {
                var hwnd = (MemorySegment) getDesktopWindow.invoke();
                arg.windowHandle(hwnd);
            } catch (RuntimeException | Error e) {
                throw e;
            } catch (Throwable e) {
                throw new RuntimeException(e);
            }
            arg.driverCompletion(driverCompletion);

            TgOdbcConnection connection = hdbc.driverConnect(arg);
            LOG.info("outConnectionString={}", arg.outConnectionString());
            return connection;
        }
    }

    enum DriverCompletion {
        SQL_DRIVER_NOPROMPT(0), SQL_DRIVER_COMPLETE(1), SQL_DRIVER_PROMPT(2), SQL_DRIVER_COMPLETE_REQUIRED(3),;

        private final short value;

        DriverCompletion(int value) {
            this.value = (short) value;
        }

        public short value() {
            return this.value;
        }

        public static DriverCompletion from(int value) {
            for (var v : values()) {
                if (v.value() == value) {
                    return v;
                }
            }
            throw new IllegalArgumentException(MessageFormat.format("invalid DriverCompletion. value={0}", value));
        }
    }
}
