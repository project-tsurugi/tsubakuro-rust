# Tsurugi ODBCドライバー 使用方法

C言語を例に、Tsurugi ODBCドライバーの使用方法を説明します。

## 使用方法

### 1. henv生成

最初にhenvハンドルを生成します。

そして、ODBCバージョンを設定します。  
Tsurugi ODBCドライバーはODBC3のみ指定できます。

```c
SQLHENV henv = NULL;
SQLAllocHandle(SQL_HANDLE_ENV, SQL_NULL_HANDLE, &henv);

SQLSetEnvAttr(henv, SQL_ATTR_ODBC_VERSION, (void*)SQL_OV_ODBC3, 0);
```

### 2. DB接続

hdbcハンドルを生成し、Tsurugiに接続します。

```c
SQLHDBC hdbc = NULL;
SQLAllocHandle(SQL_HANDLE_DBC, henv, &hdbc);

SQLCHAR connStrIn[] = "Driver={Tsurugi Driver};Endpoint=tcp://localhost:12345;UID=user;PWD=password;";
SQLCHAR outConnStr[1024];
SQLSMALLINT outConnStrLen;
SQLDriverConnectA(
    hdbc,
    NULL, // window handle
    connStrIn,
    SQL_NTS,
    outConnStr,
    sizeof(outConnStr),
    &outConnStrLen,
    SQL_DRIVER_NOPROMPT
);
```

接続文字列に記述するドライバー名には、Tsurugi ODBCドライバーをインストールしたときに登録された名前を指定します。  
（ [インストーラー](../installer) によってインストールした場合は `Tsurugi Driver` です）

Tsurugiの接続先として、接続文字列内に `Endpoint` でエンドポイントのURLを記述します。

認証には、以下のいずれかを指定します。

- `UID`, `PWD`
  - ユーザー・パスワードを指定します。
- `AuthToken`
  - 認証トークンを指定します。
- `Credentials`
  - 認証ファイルのパスを指定します。

認証を指定しなかった場合は無認証で接続を試みます。

なお、接続文字列内のキー（ `Endpoint` 等）は大文字小文字を無視して解釈されます。（内部では小文字に変換して扱います）  
（ `Driver` はODBCドライバーマネージャーが解釈するキーですが、大文字小文字は無視されるようです）

### 3. SQL実行

後は、一般的なODBCの使用方法と同じです。

#### プリペアードステートメントについて

プリペアードステートメントを使用してSQLを実行する場合、プレースホルダーには `?` を使用します。

- 一般的にODBCではプレースホルダーに `?` を使用します。
- Tsurugi固有のライブラリーでは基本的にプレースホルダーは `:name` を使用しますが、Tsurugi ODBCドライバーでは使用できません。
  - SQL文の中に `:name` を記述することはできても、値をバインドする SQLBindParameter関数でその名前を指定することができません。

## ログ出力

tsubakuro-rust-odbcは [env_logger](https://crates.io/crates/env_logger) を使ってログ出力しています。

Tsurugi ODBCドライバーを呼び出すアプリケーションの実行時に 環境変数TSURUGI_ODBC_DRIVER_LOG_LEVELを設定しておくと、トレースログやデバッグログが出力されます。  
（設定値は、env_loggerの環境変数RUST_LOGの仕様と同じです）

```dos
set TSURUGI_ODBC_DRIVER_LOG_LEVEL=tsubakuro_rust_odbc=trace
```

ログの出力先は、デフォルトではコンソールの標準エラーですが、環境変数TSURUGI_ODBC_DRIVER_LOG_FILEにログファイルのパスを指定することで、ファイル出力に切り替えることができます。

```dos
set TSURUGI_ODBC_DRIVER_LOG_FILE=/path/to/logfile
```

