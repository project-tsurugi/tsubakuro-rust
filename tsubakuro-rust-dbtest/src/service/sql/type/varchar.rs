#[cfg(test)]
mod test {
    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn literal() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (pk int primary key, v varchar(4))",
        )
        .await;

        let values = generate_values();

        insert_literal(&client, &values).await;
        select(&client, &values).await;
    }

    #[test]
    async fn prepare() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (pk int primary key, v varchar(4))",
        )
        .await;

        let values = generate_values();

        insert_prepared(&client, &values).await;
        select(&client, &values).await;
    }

    fn generate_values() -> Vec<(i32, String)> {
        let mut values = vec![];

        for i in 0..=4 {
            let v = "aaaa"[0..i].to_string();
            values.push((i as i32, v));
        }

        values
    }

    async fn insert_literal(client: &SqlClient, values: &Vec<(i32, String)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            let sql = format!("insert into test values({}, '{}')", value.0, value.1);
            client.execute(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, String)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<String>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in values {
            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", &value.1),
            ];
            client
                .prepared_execute(&transaction, &ps, parameters)
                .await
                .unwrap();
        }

        commit_and_close(client, &transaction).await;

        ps.close().await.unwrap();
    }

    async fn select(client: &SqlClient, expected: &Vec<(i32, String)>) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();
        let mut i = 0;
        while query_result.next_row().await.unwrap() {
            let expected = &expected[i];

            assert_eq!(true, query_result.next_column().await.unwrap());
            let pk = query_result.fetch().await.unwrap();
            assert_eq!(expected.0, pk);

            assert_eq!(true, query_result.next_column().await.unwrap());
            let v: String = query_result.fetch().await.unwrap();
            assert_eq!(expected.1, v);

            i += 1;
        }
        assert_eq!(expected.len(), i);

        commit_and_close(client, &transaction).await;
    }
}
