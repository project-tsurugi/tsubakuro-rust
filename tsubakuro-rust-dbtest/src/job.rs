use std::time::Duration;

use tsubakuro_rust_client::prelude::*;

use crate::create_connection_option;

const TIMEOUT: Duration = Duration::from_secs(10);

pub(super) async fn execute(endpoint: &str) -> Result<(), TgError> {
    println!("job::execute start");

    let connection_option = create_connection_option(endpoint)?;

    let session_job = Session::connect_async(&connection_option, TIMEOUT).await?;
    let session = session_job.take(TIMEOUT).await?;
    println!("{session:?}");

    let mut client: SqlClient = session.make_client();
    client.set_default_timeout(TIMEOUT);

    drop_table_if_exists(&client, "test").await?;
    execute_statement(
        &client,
        "
create table test (
  foo int primary key,
  bar bigint,
  zzz varchar(10)
)",
    )
    .await?;

    list_tables(&client).await?;

    println!("insert start");
    let r = execute_statement(&client, "insert into test values(1, 11, 'aaa')").await?;
    assert_eq!(1, r.inserted_rows());
    assert_eq!(1, r.rows());
    let r = execute_statement(&client, "insert into test values(2, 22, 'bbb')").await?;
    assert_eq!(1, r.inserted_rows());
    assert_eq!(1, r.rows());
    let r = execute_statement(&client, "insert into test values(3, 33, 'ccc')").await?;
    assert_eq!(1, r.inserted_rows());
    assert_eq!(1, r.rows());
    println!("insert end");

    println!("select start");
    {
        let tx = start_occ(&client).await?;

        let sql = "select * from test order by foo";
        let job = client.execute_query_async(&tx, sql).await?;
        let mut result = job.take(TIMEOUT).await?;
        result.set_default_timeout(TIMEOUT);
        while result.next_row().await? {
            assert_eq!(true, result.next_column().await?);
            let foo: i32 = result.fetch().await?;
            assert_eq!(true, result.next_column().await?);
            let bar: i64 = result.fetch().await?;
            assert_eq!(true, result.next_column().await?);
            let zzz: String = result.fetch().await?;
            assert_eq!(false, result.next_column().await?);
            println!("{foo}, {bar}, {zzz}");
        }

        commit(&client, &tx).await?;
    }
    println!("select end");

    println!("job::execute end");
    Ok(())
}

async fn list_tables(client: &SqlClient) -> Result<(), TgError> {
    let job = client.list_tables_async().await?;
    let table_list = job.take(TIMEOUT).await?;
    println!("list_tables={:?}", table_list.get_table_names());
    Ok(())
}

async fn start_occ(client: &SqlClient) -> Result<Transaction, TgError> {
    let option = TransactionOption::from(TransactionType::Occ);
    client.start_transaction(&option).await
}

async fn commit(client: &SqlClient, transaction: &Transaction) -> Result<(), TgError> {
    let option = CommitOption::new();
    let job = client.commit_async(transaction, &option).await?;
    job.take(TIMEOUT).await?;

    transaction.close().await?;
    Ok(())
}

async fn execute_statement(client: &SqlClient, sql: &str) -> Result<SqlExecuteResult, TgError> {
    let tx = start_occ(client).await?;

    let job = client.execute_statement_async(&tx, &sql).await?;
    let result = job.take(TIMEOUT).await?;

    commit(&client, &tx).await?;
    Ok(result)
}

async fn drop_table_if_exists(client: &SqlClient, table_name: &str) -> Result<(), TgError> {
    let sql = format!("drop table if exists {table_name}");
    let r = execute_statement(client, &sql).await?;
    assert_eq!(0, r.rows());
    Ok(())
}
