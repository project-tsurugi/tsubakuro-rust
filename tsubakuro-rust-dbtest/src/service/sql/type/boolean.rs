#[cfg(test)]
mod test {
    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn literal() {
        let client = create_test_sql_client().await;

        create_test_table(&client).await;

        let values = generate_values();

        insert_literal(&client, &values).await;
        select(&client, &values, false).await;
        select(&client, &values, true).await;
    }

    #[test]
    async fn prepare() {
        let client = create_test_sql_client().await;

        create_test_table(&client).await;

        let values = generate_values();

        insert_prepared(&client, &values).await;
        select(&client, &values, false).await;
        select(&client, &values, true).await;
    }

    async fn create_test_table(client: &SqlClient) {
        create_table(
            &client,
            "test",
            "create table test (pk int primary key, v int, r int default 999)", // TODO boolean type
        )
        .await;

        let metadata = client.get_table_metadata("test").await.unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::Int4), c.atom_type()); // TODO boolean type
        assert_eq!(None, c.length());
        assert_eq!(None, c.precision());
        assert_eq!(None, c.scale());
        assert_eq!(Some(true), c.nullable());
        assert_eq!(None, c.varying());
    }

    fn generate_values() -> Vec<(i32, Option<bool>)> {
        let mut values = vec![];

        values.push((0, None));
        values.push((1, Some(true)));
        values.push((2, Some(false)));

        values
    }

    async fn insert_literal(client: &SqlClient, values: &Vec<(i32, Option<bool>)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            let sql = if let Some(v) = value.1 {
                let v = if v { 1 } else { 0 }; // TODO boolean type
                format!("insert into test (pk, v) values({}, {})", value.0, v)
            } else {
                format!("insert into test (pk, v) values({}, null)", value.0)
            };
            client.execute(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, Option<bool>)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<i32>("value"), // TODO boolean type
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in values {
            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", value.1.map(|b| if b { 1 } else { 0 })), // TODO boolean type
            ];
            client
                .prepared_execute(&transaction, &ps, parameters)
                .await
                .unwrap();
        }

        commit_and_close(client, &transaction).await;

        ps.close().await.unwrap();
    }

    async fn select(client: &SqlClient, expected: &Vec<(i32, Option<bool>)>, skip: bool) {
        // let sql = "select * from test order by pk";
        let sql = "select pk, v <> 0 as v, r from test order by pk"; // TODO boolean type
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();

        let metadata = query_result.get_metadata().unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::Boolean), c.atom_type());
        assert_eq!(None, c.length());
        assert_eq!(None, c.precision());
        assert_eq!(None, c.scale());
        assert_eq!(None, c.nullable());
        assert_eq!(None, c.varying());

        let mut i = 0;
        while query_result.next_row().await.unwrap() {
            let expected = expected[i];

            assert_eq!(true, query_result.next_column().await.unwrap());
            let pk = query_result.fetch().await.unwrap();
            assert_eq!(expected.0, pk);

            assert_eq!(true, query_result.next_column().await.unwrap());
            if !skip {
                let v = query_result.fetch().await.unwrap();
                assert_eq!(expected.1, v);
            }

            assert_eq!(true, query_result.next_column().await.unwrap());
            let r = query_result.fetch().await.unwrap();
            assert_eq!(999, r);

            i += 1;
        }
        assert_eq!(expected.len(), i);

        commit_and_close(client, &transaction).await;
    }
}
