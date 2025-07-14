# Tsurugi ODBCドライバー 関数一覧

Tsurugi ODBCドライバーが対応しているODBC関数を示します。

## ハンドル関連

### SQLAllocHandle

ハンドルを生成します。

対応しているハンドルタイプは以下の通りです。

- SQL_HANDLE_ENV
- SQL_HANDLE_DBC
- SQL_HANDLE_STMT

### SQLFreeHandle

ハンドルを解放します。

### SQLFreeStmt

hstmtの各種クローズ処理を行います。

### SQLSetEnvAttr・SQLGetEnvAttr

henvの属性を設定・取得します。

対応している属性は以下の通りです。

- SQL_ATTR_ODBC_VERSION
  - 設定可能な値はSQL_OV_ODBC3のみです。

### SQLSetConnectAttr・SQLGetConnectAttr

hdbcの属性を設定・取得します。

対応している主な属性は以下の通りです。

- SQL_ATTR_CONNECTION_TIMEOUT
  - 接続タイムアウト時間（秒）。0の場合、タイムアウトしません。
- SQL_ATTR_LOGIN_TIMEOUT
  - SQL_ATTR_CONNECTION_TIMEOUTとして扱われます。
- SQL_ATTR_AUTOCOMMIT
  - デフォルトはSQL_AUTOCOMMIT_ON（SQLを実行する度に自動的にコミットする）

### SQLSetStmtAttr・SQLGetStmtAttr

hstmtの属性を設定・取得します。

対応している属性は以下の通りです。

- SQL_ATTR_QUERY_TIMEOUT
  - SQL実行のタイムアウト時間（秒）。0の場合、タイムアウトしません。
- SQL_ATTR_APP_ROW_DESC
  - 適当な値（意味のない値）を返します。
- SQL_ATTR_APP_PARAM_DESC
  - 適当な値（意味のない値）を返します。
- SQL_ATTR_IMP_ROW_DESC
  - 適当な値（意味のない値）を返します。
- SQL_ATTR_IMP_PARAM_DESC
  - 適当な値（意味のない値）を返します。

## DB接続関連

### SQLDriverConnect

Tsurugiに接続します。

接続文字列に指定できるキーは以下の通りです。（大文字小文字は区別されません）

- DRIVER
  - Tsurugi ODBCドライバーをインストールした際に登録したドライバー名を指定します。
- ENDPOINT
  - Tsurugiのエンドポイントを指定します。

#### 接続文字列の例

```c
SQLCHAR connStrIn[] = "DRIVER={Tsurugi Driver};ENDPOINT=tcp://localhost:12345;";
```

### SQLDisconnect

Tsurugiとの接続を切断します。

## 処理実行関連

### SQLTables

テーブル一覧を取得します。

> [!WARNING]
>
> SQLTables関数の引数でカタログ名・スキーマ名・テーブル名・テーブル種別を指定することができますが、現在のTsurugi ODBCドライバーでは無視します。
>
> 本来は指定された条件に合致するテーブルのみを返しますが、現在は全てのテーブルを返します。

### SQLColumns

指定されたテーブルのカラム一覧を取得します。

> [!WARNING]
>
> SQLColumns関数の引数でカタログ名・スキーマ名・テーブル名・カラム名を指定することができますが、現在のTsurugi ODBCドライバーではテーブル名以外は無視します。

### SQLPrimaryKeys

指定されたテーブルのプライマリキーを取得します。

> [!WARNING]
>
> SQLPrimaryKeys関数の引数でカタログ名・スキーマ名・テーブル名を指定することができますが、現在のTsurugi ODBCドライバーではテーブル名以外は無視します。

### SQLExecDirect

SQLを実行します。

### SQLPrepare

プリペアードステートメントを作成します。

プレースホルダーには `?` を使用します。

> [!NOTE]
>
> Tsurugi固有のライブラリーでは基本的にプレースホルダーは `:name` を使用しますが、Tsurugi ODBCドライバーでは使用できません。

### SQLBindParameter

プリペアードステートメントのプレースホルダーに値を設定します。

### SQLExecute

プリペアードステートメントのSQLを実行します。

### SQLEndTran

（オートコミットでない場合）コミットまたはロールバックを行います。

## 実行結果取得関連

### SQLNumResultCols

実行結果の列数を返します。

### SQLDescribeCol

実行結果の列の情報を返します。

> [!WARNING]
>
> select文の実行結果の場合、現在のTsurugi ODBCドライバーでは、columnSizeやdecimalGigitsは正しい値を返しません。（常に0を返します）

### SQLColAttribute

実行結果の列の情報を返します。

- 一部のfield_identifierのみ対応しています。

### SQLRowCount

実行結果の行数を返します。

- select文の実行結果の場合は、常に0を返します。
- 更新系SQLの実行結果の場合は、更新対象の件数を返します。

### SQLBindCol

実行結果の値を格納する領域を事前に登録します。

### SQLFetch

実行結果の値の取得対象を次の行に移動します。

### SQLGetData

実行結果の値を取得します。

- Tsurugi ODBCドライバーの制約として、select文の実行結果の場合、SQLGetData関数を呼び出す順序はcolumn_numberの昇順でなければなりません。また、（同一行内で）同じcolumn_numberで再度SQLGetData関数を呼び出すことはできません。

### SQLMoreResults

ひとつのSQLExecDirectやSQLExecute関数で複数のSQLを実行した際に 次の実行結果へ移動する関数ですが、Tsurugiでは1回の呼び出しで複数のSQLを実行することはできないので、常にSQL_NO_DATAを返します。

## エラー関連

### SQLGetDiagRec

エラーコード・エラーメッセージを返します。

### SQLGetDiagField

エラー情報を返します。

- 一部のdiag_identifierのみ対応しています。

## ドライバー情報関連

### SQLGetInfo

Tsurugi ODBCドライバーの情報を返します。

対応している情報種別は以下の通りです。

- SQL_CURSOR_COMMIT_BEHAVIOR
- SQL_CURSOR_ROLLBACK_BEHAVIOR
- SQL_DRIVER_ODBC_VER
- SQL_GETDATA_EXTENSIONS

### SQLGetFunctions

Tsurugi ODBCドライバーが対応しているODBC関数の一覧を返します。

### SQLGetTypeInfo

Tsurugi ODBCドライバーが対応しているデータ型の情報を返します。



## WCHARについて

Tsurugi ODBCドライバーは、関数名の末尾にWが付く関数も提供しています。

関数名の末尾がWの関数は、文字列（SQLWCHAR）をUTF-16LEとして扱います。

それ以外の関数については、文字列（SQLCHAR）をUTF-8として扱います。  |
（ASCIIの範囲内であれば問題ありませんが、それ以外の文字だと、MS-WindowsのODBCドライバーマネージャーが文字列をShift_JISとして扱うことがあるようで、文字化けする可能性があります）