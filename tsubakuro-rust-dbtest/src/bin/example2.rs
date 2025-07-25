use std::time::Duration;

use log::warn;
use tsubakuro_rust_core::prelude::*;

/*
The table should be created before execution.

create table customer (
    c_id   bigint primary key,
    c_name varchar(30),
    c_age  int
);
insert into customer values(1, 'Hello', 51);
insert into customer values(2, 'World', 138);
insert into customer values(3, 'Tsurugi', 1);
*/

#[tokio::main]
async fn main() -> Result<(), TgError> {
    example().await
}

/// connect example
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

/// transaction example
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

/// execute SQLs
async fn example_sql(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    example_statement(client, transaction).await?;
    example_query(client, transaction).await?;
    example_prepared_statement_insert(client, transaction).await?;
    example_prepared_statement_query0(client, transaction).await?;
    example_prepared_statement_query1(client, transaction).await?;
    Ok(())
}

/// execute SQL (update) example
async fn example_statement(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    println!("---example_statement---");

    let sql = "update customer set c_age = 2 where c_id = 3";
    let execute_result = client.execute(&transaction, sql).await?;
    println!("updated_rows={}", execute_result.updated_rows());

    Ok(())
}

/// execute SQL (select) example
async fn example_query(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    println!("---example_query---");

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

    query_result.close().await?;

    Ok(())
}

/// PreparedStatement (insert) example
async fn example_prepared_statement_insert(
    client: &SqlClient,
    transaction: &Transaction,
) -> Result<(), TgError> {
    println!("---example_prepared_statement_insert---");

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

/// execute PreparedStatement (insert) example
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

/// PreparedStatement (select) example
async fn example_prepared_statement_query0(
    client: &SqlClient,
    transaction: &Transaction,
) -> Result<(), TgError> {
    println!("---example_prepared_statement_query0---");

    let sql = "select c_id, c_name, c_age from customer order by c_id";
    let placeholders = vec![];
    let ps = client.prepare(sql, placeholders).await?;

    let result = example_prepared_query0(client, transaction, &ps).await;

    if let Err(e) = ps.close().await {
        warn!("preparedStatement close error. {}", e);
    }

    result
}

/// execute PreparedStatement (select) example
async fn example_prepared_query0(
    client: &SqlClient,
    transaction: &Transaction,
    ps: &SqlPreparedStatement,
) -> Result<(), TgError> {
    let parameters = vec![];
    let mut query_result = client.prepared_query(transaction, ps, parameters).await?;

    while query_result.next_row().await? {
        assert!(query_result.next_column().await?);
        let id: i64 = query_result.fetch().await?; // not null

        assert!(query_result.next_column().await?);
        let name: Option<String> = query_result.fetch().await?; // nullable

        assert!(query_result.next_column().await?);
        let age: Option<i32> = query_result.fetch().await?; // nullable

        println!("id={id}, name={name:?}, age={age:?}");
    }

    query_result.close().await?;

    Ok(())
}

/// PreparedStatement (select) example
async fn example_prepared_statement_query1(
    client: &SqlClient,
    transaction: &Transaction,
) -> Result<(), TgError> {
    println!("---example_prepared_statement_query1---");

    let sql = "select * from customer where c_id = :id";
    let placeholders = vec![SqlPlaceholder::of::<i64>("id")];
    let ps = client.prepare(sql, placeholders).await?;

    let result = example_prepared_query1(client, transaction, &ps).await;

    if let Err(e) = ps.close().await {
        warn!("preparedStatement close error. {}", e);
    }

    result
}

/// execute PreparedStatement (select) example
async fn example_prepared_query1(
    client: &SqlClient,
    transaction: &Transaction,
    ps: &SqlPreparedStatement,
) -> Result<(), TgError> {
    let parameters = vec![SqlParameter::of("id", 3_i64)];
    let mut query_result = client.prepared_query(transaction, ps, parameters).await?;

    while query_result.next_row().await? {
        assert!(query_result.next_column().await?);
        let id: i64 = query_result.fetch().await?; // not null

        assert!(query_result.next_column().await?);
        let name: Option<String> = query_result.fetch().await?; // nullable

        assert!(query_result.next_column().await?);
        let age: Option<i32> = query_result.fetch().await?; // nullable

        println!("id={id}, name={name:?}, age={age:?}");
    }

    query_result.close().await?;

    Ok(())
}
