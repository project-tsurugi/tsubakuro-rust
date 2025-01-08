#[cfg(test)]
mod test {
    use crate::test::{create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn normal() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let status = client.get_transaction_status(&transaction).await.unwrap();
        assert_eq!(true, status.server_error().is_none());
        assert_eq!(true, status.is_normal());
        assert_eq!(false, status.is_error());
        assert_eq!(None, status.diagnostic_code());

        transaction.close().await.unwrap();
    }

    #[test]
    async fn normal_async() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let mut job = client
            .get_transaction_status_async(&transaction)
            .await
            .unwrap();
        assert_eq!("TransactionStatus", job.name());
        let status = job.take().await.unwrap();
        assert_eq!(true, status.server_error().is_none());
        assert_eq!(true, status.is_normal());
        assert_eq!(false, status.is_error());
        assert_eq!(None, status.diagnostic_code());

        transaction.close().await.unwrap();
    }

    #[test]
    async fn error() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let error = client.execute(&transaction, "").await.unwrap_err();
        let error_code;
        let error_message;
        if let TgError::ServerError(_, code, message) = &error {
            error_code = code;
            error_message = message;
        } else {
            panic!("{error}");
        }

        let status = client.get_transaction_status(&transaction).await.unwrap();
        if let Some(TgError::ServerError(_, code, message)) = status.server_error() {
            assert_eq!(error_code, code);
            assert_eq!(error_message, message);
        } else {
            panic!("left={:?}, right={:?}", error, status.server_error());
        }
        assert_eq!(false, status.is_normal());
        assert_eq!(true, status.is_error());
        assert_eq!(Some(error_code), status.diagnostic_code());

        transaction.close().await.unwrap();
    }

    #[test]
    async fn error_async() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let error = client.execute(&transaction, "").await.unwrap_err();
        let error_code;
        let error_message;
        if let TgError::ServerError(_, code, message) = &error {
            error_code = code;
            error_message = message;
        } else {
            panic!("{error}");
        }

        let mut job = client
            .get_transaction_status_async(&transaction)
            .await
            .unwrap();
        let status = job.take().await.unwrap();
        if let Some(TgError::ServerError(_, code, message)) = status.server_error() {
            assert_eq!(error_code, code);
            assert_eq!(error_message, message);
        } else {
            panic!("left={:?}, right={:?}", error, status.server_error());
        }
        assert_eq!(false, status.is_normal());
        assert_eq!(true, status.is_error());
        assert_eq!(error_code, status.diagnostic_code().unwrap());

        transaction.close().await.unwrap();
    }
}
