#[cfg(test)]
mod test {
    use std::{
        time::{Duration, Instant},
        vec,
    };

    use crate::test::{
        commit_and_close, create_table, create_test_connection_option, create_test_session,
        start_occ,
    };
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn update_expiration_time() {
        let session = create_test_session().await;

        session.update_expiration_time(None).await.unwrap();
        session
            .update_expiration_time(Some(Duration::from_secs(1)))
            .await
            .unwrap();
    }

    #[test]
    async fn update_expiration_time_async() {
        let session = create_test_session().await;

        let mut job = session.update_expiration_time_async(None).await.unwrap();
        assert_eq!("updateExpirationTime", job.name());
        job.take().await.unwrap();

        let mut job = session
            .update_expiration_time_async(Some(Duration::from_secs(1)))
            .await
            .unwrap();
        job.take().await.unwrap();
    }

    #[test]
    async fn keep_alive_on() {
        // env_logger::builder().is_test(true).try_init().unwrap();
        keep_alive_test(Duration::from_millis(500)).await;
    }

    #[test]
    async fn keep_alive_off() {
        // env_logger::builder().is_test(true).try_init().unwrap();
        keep_alive_test(Duration::ZERO).await;
    }

    async fn keep_alive_test(keep_alive: Duration) {
        let mut option = create_test_connection_option();
        option.set_keep_alive(keep_alive);
        let session = Session::connect(&option).await.unwrap();
        let client: SqlClient = session.make_client();

        create_table(&client, "test", "create table test(pk int primary key)").await;

        let time = Duration::from_secs(3); // テストを実行する時間
        let start = Instant::now();
        let mut pk = 0;
        while start.elapsed() < time {
            let ps = client
                .prepare(
                    "insert into test values(:pk)",
                    vec![SqlPlaceholder::of::<i32>("pk")],
                )
                .await
                .unwrap();

            let tx = start_occ(&client).await;
            let result = client
                .prepared_execute(&tx, &ps, vec![SqlParameter::of("pk", pk)])
                .await
                .unwrap();
            assert_eq!(1, result.inserted_rows());
            commit_and_close(&client, &tx).await;

            ps.close().await.unwrap();

            pk += 1;
        }

        session.close().await.unwrap();
    }

    #[test]
    async fn shutdown() {
        shutdown_test(ShutdownType::Graceful).await;
        shutdown_test(ShutdownType::Forceful).await;
    }

    async fn shutdown_test(shutdown_type: ShutdownType) {
        let session = create_test_session().await;
        assert_eq!(false, session.is_shutdowned());

        session.shutdown(shutdown_type).await.unwrap();

        assert_eq!(true, session.is_shutdowned());

        // second shutdown
        let error = session.shutdown(shutdown_type).await.unwrap_err();
        assert_eq!(true, error.message().contains("already closed"));
    }

    #[test]
    async fn shutdown_async() {
        shutdown_async_test(ShutdownType::Graceful).await;
        shutdown_async_test(ShutdownType::Forceful).await;
    }

    async fn shutdown_async_test(shutdown_type: ShutdownType) {
        let session = create_test_session().await;
        assert_eq!(false, session.is_shutdowned());

        let mut job = session.shutdown_async(shutdown_type).await.unwrap();
        assert_eq!("Shutdown", job.name());
        job.take().await.unwrap();

        assert_eq!(true, session.is_shutdowned());

        // second shutdown
        let error = session.shutdown_async(shutdown_type).await.unwrap_err();
        assert_eq!(true, error.message().contains("already closed"));
    }

    #[test]
    async fn shutdown_after_close() {
        let session = create_test_session().await;
        session.close().await.unwrap();
        assert_eq!(true, session.is_closed());

        let error = session.shutdown(ShutdownType::Graceful).await.unwrap_err();
        assert_eq!(true, error.message().contains("already closed"));
    }

    #[test]
    async fn shutdown_async_after_close() {
        let session = create_test_session().await;
        session.close().await.unwrap();
        assert_eq!(true, session.is_closed());

        let error = session
            .shutdown_async(ShutdownType::Graceful)
            .await
            .unwrap_err();
        assert_eq!(true, error.message().contains("already closed"));
    }

    #[test]
    async fn close() {
        let session = create_test_session().await;
        assert_eq!(false, session.is_closed());

        session.close().await.unwrap();

        assert_eq!(true, session.is_closed());

        // second close
        session.close().await.unwrap();
    }
}
