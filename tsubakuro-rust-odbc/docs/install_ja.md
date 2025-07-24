# Tsurugi ODBCドライバー インストール方法

Tsurugi ODBCドライバーのインストール方法を説明します。

## MS-Windows

### 手動インストール

#### 前提

Tsurugi ODBCドライバーのdllファイル（  `tsubakuro_rust_odbc.dll`  ）は、tsubakuro-rust-odbcをビルドして生成します。

> [!NOTE]
>
> ビルドしたのと異なる環境でdllファイルを使用する場合、ビルド環境と同等のライブラリーが必要になることがあります。
>
> 例えばVisual C++ランタイムライブラリー（VC++ランタイムライブラリー）を使ってビルドされている場合、実行環境に「ビルド環境と互換性のあるバージョンのVC++ランタイムライブラリー」が必要です。
>
> （VC++ランタイムライブラリーに依存しているかどうかは、`rustup show` や `cargo build -v`, `dumpbin /dependents tsubakuro_rust_odbc.dll` 等で確認できます）

#### ドライバーのインストール

1. `tsubakuro_rust_odbc.dll` を適当な場所に配置します。

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
   - この例の場合、Tsurugi ODBCドライバーの登録名は `Tsurugi Driver` です。（接続文字列を使用するアプリケーションやDSNでは、ドライバー名にこの名前を指定します）

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
   
   [HKEY_CURRENT_USER\SOFTWARE\ODBC\ODBC.INI\MyTsurugiDSN]
   "Driver"="Tsurugi Driver"
   "Description"="My Tsurugi Database"
   "Endpoint"="tcp://localhost:12345"
   
   [HKEY_CURRENT_USER\SOFTWARE\ODBC\ODBC.INI\ODBC Data Sources]
   "MyTsurugiDSN"="Tsurugi Driver"
   ```

   - 設定値は適宜変更してください。
     - システムDSNに登録する場合はHKEY_CURRENT_USERをHKEY_LOCAL_MACHINEに変更してください。
   - この例の場合、DSNの名称は `MyTsurugiDSN` です。（ODBCを使用するアプリケーションからは、DSNにこの名前を指定します）

2. regファイルを実行します。（regファイルをダブルクリックする）  
   これにより、レジストリーにTsurugiのDSNが登録されます。