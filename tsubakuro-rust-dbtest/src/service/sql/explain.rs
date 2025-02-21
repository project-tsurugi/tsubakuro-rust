#[cfg(test)]
mod test {
    use crate::test::{create_table, create_test_sql_client};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn explain() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        insert(&client).await;
        select(&client).await;
    }

    async fn insert(client: &SqlClient) {
        let sql = "insert into test values(1, 11, 'abc')";
        let explain_result = client.explain(sql).await.unwrap();

        assert_eq!("jogasaki-statement.json", explain_result.format_id());
        assert_eq!(1, explain_result.format_version());
        // println!("explain={}", explain_result.contents());
        assert_eq!(0, explain_result.columns().len());
    }

    async fn select(client: &SqlClient) {
        let sql = "select * from test";
        let explain_result = client.explain(sql).await.unwrap();

        assert_eq!("jogasaki-statement.json", explain_result.format_id());
        assert_eq!(1, explain_result.format_version());
        // println!("explain={}", explain_result.contents());
        assert_eq!(3, explain_result.columns().len());
    }

    #[test]
    async fn explain_async() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        insert_async(&client).await;
        select_async(&client).await;
    }

    async fn insert_async(client: &SqlClient) {
        let sql = "insert into test values(1, 11, 'abc')";
        let mut job = client.explain_async(sql).await.unwrap();
        let explain_result = job.take().await.unwrap();

        assert_eq!("jogasaki-statement.json", explain_result.format_id());
        assert_eq!(1, explain_result.format_version());
        // println!("explain={}", explain_result.contents());
        assert_eq!(0, explain_result.columns().len());
    }

    async fn select_async(client: &SqlClient) {
        let sql = "select * from test";
        let mut job = client.explain_async(sql).await.unwrap();
        let explain_result = job.take().await.unwrap();

        assert_eq!("jogasaki-statement.json", explain_result.format_id());
        assert_eq!(1, explain_result.format_version());
        // println!("explain={}", explain_result.contents());
        assert_eq!(3, explain_result.columns().len());
    }

    #[test]
    async fn prepared_explain() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        prepared_insert(&client).await;
        prepared_select(&client).await;
    }

    async fn prepared_insert(client: &SqlClient) {
        let sql = "insert into test values(:foo, :bar, :zzz)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("foo"),
            SqlPlaceholder::of::<i64>("bar"),
            SqlPlaceholder::of::<String>("zzz"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        let parameters = vec![
            SqlParameter::of("foo", 1),
            SqlParameter::of("bar", 11_i64),
            SqlParameter::of("zzz", "abc"),
        ];
        let explain_result = client.prepared_explain(&ps, parameters).await.unwrap();

        assert_eq!("jogasaki-statement.json", explain_result.format_id());
        assert_eq!(1, explain_result.format_version());
        // println!("explain={}", explain_result.contents());
        assert_eq!(0, explain_result.columns().len());

        ps.close().await.unwrap();
    }

    async fn prepared_select(client: &SqlClient) {
        let sql = "select * from test where foo = :foo";
        let placeholders = vec![SqlPlaceholder::of::<i32>("foo")];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        let parameters = vec![SqlParameter::of("foo", 1)];
        let explain_result = client.prepared_explain(&ps, parameters).await.unwrap();

        assert_eq!("jogasaki-statement.json", explain_result.format_id());
        assert_eq!(1, explain_result.format_version());
        // println!("explain={}", explain_result.contents());
        assert_eq!(3, explain_result.columns().len());

        ps.close().await.unwrap();
    }

    #[test]
    async fn prepared_explain_async() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        prepared_insert_async(&client).await;
        prepared_select_async(&client).await;
    }

    async fn prepared_insert_async(client: &SqlClient) {
        let sql = "insert into test values(:foo, :bar, :zzz)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("foo"),
            SqlPlaceholder::of::<i64>("bar"),
            SqlPlaceholder::of::<String>("zzz"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        let parameters = vec![
            SqlParameter::of("foo", 1),
            SqlParameter::of("bar", 11_i64),
            SqlParameter::of("zzz", "abc"),
        ];
        let mut job = client
            .prepared_explain_async(&ps, parameters)
            .await
            .unwrap();
        let explain_result = job.take().await.unwrap();

        assert_eq!("jogasaki-statement.json", explain_result.format_id());
        assert_eq!(1, explain_result.format_version());
        // println!("explain={}", explain_result.contents());
        assert_eq!(0, explain_result.columns().len());

        ps.close().await.unwrap();
    }

    async fn prepared_select_async(client: &SqlClient) {
        let sql = "select * from test where foo = :foo";
        let placeholders = vec![SqlPlaceholder::of::<i32>("foo")];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        let parameters = vec![SqlParameter::of("foo", 1)];
        let mut job = client
            .prepared_explain_async(&ps, parameters)
            .await
            .unwrap();
        let explain_result = job.take().await.unwrap();

        assert_eq!("jogasaki-statement.json", explain_result.format_id());
        assert_eq!(1, explain_result.format_version());
        // println!("explain={}", explain_result.contents());
        assert_eq!(3, explain_result.columns().len());

        ps.close().await.unwrap();
    }
}
