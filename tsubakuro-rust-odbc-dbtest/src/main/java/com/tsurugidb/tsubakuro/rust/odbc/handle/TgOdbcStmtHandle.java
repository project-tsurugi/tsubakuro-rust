package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcFunction;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.FreeStmtOption;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.StatementAttribute;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcBindParameter;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcColAttributeArgument;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcColAttributeArgument.FieldIdentifier;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcGetDataArgument;

public class TgOdbcStmtHandle extends TgOdbcHandle {

    public static TgOdbcStmtHandle allocStmtHandle(TgOdbcDbcHandle hdbc) {
        return sqlAllocHandle(hdbc.manager(), HandleType.SQL_HANDLE_STMT, hdbc, TgOdbcStmtHandle::new);
    }

    public TgOdbcStmtHandle(TgOdbcManager manager, MemorySegment handleAddress) {
        super(manager, handleAddress);
    }

    @Override
    protected HandleType handleType() {
        return HandleType.SQL_HANDLE_STMT;
    }

    public void getTypeInfo(SqlDataType dataType, boolean wideChar) {
        getTypeInfo(dataType.value(), wideChar);
    }

    public void getTypeInfo(short dataType, boolean wideChar) {
        try {
            if (wideChar) {
                getTypeInfoW(dataType);
            } else {
                getTypeInfoA(dataType);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private void getTypeInfoA(short dataType) throws Throwable {
        MemorySegment statementHandle = handleAddress();

        short rc = (short) OdbcFunction.sqlGetTypeInfoA.invoke(statementHandle, //
                dataType);
        SqlReturn.check("SQLGetTypeInfoW", rc, this);
    }

    private void getTypeInfoW(short dataType) throws Throwable {
        MemorySegment statementHandle = handleAddress();

        short rc = (short) OdbcFunction.sqlGetTypeInfoW.invoke(statementHandle, //
                dataType);
        SqlReturn.check("SQLGetTypeInfoW", rc, this);
    }

    public void setStmtAttr(StatementAttribute attribute, Object value, boolean wideChar) {
        MemorySegment valuePtr;
        int stringLength = 0;
        switch (attribute.type()) {
        case SQLULEN:
            valuePtr = MemorySegment.ofAddress((Long) value);
            break;
        default:
            throw new UnsupportedOperationException(attribute.name());
        }

        short rc = setStmtAttr0(attribute, valuePtr, stringLength, wideChar);
        SqlReturn.check(wideChar ? "SQLSetStmtAttrW" : "SQLSetStmtAttrA", rc, this);
    }

    private short setStmtAttr0(StatementAttribute attribute, MemorySegment valuePtr, int stringLength, boolean wideChar) {
        MemorySegment connectionHandle = handleAddress();
        int attributeValue = attribute.value();
        try {
            if (wideChar) {
                return (short) OdbcFunction.sqlSetStmtAttrW.invoke(connectionHandle, attributeValue, valuePtr, stringLength);
            } else {
                return (short) OdbcFunction.sqlSetStmtAttrA.invoke(connectionHandle, attributeValue, valuePtr, stringLength);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public long getStmtAttrLong(StatementAttribute attribute, boolean wideChar) {
        return (long) getStmtAttr(attribute, wideChar);
    }

    private Object getStmtAttr(StatementAttribute attribute, boolean wideChar) {
        MemorySegment valuePtr;
        int bufferLength;
        switch (attribute.type()) {
        case SQLULEN:
            valuePtr = manager.allocateLong();
            bufferLength = 8;
            break;
        default:
            throw new UnsupportedOperationException(attribute.type().name());
        }

        MemorySegment stringLengthPtr = manager.allocateInt();

        short rc = getStmtAttr0(attribute, valuePtr, bufferLength, stringLengthPtr, wideChar);
        SqlReturn.check(wideChar ? "SQLGetStmtAttrW" : "SQLGetStmtAttrA", rc, this);

        switch (attribute.type()) {
        case SQLULEN:
            return valuePtr.get(ValueLayout.JAVA_LONG, 0);
        default:
            throw new UnsupportedOperationException(attribute.type().name());
        }
    }

    private short getStmtAttr0(StatementAttribute attribute, MemorySegment valuePtr, int bufferLength, MemorySegment stringLengthPtr, boolean wideChar) {
        MemorySegment connectionHandle = handleAddress();
        int attributeValue = attribute.value();
        try {
            if (wideChar) {
                return (short) OdbcFunction.sqlGetStmtAttrW.invoke(connectionHandle, attributeValue, valuePtr, bufferLength, stringLengthPtr);
            } else {
                return (short) OdbcFunction.sqlGetStmtAttrW.invoke(connectionHandle, attributeValue, valuePtr, bufferLength, stringLengthPtr);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public void tables(boolean wideChar) {
        try {
            if (wideChar) {
                tablesW();
            } else {
                tablesA();
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private void tablesA() throws Throwable {
        MemorySegment statementHandle = handleAddress();
//      MemorySegment allStr = manager.allocateUtf8("%");
//      short length = 1;
        MemorySegment allStr = MemorySegment.NULL;
        short length = 0;
        String type = "TABLE";
        MemorySegment tableType = manager.allocateUtf8(type);
        short tableTypeLength = 5;

        short rc = (short) OdbcFunction.sqlTablesA.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                allStr, length, // tableName
                tableType, tableTypeLength);
        SqlReturn.check("SQLTablesA", rc, this);
    }

    private void tablesW() throws Throwable {
        MemorySegment statementHandle = handleAddress();
//      MemorySegment allStr = manager.allocateUtf16("%");
//      short length = 1;
        MemorySegment allStr = MemorySegment.NULL;
        short length = 0;
        String type = "TABLE";
        MemorySegment tableType = manager.allocateUtf16(type);
        short tableTypeLength = 5;

        short rc = (short) OdbcFunction.sqlTablesW.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                allStr, length, // tableName
                tableType, tableTypeLength);
        SqlReturn.check("SQLTablesW", rc, this);
    }

    public void columns(String tableName, boolean wideChar) {
        columns(tableName, OdbcConst.SQL_NTS, wideChar);
    }

    public void columns(String tableName, int tableNameLength, boolean wideChar) {
        try {
            if (wideChar) {
                columnsW(tableName, (short) tableNameLength);
            } else {
                columnsA(tableName, (short) tableNameLength);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private void columnsA(String tableName, short tableNameLength) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment allStr = MemorySegment.NULL;
        short length = 0;
        MemorySegment tableNamePtr = manager.allocateUtf8(tableName);

        short rc = (short) OdbcFunction.sqlColumnsA.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                tableNamePtr, tableNameLength, // tableName
                allStr, length // columnName
        );
        SqlReturn.check("SQLColumnsA", rc, this);
    }

    private void columnsW(String tableName, short tableNameLength) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment allStr = MemorySegment.NULL;
        short length = 0;
        MemorySegment tableNamePtr = manager.allocateUtf16(tableName);

        short rc = (short) OdbcFunction.sqlColumnsW.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                tableNamePtr, tableNameLength, // tableName
                allStr, length // columnName
        );
        SqlReturn.check("SQLColumnsW", rc, this);
    }

    public record PrimaryKey(String databaseName, String schemaName, String tableName, String columnName, int keySeq, String pkName) {

    }

    public void primaryKeys(String tableName, boolean wideChar) {
        primaryKeys(tableName, OdbcConst.SQL_NTS, wideChar);
    }

    public void primaryKeys(String tableName, int tableNameLength, boolean wideChar) {
        try {
            if (wideChar) {
                primaryKeysW(tableName, (short) tableNameLength);
            } else {
                primaryKeysA(tableName, (short) tableNameLength);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private void primaryKeysA(String tableName, short tableNameLength) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment allStr = MemorySegment.NULL;
        short length = 0;
        MemorySegment tableNamePtr = manager.allocateUtf8(tableName);

        short rc = (short) OdbcFunction.sqlPrimaryKeysA.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                tableNamePtr, tableNameLength // tableName
        );
        SqlReturn.check("SQLPrimaryKeysA", rc, this);
    }

    private void primaryKeysW(String tableName, short tableNameLength) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment allStr = MemorySegment.NULL;
        short length = 0;
        MemorySegment tableNamePtr = manager.allocateUtf16(tableName);

        short rc = (short) OdbcFunction.sqlPrimaryKeysW.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                tableNamePtr, tableNameLength // tableName
        );
        SqlReturn.check("SQLPrimaryKeysW", rc, this);
    }

    public void execDirect(String sql, boolean wideChar) {
        short rc = execDirect0(sql, OdbcConst.SQL_NTS, wideChar);
        SqlReturn.check(wideChar ? "SQLExecDirectW" : "SQLExecDirectA", rc, this);
    }

    public short execDirect0(String sql, int length, boolean wideChar) {
        MemorySegment statementHandle = handleAddress();
        short statementLength = (short) length;
        try {
            if (wideChar) {
                MemorySegment statementPtr = manager.allocateUtf16(sql);
                return (short) OdbcFunction.sqlExecDirectW.invoke(statementHandle, statementPtr, statementLength);
            } else {
                MemorySegment statementPtr = manager.allocateUtf8(sql);
                return (short) OdbcFunction.sqlExecDirectA.invoke(statementHandle, statementPtr, statementLength);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public void prepare(String sql, boolean wideChar) {
        try {
            if (wideChar) {
                prepareW(sql);
            } else {
                prepareA(sql);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private void prepareA(String sql) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment statementPtr = manager.allocateUtf8(sql);
        short statementLength = OdbcConst.SQL_NTS;

        short rc = (short) OdbcFunction.sqlPrepareA.invoke(statementHandle, statementPtr, statementLength);
        SqlReturn.check("SQLPrepareA", rc, this);
    }

    private void prepareW(String sql) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment statementPtr = manager.allocateUtf16(sql);
        short statementLength = OdbcConst.SQL_NTS;

        short rc = (short) OdbcFunction.sqlPrepareW.invoke(statementHandle, statementPtr, statementLength);
        SqlReturn.check("SQLPrepareW", rc, this);
    }

    public void bindParameter(int parameterNumber, TgOdbcBindParameter parameter) {
        MemorySegment statementHandle = handleAddress();
        short parameterNumberValue = (short) parameterNumber;
        short inputOutputType = OdbcConst.SQL_PARAM_INPUT;
        short valueType = parameter.valueType().value();
        short parameterType = parameter.parameterType().value();
        long columnSize = parameter.columnSize();
        short decimalDigits = parameter.decimalDigits();
        MemorySegment parameterValuePtr = parameter.parameterValuePtr();
        long bufferLength = 0;
        MemorySegment strLenOrIndPtr = manager.allocateLong(parameter.lengthOrInd());
        try {
            short rc = (short) OdbcFunction.sqlBindParameter.invoke(statementHandle, parameterNumberValue, inputOutputType, valueType, parameterType, columnSize, decimalDigits, parameterValuePtr,
                    bufferLength, strLenOrIndPtr);
            SqlReturn.check("SQLBindParameter", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public void execute() {
        MemorySegment statementHandle = handleAddress();
        try {
            short rc = (short) OdbcFunction.sqlExecute.invoke(statementHandle);
            SqlReturn.check("SQLExecute", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    // true: has row
    public boolean fetch() {
        MemorySegment statementHandle = handleAddress();
        try {
            short rc = (short) OdbcFunction.sqlFetch.invoke(statementHandle);
            switch (rc) {
            case SqlReturn.SQL_SUCCESS:
            case SqlReturn.SQL_SUCCESS_WITH_INFO:
                return true;
            case SqlReturn.SQL_NO_DATA:
                return false;
            default:
                SqlReturn.check("SQLFetch", rc, this);
                return false;
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public int numResultCols() {
        MemorySegment statementHandle = handleAddress();
        MemorySegment countPtr = manager.allocateShort();
        try {
            short rc = (short) OdbcFunction.sqlNumResultCols.invoke(statementHandle, countPtr);
            SqlReturn.check("SQLNumResultCols", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }

        return countPtr.get(ValueLayout.JAVA_SHORT, 0);
    }

    public DescribeColumn describeCol(int columnNumber, boolean wideChar) {
        try {
            if (wideChar) {
                return describeColW(columnNumber);
            } else {
                return describeColA(columnNumber);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public record DescribeColumn(String columnName, SqlDataType dataType, int columnSize, int decimalDigits, int nullabple) {
    }

    private DescribeColumn describeColA(int columnNumber) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        short bufferLength = 1024;
        MemorySegment columnNamePtr = manager.allocateBytes(bufferLength);
        MemorySegment nameLengthPtr = manager.allocateShort();
        MemorySegment dataTypePtr = manager.allocateShort();
        MemorySegment columnSizePtr = manager.allocateShort();
        MemorySegment decimalDigitsPtr = manager.allocateShort();
        MemorySegment nullablePtr = manager.allocateShort();

        short rc = (short) OdbcFunction.sqlDescribeColA.invoke(statementHandle, (short) columnNumber, columnNamePtr, bufferLength, nameLengthPtr, dataTypePtr, columnSizePtr, decimalDigitsPtr,
                nullablePtr);
        SqlReturn.check("SQLDescribeColA", rc, this);

        String columnName = TgOdbcManager.stringFromUtf8(columnNamePtr, nameLengthPtr);
        var dataType = SqlDataType.fromValue(dataTypePtr.get(ValueLayout.JAVA_SHORT, 0));
        short columnSize = columnSizePtr.get(ValueLayout.JAVA_SHORT, 0);
        short decimalDigits = decimalDigitsPtr.get(ValueLayout.JAVA_SHORT, 0);
        short nullable = nullablePtr.get(ValueLayout.JAVA_SHORT, 0);
        return new DescribeColumn(columnName, dataType, columnSize, decimalDigits, nullable);
    }

    private DescribeColumn describeColW(int columnNumber) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        short bufferLength = 1024;
        MemorySegment columnNamePtr = manager.allocateBytes(bufferLength * 2);
        MemorySegment nameLengthPtr = manager.allocateShort();
        MemorySegment dataTypePtr = manager.allocateShort();
        MemorySegment columnSizePtr = manager.allocateShort();
        MemorySegment decimalDigitsPtr = manager.allocateShort();
        MemorySegment nullablePtr = manager.allocateShort();

        short rc = (short) OdbcFunction.sqlDescribeColW.invoke(statementHandle, (short) columnNumber, columnNamePtr, bufferLength, nameLengthPtr, dataTypePtr, columnSizePtr, decimalDigitsPtr,
                nullablePtr);
        SqlReturn.check("SQLDescribeColW", rc, this);

        String columnName = TgOdbcManager.stringFromUtf16(columnNamePtr, nameLengthPtr);
        var dataType = SqlDataType.fromValue(dataTypePtr.get(ValueLayout.JAVA_SHORT, 0));
        short columnSize = columnSizePtr.get(ValueLayout.JAVA_SHORT, 0);
        short decimalDigits = decimalDigitsPtr.get(ValueLayout.JAVA_SHORT, 0);
        short nullable = nullablePtr.get(ValueLayout.JAVA_SHORT, 0);
        return new DescribeColumn(columnName, dataType, columnSize, decimalDigits, nullable);
    }

    public String colAttributeString(int columnNumber, FieldIdentifier fieldIdentifier, boolean wideChar) {
        var arg = TgOdbcColAttributeArgument.ofString(manager, fieldIdentifier, 1024);

        colAttribute(columnNumber, arg, wideChar);

        return arg.characterAttribute(wideChar);
    }

    public long colAttributeNumeric(int columnNumber, FieldIdentifier fieldIdentifier, boolean wideChar) {
        var arg = TgOdbcColAttributeArgument.ofNumeric(manager, fieldIdentifier);

        colAttribute(columnNumber, arg, wideChar);

        return arg.numericAttribute();
    }

    private void colAttribute(int columnNumber, TgOdbcColAttributeArgument arg, boolean wideChar) {
        MemorySegment statementHandle = handleAddress();

        try {
            if (wideChar) {
                short rc = (short) OdbcFunction.sqlColAttributeW.invoke(statementHandle, (short) columnNumber, arg.fieldIdentifier().value(), arg.characterAttributePtr(), arg.bufferLength(),
                        arg.stringLengthPtr(), arg.numericAttributePtr());
                SqlReturn.check("SQLColAttributeW", rc, this);
            } else {
                short rc = (short) OdbcFunction.sqlColAttributeA.invoke(statementHandle, (short) columnNumber, arg.fieldIdentifier().value(), arg.characterAttributePtr(), arg.bufferLength(),
                        arg.stringLengthPtr(), arg.numericAttributePtr());
                SqlReturn.check("SQLColAttributeA", rc, this);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public Short getDataShort(int columnNumber) {
        return getData(columnNumber, TgOdbcGetDataArgument.ofShort(manager));
    }

    public Integer getDataInt(int columnNumber) {
        return getData(columnNumber, TgOdbcGetDataArgument.ofInt(manager));
    }

    public Long getDataLong(int columnNumber) {
        return getData(columnNumber, TgOdbcGetDataArgument.ofLong(manager));
    }

    public String getDataString(int columnNumber) {
        return getDataString(columnNumber, 1024, false);
    }

    public String getDataString(int columnNumber, boolean wideChar) {
        return getDataString(columnNumber, 1024, wideChar);
    }

    public String getDataString(int columnNumber, int bufferSize, boolean wideChar) {
        var arg = TgOdbcGetDataArgument.ofString(manager, bufferSize, wideChar);
        String value = getData(columnNumber, arg);
        long length = arg.lengthOrInd();
        if (length > bufferSize) {
            return getDataString(columnNumber, (int) length + 1, wideChar);
        }
        return value;
    }

    /**
     * SQLGetData.
     * 
     * @param columnNumber 1 origin
     * @return data (nullable)
     */
    public <T> T getData(int columnNumber, TgOdbcGetDataArgument<T> arg) {
        short rc = getData0(columnNumber, arg);
        SqlReturn.check("SQLGetData", rc, this);

        if (arg.isDataNull()) {
            return null;
        }
        return arg.getData();
    }

    public <T> short getData0(int columnNumber, TgOdbcGetDataArgument<T> arg) {
        MemorySegment statementHandle = handleAddress();
        short columnNumberValue = (short) columnNumber;
        short targetTypeValue = arg.targetType().value();
        MemorySegment valuePtr = arg.valuePtr();
        long bufferSize = arg.bufferSize();
        MemorySegment lengthOrIndPtr = arg.lengthOrIndPtr();
        try {
            return (short) OdbcFunction.sqlGetData.invoke(statementHandle, columnNumberValue, targetTypeValue, valuePtr, bufferSize, lengthOrIndPtr);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    /**
     * SQLBindCol.
     *
     * @param columnNumber 1 origin
     */
    public <T> void bindCol(int columnNumber, TgOdbcGetDataArgument<T> arg) {
        MemorySegment statementHandle = handleAddress();
        short columnNumberValue = (short) columnNumber;
        short targetTypeValue = arg.targetType().value();
        MemorySegment valuePtr = arg.valuePtr();
        long bufferSize = arg.bufferSize();
        MemorySegment lengthOrIndPtr = arg.lengthOrIndPtr();
        try {
            short rc = (short) OdbcFunction.sqlBindCol.invoke(statementHandle, columnNumberValue, targetTypeValue, valuePtr, bufferSize, lengthOrIndPtr);
            SqlReturn.check("SQLBindCol", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public long rowCount() {
        MemorySegment statementHandle = handleAddress();
        MemorySegment rowCountPtr = manager.allocateLong();
        try {
            short rc = (short) OdbcFunction.sqlRowCount.invoke(statementHandle, rowCountPtr);
            SqlReturn.check("SQLRowCount", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }

        return rowCountPtr.get(ValueLayout.JAVA_LONG, 0);
    }

    public boolean moreResults() {
        MemorySegment statementHandle = handleAddress();
        try {
            short rc = (short) OdbcFunction.sqlMoreResults.invoke(statementHandle);
            if (rc == SqlReturn.SQL_NO_DATA) {
                return false;
            }
            SqlReturn.check("SQLMoreResults", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }

        return true;
    }

    public void freeStmt(FreeStmtOption option) {
        MemorySegment statementHandle = handleAddress();
        short optionValue = option.value();
        try {
            short rc = (short) OdbcFunction.sqlFreeStmt.invoke(statementHandle, optionValue);
            SqlReturn.check("SQLFreeStmt", rc, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        } finally {
            if (option == FreeStmtOption.SQL_DROP) {
                clearHandleAddress();
            }
        }
    }
}
