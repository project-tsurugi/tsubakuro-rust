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
}
