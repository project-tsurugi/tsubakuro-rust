# tsubakuro-rust-coreとTsubakuro/Javaの比較

tsubakuro-rust-coreは [Tsubakuro/Java](https://github.com/project-tsurugi/tsubakuro) からの移植と言えますが、全機能が対象というわけではなく、また、JavaとRustとの違い等により意図的にAPIを変えているところもあります。

当文書では、tsubakuro-rust-coreとTsubakuro/Javaの主な機能の比較を掲示します。

## サービス

| サービス       | Tsubakuro/Rust | Tsubakuro/Java   | 備考                                         |
| -------------- | -------------- | ---------------- | -------------------------------------------- |
| エンドポイント | TCP接続        | IPC接続・TCP接続 | Tsubakuro/RustではIPC接続に対応しない        |
| 認証           | なし           | Credential       | 現時点では、認証は実質的に機能していないため |
| SQL            | SqlClient      | SqlClient        |                                              |
| KVS            | ×              | KvsClient        |                                              |
| Datastore      | ×              | DatastoreClient  |                                              |
| デバッグ       | ×              | DebugClient      |                                              |

## API

### DB接続

| API                      | tsubakuro-rust-core | Tsubakuro/Java          |
| ------------------------ | ------------------- | ----------------------- |
| セッション               | Session             | Session                 |
| DB接続オプション         | ConnectionOption    | SessionBuilder          |
| セッション生成（DB接続） | Session::connect()  | SessionBuilder#create() |

### SQLサービス

| API                            | tsubakuro-rust-core             | Tsubakuro/Java               |
| ------------------------------ | ------------------------------- | ---------------------------- |
| SQLクライアント                | SqlClient                       | SqlClient                    |
| SQLクライアント作成            | session.make_client()           | SqlClient.attach(session)    |
| テーブル一覧取得               | SqlClient::list_tables()        | SqlClient#listTables()       |
| テーブルメタデータ取得         | SqlClient::get_table_metadata() | SqlClient#getTableMetadata() |
| プリペアードステートメント生成 | SqlClient::prepare()            | SqlClient#prepare()          |
| PS用プレースホルダー           | SqlPlaceholder                  | Placeholder                  |
| PS用パラメーター               | SqlParameter                    | Parameter                    |
| 実行計画取得                   | SqlClient::explain()            | SqlClient#explain()          |
| 実行計画取得（PS）             | SqlClient::prepared_explain()   | SqlClient#explain()          |
| ロード                         | ×                               | SqlClient#executeLoad()      |

#### データ型

| SQLのデータ型            | tsubakuro-rust-core（AtomType） | Rustの型                                  | Tsubakuro/Java（AtomType） |
| ------------------------ | ------------------------------- | ----------------------------------------- | -------------------------- |
|                          | Boolean                         | bool                                      | BOOLEAN                    |
| int                      | Int4                            | i32                                       | INT4                       |
| bigint                   | Int8                            | i64                                       | INT8                       |
| real                     | Float4                          | f32                                       | FLOAT4                     |
| double                   | Float8                          | f64                                       | FLOAT8                     |
| decimal                  | Decimal                         | TgDecimal, TgDecimalI128, TgDecimalResult | DECIMAL                    |
| char, varchar            | Character                       | &str, String                              | CHARACTER                  |
| binary, varbinary        | Octet                           | [u8], Vec＜u8＞                           | OCTET                      |
|                          | Bit                             |                                           | BIT                        |
| date                     | Date                            | TgDate                                    | DATE                       |
| time                     | TimeOfDay                       | TgTimeOfDay                               | TIME_OF_DAY                |
| timestamp                | TimePoint                       | TgTimePoint                               | TIME_POINT                 |
|                          | DatetimeInterval                |                                           | DATETIME_INTERVAL          |
| time with time zone      | TimeOfDayWithTimeZone           | TgTimeOfDayWithTimeZone                   | TIME_OF_DAY_WITH_TIME_ZONE |
| timestamp with time zone | TimePointWithTimeZone           | TgTimePointWithTimeZone                   | TIME_POINT_WITH_TIME_ZONE  |
| clob                     | Clob                            | TgClob, TgClobReference                   | CLOB                       |
| blob                     | Blob                            | TgBlob, TgBlobReference                   | BLOB                       |

- decimalに関しては、[bigdecimal](https://crates.io/crates/bigdecimal), [rust_decimal](https://crates.io/crates/rust_decimal) も使用可能。
- 日付時刻に関しては、[chrono](https://crates.io/crates/chrono), [time](https://crates.io/crates/time) も使用可能。

#### トランザクション


| API                            | tsubakuro-rust-core                 | Tsubakuro/Java                       |
| ------------------------------ | ----------------------------------- | ------------------------------------ |
| トランザクション               | Transaction                         | Transaction                          |
| トランザクションオプション     | TransactionOption                   | TransactionOption                    |
| トランザクション種別           | TransactionType                     | TransactionType                      |
| トランザクション開始           | SqlClient::start_transaction()      | SqlClient#createTransaction()        |
| トランザクションID取得         | Transaction::get_transaction_id()   | Transaction#getTransactionId()       |
| 更新系SQL実行                  | SqlClient::execute()                | Transaction#executeStatement()       |
| 更新系SQL実行（PS）            | SqlClient::prepared_execute()       | Transaction#executeStatement()       |
| 更新系SQL実行結果              | SqlExecuteResult                    | ExecuteResult                        |
| SELECT実行                     | SqlClient::query()                  | Transaction#executeQuery()           |
| SELECT実行（PS）               | SqlClient::prepared_query()         | Transaction#executeQuery()           |
| SELECT実行結果                 | SqlQueryResult                      | ResultSet                            |
| SQLバッチ実行（PS）            | ×                                   | Transaction#batch()                  |
| ダンプ                         | ×                                   | Transaction#executeDump()            |
| ロード                         | ×                                   | Transaction#executeLoad()            |
| コミットオプション             | CommitOption                        | CommitOption                         |
| コミット種別                   | CommitType                          | CommitStatus                         |
| トランザクションコミット       | SqlClient::commit()                 | Transaction#commit()                 |
| トランザクションロールバック   | SqlClient::rollback()               | Transaction#rollback()               |
| トランザクションステータス取得 | SqlClient::get_transaction_status() | Transaction#getSqlServiceException() |

- トランザクションは将来KVSクライアントでも使われる想定で、SQL関連のトランザクション機能はSqlClientのメソッドとなっている。（Transactionを引数で渡す）

#### 更新系SQL実行結果

| API                       | tsubakuro-rust-core               | Tsubakuro/Java                  |
| ------------------------- | --------------------------------- | ------------------------------- |
| 更新系SQL実行結果         | SqlExecuteResult                  | ExecuteResult                   |
| 更新件数の種類取得        | ×                                 | ExecuteResult#getCounterTypes() |
| 更新件数取得              | SqlExecuteResult::counters()      | ExecuteResult#getCounters()     |
| insert件数取得            | SqlExecuteResult::inserted_rows() | ×                               |
| update件数取得            | SqlExecuteResult::updated_rows()  | ×                               |
| insert or replace件数取得 | SqlExecuteResult::merged_rows()   | ×                               |
| delete件数取得            | SqlExecuteResult::deleted_rows()  | ×                               |
| 合計件数取得              | SqlExecuteResult::rows()          | ×                               |

- 個別の件数取得メソッドは、tsubakuro-rust-ffiから呼ぶ想定のもの。（FFIではMapを扱いにくいため）

#### SELECT実行結果

| API            | tsubakuro-rust-core            | Tsubakuro/Java              |
| -------------- | ------------------------------ | --------------------------- |
| SELECT実行結果 | SqlQueryResult                 | ResultSet                   |
| メタデータ取得 | SqlQueryResult::get_metadata() | ResultSet#getMetadata()     |
| 行移動         | SqlQueryResult::next_row()     | ResultSet#nextRow()         |
| カラム移動     | SqlQueryResult::next_column()  | ResultSet#nextColumn()      |
| NULLチェック   | SqlQueryResult::is_null()      | ResultSet#isNull()          |
| 値取得         | SqlQueryResult::fetch()        | ResultSet#fetchXxxValue()   |
| 配列の開始     | ×                              | ResultSet#beginArrayValue() |
| 配列の終了     | ×                              | ResultSet#endArrayValue()   |
| ローの開始     | ×                              | ResultSet#beginRowValue()   |
| ローの終了     | ×                              | ResultSet#endRowValue()     |

- 値を取得するメソッドは、Tsubakuro/Javaではデータ型毎に異なるメソッド名になっているが、tsubakuro-rust-coreではデータ型が異なってもメソッド名は同じ。
  - Rustでは代入先の型に応じたメソッドが呼ばれる。例： `let value: i32 = query_result.fetch().await?;`

#### BLOB関連

| API                        | tsubakuro-rust-core                           | Tsubakuro/Java                         |
| -------------------------- | --------------------------------------------- | -------------------------------------- |
| プレースホルダー生成       | SqlPlaceholder::of("name", AtomType::Blob)    | Placeholders.of("name", AtomType.BLOB) |
| パラメーター生成           | SqlParameter::of("name", TgBlob::new("path")) | Parameters.blobOf("name", "path")      |
| SELECT結果の値取得         | SqlQueryResult::fetch()                       | ResultSet#fetchBlob()                  |
| 取得用BLOB型               | TgBlobReference                               | BlobReference                          |
| BLOB読み込み               | SqlClient::open_blob()                        | Transaction#openInputStream()          |
| BLOBキャッシュ取得         | SqlClient::get_blob_cache()                   | Transaction#getLargeObjectCache()      |
| ローカルファイルへのコピー | SqlClient::copy_blob_to()                     | Transaction#copyTo()                   |

### 非同期API

| API            | tsubakuro-rust-core | Tsubakuro/Java       |
| -------------- | ------------------- | -------------------- |
| 非同期を表す型 | Job＜T＞            | FutureResponse＜T＞  |
| 値の取得       | Job::take()         | FutureResponse#get() |

- FutureResponseのget()は何回呼んでもいいが、Jobのtake()は一度しか呼べない。