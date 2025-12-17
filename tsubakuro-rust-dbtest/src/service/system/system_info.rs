#[cfg(test)]
mod test {
    use crate::service::system::test::create_test_system_client;
    use tokio::test;

    #[test]
    async fn get_system_info() {
        let client = create_test_system_client().await;

        let system_info = client.get_system_info().await.unwrap();
        let version = system_info.version();

        let mut iter = version.split(".");
        assert!(iter.next().unwrap().parse::<u32>().is_ok());
        assert!(iter.next().unwrap().parse::<u32>().is_ok());
        assert!(iter.next().unwrap().parse::<String>().is_ok());
    }
}
