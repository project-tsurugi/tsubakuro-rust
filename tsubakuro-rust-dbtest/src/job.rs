#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::test::create_test_sql_client;
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn take() {
        let client = create_test_sql_client().await;

        let mut job = client.list_tables_async().await.unwrap();

        let wait = job.wait(Duration::from_secs(10)).await.unwrap();
        assert_eq!(true, wait);

        let done = job.is_done().await.unwrap();
        assert_eq!(true, done);

        let _result = job.take().await.unwrap();

        // second take
        let error = job.take().await.unwrap_err();
        match error {
            TgError::ClientError(message, _cause) => {
                assert_eq!("Job<ListTables> already taked", message);
            }
            _ => panic!("{error:?}"),
        }
    }

    #[test]
    async fn take_if_ready() {
        for _i in 0..20 {
            let done = take_if_ready_main().await;
            if done {
                return;
            }
        }
        eprintln!("take_if_ready test end (no wait)");
    }

    async fn take_if_ready_main() -> bool {
        let client = create_test_sql_client().await;

        let mut job = client.list_tables_async().await.unwrap();

        // FIXME 処理の完了を遅延させたい

        let result = job.take_if_ready().await.unwrap();
        if result.is_some() {
            return false;
        }

        let wait = job.wait(Duration::from_secs(10)).await.unwrap();
        assert_eq!(true, wait);

        let result = job.take_if_ready().await.unwrap();
        assert_eq!(true, result.is_some());

        // second take
        let error = job.take_if_ready().await.unwrap_err();
        match error {
            TgError::ClientError(message, _cause) => {
                assert_eq!("Job<ListTables> already taked", message);
            }
            _ => panic!("{error:?}"),
        }
        true
    }

    #[test]
    async fn cancel() {
        let client = create_test_sql_client().await;

        let job = client.list_tables_async().await.unwrap();

        let done = job.cancel().await.unwrap();
        assert_eq!(true, done);
    }

    #[test]
    async fn cancel_after_done() {
        let client = create_test_sql_client().await;

        let mut job = client.list_tables_async().await.unwrap();
        let _result = job.take().await.unwrap();

        let done = job.cancel().await.unwrap();
        assert_eq!(true, done);
    }

    #[test]
    async fn cancel_async() {
        let client = create_test_sql_client().await;

        let job = client.list_tables_async().await.unwrap();

        let job = job.cancel_async().await.unwrap();
        let mut job = job.unwrap();

        let wait = job.wait(Duration::from_secs(10)).await.unwrap();
        assert_eq!(true, wait);

        let done = job.is_done().await.unwrap();
        assert_eq!(true, done);
    }

    #[test]
    async fn cancel_async_after_done() {
        let client = create_test_sql_client().await;

        let mut job = client.list_tables_async().await.unwrap();
        let _result = job.take().await.unwrap();

        let job = job.cancel_async().await.unwrap();
        assert_eq!(true, job.is_none());
    }

    #[test]
    async fn close() {
        let client = create_test_sql_client().await;

        let job = client.list_tables_async().await.unwrap();

        job.close().await.unwrap();
    }

    #[test]
    async fn close_after_done() {
        let client = create_test_sql_client().await;

        let mut job = client.list_tables_async().await.unwrap();
        let _result = job.take().await.unwrap();

        let done = job.is_done().await.unwrap();
        assert_eq!(true, done);

        job.close().await.unwrap();
    }

    #[test]
    async fn drop() {
        let client = create_test_sql_client().await;

        let _job = client.list_tables_async().await.unwrap();
    }

    #[test]
    async fn drop_after_done() {
        let client = create_test_sql_client().await;

        let mut job = client.list_tables_async().await.unwrap();
        let _result = job.take().await.unwrap();
    }

    #[test]
    async fn drop_after_cancel() {
        let client = create_test_sql_client().await;

        let job = client.list_tables_async().await.unwrap();

        let _job = job.cancel_async().await.unwrap();
    }
}
