use std::{env, process, time::Duration};

use execute::{job::execute as job_execute, sub::execute as sub_execute};
use tsubakuro_rust_core::prelude::*;

mod credential;
mod execute;
mod job;
mod service;
mod session;
mod transaction;

#[tokio::main]
async fn main() -> Result<(), TgError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 3 {
        eprintln!("Usage: {} <endpoint-url> <user> <password>", args[0]);
        process::exit(1);
    }

    let endpoint = args.get(1).unwrap();
    let user = args.get(2).unwrap();
    let password = args.get(3).unwrap();
    let credential = Credential::from_user_password(user, Some(password));

    // env_logger::builder().format_timestamp_millis().init();
    sub_execute(endpoint, credential.clone()).await?;
    job_execute(endpoint, credential).await?;

    Ok(())
}

fn create_connection_option(
    endpoint: &str,
    credential: Credential,
) -> Result<ConnectionOption, TgError> {
    let mut option = ConnectionOption::new();
    option.set_application_name("tsubakuro-rust-dbtest");
    option.set_endpoint_url(endpoint)?;
    option.set_credential(credential);
    option.set_keep_alive(Duration::ZERO); // not keep alive
    option.set_default_timeout(Duration::from_secs(10));

    Ok(option)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{env, panic, sync::Arc};

    pub(crate) fn create_test_connection_option() -> ConnectionOption {
        let args = create_test_args();
        let option = create_connection_option(args.endpoint(), args.credential()).unwrap();
        option
    }

    pub(crate) fn create_test_args() -> TestArgs {
        let env_args: Vec<String> = env::args().collect();

        let mut args = TestArgs::new();
        for arg in env_args {
            if arg.starts_with("endpoint=") {
                args.endpoint = arg.clone().split_off(9);
            } else if arg.starts_with("user=") {
                args.user = Some(arg.clone().split_off(5));
            } else if arg.starts_with("password=") {
                args.password = Some(arg.clone().split_off(9));
            } else if arg.starts_with("auth-token=") {
                args.auth_token = Some(arg.clone().split_off(11));
            } else if arg.starts_with("credentials=") {
                args.file_path = Some(arg.clone().split_off(12));
            }
        }

        if args.endpoint.is_empty() {
            panic!("endpoint is not specified");
        }

        args
    }

    pub(crate) struct TestArgs {
        endpoint: String,
        user: Option<String>,
        password: Option<String>,
        auth_token: Option<String>,
        file_path: Option<String>,
    }

    impl TestArgs {
        fn new() -> Self {
            Self {
                endpoint: String::new(),
                user: None,
                password: None,
                auth_token: None,
                file_path: None,
            }
        }

        pub fn endpoint(&self) -> &str {
            &self.endpoint
        }

        pub fn user(&self) -> Option<&String> {
            self.user.as_ref()
        }

        pub fn credential(&self) -> Credential {
            if let Some(c) = self.user_password_credential() {
                c
            } else if let Some(c) = self.auth_token_credential() {
                c
            } else if let Some(c) = self.file_credential() {
                c
            } else {
                Credential::null()
            }
        }

        pub fn user_password_credential(&self) -> Option<Credential> {
            if let Some(user) = &self.user {
                Some(Credential::from_user_password(user, self.password.as_ref()))
            } else {
                None
            }
        }

        pub fn auth_token_credential(&self) -> Option<Credential> {
            if let Some(token) = &self.auth_token {
                Some(Credential::from_auth_token(token))
            } else {
                None
            }
        }

        pub fn file_credential(&self) -> Option<Credential> {
            if let Some(path) = &self.file_path {
                Some(Credential::load(path).unwrap())
            } else {
                None
            }
        }
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
