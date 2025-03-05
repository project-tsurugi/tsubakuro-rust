# tsubakuro-rust-core

tsubakuro-rust-core is the core library to access [Tsurugi DB](https://github.com/project-tsurugi/tsurugidb) for Rust.

tsubakuro-rust-core is a port from [Tsubakuro/Java](https://github.com/project-tsurugi/tsubakuro), but it does not cover all functions.

## Limitations

- Provide SQL service only.
- Only TCP connection is available.

## Crate features

Default feature include the following features.

- `with_bigdecimal` - Enable decimal via [bigdecimal](https://crates.io/crates/bigdecimal).
- `with_rust_decimal` - Enable decimal via [rust_decimal](https://crates.io/crates/rust_decimal).
- `with_chrono` - Enable date/time via [chrono](https://crates.io/crates/chrono).
- `with_time` - Enable date/time via [time](https://crates.io/crates/time).

## Rust version requirements

The Minimum Supported Rust Version (MSRV) is currently **Rust 1.75.0**.

## How to use

Add `tsubakuro-rust-core` as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
tsubakuro-rust-core = "0.1.0"
```



## Example

### connect example

```rust
use std::time::Duration;
use log::warn;
use tsubakuro_rust_core::prelude::*;

async fn example() -> Result<(), TgError> {
    let endpoint = Endpoint::parse("tcp://localhost:12345")?;

    let mut connection_option = ConnectionOption::new();
    connection_option.set_endpoint(endpoint);
    connection_option.set_application_name("Tsubakuro/Rust example");
    connection_option.set_session_label("example session");
    connection_option.set_default_timeout(Duration::from_secs(10));

    // connect
    let session = Session::connect(&connection_option).await?;

    // make SqlClient
    let client: SqlClient = session.make_client();

    // execute SQL
    let result = example_transaction(&client).await;

    // session close
    if let Err(e) = session.close().await {
        warn!("session close error. {}", e);
    }

    result
}
```

### transaction example

```rust
async fn example_transaction(client: &SqlClient) -> Result<(), TgError> {
    // transaction start
    let mut transaction_option = TransactionOption::new();
    transaction_option.set_transaction_type(TransactionType::Short);
    let transaction = client.start_transaction(&transaction_option).await?;

    // execute SQL
    let mut result = example_sql(&client, &transaction).await;

    // transaction commit
    if result.is_ok() {
        let commit_option = CommitOption::default();
        result = client.commit(&transaction, &commit_option).await;
    }

    // transaction close
    if let Err(e) = transaction.close().await {
        warn!("transaction close error. {}", e);
    }

    result
}
```

### execute SQL(update) example

```rust
async fn example_statement(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    let sql = "update customer set c_age = 2 where c_id = 3";
    let execute_result = client.execute(&transaction, sql).await?;
    println!("updated_rows={}", execute_result.updated_rows());

    Ok(())
}
```

### execute SQL(select) example

```rust
async fn example_query(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    let sql = "select c_id, c_name, c_age from customer order by c_id";
    let mut query_result = client.query(&transaction, sql).await?;

    while query_result.next_row().await? {
        assert!(query_result.next_column().await?);
        let id: i64 = query_result.fetch().await?; // not null

        assert!(query_result.next_column().await?);
        let name: Option<String> = query_result.fetch().await?; // nullable

        assert!(query_result.next_column().await?);
        let age: Option<i32> = query_result.fetch().await?; // nullable

        println!("id={id}, name={name:?}, age={age:?}");
    }

    Ok(())
}
```

### prepared statement(insert) example

```rust
async fn example_prepared_statement_insert(
    client: &SqlClient,
    transaction: &Transaction,
) -> Result<(), TgError> {
    let sql = "insert into customer values(:id, :name, :age)";
    let placeholders = vec![
        SqlPlaceholder::of::<i64>("id"),
        SqlPlaceholder::of::<String>("name"),
        SqlPlaceholder::of::<i32>("age"),
    ];
    let ps = client.prepare(sql, placeholders).await?;

    let result = example_prepared_execute(client, transaction, &ps).await;

    if let Err(e) = ps.close().await {
        warn!("preparedStatement close error. {}", e);
    }

    result
}

async fn example_prepared_execute(
    client: &SqlClient,
    transaction: &Transaction,
    ps: &SqlPreparedStatement,
) -> Result<(), TgError> {
    let parameters = vec![
        SqlParameter::of("id", 4_i64),
        SqlParameter::of("name", "example"),
        SqlParameter::of("age", 20_i32),
    ];
    let execute_result = client.prepared_execute(transaction, ps, parameters).await?;
    println!("inserted_rows={}", execute_result.inserted_rows());

    Ok(())
}
```

### prepared statement(select) example

```rust
async fn example_prepared_statement_query(
    client: &SqlClient,
    transaction: &Transaction,
) -> Result<(), TgError> {
    let sql = "select c_id, c_name, c_age from customer order by c_id";
    let placeholders = vec![];
    let ps = client.prepare(sql, placeholders).await?;

    let result = example_prepared_query(client, transaction, &ps).await;

    if let Err(e) = ps.close().await {
        warn!("preparedStatement close error. {}", e);
    }

    result
}

async fn example_prepared_query(
    client: &SqlClient,
    transaction: &Transaction,
    ps: &SqlPreparedStatement,
) -> Result<(), TgError> {
    let parameters = vec![];
    let mut query_result = client.prepared_query(transaction, ps, parameters).await?;

    while query_result.next_row().await? {
        assert!(query_result.next_column().await?);
        let id: i32 = query_result.fetch().await?; // not null

        assert!(query_result.next_column().await?);
        let name: Option<String> = query_result.fetch().await?; // nullable

        assert!(query_result.next_column().await?);
        let age: Option<i64> = query_result.fetch().await?; // nullable

        println!("id={id}, name={name:?}, age={age:?}");
    }

    Ok(())
}
```



## How to build

Need to install [protoc](https://github.com/protocolbuffers/protobuf?tab=readme-ov-file#protobuf-compiler-installation) since used [prost](https://crates.io/crates/prost).

If proto files in [tsubakuro-proto](https://github.com/project-tsurugi/tsubakuro/tree/master/modules/proto) has been modified, copy from there.

```bash
cd tsubakuro-rust-core
cp -rp /path/to/tsubakuro/modules/proto/src/main/protos .
```

Then build with `cargo`.

```bash
cd tsubakuro-rust-core
cargo build
```

## How to test

```bash
cd tsubakuro-rust-core
cargo test
```

See also [tsubakuro-rust-dbtest](../tsubakuro-rust-dbtest).

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)