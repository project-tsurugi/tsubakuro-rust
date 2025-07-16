package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcFunction;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.dbc.ConnectionAttribute;
import com.tsurugidb.tsubakuro.rust.odbc.dbc.InfoType;
import com.tsurugidb.tsubakuro.rust.odbc.dbc.TgOdbcDriverConnectArgument;

public class TgOdbcDbcHandle extends TgOdbcHandle {

    public static TgOdbcDbcHandle allocDbcHandle(TgOdbcEnvHandle henv) {
        return sqlAllocHandle(henv.manager(), HandleType.SQL_HANDLE_DBC, henv, TgOdbcDbcHandle::new);
    }

    public TgOdbcDbcHandle(TgOdbcManager manager, MemorySegment handleAddress) {
        super(manager, handleAddress);
    }

    @Override
    protected HandleType handleType() {
        return HandleType.SQL_HANDLE_DBC;
    }

    public int getInfoInt(InfoType<Integer> infoType, boolean wideChar) {
        return getInfoValue(infoType, infoType.bufferLength(), wideChar);
    }

    public String getInfoString(InfoType<String> infoType, int bufferLength, boolean wideChar) {
        return getInfoValue(infoType, bufferLength, wideChar);
    }

    private <T> T getInfoValue(InfoType<T> infoType, int bufferLength, boolean wideChar) {
        var result = getInfo(infoType, bufferLength, wideChar);
        SqlReturn.check(wideChar ? "SQLGetInfoW" : "SQLGetInfoA", result.rc(), this);

        return result.infoValue();
    }

    public static class GetInfoResult<T> {
        private short rc;
        private InfoType<T> infoType;
        private MemorySegment infoValuePtr;
        private short stringLength;
        private boolean wideChar;

        public GetInfoResult(short rc, InfoType<T> infoType, MemorySegment infoValuePtr, short stringLength, boolean wideChar) {
            this.rc = rc;
            this.infoType = infoType;
            this.infoValuePtr = infoValuePtr;
            this.stringLength = stringLength;
            this.wideChar = wideChar;
        }

        public short rc() {
            return this.rc;
        }

        public T infoValue() {
            return infoType.infoValue(infoValuePtr, stringLength, wideChar);
        }
    }

    public <T> GetInfoResult<T> getInfo(InfoType<T> infoType, int bufferLength, boolean wideChar) {
        MemorySegment connectionHandle = handleAddress();
        short infoTypeValue = infoType.infoType();
        short bufferLengthValue = (short) bufferLength;
        MemorySegment infoValuePtr = manager.allocateBytes(bufferLengthValue);
        MemorySegment stringLengthPtr = manager.allocateShort();

        short rc;
        try {
            if (wideChar) {
                rc = (short) OdbcFunction.sqlGetInfoW.invoke(connectionHandle, infoTypeValue, infoValuePtr, bufferLengthValue, stringLengthPtr);
            } else {
                rc = (short) OdbcFunction.sqlGetInfoA.invoke(connectionHandle, infoTypeValue, infoValuePtr, bufferLengthValue, stringLengthPtr);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }

        short stringLength = stringLengthPtr.get(ValueLayout.JAVA_SHORT, 0);
        return new GetInfoResult<>(rc, infoType, infoValuePtr, stringLength, wideChar);
    }

    public TgOdbcConnection connect(String dsn, boolean wideChar) {
        short rc = connect0(dsn, OdbcConst.SQL_NTS, null, OdbcConst.SQL_NTS, null, OdbcConst.SQL_NTS, wideChar);
        SqlReturn.check(wideChar ? "SQLConnectW" : "SQLConnectA", rc, this);

        return new TgOdbcConnection(this, "DSN=" + dsn);
    }

    public short connect0(String dsn, int dsnLength, String userName, int userNameLength, String authentication, int authenticationLength, boolean wideChar) {
        MemorySegment connectionHandle = handleAddress();
        try {
            if (wideChar) {
                MemorySegment dsnPtr = manager.allocateUtf16(dsn);
                MemorySegment userNamePtr = manager.allocateUtf16(userName);
                MemorySegment authenticationPtr = manager.allocateUtf16(authentication);
                return (short) OdbcFunction.sqlConnectW.invoke(connectionHandle, dsnPtr, (short) dsnLength, userNamePtr, (short) userNameLength, authenticationPtr, (short) authenticationLength);
            } else {
                MemorySegment dsnPtr = manager.allocateUtf8(dsn);
                MemorySegment userNamePtr = manager.allocateUtf8(userName);
                MemorySegment authenticationPtr = manager.allocateUtf8(authentication);
                return (short) OdbcFunction.sqlConnectA.invoke(connectionHandle, dsnPtr, (short) dsnLength, userNamePtr, (short) userNameLength, authenticationPtr, (short) authenticationLength);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public TgOdbcConnection driverConnect(String connectionString, boolean wideChar) {
        var arg = new TgOdbcDriverConnectArgument(manager, wideChar) //
                .inConnectionString(connectionString) //
                .bufferLength(1024);

        short rc = driverConnect0(arg);
        SqlReturn.check(wideChar ? "SQLDriverConnectW" : "SQLDriverConnectA", rc, this);

        return new TgOdbcConnection(this, arg.outConnectionString());
    }

    public short driverConnect0(TgOdbcDriverConnectArgument arg) {
        MemorySegment connectionHandle = handleAddress();
        try {
            if (arg.wideChar()) {
                return (short) OdbcFunction.sqlDriverConnectW.invoke(connectionHandle, arg.windowHandle(), arg.inConnectionStringPtr(), arg.inConnectionStringLength(), arg.outConnectionStringPtr(),
                        arg.bufferLength(), arg.outConnectionStringLengthPtr(), arg.driverCompletion());
            } else {
                return (short) OdbcFunction.sqlDriverConnectA.invoke(connectionHandle, arg.windowHandle(), arg.inConnectionStringPtr(), arg.inConnectionStringLength(), arg.outConnectionStringPtr(),
                        arg.bufferLength(), arg.outConnectionStringLengthPtr(), arg.driverCompletion());
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public void setConnectAttr(ConnectionAttribute attribute, Object value, boolean wideChar) {
        MemorySegment valuePtr;
        int stringLength = 0;
        switch (attribute.type()) {
        case SQLUINTEGER:
            valuePtr = MemorySegment.ofAddress((Integer) value);
            break;
        default:
            throw new UnsupportedOperationException(attribute.name());
        }

        short rc = setConnectAttr0(attribute, valuePtr, stringLength, wideChar);
        SqlReturn.check(wideChar ? "SQLSetConnectAttrW" : "SQLSetConnectAttrA", rc, this);
    }

    private short setConnectAttr0(ConnectionAttribute attribute, MemorySegment valuePtr, int stringLength, boolean wideChar) {
        MemorySegment connectionHandle = handleAddress();
        int attributeValue = attribute.value();
        try {
            if (wideChar) {
                return (short) OdbcFunction.sqlSetConnectAttrW.invoke(connectionHandle, attributeValue, valuePtr, stringLength);
            } else {
                return (short) OdbcFunction.sqlSetConnectAttrA.invoke(connectionHandle, attributeValue, valuePtr, stringLength);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public int getConnectAttrInt(ConnectionAttribute attribute, boolean wideChar) {
        return (int) getConnectAttr(attribute, wideChar);
    }

    private Object getConnectAttr(ConnectionAttribute attribute, boolean wideChar) {
        MemorySegment valuePtr;
        int bufferLength;
        switch (attribute.type()) {
        case SQLUINTEGER:
            valuePtr = manager.allocateInt();
            bufferLength = 4;
            break;
        default:
            throw new UnsupportedOperationException(attribute.type().name());
        }

        MemorySegment stringLengthPtr = manager.allocateInt();

        short rc = getConnectAttr0(attribute, valuePtr, bufferLength, stringLengthPtr, wideChar);
        SqlReturn.check(wideChar ? "SQLGetConnectAttrW" : "SQLGetConnectAttrA", rc, this);

        switch (attribute.type()) {
        case SQLUINTEGER:
            return valuePtr.get(ValueLayout.JAVA_INT, 0);
        default:
            throw new UnsupportedOperationException(attribute.type().name());
        }
    }

    private short getConnectAttr0(ConnectionAttribute attribute, MemorySegment valuePtr, int bufferLength, MemorySegment stringLengthPtr, boolean wideChar) {
        MemorySegment connectionHandle = handleAddress();
        int attributeValue = attribute.value();
        try {
            if (wideChar) {
                return (short) OdbcFunction.sqlGetConnectAttrW.invoke(connectionHandle, attributeValue, valuePtr, bufferLength, stringLengthPtr);
            } else {
                return (short) OdbcFunction.sqlGetConnectAttrW.invoke(connectionHandle, attributeValue, valuePtr, bufferLength, stringLengthPtr);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public void disconnect() {
        MemorySegment connectionHandle = handleAddress();
        try {
            short rc = (short) OdbcFunction.sqlDisconnect.invoke(connectionHandle);
            SqlReturn.check("SQLDisconnect", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }
}
