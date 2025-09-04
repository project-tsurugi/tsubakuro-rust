# Tsurugi ODBCドライバー インストール方法

Tsurugi ODBCドライバーのインストール方法を説明します。

## 前提（実行環境について）

Tsurugi ODBCドライバーは、それを生成（ビルド）したのと異なる環境で使用する場合、ビルド環境と同等のライブラリーが必要になることがあります。

例えばMS-Windows用のTsurugi ODBCドライバー（ `tsubakuro_rust_odbc.dll`  ）がVisual C++ランタイムライブラリー（VC++ランタイムライブラリー）を使ってビルドされている場合、実行環境に「ビルド環境と互換性のあるバージョンのVC++ランタイムライブラリー」が必要です。

> [!NOTE]
>
> VC++ランタイムライブラリーに依存しているかどうかは、ビルド環境の `rustup show` や `cargo build -v` で確認できます。
> また、Windows SDKの `dumpbin /dependents tsubakuro_rust_odbc.dll` でも確認することができます。

## ドライバーのインストール（MS-Windows）

### インストーラーを使う方法

Tsurugi ODBCドライバーの[インストーラー](../installer)（ `tsurugi_odbc_driver_installer.exe` ）を実行します。

インストーラーの画面が表示されますので、それに従ってください。

### 手動インストール方法

インストーラーを使わずにインストールする方法です。

1. `tsubakuro_rust_odbc.dll` を適当な場所に配置します。

2. Windowsのレジストリーに登録するための、拡張子regのファイルを作成します。

   ```reg
   Windows Registry Editor Version 5.00
   
   [HKEY_LOCAL_MACHINE\SOFTWARE\ODBC\ODBCINST.INI\Tsurugi Driver]
   "APILevel"="1"
   "ConnectFunctions"="YYN"
   "Driver"="/path/to/tsubakuro_rust_odbc.dll"
   "DriverODBCVer"="03.51"
   "FileUsage"="0"
   "Setup"="/path/to/tsubakuro_rust_odbc.dll"
   "SQLLevel"="0"
   "UsageCount"=dword:00000001
   
   [HKEY_LOCAL_MACHINE\SOFTWARE\ODBC\ODBCINST.INI\ODBC Drivers]
   "Tsurugi Driver"="Installed"
   ```

   - dllファイルの場所を絶対パスで記述してください。パス区切り文字は `/` または `\\` が使用できます。
   - この例の場合、Tsurugi ODBCドライバーの登録名は `Tsurugi Driver` です。（接続文字列を使用するアプリケーションやDSNでは、ドライバー名にこの名前を指定します）

3. regファイルを実行します。（regファイルをダブルクリックすると実行されます）  
   これにより、レジストリーにTsurugi ODBCドライバーが登録されます。

### 接続文字列の例

上記の手順でTsurugi ODBCドライバーをインストールした場合、接続文字列は以下のようになります。

```
Driver={Tsurugi Driver};Enedpoint=tcp://localhost:12345;UID=user;PWD=password;
```

## DSNの設定（MS-Windows）

### ODBC データソース アドミニストレーターによる設定方法

『ODBC データソース アドミニストレーター（64ビット）』からTsurugiのDSNを設定することができます。

- 「追加」 - 新しいDSNを作成します。
- 「削除」 - DSNを削除します。
- 「構成」 - DSNの内容を変更します。

### 手動インストール方法

ODBC データソース アドミニストレーターを使わずに設定する方法です。

1. Windowsのレジストリーに登録するための、拡張子regのファイルを作成します。

   ```reg
   Windows Registry Editor Version 5.00
   
   [HKEY_CURRENT_USER\SOFTWARE\ODBC\ODBC.INI\MyTsurugiDSN]
   "Driver"="Tsurugi Driver"
   "Endpoint"="tcp://localhost:12345"
   "Credentials"="/path/to/credentials.key"
   
   [HKEY_CURRENT_USER\SOFTWARE\ODBC\ODBC.INI\ODBC Data Sources]
   "MyTsurugiDSN"="Tsurugi Driver"
   ```
   
- 設定値は適宜変更してください。
     - システムDSNに登録する場合はHKEY_CURRENT_USERをHKEY_LOCAL_MACHINEに変更してください。
   - この例の場合、DSNの名称は `MyTsurugiDSN` です。（ODBCを使用するアプリケーションからは、DSNにこの名前を指定します）
   
2. regファイルを実行します。（regファイルをダブルクリックすると実行されます）  
   これにより、レジストリーにTsurugiのDSNが登録されます。