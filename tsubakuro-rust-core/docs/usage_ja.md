# tsubakuro-rust-coreの使用方法

当文書では、tsubakuro-rust-coreの基本的な使用方法を説明します。

## はじめに

もともと『Tsubakuro』は「JavaでTsurugiにアクセスする為のライブラリー」の名称でしたが、RustでTsurugiにアクセスするライブラリーが作られたことで、『Tsubakuro』は「Tsurugiにアクセスするライブラリーの総称」という位置付けに変わりました。
そのため、従来のJava版は『Tsubakuro/Java』、Rust版は『Tsubakuro/Rust』と呼称することにします。

ソースコードを管理するGitリポジトリーについては、[tsubakuroリポジトリー](https://github.com/project-tsurugi/tsubakuro) の下には既にTsubakuro/Java用の複数のサブプロジェクトが存在しており、そこにTsubakuro/Rustを追加すると混乱を招くため、別リポジトリーとなっています。

Tsubakuro/RustはC ABI形式のライブラリーを提供してRust以外の言語から呼べるようにすることも目的のひとつです。
そのためにC ABI形式の関数群を提供する外部関数インターフェース（ [tsubakuro-rust-ffi](../../tsubakuro-rust-ffi) ）も用意していますが、これはRustでTsurugiにアクセスする中核となるライブラリー（tsubakuro-rust-core）とは分けています。

tsubakuro-rust-coreはRustのみで扱うことを想定しています。

Tsubakuro/RustはTsubakuro/Javaからの移植とも言えますが、全機能が対象というわけではなく、また、JavaとRustとの違い等により意図的にAPIを変えているところもあります。



## tsubakuro-rust-coreの機能

tsubakuro-rust-coreはTCP接続でSQLを実行する機能を持っています。

Tsubakuro/JavaはTCP接続以外にIPC接続も可能でしたが、Tsubakuro/RustではIPC接続を提供する予定はありません。

SQL実行以外の機能（例えばKVSアクセス）は将来追加されるかもしれません。

Tsubakuro/Javaは非同期API（メソッドは基本的に `FutureResponse` を返す）でしたが、tsubakuro-rust-coreは同期API（通信の完了を待って値を返す）と非同期API（メソッドは `Job` を返してそこから値を取得する）の両方を提供しています。
これは、tsubakuro-rust-ffi経由で他言語から呼ばれる際には同期APIの方が都合が良いことがあるかもしれないと思われるからです。

### 非同期APIについて

tsubakuro-rust-coreの非同期APIは、メソッド名の末尾に `_async` が付き、返り値の型が `Job<T>` となっています。Jobのtakeメソッド等を使って `T` を取得します。

例えば Transactionを返す同期APIである `SqlClient::start_transaction() -> Result<Transaction, TgError>` の非同期APIバージョンは、`SqlClient::start_transaction_async() -> Result<Job<Transaction>, TgError>` です。このJobのtakeメソッドで `Transaction` インスタンスが取得できます。



## tsubakuro-rust-coreでSQLを実行する手順

tsubakuro-rust-coreでSQLを実行する手順は、概ね以下の手順になります。

1. Tsurugi DBサーバーのエンドポイントを指定してSessionを生成する（DBサーバーに接続する）。
2. SessionからSqlClientを生成する。
3. プリペアードステートメントを使う場合はSqlClientからSqlPreparedStatementを生成する。
4. SqlClientを使ってトランザクションを開始する（Transactionを生成する）。
5. SqlClientとTransaction（およびSqlPreparedStatement）を使ってSQLを実行する。
6. SqlClientとTransactionを使ってトランザクションをコミットする。
7. Transactionをクローズする。
8. 使い終わったSqlPreparedStatementをクローズする。
9. Sessionをクローズする。

なお、TransactionとSqlPreparedStatementを生成する順序は任意です。

コードの例を以下に示します。

```rust
use std::time::Duration;
use tsubakuro_rust_core::prelude::*;

pub async fn example() -> Result<(), TgError> {
    let endpoint = Endpoint::parse("tcp://localhost:12345")?;

    let mut connection_option = ConnectionOption::new();
    connection_option.set_endpoint(endpoint);
    connection_option.set_application_name("Tsubakuro/Rust example");
    connection_option.set_session_label("example session");
    connection_option.set_default_timeout(Duration::from_secs(10));

    // Session生成
    let session = Session::connect(&connection_option).await?;

    // SqlClient生成
    let client: SqlClient = session.make_client();

    // トランザクション開始
    let mut transaction_option = TransactionOption::new();
    transaction_option.set_transaction_type(TransactionType::Short);
    let transaction = client.start_transaction(&transaction_option).await?;

    // SQL実行
    let sql = "update customer set c_age = 2 where c_id = 3";
    client.execute(&transaction, sql).await?;

    // トランザクションコミット
    let commit_option = CommitOption::default();
    client.commit(&transaction, &commit_option).await?;

    // Transactionクローズ
    transaction.close().await?;

    // Sessionクローズ
    session.close().await?;
    Ok(())
}
```

### closeメソッドについて

SqlPreparedStatementおよびTransactionではcloseメソッドが提供されています。

これらの構造体では使用終了後にTsurugi DBサーバーに対してリソース解放を通知する必要があるので、Rustのデストラクターであるdropメソッド内でDBサーバーに対して通知（通信）を行うようにしています。

しかしdropメソッド内では本来は通信を行うべきではないでしょうし、また、通信エラーが発生してもユーザーがエラーをハンドリングすることができません。 
そこで、これらの構造体では明示的にcloseメソッドを呼び出すようにしてください。 
closeメソッドが呼ばれると、その成否にかかわらず、dropメソッド内では通信を行いません。

SessionのクローズではDBサーバーとの通信は行いませんが、エラーのハンドリングという観点では同様です。

Jobにもcloseメソッドがありますが、これはリクエストのキャンセルの通知を行いますので、キャンセルが不要な場合（既にtakeメソッド等で値を取得した後など）であればcloseメソッドの呼び出しは不要です。

