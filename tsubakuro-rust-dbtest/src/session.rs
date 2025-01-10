#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::test::create_test_session;
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
