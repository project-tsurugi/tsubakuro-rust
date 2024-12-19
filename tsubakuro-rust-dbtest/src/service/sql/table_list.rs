#[cfg(test)]
mod test {
    use crate::test::{create_table, create_test_sql_client, drop_table_if_exists};
    use tokio::test;
    use tsubakuro_rust_core::prelude::name::TName;

    #[test]
    async fn list_tables() {
        let client = create_test_sql_client().await;
        drop_table_if_exists(&client, "test").await;

        let test = TName::new(vec!["test".to_string()]);
        {
            let table_list = client.list_tables().await.unwrap();
            let names = table_list.get_table_names();
            assert_eq!(false, names.contains(&test));
        }

        create_table(&client, "test", "create table test (pk int primary key)").await;

        {
            let table_list = client.list_tables().await.unwrap();
            let names = table_list.get_table_names();
            assert_eq!(true, names.contains(&test));
        }
    }

    #[test]
    async fn list_tables_async() {
        let client = create_test_sql_client().await;
        drop_table_if_exists(&client, "test").await;

        let test = TName::new(vec!["test".to_string()]);
        {
            let mut job = client.list_tables_async().await.unwrap();
            let table_list = job.take().await.unwrap();
            let names = table_list.get_table_names();
            assert_eq!(false, names.contains(&test));
        }

        create_table(&client, "test", "create table test (pk int primary key)").await;

        {
            let mut job = client.list_tables_async().await.unwrap();
            let table_list = job.take().await.unwrap();
            let names = table_list.get_table_names();
            assert_eq!(true, names.contains(&test));
        }
    }
}
