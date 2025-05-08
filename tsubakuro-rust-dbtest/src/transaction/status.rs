#[cfg(test)]
mod test {
    use crate::test::{create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn transaction_status() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let status = client.get_transaction_status(&transaction).await.unwrap();
        assert_eq!(TransactionStatus::Running, status.status());
        assert_eq!("", status.message());

        transaction.close().await.unwrap();
    }

    #[test]
    async fn transaction_status_async() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let mut job = client
            .get_transaction_status_async(&transaction)
            .await
            .unwrap();
        assert_eq!("TransactionStatus", job.name());
        let status = job.take().await.unwrap();
        assert_eq!(TransactionStatus::Running, status.status());
        assert_eq!("", status.message());

        transaction.close().await.unwrap();
    }
}
