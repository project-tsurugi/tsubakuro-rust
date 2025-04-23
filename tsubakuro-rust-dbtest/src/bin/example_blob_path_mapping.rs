use std::time::Duration;

use log::warn;
use tsubakuro_rust_core::prelude::*;

/*
docker run -d -p 12345:12345 --name tsurugi -v D:/tmp/client:/mnt/client -v D:/tmp/tsurugi:/opt/tsurugi/var/data/log ghcr.io/project-tsurugi/tsurugidb:latest
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
    connection_option.add_large_object_path_mapping_on_send("D:/tmp/client", "/mnt/client");
    connection_option
        .add_large_object_path_mapping_on_recv("/opt/tsurugi/var/data/log", "D:/tmp/tsurugi");
    connection_option.set_default_timeout(Duration::from_secs(10));

    // connect
    let session = Session::connect(&connection_option).await?;

    // make SqlClient
    let client: SqlClient = session.make_client();

    // execute SQL
    let mut result = example_transaction1(&client).await;
    if result.is_ok() {
        result = example_transaction2(&client).await;
    }

    // session close
    if let Err(e) = session.close().await {
        warn!("session close error. {}", e);
    }

    result
}

/// transaction example
async fn example_transaction1(client: &SqlClient) -> Result<(), TgError> {
    // transaction start
    let mut transaction_option = TransactionOption::new();
    transaction_option.set_transaction_type(TransactionType::Short);
    let transaction = client.start_transaction(&transaction_option).await?;

    // execute SQL
    let mut result = example_create_table(&client, &transaction).await;

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

/// execute SQL (update) example
async fn example_create_table(
    client: &SqlClient,
    transaction: &Transaction,
) -> Result<(), TgError> {
    println!("---example_create_table---");

    let sql = "drop table if exists blob_example";
    client.execute(&transaction, sql).await?;

    let sql = "create table blob_example (
  pk int primary key,
  value blob
)";
    client.execute(&transaction, sql).await?;

    Ok(())
}

/// transaction example
async fn example_transaction2(client: &SqlClient) -> Result<(), TgError> {
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
    example_blob_insert(client, transaction).await?;
    example_blob_query(client, transaction).await?;
    Ok(())
}

/// BLOB insert example
async fn example_blob_insert(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    println!("---example_blob_insert---");

    let sql = "insert into blob_example values(:pk, :value)";
    let placeholders = vec![
        SqlPlaceholder::of::<i32>("pk"),
        SqlPlaceholder::of::<TgBlob>("value"),
    ];
    let ps = client.prepare(sql, placeholders).await?;

    let result = example_blob_insert_execute(client, transaction, &ps).await;

    if let Err(e) = ps.close().await {
        warn!("preparedStatement close error. {}", e);
    }

    result
}

/// execute BLOB insert example
async fn example_blob_insert_execute(
    client: &SqlClient,
    transaction: &Transaction,
    ps: &SqlPreparedStatement,
) -> Result<(), TgError> {
    let blob_file = "D:/tmp/client/send-rust-blob.dat";
    let data = vec![0x31_u8, 0x32_u8, 0x33_u8];
    std::fs::write(blob_file, data).unwrap();

    let parameters = vec![
        SqlParameter::of("pk", 1_i32),
        SqlParameter::of("value", TgBlob::new(blob_file)),
    ];
    let execute_result = client.prepared_execute(transaction, ps, parameters).await?;
    println!("inserted_rows={}", execute_result.inserted_rows());

    Ok(())
}

/// BLOB select example
async fn example_blob_query(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    println!("---example_blob_query---");

    let sql = "select * from blob_example";
    let mut query_result = client.query(&transaction, sql).await?;

    while query_result.next_row().await? {
        assert!(query_result.next_column().await?);
        let pk: i32 = query_result.fetch().await?;

        assert!(query_result.next_column().await?);
        let blob: TgBlobReference = query_result.fetch().await?;
        let value = client.read_blob(transaction, &blob).await?;

        println!("pk={pk}, value={value:?}");
    }

    Ok(())
}
