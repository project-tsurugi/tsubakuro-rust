
// gcc exampleW.c -lodbc32 -o exampleW.exe

#include <windows.h>
#include <stdio.h>
#include <sql.h>
#include <sqlext.h>

int main(void) {
    SQLHENV henv = NULL;
    SQLHDBC hdbc = NULL;
    SQLRETURN ret;
    SQLWCHAR outConnStr[1024];
    SQLSMALLINT outConnStrLen;

    // allocate henv
    ret = SQLAllocHandle(SQL_HANDLE_ENV, SQL_NULL_HANDLE, &henv);
    if (ret != SQL_SUCCESS && ret != SQL_SUCCESS_WITH_INFO) {
        printf("Failed to allocate environment handle\n");
        return 1;
    }

    // set ODBC version
    SQLSetEnvAttr(henv, SQL_ATTR_ODBC_VERSION, (void*)SQL_OV_ODBC3, 0);

    // allocate hdbc
    SQLAllocHandle(SQL_HANDLE_DBC, henv, &hdbc);

    // connection string (Unicode)
    SQLWCHAR connStrIn[] = L"DRIVER={Tsurugi Driver};ENDPOINT=tcp://localhost:12345;";

    // connect (Unicode version)
    ret = SQLDriverConnectW(
        hdbc,
        NULL, // window handle
        connStrIn,
        SQL_NTS,
        outConnStr,
        sizeof(outConnStr)/sizeof(SQLWCHAR),
        &outConnStrLen,
        SQL_DRIVER_NOPROMPT
    );

    if (SQL_SUCCEEDED(ret)) {
        printf("Connection succeeded!\n");

        // ...execute SQL...

        // disconnect
        SQLDisconnect(hdbc);
    } else {
        printf("Connection failed.\n");
    }

    // free handle
    SQLFreeHandle(SQL_HANDLE_DBC, hdbc);
    SQLFreeHandle(SQL_HANDLE_ENV, henv);

    return 0;
}