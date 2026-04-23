# Tsurugiの接続オプション

Tsurugiへ接続する際に指定する接続オプションに関する実装について。

接続オプションを変更（増減）する場合、以下の箇所を修正する。

## Tsurugiへの接続に関する設定を行える箇所

（ODBCで規定されていない）Tsurugi独自の接続オプションを指定することができる箇所は以下の通り。

- SQLDriverConnect関数
  - 引数の接続文字列で接続オプションを指定する。
- ConfigDSN関数
  - DSN設定ダイアログ（Windows専用）で接続オプションを設定する。

### 対象外の箇所

- SQLConnect関数
  - 任意の接続オプションを指定することはできない。
- SQLBrowseConnect関数
  - 現在のTsurugi ODBCドライバーでは未対応。
- SQLSetConnectAttr関数
  - 引数のAttribute（整数）にはODBCドライバー独自のキーを使用することも可能なようだが、Tsurugi ODBCドライバーでは使用しない方針。

## ConnectionAttributes

ConnectionAttributesは、接続オプションに関するキーと設定値を保持する構造体。

SQLDriverConnect関数で受け取った接続文字列は、ConnectionAttributesでパースする。

```rust
pub fn parse(connection_string: &str) -> ConnectionAttributes
```

ConnectionAttributesの内部では、キーと値を `HashMap<String, String>` で保持している。
すなわち、任意のキーと値が保持可能。

ただし、Tsurugi ODBCドライバーで使用するキーについては、専用のゲッターメソッドを用意している。

```rust
    pub fn endpoint(&self) -> Option<&String> {
        self.get(KEY_ENDPOINT)
    }
```

## TsurugiOdbcConnectArguments

TsurugiOdbcConnectArgumentsは、Tsurugiへの接続に必要な情報を保持する構造体。

TsurugiOdbcConnectArgumentsを元に、tsubakuro-rust-coreのConnectionOptionを構築する。

### 使用箇所

- SQLDriverConnect関数
  - エンドポイントやユーザー・パスワード等のデータをConnectionAttributesからTsurugiOdbcConnectArgumentsに詰め替えている。
- read_dsn()
  - SQLGetPrivateProfileStringW関数を使ってDSNからデータを取得し、TsurugiOdbcConnectArgumentsを構築している。
  - SQLConnect関数（引数のserver_nameがDSNである）や、SQLDriverConnect関数の接続文字列でDSNが指定された場合や、DSN設定ダイアログを構築するときに呼ばれる。

## DsnDialogValue

DsnDialogValueは、DSN設定ダイアログで扱う値（接続オプションを含む）を保持する構造体。

## dsn_dialog_windows.rs

Windows用のDSN設定ダイアログを扱う（表示・操作する）ソースファイル。

ダイアログのレイアウト自体は、tsubakuro-rust-odbc/resources/dsn_dialog.rcで定義する。

