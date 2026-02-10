## Tsurugi PythonライブラリーAPI

tsubakuro-rust-pythonの主要なAPIを説明します。

以降の例では、以下のようにtsubakuro-rust-pythonがインポートされているものとします。

```python
import tsubakuro_rust_python as tsurugi
```

## connect関数

connect関数は、Tsurugiに接続してConnectionオブジェクトを返します。

引数でConfigオブジェクトを渡します。

```python
config = tsurugi.Config()
config.endpoint = "tcp://localhost:12345"
config.user = "tsurugi"
config.password = "password"
config.default_timeout = 30 // 秒
with tsurugi.connect(config) as connection:
    pass
```

引数でConfigのプロパティーを直接指定することもできます。

```python
with tsurugi.connect(endpoint="tcp://localhost:12345", user="tsurugi", password="password", default_timeout=30) as connection:
    pass
```

## 設定クラス

### Configクラス

Configクラスで、Tsurugiへの接続方法やデフォルト値を設定します。

| プロパティー         | 型                  | 説明                                                         |
| -------------------- | ------------------- | ------------------------------------------------------------ |
| `endpoint`           | `str`               | Tsurugiのエンドポイント                                      |
| `user`               | `str`               | 接続ユーザー                                                 |
| `password`           | `str`               | 接続パスワード                                               |
| `auth_token`         | `str`               | 接続に使用する認証トークン                                   |
| `credentials`        | `str`               | 接続に使用する認証ファイルのパス                             |
| `transaction_option` | `TransactionOption` | トランザクションオプション                                   |
| `commit_option`      | `CommitOption`      | コミットオプション                                           |
| `shutdown_option`    | `ShutdownOption`    | シャットダウンオプション                                     |
| `default_timeout`    | `int`               | デフォルトのタイムアウト時間（秒）<br />デフォルトは0（タイムアウトしない） |

#### 例

```python
config = tsurigi.Config()
config.endpoint = "tcp://localhost:12345"
config.user = "tsurugi"
config.password = "password"
config.default_timeout = 30 // 秒
```

```python
config = tsurugi.Config(endpoint="tcp://localhost:12345", user="tsurugi", password="password", default_timeout=30)
```

### TransactionOptionクラス

TransactionOptionクラスで、トランザクション開始時に使用されるオプションを設定します。

| プロパティー          | 型                | 説明                                                         |
| --------------------- | ----------------- | ------------------------------------------------------------ |
| `transaction_type`    | `TransactionType` | トランザクション種別(`OCC`, `LTX`, `RTX`)<br />デフォルトは `OCC` |
| `label`               | `str`             | ラベル                                                       |
| `include_ddl`         | `bool`            | LTXのみ。DDLを実行するかどうか<br />デフォルトは `False`     |
| `write_preserve`      | `list[str]`       | LTXのみ。更新対象テーブル一覧                                |
| `inclusive_read_area` | `list[str]`       | LTXのみ。参照対象テーブル一覧                                |
| `exclusive_read_area` | `list[str]`       | LTXのみ。参照対象外テーブル一覧                              |
| `scan_parallel`       | `int`             | RTXのみ。並列実行数                                          |
| `begin_timeout`       | `int`             | トランザクション開始のタイムアウト時間（秒）                 |

#### 例

```python
tx_option = tsurugi.TransactionOption(tsurugi.TransactionType.LTX)
tx_option.label = "tsubakuro-rust-python example"
tx_option.write_preserve = ["table1", "table2"]
```

### CommitOptionクラス

CommitOptionクラスで、トランザクションコミット時に使用されるオプションを設定します。

| プロパティー     | 型           | 説明                                                         |
| ---------------- | ------------ | ------------------------------------------------------------ |
| `commit_type`    | `CommitType` | コミット種別(`DEFAULT`, `ACCEPTED`, `AVAILABLE`, `STORED`, `PROPAGATED`)<br />デフォルトは `DEFAULT` |
| `auto_dispose`   | `bool`       | コミット成功時にTsurugi側のトランザクションリソースを破棄するかどうか<br />デフォルトは `False` |
| `commit_timeout` | `int`        | コミットのタイムアウト時間[秒]                               |

#### 例

```python
commit_option = tsurugi.CommitOption(tsurugi.CommitType.DEFAULT, False, 60)
```

### ShutdownOptionクラス

ShutdownOptionクラスで、Connectionのクローズ時に使用されるオプションを設定します。

| プロパティー       | 型             | 説明                                                         |
| ------------------ | -------------- | ------------------------------------------------------------ |
| `shutdown_type`    | `ShutdownType` | シャットダウン種別(`NOTHING`, `GRACEFUL`, `FORCEFUL`)<br />デフォルトは `GRACEFUL` |
| `shutdown_timeout` | `int`          | シャットダウンのタイムアウト時間（秒）                       |

#### 例

```python
shutdown_option = tsurugi.ShutdownOption(tsurugi.ShutdownType.GRACEFUL, 30)
```

## Connectionクラス

Connectionクラスで、Tsurugiに対する操作を行います。

| メソッド・プロパティー                 | 戻り値の型             | 説明                                                         |
| -------------------------------------- | ---------------------- | ------------------------------------------------------------ |
| `list_tables()`                        | `list[str]`            | Tsurugiのテーブル一覧を取得する                              |
| `get_table_metadata(table_name: str)`  | `TableMetadata`        | テーブル定義を取得する<br />テーブルが無い場合はエラーが発生する |
| `find_table_metadata(table_name: str)` | `TableMetadata | None` | テーブル定義を取得する<br />テーブルが無い場合はNoneを返す   |
| `cursor()`                             | `Cursor`               | Cursorオブジェクトを生成する                                 |
| `transaction_option`                   |                        | トランザクションオプションを設定する                         |
| `commit_option`                        |                        | コミットオプションを設定する                                 |
| `commit(option: CommitOption | None)`  |                        | トランザクションをコミットする                               |
| `rollback()`                           |                        | トランザクションをロールバックする                           |
| `shutdown_option`                      |                        | シャットダウンオプションを設定する                           |
| `close()`                              |                        | Connectionをクローズする                                     |
| `__enter__(), __exit()__`              |                        | with文で使用する                                             |

## Cursorクラス

Cursorクラスで、SQLを実行します。

| メソッド・プロパティー                                       | 戻り値の型               | 説明                                                         |
| ------------------------------------------------------------ | ------------------------ | ------------------------------------------------------------ |
| `prepare(operaion: str, parameters: tuple | dict)`           |                          | PreparedStatementを準備する                                  |
| `execute(operation: str, parameters: tuple | dict | None)`   |                          | SQLを実行する                                                |
| `executemany(operation:str, seq_of_parameters: list[tuple | dict])` |                          | 複数パラメーターでSQLを実行する                              |
| `description`                                                |                          | select実行結果のカラム情報を取得する                         |
| `fetchone()`                                                 | `tuple[Any, ...] | None` | select結果を1行取得する<br />データが無い場合はNoneを返す    |
| `next()`                                                     | `tuple[Any, ...]`        | select結果を1行取得する<br />データが無い場合はStopIterationが発生する |
| `arraysize`                                                  |                          | fetchmany()で取得する件数を設定する                          |
| `fetchmany(size: int | None)`                                | `list[tuple[Any, ...]]`  | select結果を複数件取得する                                   |
| `fetchall()`                                                 | `list[tuple[Any, ...]]`  | select結果を全件取得する                                     |
| `__iter__()`, `__next__()`                                   | `tuple[Any, ...]`        | for文で使用する                                              |
| `rownumber`                                                  | `int | None`             | selectで取得した現在の件数を返す                             |
| `rowcount`                                                   | `int`                    | 更新系SQLの処理件数を返す                                    |
| `clear()`                                                    |                          | キャッシュしているPreparedStatementを解放する                |
| `close()`                                                    |                          | Cursorをクローズする                                         |
| `__enter__()`, `__exit__()`                                  |                          | with文で使用する                                             |

## ログ初期化関数

| 関数                                                   | 戻り値の型 | 説明                     |
| ------------------------------------------------------ | ---------- | ------------------------ |
| `env_logger_init(filters: str, file_path: str | None)` |            | ログ出力を行うようにする |



