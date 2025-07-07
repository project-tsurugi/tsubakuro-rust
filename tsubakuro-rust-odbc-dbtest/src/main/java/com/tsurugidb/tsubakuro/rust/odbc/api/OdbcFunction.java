package com.tsurugidb.tsubakuro.rust.odbc.api;

import static java.lang.foreign.ValueLayout.ADDRESS;
import static java.lang.foreign.ValueLayout.JAVA_INT;
import static java.lang.foreign.ValueLayout.JAVA_LONG;
import static java.lang.foreign.ValueLayout.JAVA_SHORT;

import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.SymbolLookup;
import java.lang.invoke.MethodHandle;

public class OdbcFunction {

    /** SQLRETURN SQLAllocHandle(SQLSMALLINT HandleType, SQLHANDLE InputHandle, SQLHANDLE *OutputHandlePtr) */
    public static final MethodHandle sqlAllocHandle;

    /** SQLRETURN SQLBindCol(SQLHSTMT StatementHandle, SQLUSMALLINT ColumnNumber, SQLSMALLINT TargetType, SQLPOINTER TargetValuePtr, SQLLEN BufferLength, SQLLEN* StrLen_or_IndPtr) */
    public static final MethodHandle sqlBindCol;

    /** SQLRETURN SQLBindParameter(SQLHSTMT, SQLUSMALLINT, SQLSMALLINT, SQLSMALLINT, SQLSMALLINT, SQLULEN, SQLSMALLINT, SQLPOINTER, SQLLEN, SQLLEN*) */
    public static final MethodHandle sqlBindParameter;

    /**
     * SQLRETURN SQLColumnsA(SQLHSTMT hstmt, SQLCHAR* szCatalogName, SQLSMALLINT cbCatalogName, SQLCHAR* szSchemaName, SQLSMALLINT cbSchemaName, SQLCHAR* szTableName, SQLSMALLINT cbTableName, SQLCHAR*
     * szColumnName, SQLSMALLINT cbColumnName)
     */
    public static final MethodHandle sqlColumnsA;
    /**
     * SQLRETURN SQLColumnsA(SQLHSTMT hstmt, SQLWCHAR* szCatalogName, SQLSMALLINT cbCatalogName, SQLWCHAR* szSchemaName, SQLSMALLINT cbSchemaName, SQLWCHAR* szTableName, SQLSMALLINT cbTableName,
     * SQLWCHAR* szColumnName, SQLSMALLINT cbColumnName)
     */
    public static final MethodHandle sqlColumnsW;

    /**
     * SQLRETURN SQLDescribeColA(SQLHSTMT StatementHandle, SQLUSMALLINT ColumnNumber, SQLCHAR* ColumnName, SQLSMALLINT BufferLength, SQLSMALLINT* NameLengthPtr, SQLSMALLINT* DataTypePtr, SQLULEN*
     * ColumnSizePtr, SQLSMALLINT* DecimalDigitsPtr, SQLSMALLINT* NullablePtr)
     */
    public static final MethodHandle sqlDescribeColA;
    /**
     * SQLRETURN SQLDescribeColW(SQLHSTMT StatementHandle, SQLUSMALLINT ColumnNumber, SQLWCHAR* ColumnName, SQLSMALLINT BufferLength, SQLSMALLINT* NameLengthPtr, SQLSMALLINT* DataTypePtr, SQLULEN*
     * ColumnSizePtr, SQLSMALLINT* DecimalDigitsPtr, SQLSMALLINT* NullablePtr)
     */
    public static final MethodHandle sqlDescribeColW;

    /** SQLRETURN SQLDisconnect(SQLHDBC ConnectionHandle) */
    public static final MethodHandle sqlDisconnect;

    /**
     * SQLRETURN SQLDriverConnect(SQLHDBC ConnectionHandle, SQLHWND WindowHandle, SQLCHAR* InConnectionString, SQLSMALLINT StringLength1, SQLCHAR* OutConnectionString, SQLSMALLINT BufferLength,
     * SQLSMALLINT* StringLength2Ptr, SQLUSMALLINT DriverCompletion)
     **/
    public static final MethodHandle sqlDriverConnectA;
    /**
     * SQLRETURN SQLDriverConnect(SQLHDBC ConnectionHandle, SQLHWND WindowHandle, SQLWCHAR* InConnectionString, SQLSMALLINT StringLength1, SQLWCHAR* OutConnectionString, SQLSMALLINT BufferLength,
     * SQLSMALLINT* StringLength2Ptr, SQLUSMALLINT DriverCompletion)
     **/
    public static final MethodHandle sqlDriverConnectW;

    /** SQLEndTran(SQLSMALLINT HandleType, SQLHANDLE Handle, SQLSMALLINT CompletionType) */
    public static final MethodHandle sqlEndTran;

    /** SQLRETURN SQLExecDirectA(SQLHSTMT StatementHandle, SQLCHAR* StatementText, SQLINTEGER TextLength) */
    public static final MethodHandle sqlExecDirectA;
    /** SQLRETURN SQLExecDirectA(SQLHSTMT StatementHandle, SQLWCHAR* StatementText, SQLINTEGER TextLength) */
    public static final MethodHandle sqlExecDirectW;

    /** SQLRETURN SQLExecute(SQLHSTMT StatementHandle) */
    public static final MethodHandle sqlExecute;

    /** SQLRETURN SQLFetch(SQLHSTMT StatementHandle) */
    public static final MethodHandle sqlFetch;

    /** SQLRETURN SQLGetConnectAttrA(SQLHDBC ConnectionHandle, SQLINTEGER Attribute, SQLPOINTER ValuePtr, SQLINTEGER BufferLength, SQLINTEGER* StringLengthPtr) */
    public static final MethodHandle sqlGetConnectAttrA;
    /** SQLRETURN SQLGetConnectAttrW(SQLHDBC ConnectionHandle, SQLINTEGER Attribute, SQLPOINTER ValuePtr, SQLINTEGER BufferLength, SQLINTEGER* StringLengthPtr) */
    public static final MethodHandle sqlGetConnectAttrW;

    /**
     * SQLRETURN SQLGetData( SQLHSTMT StatementHandle, SQLUSMALLINT ColumnNumber, SQLSMALLINT TargetType, SQLPOINTER TargetValue, SQLLEN BufferLength, SQLLEN *StrLen_or_IndPtr)
     */
    public static final MethodHandle sqlGetData;

    /**
     * SQLRETURN SQLGetDiagRec(SQLSMALLINT HandleType, SQLHANDLE Handle, SQLSMALLINT RecNumber, SQLCHAR *SQLState, SQLINTEGER *NativeErrorPtr, SQLCHAR *MessageText, SQLSMALLINT BufferLength,
     * SQLSMALLINT *TextLengthPtr)
     */
    public static final MethodHandle sqlGetDiagRecA;
    /**
     * SQLRETURN SQLGetDiagRecW(SQLSMALLINT fHandleType, SQLHANDLE handle, SQLSMALLINT iRecord, SQLWCHAR* szSqlState, SQLINTEGER* pfNativeError, SQLWCHAR* szErrorMsg, SQLSMALLINT cchErrorMsgMax,
     * SQLSMALLINT* pcchErrorMsg)
     */
    public static final MethodHandle sqlGetDiagRecW;

    /** SQLRETURN SQLGetInfoA(SQLHDBC ConnectionHandle, SQLUSMALLINT InfoType, SQLPOINTER InfoValuePtr, SQLSMALLINT BufferLength, SQLSMALLINT* StringLengthPtr) */
    public static final MethodHandle sqlGetInfoA;
    /** SQLRETURN SQLGetInfoW(SQLHDBC ConnectionHandle, SQLUSMALLINT InfoType, SQLPOINTER InfoValuePtr, SQLSMALLINT BufferLength, SQLSMALLINT* StringLengthPtr) */
    public static final MethodHandle sqlGetInfoW;

    /** SQLRETURN SQLGetTypeInfoA(SQLHSTMT StatementHandle, SQLSMALLINT DataType) */
    public static final MethodHandle sqlGetTypeInfoA;
    /** SQLRETURN SQLGetTypeInfoW(SQLHSTMT StatementHandle, SQLSMALLINT DataType) */
    public static final MethodHandle sqlGetTypeInfoW;

    /** SQLRETURN SQLGetFunctions(SQLHDBC ConnectionHandle, SQLUSMALLINT FunctionId, SQLUSMALLINT *SupportedPtr) */
    public static final MethodHandle sqlGetFunctions;

    /** SQLRETURN SQLFreeHandle(SQLSMALLINT HandleType, SQLHANDLE Handle) */
    public static final MethodHandle sqlFreeHandle;

    /** SQLRETURN SQLNumResultCols(SQLHSTMT StatementHandle, SQLSMALLINT* ColumnCountPtr) */
    public static final MethodHandle sqlNumResultCols;

    /** SQLRETURN SQLPrepareA(SQLHSTMT StatementHandle, SQLCHAR* StatementText, SQLINTEGER TextLength) */
    public static final MethodHandle sqlPrepareA;
    /** SQLRETURN SQLPrepareW(SQLHSTMT StatementHandle, SQLWCHAR* StatementText, SQLINTEGER TextLength) */
    public static final MethodHandle sqlPrepareW;

    /**
     * SQLRETURN SQLPrimaryKeysA(SQLHSTMT StatementHandle, SQLCHAR* CatalogName, SQLSMALLINT NameLength1, SQLCHAR* SchemaName, SQLSMALLINT NameLength2, SQLCHAR* TableName, SQLSMALLINT NameLength3)
     */
    public static final MethodHandle sqlPrimaryKeysA;
    /**
     * SQLRETURN SQLPrimaryKeysW(SQLHSTMT StatementHandle, SQLWCHAR* CatalogName, SQLSMALLINT NameLength1, SQLWCHAR* SchemaName, SQLSMALLINT NameLength2, SQLWCHAR* TableName, SQLSMALLINT NameLength3)
     */
    public static final MethodHandle sqlPrimaryKeysW;

    /** SQLRETURN SQLRowCount(SQLHSTMT StatementHandle, SQLLEN* RowCountPtr) */
    public static final MethodHandle sqlRowCount;

    /** SQLRETURN SQLSetConnectAttrA(SQLHDBC ConnectionHandle, SQLINTEGER Attribute, SQLPOINTER ValuePtr, SQLINTEGER StringLength) */
    public static final MethodHandle sqlSetConnectAttrA;
    /** SQLRETURN SQLSetConnectAttrW(SQLHDBC ConnectionHandle, SQLINTEGER Attribute, SQLPOINTER ValuePtr, SQLINTEGER StringLength) */
    public static final MethodHandle sqlSetConnectAttrW;

    /** SQLRETURN SQLSetEnvAttr(SQLHENV EnvironmentHandle, SQLINTEGER Attribute, SQLPOINTER ValuePtr, SQLINTEGER StringLength); */
    public static final MethodHandle sqlSetEnvAttr;

    /**
     * SQLRETURN SQLTablesA(SQLHSTMT hstmt, SQLCHAR* szCatalogName, SQLSMALLINT cbCatalogName, SQLCHAR* szSchemaName, SQLSMALLINT cbSchemaName, SQLCHAR* szTableName, SQLSMALLINT cbTableName, SQLCHAR*
     * szTableType, SQLSMALLINT cbTableType)
     */
    public static final MethodHandle sqlTablesA;
    /**
     * SQLRETURN SQLTablesA(SQLHSTMT hstmt, SQLWCHAR* szCatalogName, SQLSMALLINT cbCatalogName, SQLWCHAR* szSchemaName, SQLSMALLINT cbSchemaName, SQLWCHAR* szTableName, SQLSMALLINT cbTableName,
     * SQLWCHAR* szTableType, SQLSMALLINT cbTableType)
     */
    public static final MethodHandle sqlTablesW;

    static {
        try {
            var linker = Linker.nativeLinker();
            var lookup = SymbolLookup.libraryLookup("odbc32", Arena.global());

            {
                var symbol = lookup.find("SQLAllocHandle").orElseThrow(() -> new RuntimeException("SQLAllocHandle not found"));
                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        JAVA_SHORT, // SQLSMALLINT HandleType
                        ADDRESS, // SQLHANDLE InputHandle
                        ADDRESS // SQLHANDLE *OutputHandlePtr
                );
                sqlAllocHandle = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLBindCol").orElseThrow(() -> new RuntimeException("SQLBindCol not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        JAVA_SHORT, // SQLUSMALLINT ColumnNumber
                        JAVA_SHORT, // SQLSMALLINT TargetType
                        ADDRESS, // SQLPOINTER TargetValuePtr
                        JAVA_LONG, // SQLLEN BufferLength
                        ADDRESS // SQLLEN* StrLen_or_IndPtr
                );

                sqlBindCol = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLBindParameter").orElseThrow(() -> new RuntimeException("SQLBindParameter not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        JAVA_SHORT, // SQLUSMALLINT ParameterNumber
                        JAVA_SHORT, // SQLSMALLINT InputOutputType
                        JAVA_SHORT, // SQLSMALLINT ValueType
                        JAVA_SHORT, // SQLSMALLINT ParameterType
                        JAVA_LONG, // SQLULEN ColumnSize
                        JAVA_SHORT, // SQLSMALLINT DecimalDigits
                        ADDRESS, // SQLPOINTER ParameterValuePtr
                        JAVA_LONG, // SQLLEN BufferLength
                        ADDRESS // SQLLEN* StrLen_or_IndPtr
                );

                sqlBindParameter = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLColumnsA").orElseThrow(() -> new RuntimeException("SQLColumnsA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT hstmt
                        ADDRESS, // SQLCHAR* szCatalogName
                        JAVA_SHORT, // SQLSMALLINT cbCatalogName
                        ADDRESS, // SQLCHAR* szSchemaName
                        JAVA_SHORT, // SQLSMALLINT cbSchemaName
                        ADDRESS, // SQLCHAR* szTableName
                        JAVA_SHORT, // SQLSMALLINT cbTableName
                        ADDRESS, // SQLCHAR* szColumnName
                        JAVA_SHORT // SQLSMALLINT cbColumnName
                );

                sqlColumnsA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLColumnsW").orElseThrow(() -> new RuntimeException("SQLColumnsW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT hstmt
                        ADDRESS, // SQLWCHAR* szCatalogName
                        JAVA_SHORT, // SQLSMALLINT cbCatalogName
                        ADDRESS, // SQLWCHAR* szSchemaName
                        JAVA_SHORT, // SQLSMALLINT cbSchemaName
                        ADDRESS, // SQLWCHAR* szTableName
                        JAVA_SHORT, // SQLSMALLINT cbTableName
                        ADDRESS, // SQLWCHAR* szColumnName
                        JAVA_SHORT // SQLSMALLINT cbColumnName
                );

                sqlColumnsW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLDescribeColA").orElseThrow(() -> new RuntimeException("SQLDescribeColA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        JAVA_SHORT, // SQLUSMALLINT ColumnNumber
                        ADDRESS, // SQLCHAR* ColumnName
                        JAVA_SHORT, // SQLSMALLINT BufferLength
                        ADDRESS, // SQLSMALLINT* NameLengthPtr
                        ADDRESS, // SQLSMALLINT* DataTypePtr
                        ADDRESS, // SQLULEN* ColumnSizePtr
                        ADDRESS, // SQLSMALLINT* DecimalDigitsPtr
                        ADDRESS // SQLSMALLINT* NullablePtr
                );

                sqlDescribeColA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLDescribeColW").orElseThrow(() -> new RuntimeException("SQLDescribeColW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        JAVA_SHORT, // SQLUSMALLINT ColumnNumber
                        ADDRESS, // SQLWCHAR* ColumnName
                        JAVA_SHORT, // SQLSMALLINT BufferLength
                        ADDRESS, // SQLSMALLINT* NameLengthPtr
                        ADDRESS, // SQLSMALLINT* DataTypePtr
                        ADDRESS, // SQLULEN* ColumnSizePtr
                        ADDRESS, // SQLSMALLINT* DecimalDigitsPtr
                        ADDRESS // SQLSMALLINT* NullablePtr
                );

                sqlDescribeColW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLDisconnect").orElseThrow(() -> new RuntimeException("SQLDisconnect not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS // SQLHDBC ConnectionHandle (void*)
                );

                sqlDisconnect = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLDriverConnectA").orElseThrow(() -> new RuntimeException("SQLDriverConnectA not found"));
                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        ADDRESS, // SQLHWND WindowHandle
                        ADDRESS, // SQLCHAR* InConnectionString
                        JAVA_SHORT, // SQLSMALLINT StringLength1
                        ADDRESS, // SQLCHAR* OutConnectionString
                        JAVA_SHORT, // SQLSMALLINT BufferLength
                        ADDRESS, // SQLSMALLINT* StringLength2Ptr
                        JAVA_SHORT // SQLUSMALLINT DriverCompletion
                );
                sqlDriverConnectA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLDriverConnectW").orElseThrow(() -> new RuntimeException("SQLDriverConnectW not found"));
                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        ADDRESS, // SQLHWND WindowHandle
                        ADDRESS, // SQLWCHAR* InConnectionString
                        JAVA_SHORT, // SQLSMALLINT StringLength1
                        ADDRESS, // SQLWCHAR* OutConnectionString
                        JAVA_SHORT, // SQLSMALLINT BufferLength
                        ADDRESS, // SQLSMALLINT* StringLength2Ptr
                        JAVA_SHORT // SQLUSMALLINT DriverCompletion
                );
                sqlDriverConnectW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLEndTran").orElseThrow(() -> new RuntimeException("SQLEndTran not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        JAVA_SHORT, // SQLSMALLINT HandleType
                        ADDRESS, // SQLHANDLE Handle
                        JAVA_SHORT // SQLSMALLINT CompletionType
                );

                sqlEndTran = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLExecDirectA").orElseThrow(() -> new RuntimeException("SQLExecDirectA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        ADDRESS, // SQLCHAR* StatementText
                        JAVA_INT // SQLINTEGER TextLength
                );

                sqlExecDirectA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLExecDirectW").orElseThrow(() -> new RuntimeException("SQLExecDirectW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        ADDRESS, // SQLWCHAR* StatementText
                        JAVA_INT // SQLINTEGER TextLength
                );

                sqlExecDirectW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLExecute").orElseThrow(() -> new RuntimeException("SQLExecute not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS // SQLHSTMT StatementHandle
                );

                sqlExecute = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLFetch").orElseThrow(() -> new RuntimeException("SQLFetch not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS // SQLHSTMT StatementHandle (void*)
                );

                sqlFetch = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetConnectAttrA").orElseThrow(() -> new RuntimeException("SQLGetConnectAttrA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        JAVA_INT, // SQLINTEGER Attribute
                        ADDRESS, // SQLPOINTER ValuePtr
                        JAVA_INT, // SQLINTEGER BufferLength
                        ADDRESS // SQLINTEGER* StringLengthPtr
                );

                sqlGetConnectAttrA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetConnectAttrW").orElseThrow(() -> new RuntimeException("SQLGetConnectAttrW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        JAVA_INT, // SQLINTEGER Attribute
                        ADDRESS, // SQLPOINTER ValuePtr
                        JAVA_INT, // SQLINTEGER BufferLength
                        ADDRESS // SQLINTEGER* StringLengthPtr
                );

                sqlGetConnectAttrW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetData").orElseThrow(() -> new RuntimeException("SQLGetData not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        JAVA_SHORT, // SQLUSMALLINT ColumnNumber
                        JAVA_SHORT, // SQLSMALLINT TargetType
                        ADDRESS, // SQLPOINTER TargetValue
                        JAVA_LONG, // SQLLEN BufferLength
                        ADDRESS // SQLLEN* StrLen_or_IndPtr
                );

                sqlGetData = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetDiagRecA").orElseThrow(() -> new RuntimeException("SQLGetDiagRecA not found"));
                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        JAVA_SHORT, // SQLSMALLINT HandleType
                        ADDRESS, // SQLHANDLE Handle
                        JAVA_SHORT, // SQLSMALLINT RecNumber
                        ADDRESS, // SQLCHAR* SQLState
                        ADDRESS, // SQLINTEGER* NativeErrorPtr
                        ADDRESS, // SQLCHAR* MessageText
                        JAVA_SHORT, // SQLSMALLINT BufferLength
                        ADDRESS // SQLSMALLINT* TextLengthPtr
                );
                sqlGetDiagRecA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetDiagRecW").orElseThrow(() -> new RuntimeException("SQLGetDiagRecW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        JAVA_SHORT, // SQLSMALLINT HandleType
                        ADDRESS, // SQLHANDLE Handle
                        JAVA_SHORT, // SQLSMALLINT RecNumber
                        ADDRESS, // SQLWCHAR* SqlState
                        ADDRESS, // SQLINTEGER* NativeErrorPtr
                        ADDRESS, // SQLWCHAR* MessageText
                        JAVA_SHORT, // SQLSMALLINT BufferLength
                        ADDRESS // SQLSMALLINT* TextLengthPtr
                );

                sqlGetDiagRecW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetInfoA").orElseThrow(() -> new RuntimeException("SQLGetInfoA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        JAVA_SHORT, // SQLUSMALLINT InfoType
                        ADDRESS, // SQLPOINTER InfoValuePtr
                        JAVA_SHORT, // SQLSMALLINT BufferLength
                        ADDRESS // SQLSMALLINT* StringLengthPtr
                );

                sqlGetInfoA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetInfoW").orElseThrow(() -> new RuntimeException("SQLGetInfoW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        JAVA_SHORT, // SQLUSMALLINT InfoType
                        ADDRESS, // SQLPOINTER InfoValuePtr
                        JAVA_SHORT, // SQLSMALLINT BufferLength
                        ADDRESS // SQLSMALLINT* StringLengthPtr
                );

                sqlGetInfoW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetTypeInfoA").orElseThrow(() -> new RuntimeException("SQLGetTypeInfoA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        JAVA_SHORT // SQLSMALLINT DataType
                );

                sqlGetTypeInfoA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetTypeInfoW").orElseThrow(() -> new RuntimeException("SQLGetTypeInfoW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        JAVA_SHORT // SQLSMALLINT DataType
                );

                sqlGetTypeInfoW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLGetFunctions").orElseThrow(() -> new RuntimeException("SQLGetFunctions not found"));
                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        JAVA_SHORT, // SQLUSMALLINT FunctionId
                        ADDRESS // SQLUSMALLINT *SupportedPtr
                );
                sqlGetFunctions = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLFreeHandle").orElseThrow(() -> new RuntimeException("SQLFreeHandle not found"));
                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        JAVA_SHORT, // SQLSMALLINT HandleType
                        ADDRESS // SQLHANDLE Handle
                );
                sqlFreeHandle = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLNumResultCols").orElseThrow(() -> new RuntimeException("SQLNumResultCols not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        ADDRESS // SQLSMALLINT* ColumnCountPtr
                );

                sqlNumResultCols = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLPrepareA").orElseThrow(() -> new RuntimeException("SQLPrepareA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        ADDRESS, // SQLCHAR* StatementText
                        JAVA_INT // SQLINTEGER TextLength
                );

                sqlPrepareA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLPrepareW").orElseThrow(() -> new RuntimeException("SQLPrepareW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        ADDRESS, // SQLWCHAR* StatementText
                        JAVA_INT // SQLINTEGER TextLength
                );

                sqlPrepareW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLPrimaryKeysA").orElseThrow(() -> new RuntimeException("SQLPrimaryKeysA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        ADDRESS, // SQLCHAR* CatalogName
                        JAVA_SHORT, // SQLSMALLINT NameLength1
                        ADDRESS, // SQLCHAR* SchemaName
                        JAVA_SHORT, // SQLSMALLINT NameLength2
                        ADDRESS, // SQLCHAR* TableName
                        JAVA_SHORT // SQLSMALLINT NameLength3
                );

                sqlPrimaryKeysA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLPrimaryKeysW").orElseThrow(() -> new RuntimeException("SQLPrimaryKeysW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        ADDRESS, // SQLWCHAR* CatalogName
                        JAVA_SHORT, // SQLSMALLINT NameLength1
                        ADDRESS, // SQLWCHAR* SchemaName
                        JAVA_SHORT, // SQLSMALLINT NameLength2
                        ADDRESS, // SQLWCHAR* TableName
                        JAVA_SHORT // SQLSMALLINT NameLength3
                );

                sqlPrimaryKeysW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLRowCount").orElseThrow(() -> new RuntimeException("SQLRowCount not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT StatementHandle
                        ADDRESS // SQLLEN* RowCountPtr
                );

                sqlRowCount = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLSetConnectAttrA").orElseThrow(() -> new RuntimeException("SQLSetConnectAttrA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        JAVA_INT, // SQLINTEGER Attribute
                        ADDRESS, // SQLPOINTER ValuePtr
                        JAVA_INT // SQLINTEGER StringLength
                );

                sqlSetConnectAttrA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLSetConnectAttrW").orElseThrow(() -> new RuntimeException("SQLSetConnectAttrW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHDBC ConnectionHandle
                        JAVA_INT, // SQLINTEGER Attribute
                        ADDRESS, // SQLPOINTER ValuePtr
                        JAVA_INT // SQLINTEGER StringLength
                );

                sqlSetConnectAttrW = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLSetEnvAttr").orElseThrow(() -> new RuntimeException("SQLSetEnvAttr not found"));
                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHENV EnvironmentHandle
                        JAVA_INT, // SQLINTEGER Attribute
                        ADDRESS, // SQLPOINTER ValuePtr
                        JAVA_INT // SQLINTEGER StringLength
                );
                sqlSetEnvAttr = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLTablesA").orElseThrow(() -> new RuntimeException("SQLTablesA not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT hstmt
                        ADDRESS, // SQLCHAR* szCatalogName
                        JAVA_SHORT, // SQLSMALLINT cbCatalogName
                        ADDRESS, // SQLCHAR* szSchemaName
                        JAVA_SHORT, // SQLSMALLINT cbSchemaName
                        ADDRESS, // SQLCHAR* szTableName
                        JAVA_SHORT, // SQLSMALLINT cbTableName
                        ADDRESS, // SQLCHAR* szTableType
                        JAVA_SHORT // SQLSMALLINT cbTableType
                );

                sqlTablesA = linker.downcallHandle(symbol, desc);
            }
            {
                var symbol = lookup.find("SQLTablesW").orElseThrow(() -> new RuntimeException("SQLTablesW not found"));

                var desc = FunctionDescriptor.of(JAVA_SHORT, // SQLRETURN
                        ADDRESS, // SQLHSTMT hstmt
                        ADDRESS, // SQLWCHAR* szCatalogName
                        JAVA_SHORT, // SQLSMALLINT cbCatalogName
                        ADDRESS, // SQLWCHAR* szSchemaName
                        JAVA_SHORT, // SQLSMALLINT cbSchemaName
                        ADDRESS, // SQLWCHAR* szTableName
                        JAVA_SHORT, // SQLSMALLINT cbTableName
                        ADDRESS, // SQLWCHAR* szTableType
                        JAVA_SHORT // SQLSMALLINT cbTableType
                );

                sqlTablesW = linker.downcallHandle(symbol, desc);
            }
        } catch (Throwable e) {
            throw new RuntimeException("Failed to initialize ODBC functions", e);
        }
    }
}
