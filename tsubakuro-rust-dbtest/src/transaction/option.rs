#[cfg(test)]
mod test {
    use std::{ops::Add, time::Duration};

    use crate::test::{create_table, create_test_sql_client, drop_table_if_exists};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn occ() {
        let client = create_test_sql_client().await;

        let mut option = TransactionOption::new();
        option.set_transaction_type(TransactionType::Occ);
        option.set_transaction_label("tsubakuro-rust-dbtest.occ");

        let transaction = client.start_transaction(&option).await.unwrap();
        transaction.close().await.unwrap();
    }

    #[test]
    async fn ltx() {
        ltx_test_main(|option| {
            option.set_write_preserve(&vec!["test"]);
        })
        .await;
    }

    async fn ltx_test_main<F: Fn(&mut TransactionOption) -> ()>(f: F) {
        let client = create_test_sql_client().await;
        {
            create_table(&client, "test", "create table test (pk int primary key)").await;

            let mut option = TransactionOption::new();
            option.set_transaction_type(TransactionType::Ltx);
            option.set_transaction_label("tsubakuro-rust-dbtest.ltx");
            f(&mut option);

            let transaction = client.start_transaction(&option).await.unwrap();
            transaction.close().await.unwrap();
        }
        {
            drop_table_if_exists(&client, "test").await;

            let mut option = TransactionOption::new();
            option.set_transaction_type(TransactionType::Ltx);
            option.set_transaction_label("tsubakuro-rust-dbtest.ltx");
            f(&mut option);

            let error = client.start_transaction(&option).await.unwrap_err();
            if let TgError::ServerError(_message, code, _server_message) = error {
                assert_eq!("SQL-02014", code.structured_code());
                assert_eq!("TARGET_NOT_FOUND_EXCEPTION", code.name());
            } else {
                panic!("{error:?}")
            }
        }
    }

    #[test]
    async fn inclusive_read_area() {
        ltx_test_main(|option| {
            option.set_inclusive_read_area(&vec!["test"]);
        })
        .await;
    }

    #[test]
    async fn exclusive_read_area() {
        ltx_test_main(|option| {
            option.set_exclusive_read_area(&vec!["test"]);
        })
        .await;
    }

    #[test]
    async fn modifies_definitions() {
        let client = create_test_sql_client().await;
        drop_table_if_exists(&client, "test").await;

        let mut option = TransactionOption::new();
        option.set_transaction_type(TransactionType::Ltx);
        option.set_transaction_label("tsubakuro-rust-dbtest.ltx");
        option.set_modifies_definitions(true);

        let transaction = client.start_transaction(&option).await.unwrap();
        client
            .execute_statement(&transaction, "create table test (pk int)")
            .await
            .unwrap();
        client
            .commit(&transaction, &CommitOption::new())
            .await
            .unwrap();
        transaction.close().await.unwrap();
    }

    #[test]
    async fn rtx() {
        let client = create_test_sql_client().await;

        let mut option = TransactionOption::new();
        option.set_transaction_type(TransactionType::Rtx);
        option.set_transaction_label("tsubakuro-rust-dbtest.rtx");

        let transaction = client.start_transaction(&option).await.unwrap();
        transaction.close().await.unwrap();
    }

    #[test]
    async fn close_timeout() {
        let client = create_test_sql_client().await;

        {
            let timeout = client.default_timeout().add(Duration::from_secs(1));

            let mut option = TransactionOption::new();
            option.set_transaction_label("tsubakuro-rust-dbtest.tx.close_timeout");
            option.set_close_timeout(timeout);

            let transaction = client.start_transaction(&option).await.unwrap();
            assert_eq!(timeout, transaction.close_timeout());
            transaction.close().await.unwrap();
        }
        {
            let mut option = TransactionOption::new();
            option.set_transaction_label("tsubakuro-rust-dbtest.tx.close_timeout");

            let transaction = client.start_transaction(&option).await.unwrap();
            assert_eq!(client.default_timeout(), transaction.close_timeout());
            transaction.close().await.unwrap();
        }
    }
}
