mod system_info;

#[cfg(test)]
mod test {
    use tsubakuro_rust_core::prelude::SystemClient;

    use crate::test::create_test_session;

    pub(crate) async fn create_test_system_client() -> SystemClient {
        let session = create_test_session().await;
        session.make_client()
    }
}
