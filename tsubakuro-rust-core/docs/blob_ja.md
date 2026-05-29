# tsubakuro-rust-core BLOB, CLOB使用方法

当文書では、BLOB, CLOBをtsubakuro-rust-coreで使用する基本的な方法を説明します。

## はじめに

Tsurugiでは、（メモリーに載せるのが躊躇われるような）大きなサイズのデータをBLOB, CLOB型で扱います。
BLOB（binary large object）はバイト列、CLOB（character large object）は文字列が対象です。

以降、BLOB, CLOBを共通で扱う場合は large object（LOB）という用語を用います。

TsurugiはインメモリーDBなので、基本的にデータは全てDBサーバーのメモリー上に置くのですが、LOBデータはメモリー上には置かず、データごとに個別のファイルとして保存されます。

クライアント（tsubakuro-rust-core）とTsurugi DBの間でLOBデータを受け渡す方法はいくつかあり、どの方法を使うかはクライアントアプリケーション（tsubakuro-rust-coreを利用するアプリケーション）側で指定することができますが、データを受け渡すAPI（コーディング方法）は同一です。（ただし、LOB転送モードによっては使用できないメソッドもあります）

## LOB転送モード

クライアント（tsubakuro-rust-core）とTsurugi DBの間でLOBデータを受け渡す方法（LOB転送を行う方法）は、LOB転送モードと呼びます。

LOB転送モードには以下のようなものがあります。

### 特権モード

特権モードでは、クライアントアプリケーションとTsurugi DBの間でのLOBデータの受け渡しをファイル経由で行います。  
すなわち、LOBデータをDBに登録する際は、ユーザー（クライアントアプリケーション）がLOBデータのファイルを用意し、tsubakuro-rust-core経由でそのファイルのパスをTsurugi DBに渡します（Tsurugi DBは渡されたパスのファイルをコピーします）。  
DBからLOBデータを取得する際も、DB内で保持されているファイルのパスが返るので、そのファイルを読むことになります（ユーザーに対しては、ファイルのパスは隠蔽されます）。

このように特権モードではLOBデータをファイルで受け渡しするため、クライアントアプリケーションはTsurugi DBと同じサーバー上で実行する必要があります。

なお、この方式が使えるのは、Tsurugi DBのTCPエンドポイントが特権モードで稼働している場合のみです。

### BLOB中継サービス利用モード

BLOB中継サービスはtsubakuro-rust-core 0.10.0（Tsurugi 1.11.0）以降で使用できます。

BLOB中継サービスは、クライアントアプリケーションとTsurugi DBの間でのLOBデータの受け渡しをgRPC（TCP/IP）で行います。  
（このため、クライアントアプリケーションとTsurugi DBが異なるサーバー上で動いていても使用できます）

なお、この方式が使えるのは、BLOB中継サービスが稼働している場合のみです。

> [!NOTE]
>
> BLOB中継サービスはCLOBも扱います。（CLOB中継サービスというものはありません）

## LobTransferType

どのLOB転送モードを使うのかは、ConnectionOptionにLobTransferType列挙型で指定します。

何も指定しなかった場合は `Default` として扱われます。

```rust
use tsubakuro_rust_core::prelude::*;

connection_option.set_lob_transfer_type(LobTransferType::Default);
```

- `Default`
  - BLOB中継サービスを使用します。Tsurugi側でBLOB中継サービスが使用できなくても、セッション接続は成功します。
- `NotUse`
  - LOB転送を行いません。
- `Privileged`
  - 特権モードを使用します。Tsurugi側で特権モードが使用できない場合、セッション接続が失敗します。
- `Relay`
  - BLOB中継サービスを使用します。Tsurugi側でBLOB中継サービスが使用できない場合、セッション接続が失敗します。

> [!NOTE]
>
> tsubakuro-rust-core 0.10.0より前は、LOB転送のデフォルトは特権モードでした。
> （そもそも特権モードしか無かったため、LOB転送モードを指定することはできませんでした）
>
> tsubakuro-rust-core 0.10.0で、LOB転送のデフォルトはBLOB中継サービスになりました。

また、セッション接続後に実際にどのLOB転送モードになったのかを、Sessionから取得することができます。

```rust
let lobTransferType = session.lob_transfer_type();
```

返ってくる型は（セッション接続時のオプションと同じく）LobTransferTypeですが、`Default` が返ることはありません。

## tsubakuro-rust-coreにおけるBLOB, CLOBの型

tsubakuro-rust-coreでは、BLOB, CLOBを以下の表にある型で扱います。

|                                                  | BLOB            | CLOB            |
| ------------------------------------------------ | --------------- | --------------- |
| プリペアードステートメントのパラメーターで使う型 | TgBlob          | TgClob          |
| select結果から取得する型                         | TgBlobReference | TgClobReference |
| データの型                                       | [u8]            | String          |

以降、当文書ではBLOBの説明のみを記載します。
tsubakuro-rust-coreにおけるCLOBの操作方法はBLOBと同等ですので、CLOBについては上記の点を読み替えてください。

SqlClientのBLOB関連メソッドを以下に示します。

メソッドの使用方法については [API ドキュメント](https://docs.rs/tsubakuro-rust-core/latest/tsubakuro_rust_core/prelude/struct.SqlClient.html) を参照してください。

| メソッド                                                     | 説明                                                         |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| `async fn allows_lob_operation(&self, operation: LobOperation) -> Result<bool, TgError>` | 現在のLOB転送モードでBLOB関連メソッドが使用できるかどうかを返す。（引数のoperationで対象メソッドを指定する） |
| `async fn upload_blob_file(&self, path: &Path) -> Result<TgBlob, TgError>` | BLOBファイルのパスを指定してアップロードする。               |
| `async fn upload_blob(&self, value: &[u8]) -> Result<TgBlob, TgError>` | BLOBデータをアップロードする。<br />（特権モードでは使用不可） |
| `async fn create_blob_uploader(&self) -> Result<BlobUploader, TgError>` | アップローダーを作成する。（BLOBデータをチャンクに分けてアップロードできる）<br />（特権モードでは使用不可） |
| `async fn open_blob(&self, transaction: &Transaction, blob: &TgBlobReference) -> Result<std::fs::File, TgError>` | ダウンロード用BLOBファイルを開く。<br />（BLOB中継サービスでは使用不可） |
| `async fn get_blob_cache(&self, transaction: &Transaction, blob: &TgBlobReference) -> Result<TgLargeObjectCache, TgError>` | ダウンロード用TgLargeObjectCacheを取得する。                 |
| `async fn read_blob(&self, transaction: &Transaction, blob: &TgBlobReference) -> Result<Vec<u8>, TgError>` | BLOBデータをダウンロードする。                               |
| `async fn copy_blob_to(&self, transaction: &Transaction, blob: &TgBlobReference, destination: &Path) -> Result<(), TgError>` | BLOBデータをダウンロードしてファイルに書き込む。             |
| `async fn create_blob_downloader(&self, transaction: &Transaction, blob: &TgBlobReference, timeout: Duration) -> Result<BlobDownloader, TgError>` | ダウンローダーを作成する。（BLOBデータをチャンクに分けてダウンロードできる） |

## プリペアードステートメントでBLOBを使用する方法

プリペアードステートメントでBLOBを扱う手順は以下のようになります。

1. SqlClientのアップロードメソッドを使ってBLOBデータをアップロードし、TgBlobを取得する。
3. TgBlobを使ってSqlParameterを生成する。
4. SqlPreparedStatementを使ったSQL実行メソッドや実行計画取得メソッドの引数にSqlParameterを渡す。
   - SQL実行メソッドや実行計画取得メソッドで一度使用すると、そのTgBlobは再利用できない。

### 特権モードの注意

特権モードで引数がBLOBファイルのパスを指定してアップロードメソッドを使う場合、そのファイルはSQL実行や実行計画取得時点でも存在している必要があります。

引数が&[u8]のアップロードメソッドは、特権モードでは使用できません。（特権モードでは、ファイルのパスのみを受け渡すため）

### insertの例

```rust
    let sql = "insert into blob_example values(:pk, :value)";
    let placeholders = vec![
        SqlPlaceholder::of::<i32>("pk"),
        SqlPlaceholder::of::<TgBlob>("value"),
    ];
    let ps = sql_client.prepare(sql, placeholders).await?;

    let data = vec![0x31_u8, 0x32_u8, 0x33_u8];
    let blob = sql_client.upload_blob(&data).await?; // TgBlob取得

    let parameters = vec![
        SqlParameter::of("pk", 1_i32),
        SqlParameter::of("value", blob),
    ];
    let execute_result = sql_client.prepared_execute(transaction, ps, parameters).await?;
```

## select文の実行結果からBLOBを取得する方法

select文の実行結果からBLOBを取得する手順は以下のようになります。

1. SqlQueryResultのfetchメソッドで、TgBlobReferenceを取得する。
   - TgBlobReferenceは **トランザクションが有効な間のみ** 使用可能。
2. SqlClientのダウンロードメソッドにTransactionとTgBlobReferenceを指定してBLOBデータをダウンロードする。

### BLOB中継サービス利用モードの注意

ファイルのパスを取得するダウンロードメソッドは、BLOB中継サービスでは使用できません。

（ファイルのパスを指定するアップロードメソッドは、ファイルを読んでそのデータをアップロードするので使用できます）

### selectの例

```rust
    let sql = "select * from blob_example";
    let mut query_result = sql_client.query(&transaction, sql).await?;

    while query_result.next_row().await? {
        assert!(query_result.next_column().await?);
        let pk: i32 = query_result.fetch().await?;

        assert!(query_result.next_column().await?);
        let blob: TgBlobReference = query_result.fetch().await?;
        let value = sql_client.read_blob(transaction, &blob).await?;

        println!("pk={pk}, value={value:?}");
    }

    query_result.close().await?;
```

### 特権モードのパスマッピング

tsubakuro-rust-coreでは、特権モードでLOBファイルを扱う際にクライアント側のパスとサーバー側のパスを変換するパスマッピング機能を提供しています。

特権モードでLOBファイルを扱う場合、クライアントとサーバーが同一ファイルシステムにアクセスできることを前提としていますが、環境によってはクライアントとサーバーのパスが一致しないことがあります。  
このとき、パスマッピングを設定することで、LOBファイルのパスをクライアントからサーバーに送信する際にクライアント側のパスがサーバー側のパスに変換されます。同様に、サーバーからファイルのパスを受信した際にクライアント側のパスに変換されます。

例えば以下のようにMS-Windows上のTsurugiのDockerでボリュームマウントしてtsubakuro-rust-coreでパスマッピングを指定すると、`C:/tmp/client/blob.bin` のBLOBファイルをinsertすることができます。  
また、selectする際に、tsubakuro-rust-coreは `C:/tmp/tsurugi` の下にあるBLOBファイルを読みます。

```
docker run -d -p 12345:12345 --name tsurugi -v C:/tmp/client:/mnt/client -v C:/tmp/tsurugi:/opt/tsurugi/var/data/log -e GLOG_v=30 ghcr.io/project-tsurugi/tsurugidb:latest
```

```rust
    let mut connection_option = ConnectionOption::new();
    connection_option.set_endpoint_url("tcp://localhost:12345")?;

    connection_option.set_lob_transfer_type(LobTransferType::Privileged);
    connection_option.add_large_object_path_mapping_on_send("C:/tmp/client", "/mnt/client");
    connection_option.add_large_object_path_mapping_on_recv("/opt/tsurugi/var/data/log", "C:/tmp/tsurugi");

    // connect
    let session = Session::connect(&connection_option).await?;
```

## BLOB中継サービスのエンドポイント

BLOB中継サービスの接続先URI（エンドポイント）は、Tsurugi DBから送られ、tsubakuro-rust-core内部で使用しています。

しかし、ネットワーク環境によっては、Tsurugi DB内部で管理しているホスト名やIPアドレスでは、クライアントから接続できないことがあります。

このため、tsubakuro-rust-coreでBLOB中継サービスのエンドポイントを指定することができるようになっています。

```rust
    let mut connection_option = ConnectionOption::new();
    connection_option.set_endpoint_url("tcp://localhost:12345")?;

    connection_option.set_lob_transfer_type(LobTransferType::Relay);
    connection_option.set_blob_relay_service_endpoint("http://localhost:52345");

    // connect
    let session = Session::connect(&connection_option).await?;
```

なお、tsubakuro-rust-coreでは `dns:///` は使用できません。`http://` に置き換えてください。

