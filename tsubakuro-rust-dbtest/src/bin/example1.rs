use std::time::Duration;

use tsubakuro_rust_core::prelude::*;

#[tokio::main]
async fn main() -> Result<(), TgError> {
    example().await
}

async fn example() -> Result<(), TgError> {
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
