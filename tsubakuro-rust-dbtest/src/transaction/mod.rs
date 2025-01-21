mod option;
mod status;

#[cfg(test)]
mod test {
    use crate::test::{create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn transaction_close() {
        let client = create_test_sql_client().await;

        for _i in 0..30 {
            transaction_dispose_test(&client, true, false).await;
        }
    }

    async fn transaction_dispose_test(client: &SqlClient, close: bool, logger_init: bool) {
        let tx = start_occ(client).await;

        if logger_init {
            env_logger::builder().is_test(true).try_init().unwrap();
        }
        if close {
            tx.close().await.unwrap();
        }
    }

    #[test]
    async fn transaction_drop() {
        let client = create_test_sql_client().await;

        for _i in 0..10 {
            transaction_dispose_test(&client, false, false).await;
        }
    }

    // #[test]
    async fn _transaction_drop1() {
        let client = create_test_sql_client().await;

        transaction_dispose_test(&client, false, true).await;
    }
}
