# Tsurugi ODBCドライバー データ型一覧

Tsurugi ODBCドライバーで使用できるデータ型を示します。

## データ型
| Tsurugi SQL              | Tsurugi AtomType           | SqlDataType                      | CDataType                                     |
| ------------------------ | -------------------------- | -------------------------------- | --------------------------------------------- |
|                          | BOOLEAN                    | SQL_BIT                          | SQL_C_BIT                                     |
|                          |                            |                                  | SQL_C_TINYINT, SQL_C_STINYINT, SQL_C_UTINYINT |
|                          |                            |                                  | SQL_C_SHORT, SQL_C_SSHORT, SQL_C_USHORT       |
| INT                      | INT4                       | SQL_INTEGER                      | SQL_C_LONG, SQL_C_SLONG, SQL_C_ULONG          |
| BIGINT                   | INT8                       | SQL_BIGINT                       | SQL_C_SBIGINT, SQL_C_UBIGINT                  |
| REAL                     | FLOAT4                     | SQL_FLOAT, SQL_REAL              | SQL_C_FLOAT                                   |
| DOUBLE                   | FLOAT8                     | SQL_DOUBLE                       | SQL_C_DOUBLE                                  |
| DECIMAL                  | DECIMAL                    | SQL_NUMERIC, SQL_DECIMAL         | SQL_C_NUMERIC                                 |
| CHAR, VARCHAR            | CHARACTER                  | SQL_CHAR, SQL_VARCHAR            | SQL_C_CHAR, SQL_C_WCHAR                       |
| BINARY, VARBINARY        | OCTET                      | SQL_BINARY, SQL_VARBINARY        | SQL_C_BINARY                                  |
| DATE                     | DATE                       | SQL_TYPE_DATE                    | SQL_C_DATE, SQL_C_TYPE_DATE                   |
| TIME                     | TIME_OF_DAY                | SQL_TYPE_TIME                    | SQL_C_TIME, SQL_C_TYPE_TIME                   |
| TIMESTAMP                | TIME_POINT                 | SQL_DATETIME, SQL_TYPE_TIMESTAMP | SQL_C_TIMESTAMP, SQL_C_TYPE_TIMESTAMP         |
| TIME WITH TIME ZONE      | TIME_OF_DAY_WITH_TIME_ZONE | -                                | -                                             |
| TIMESTAMP WITH TIME ZONE | TIME_POINT_WITH_TIME_ZONE  | -                                | -                                             |
| BLOB                     | BLOB（未対応）             | SQL_LONGVARBINARY                | SQL_C_BINARY                                  |
| CLOB                     | CLOB（未対応）             | SQL_LONGVARCHAR                  | SQL_C_CHAR, SQL_C_WCHAR                       |

- BLOB, CLOBは、現在のTsurugi ODBCドライバーでは未対応です。
- SQLGetDataやSQLBindParameter関数では、指定されたデータ型に値を変換します。変換できないデータ型や範囲外の値だった場合の挙動は、データ型によって異なります。（値が丸められたり切り捨てられたりエラーになったりします）