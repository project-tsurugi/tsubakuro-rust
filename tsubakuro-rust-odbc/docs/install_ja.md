# Tsurugi ODBCドライバー インストール方法

Tsurugi ODBCドライバーのインストール方法を説明します。

## MS-Windows

### 手動インストール

#### ドライバーのインストール

1. tsubakuro-rust-odbcをビルドして生成された `tsubakuro_rust_odbc.dll` を適当な場所に配置します。

2. Windowsのレジストリーに登録するための、拡張子regのファイルを作成します。

   ```reg
   Windows Registry Editor Version 5.00
   
   [HKEY_LOCAL_MACHINE\SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver]
   "Driver"="/path/to/tsubakuro_rust_odbc.dll"
   "UsageCount"=dword:00000001
   
   [HKEY_LOCAL_MACHINE\SOFTWARE\ODBC\ODBCINST.INI\ODBC Drivers]
   "Tsurugi Driver"="Installed"
   ```

   - dllファイルの場所を絶対パスで記述してください。パス区切り文字は `/` または `\\` が使用できます。
   - この例の場合、Tsurugi ODBCドライバーの登録名は `Tsurugi Driver` です。（ODBCを使用するアプリケーションからは、接続文字列のドライバー名にこの名前を指定します）

3. regファイルを実行します。（regファイルをダブルクリックする）
   これにより、レジストリーにTsurugi ODBCドライバーが登録されます。

##### 接続文字列の例

上記の手順でTsurugi ODBCドライバーをインストールした場合、接続文字列は以下のようになります。

```
DRIVER={Tsurugi Driver};ENDPOINT=tcp://localhost:12345;
```

#### DSNのインストール

必要であれば、DSNをインストールしてください。

1. Windowsのレジストリーに登録するための、拡張子regのファイルを作成します。

   ```reg
   Windows Registry Editor Version 5.00
   
   [HKEY_LOCAL_MACHINE\SOFTWARE\ODBC\ODBC.INI\MyTsurugiDSN]
   "Driver"="Tsurugi Driver"
   "Description"="My Tsurugi Database"
   "Endpoint"="tcp://localhost:12345"
   
   [HKEY_LOCAL_MACHINE\SOFTWARE\ODBC\ODBC.INI\ODBC Data Sources]
   "MyTsurugiDSN"="Tsurugi Driver"
   ```

   - 設定値は適宜変更してください。
   - この例の場合、DSNの名称は `MyTsurugiDSN` です。（ODBCを使用するアプリケーションからは、DSNにこの名前を指定します）

2. regファイルを実行します。（regファイルをダブルクリックする）
   これにより、レジストリーにTsurugiのDSNが登録されます。