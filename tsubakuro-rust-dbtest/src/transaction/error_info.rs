#[cfg(test)]
mod test {
    use crate::test::{create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn normal() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let info = client
            .get_transaction_error_info(&transaction)
            .await
            .unwrap();
        assert_eq!(true, info.server_error().is_none());
        assert_eq!(true, info.is_normal());
        assert_eq!(false, info.is_error());
        assert_eq!(None, info.diagnostic_code());

        transaction.close().await.unwrap();
    }

    #[test]
    async fn normal_async() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let mut job = client
            .get_transaction_error_info_async(&transaction)
            .await
            .unwrap();
        assert_eq!("TransactionErrorInfo", job.name());
        let info = job.take().await.unwrap();
        assert_eq!(true, info.server_error().is_none());
        assert_eq!(true, info.is_normal());
        assert_eq!(false, info.is_error());
        assert_eq!(None, info.diagnostic_code());

        transaction.close().await.unwrap();
    }

    #[test]
    async fn error() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let error = client.execute(&transaction, "").await.unwrap_err();
        let error_code;
        let error_message;
        if let TgError::ServerError(_, _, code, message) = &error {
            error_code = code;
            error_message = message;
        } else {
            panic!("{error}");
        }

        let info = client
            .get_transaction_error_info(&transaction)
            .await
            .unwrap();
        if let Some(TgError::ServerError(_, _, code, message)) = info.server_error() {
            assert_eq!(error_code, code);
            assert_eq!(error_message, message);
        } else {
            panic!("left={:?}, right={:?}", error, info.server_error());
        }
        assert_eq!(false, info.is_normal());
        assert_eq!(true, info.is_error());
        assert_eq!(Some(error_code), info.diagnostic_code());

        transaction.close().await.unwrap();
    }

    #[test]
    async fn error_async() {
        let client = create_test_sql_client().await;

        let transaction = start_occ(&client).await;
        let error = client.execute(&transaction, "").await.unwrap_err();
        let error_code;
        let error_message;
        if let TgError::ServerError(_, _, code, message) = &error {
            error_code = code;
            error_message = message;
        } else {
            panic!("{error}");
        }

        let mut job = client
            .get_transaction_error_info_async(&transaction)
            .await
            .unwrap();
        let info = job.take().await.unwrap();
        if let Some(TgError::ServerError(_, _, code, message)) = info.server_error() {
            assert_eq!(error_code, code);
            assert_eq!(error_message, message);
        } else {
            panic!("left={:?}, right={:?}", error, info.server_error());
        }
        assert_eq!(false, info.is_normal());
        assert_eq!(true, info.is_error());
        assert_eq!(error_code, info.diagnostic_code().unwrap());

        transaction.close().await.unwrap();
    }
}
