package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcConnection;
import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcFunction;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
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

    public int getInfoTypeInt(InfoType<Integer> infoType, boolean wideChar) {
        return getInfoTypeValue(infoType, infoType.bufferLength(), wideChar);
    }

    public String getInfoTypeString(InfoType<String> infoType, int bufferLength, boolean wideChar) {
        return getInfoTypeValue(infoType, bufferLength, wideChar);
    }

    private <T> T getInfoTypeValue(InfoType<T> infoType, int bufferLength, boolean wideChar) {
        var result = getInfoType(infoType, bufferLength, wideChar);
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

    public <T> GetInfoResult<T> getInfoType(InfoType<T> infoType, int bufferLength, boolean wideChar) {
        MemorySegment connectionHandle = handleAddress();
        short infoTypeValue = infoType.infoType();
        short bufferLengthValue = (short) bufferLength;
        MemorySegment infoValuePtr = manager.allocateBytes(bufferLengthValue);
        MemorySegment stringLengthPtr = manager.allocateShort();

        short result;
        try {
            if (wideChar) {
                result = (short) OdbcFunction.sqlGetInfoW.invoke(connectionHandle, infoTypeValue, infoValuePtr, bufferLengthValue, stringLengthPtr);
            } else {
                result = (short) OdbcFunction.sqlGetInfoA.invoke(connectionHandle, infoTypeValue, infoValuePtr, bufferLengthValue, stringLengthPtr);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }

        short stringLength = stringLengthPtr.get(ValueLayout.JAVA_SHORT, 0);
        return new GetInfoResult<>(result, infoType, infoValuePtr, stringLength, wideChar);
    }

    public TgOdbcConnection driverConnect(String connectionString, boolean wideChar) {
        var arg = new TgOdbcDriverConnectArgument(manager, wideChar) //
                .inConnectionString(connectionString) //
                .bufferLength(connectionString.length() * 2);

        short result = driverConnect(arg);
        SqlReturn.check(wideChar ? "SQLDriverConnectW" : "SQLDriverConnectA", result, this);

        return new TgOdbcConnection(this, arg.outConnectionString());
    }

    public short driverConnect(TgOdbcDriverConnectArgument arg) {
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

    public enum ConnectionAttribute {
        SQL_ATTR_AUTOCOMMIT(102), //
        SQL_ATTR_ANSI_APP(115), //

        ;

        private int value;

        ConnectionAttribute(int value) {
            this.value = value;
        }
    }

    public void setConnectAttr(ConnectionAttribute attribute, Object value, boolean wideChar) {
        MemorySegment valuePtr;
        int stringLength = 0;
        switch (attribute) {
        case SQL_ATTR_AUTOCOMMIT:
        case SQL_ATTR_ANSI_APP:
            valuePtr = MemorySegment.ofAddress((Integer) value);
            break;
        default:
            throw new UnsupportedOperationException(attribute.name());
        }
        try {
            if (wideChar) {
                setConnectAttrW(attribute, valuePtr, stringLength);
            } else {
                setConnectAttrA(attribute, valuePtr, stringLength);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private void setConnectAttrA(ConnectionAttribute attribute, MemorySegment valuePtr, int stringLength) throws Throwable {
        MemorySegment connectionHandle = handleAddress();
        int attributeValue = attribute.value;

        short result = (short) OdbcFunction.sqlSetConnectAttrA.invoke(connectionHandle, attributeValue, valuePtr, stringLength);
        SqlReturn.check("SQLSetConnectAttrA", result, this);
    }

    private void setConnectAttrW(ConnectionAttribute attribute, MemorySegment valuePtr, int stringLength) throws Throwable {
        MemorySegment connectionHandle = handleAddress();
        int attributeValue = attribute.value;

        short result = (short) OdbcFunction.sqlSetConnectAttrW.invoke(connectionHandle, attributeValue, valuePtr, stringLength);
        SqlReturn.check("SQLSetConnectAttrW", result, this);
    }

    public Object getConnectAttr(ConnectionAttribute attribute, boolean wideChar) {
        MemorySegment valuePtr;
        int bufferLength;
        switch (attribute) {
        case SQL_ATTR_AUTOCOMMIT:
            valuePtr = manager.allocateInt();
            bufferLength = 4;
            break;
        default:
            throw new UnsupportedOperationException(attribute.name());
        }
        try {
            if (wideChar) {
                return getConnectAttrW(attribute, valuePtr, bufferLength);
            } else {
                return getConnectAttrA(attribute, valuePtr, bufferLength);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private Object getConnectAttrA(ConnectionAttribute attribute, MemorySegment valuePtr, int bufferLength) throws Throwable {
        MemorySegment connectionHandle = handleAddress();
        int attributeValue = attribute.value;
        MemorySegment stringLengthPtr = manager.allocateInt();

        short result = (short) OdbcFunction.sqlGetConnectAttrA.invoke(connectionHandle, attributeValue, valuePtr, bufferLength, stringLengthPtr);
        SqlReturn.check("SQLGetConnectAttrA", result, this);

        return getConnectAttr(attribute, valuePtr, stringLengthPtr, false);
    }

    private Object getConnectAttrW(ConnectionAttribute attribute, MemorySegment valuePtr, int bufferLength) throws Throwable {
        MemorySegment connectionHandle = handleAddress();
        int attributeValue = attribute.value;
        MemorySegment stringLengthPtr = manager.allocateInt();

        short result = (short) OdbcFunction.sqlGetConnectAttrW.invoke(connectionHandle, attributeValue, valuePtr, bufferLength, stringLengthPtr);
        SqlReturn.check("SQLGetConnectAttrW", result, this);

        return getConnectAttr(attribute, valuePtr, stringLengthPtr, true);
    }

    private Object getConnectAttr(ConnectionAttribute attribute, MemorySegment valuePtr, MemorySegment stringLengthPtr, boolean wideChar) {
        switch (attribute) {
        case SQL_ATTR_AUTOCOMMIT:
            return valuePtr.get(ValueLayout.JAVA_INT, 0);
        default:
            throw new UnsupportedOperationException(attribute.name());
        }
    }

    public void disconnect() {
        MemorySegment connectionHandle = handleAddress();
        try {
            short result = (short) OdbcFunction.sqlDisconnect.invoke(connectionHandle);
            SqlReturn.check("SQLDisconnect", result, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }
}
