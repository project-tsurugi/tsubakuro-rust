#[cfg(test)]
mod test {
    use crate::test::create_test_session;
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn shutdown() {
        shutdown_main(ShutdownType::Graceful).await;
        shutdown_main(ShutdownType::Forceful).await;
    }

    async fn shutdown_main(shutdown_type: ShutdownType) {
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
        shutdown_async_main(ShutdownType::Graceful).await;
        shutdown_async_main(ShutdownType::Forceful).await;
    }

    async fn shutdown_async_main(shutdown_type: ShutdownType) {
        let session = create_test_session().await;
        assert_eq!(false, session.is_shutdowned());

        let mut job = session.shutdown_async(shutdown_type).await.unwrap();
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
