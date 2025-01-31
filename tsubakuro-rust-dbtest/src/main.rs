use std::{env, process, time::Duration};

use execute::{job::execute as job_execute, sub::execute as sub_execute};
use tsubakuro_rust_core::prelude::*;

mod execute;
mod job;
mod service;
mod session;
mod transaction;

#[tokio::main]
async fn main() -> Result<(), TgError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} <endpoint-url>", args[0]);
        process::exit(1);
    }

    let endpoint = args.get(1).unwrap();

    // env_logger::builder().format_timestamp_millis().init();
    sub_execute(endpoint).await?;
    job_execute(endpoint).await?;

    Ok(())
}

fn create_connection_option(endpoint: &str) -> Result<ConnectionOption, TgError> {
    let mut option = ConnectionOption::new();
    option.set_application_name("tsubakuro-rust-dbtest");
    option.set_endpoint_url(endpoint)?;
    option.set_keep_alive(Duration::ZERO); // not keep alive
    option.set_default_timeout(Duration::from_secs(10));

    Ok(option)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, sync::Arc};

    pub(crate) fn create_test_connection_option() -> ConnectionOption {
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
        option
    }

    pub(crate) async fn create_test_session() -> Arc<Session> {
        let option = create_test_connection_option();
        let session = Session::connect(&option).await.unwrap();
        session.set_fail_on_drop_error(true);
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
        // println!("drop_table_if_exists() start");
        execute_ddl(client, &format!("drop table if exists {table_name}")).await;
        // println!("drop_table_if_exists() end");
    }

    pub(crate) async fn execute_ddl(client: &SqlClient, sql: &str) {
        let transaction = start_occ(client).await;

        // println!("execute_ddl() execute start");
        client.execute(&transaction, sql).await.unwrap();
        // println!("execute_ddl() execute end");

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
        // println!("commit start");
        client.commit(&transaction, &commit_option).await.unwrap();
        // println!("commit end");

        // println!("transaction.close() start");
        transaction.close().await.unwrap();
        // println!("transaction.close() end");
    }
}
