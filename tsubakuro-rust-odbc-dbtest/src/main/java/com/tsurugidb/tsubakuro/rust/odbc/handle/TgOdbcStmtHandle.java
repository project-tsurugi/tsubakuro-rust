package com.tsurugidb.tsubakuro.rust.odbc.handle;

import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

import com.tsurugidb.tsubakuro.rust.odbc.TgOdbcManager;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcConst;
import com.tsurugidb.tsubakuro.rust.odbc.api.OdbcFunction;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlDataType;
import com.tsurugidb.tsubakuro.rust.odbc.api.SqlReturn;
import com.tsurugidb.tsubakuro.rust.odbc.stmt.TgOdbcBindParameter;
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

        short result = (short) OdbcFunction.sqlGetTypeInfoA.invoke(statementHandle, //
                dataType);
        SqlReturn.check("SQLGetTypeInfoW", result, this);
    }

    private void getTypeInfoW(short dataType) throws Throwable {
        MemorySegment statementHandle = handleAddress();

        short result = (short) OdbcFunction.sqlGetTypeInfoW.invoke(statementHandle, //
                dataType);
        SqlReturn.check("SQLGetTypeInfoW", result, this);
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

        short result = (short) OdbcFunction.sqlTablesA.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                allStr, length, // tableName
                tableType, tableTypeLength);
        SqlReturn.check("SQLTablesA", result, this);
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

        short result = (short) OdbcFunction.sqlTablesW.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                allStr, length, // tableName
                tableType, tableTypeLength);
        SqlReturn.check("SQLTablesW", result, this);
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

        short result = (short) OdbcFunction.sqlColumnsA.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                tableNamePtr, tableNameLength, // tableName
                allStr, length // columnName
        );
        SqlReturn.check("SQLColumnsA", result, this);
    }

    private void columnsW(String tableName, short tableNameLength) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment allStr = MemorySegment.NULL;
        short length = 0;
        MemorySegment tableNamePtr = manager.allocateUtf16(tableName);

        short result = (short) OdbcFunction.sqlColumnsW.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                tableNamePtr, tableNameLength, // tableName
                allStr, length // columnName
        );
        SqlReturn.check("SQLColumnsW", result, this);
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

        short result = (short) OdbcFunction.sqlPrimaryKeysA.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                tableNamePtr, tableNameLength // tableName
        );
        SqlReturn.check("SQLPrimaryKeysA", result, this);
    }

    private void primaryKeysW(String tableName, short tableNameLength) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment allStr = MemorySegment.NULL;
        short length = 0;
        MemorySegment tableNamePtr = manager.allocateUtf16(tableName);

        short result = (short) OdbcFunction.sqlPrimaryKeysW.invoke(statementHandle, //
                allStr, length, // catalogName
                allStr, length, // schemaName
                tableNamePtr, tableNameLength // tableName
        );
        SqlReturn.check("SQLPrimaryKeysW", result, this);
    }

    public void execDirect(String sql, boolean wideChar) {
        try {
            if (wideChar) {
                execDirectW(sql);
            } else {
                execDirectA(sql);
            }
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    private void execDirectA(String sql) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment statementPtr = manager.allocateUtf8(sql);
        short statementLength = OdbcConst.SQL_NTS;

        short result = (short) OdbcFunction.sqlExecDirectA.invoke(statementHandle, statementPtr, statementLength);
        SqlReturn.check("SQLExecDirectA", result, this);
    }

    private void execDirectW(String sql) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment statementPtr = manager.allocateUtf16(sql);
        short statementLength = OdbcConst.SQL_NTS;

        short result = (short) OdbcFunction.sqlExecDirectW.invoke(statementHandle, statementPtr, statementLength);
        SqlReturn.check("SQLExecDirectW", result, this);
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

        short result = (short) OdbcFunction.sqlPrepareA.invoke(statementHandle, statementPtr, statementLength);
        SqlReturn.check("SQLPrepareA", result, this);
    }

    private void prepareW(String sql) throws Throwable {
        MemorySegment statementHandle = handleAddress();
        MemorySegment statementPtr = manager.allocateUtf16(sql);
        short statementLength = OdbcConst.SQL_NTS;

        short result = (short) OdbcFunction.sqlPrepareW.invoke(statementHandle, statementPtr, statementLength);
        SqlReturn.check("SQLPrepareW", result, this);
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
            short result = (short) OdbcFunction.sqlBindParameter.invoke(statementHandle, parameterNumberValue, inputOutputType, valueType, parameterType, columnSize, decimalDigits, parameterValuePtr,
                    bufferLength, strLenOrIndPtr);
            SqlReturn.check("SQLBindParameter", result, this);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public void execute() {
        MemorySegment statementHandle = handleAddress();
        try {
            short result = (short) OdbcFunction.sqlExecute.invoke(statementHandle);
            SqlReturn.check("SQLExecute", result, this);
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
            short result = (short) OdbcFunction.sqlFetch.invoke(statementHandle);
            switch (result) {
            case SqlReturn.SQL_SUCCESS:
            case SqlReturn.SQL_SUCCESS_WITH_INFO:
                return true;
            case SqlReturn.SQL_NO_DATA:
                return false;
            default:
                SqlReturn.check("SQLFetch", result, this);
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
            short result = (short) OdbcFunction.sqlNumResultCols.invoke(statementHandle, countPtr);
            SqlReturn.check("SQLNumResultCols", result, this);

            return countPtr.get(ValueLayout.JAVA_SHORT, 0);
        } catch (RuntimeException e) {
            throw e;
        } catch (Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
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

        short result = (short) OdbcFunction.sqlDescribeColA.invoke(statementHandle, (short) columnNumber, columnNamePtr, bufferLength, nameLengthPtr, dataTypePtr, columnSizePtr, decimalDigitsPtr,
                nullablePtr);
        SqlReturn.check("SQLDescribeColA", result, this);

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

        short result = (short) OdbcFunction.sqlDescribeColW.invoke(statementHandle, (short) columnNumber, columnNamePtr, bufferLength, nameLengthPtr, dataTypePtr, columnSizePtr, decimalDigitsPtr,
                nullablePtr);
        SqlReturn.check("SQLDescribeColW", result, this);

        String columnName = TgOdbcManager.stringFromUtf16(columnNamePtr, nameLengthPtr);
        var dataType = SqlDataType.fromValue(dataTypePtr.get(ValueLayout.JAVA_SHORT, 0));
        short columnSize = columnSizePtr.get(ValueLayout.JAVA_SHORT, 0);
        short decimalDigits = decimalDigitsPtr.get(ValueLayout.JAVA_SHORT, 0);
        short nullable = nullablePtr.get(ValueLayout.JAVA_SHORT, 0);
        return new DescribeColumn(columnName, dataType, columnSize, decimalDigits, nullable);
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
        return getDataString(columnNumber, 1024);
    }

    public String getDataString(int columnNumber, int bufferSize) {
        var arg = TgOdbcGetDataArgument.ofString(manager, bufferSize, false);
        String value = getData(columnNumber, arg);
        long length = arg.lengthOrInd();
        if (length > bufferSize) {
            return getDataString(columnNumber, (int) length + 1);
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
        short result = getData0(columnNumber, arg);
        SqlReturn.check("SQLGetData", result, this);

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
            short result = (short) OdbcFunction.sqlBindCol.invoke(statementHandle, columnNumberValue, targetTypeValue, valuePtr, bufferSize, lengthOrIndPtr);
            SqlReturn.check("SQLBindCol", result, this);
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
            short result = (short) OdbcFunction.sqlRowCount.invoke(statementHandle, rowCountPtr);
            SqlReturn.check("SQLRowCount", result, this);

            return rowCountPtr.get(ValueLayout.JAVA_LONG, 0);
        } catch (RuntimeException e) {
            throw e;
        } catch (Error e) {
            throw e;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }
}
