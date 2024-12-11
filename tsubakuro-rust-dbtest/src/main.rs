use std::{env, process, time::Duration};

use tsubakuro_rust_client::prelude::*;

mod job;
mod service;
mod sub;

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

        // println!("+++---create_table({table_name}) start");
        execute_ddl(client, sql).await;
        // println!("+++---create_table({table_name}) end");
    }

    pub(crate) async fn drop_table_if_exists(client: &SqlClient, table_name: &str) {
        // println!("+++---drop_table_if_exists({table_name}) start");
        execute_ddl(client, &format!("drop table if exists {table_name}")).await;
        // println!("+++---drop_table_if_exists({table_name}) end");
    }

    pub(crate) async fn execute_ddl(client: &SqlClient, sql: &str) {
        let transaction_option = TransactionOption::from(TransactionType::Occ);
        let transaction = client.start_transaction(&transaction_option).await.unwrap();

        client.execute_statement(&transaction, sql).await.unwrap();

        let commit_option = CommitOption::new();
        client.commit(&transaction, &commit_option).await.unwrap();

        transaction.close().await.unwrap()
    }
}
