use std::{env, process, time::Duration};

use tsubakuro_rust_core::prelude::*;

mod job;
mod service;
mod sub;
mod transaction;

#[tokio::main]
async fn main() -> Result<(), TgError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} <endpoint-url>", args[0]);
        process::exit(1);
    }

    let endpoint = match args.get(1) {
        Some(arg) => arg,
        None => panic!(),
    };

    sub::execute(endpoint).await?;
    job::execute(endpoint).await?;

    Ok(())
}

fn create_connection_option(endpoint: &str) -> Result<ConnectionOption, TgError> {
    let mut option = ConnectionOption::new();
    option.set_application_name("tsubakuro-rust-dbtest");
    option.set_endpoint_url(endpoint)?;
    option.set_default_timeout(Duration::from_secs(10));

    Ok(option)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, sync::Arc};

    pub(crate) async fn create_test_session() -> Arc<Session> {
        let args: Vec<String> = env::args().collect();

        let endpoint = {
            let mut i = 0;
            loop {
                if let Some(s) = args.get(i) {
                    if s.starts_with("endpoint=") {
                        break s.clone().split_off(9);
                    }
                } else {
                    panic!("endpoint not specified");
                }
                i += 1;
            }
        };

        let option = create_connection_option(&endpoint).unwrap();
        let session = Session::connect(&option).await.unwrap();
        session
    }

    pub(crate) async fn create_test_sql_client() -> SqlClient {
        let session = create_test_session().await;
        session.make_client()
    }

    pub(crate) async fn create_table(client: &SqlClient, table_name: &str, sql: &str) {
        drop_table_if_exists(client, table_name).await;

        execute_ddl(client, sql).await;
    }

    pub(crate) async fn drop_table_if_exists(client: &SqlClient, table_name: &str) {
        execute_ddl(client, &format!("drop table if exists {table_name}")).await;
    }

    pub(crate) async fn execute_ddl(client: &SqlClient, sql: &str) {
        let transaction = start_occ(client).await;

        client.execute(&transaction, sql).await.unwrap();

        commit_and_close(client, &transaction).await;
    }

    pub(crate) async fn start_occ(client: &SqlClient) -> Transaction {
        let mut option = TransactionOption::from(TransactionType::Short);
        option.set_transaction_label("tsubakuro-rust-dbtest.occ");
        let transaction = client.start_transaction(&option).await.unwrap();
        transaction
    }

    pub(crate) async fn commit_and_close(client: &SqlClient, transaction: &Transaction) {
        let commit_option = CommitOption::new();
        client.commit(&transaction, &commit_option).await.unwrap();

        transaction.close().await.unwrap()
    }
}
