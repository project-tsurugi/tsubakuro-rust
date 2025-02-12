# tsubakuro-rust-core

tsubakuro-rust-core is the core library to access Tsurugi written in Rust.

## How to use

```toml
[dependencies]
tsubakuro-rust-core = { path = "/path/to/tsubakuro-rust-core" }
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
        let id: i64 = query_result.fetch().await?;

        assert!(query_result.next_column().await?);
        let name: String = query_result.fetch().await?;

        assert!(query_result.next_column().await?);
        let age: i32 = query_result.fetch().await?;

        println!("id={id}, name={name}, age={age}");
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
        let id: i32 = query_result.fetch().await?;

        assert!(query_result.next_column().await?);
        let name: String = query_result.fetch().await?;

        assert!(query_result.next_column().await?);
        let age: i64 = query_result.fetch().await?;

        println!("id={id}, name={name}, age={age}");
    }

    Ok(())
}
```



## How to build

First, copy the proto files from [tsubakuro-proto](https://github.com/project-tsurugi/tsubakuro/tree/master/modules/proto).

```bash
cd tsubakuro-rust-core
cp -rp tsubakuro/modules/proto/src/main/protos .
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