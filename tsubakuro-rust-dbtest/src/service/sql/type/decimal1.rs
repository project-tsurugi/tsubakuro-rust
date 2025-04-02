#[cfg(test)]
mod test {
    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    const DECIMAL_SIZE: u32 = 38;

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
            &format!(
                "create table test (pk int primary key, v decimal({}, 1), r int default 999)",
                DECIMAL_SIZE
            ),
        )
        .await;

        let metadata = client.get_table_metadata("test").await.unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::Decimal), c.atom_type());
        assert_eq!(None, c.length());
        assert_eq!(None, c.precision());
        assert_eq!(None, c.scale());
        assert_eq!(None, c.nullable());
        assert_eq!(None, c.varying());
    }

    fn generate_values() -> Vec<(i32, Option<TgDecimalI128>)> {
        let mut values = vec![];

        values.push((0, None));
        values.push((1, Some(TgDecimalI128::new(0, -1))));
        values.push((2, Some(TgDecimalI128::new(10, -1))));
        values.push((3, Some(TgDecimalI128::new(-10, -1))));
        values.push((4, Some(TgDecimalI128::new(1, -1))));
        values.push((5, Some(TgDecimalI128::new(-1, -1))));
        values.push((6, Some(TgDecimalI128::new(12, -1))));
        values.push((7, Some(TgDecimalI128::new(-12, -1))));

        let mut i = 10;
        let max = 10i128.pow(DECIMAL_SIZE) - 1;
        let mut v = -max;
        let step = max / 50 * 2;
        loop {
            values.push((i, Some(TgDecimalI128::new(v, -1))));

            if v > max - step {
                break;
            }
            i += 1;
            v += step;
        }

        values
    }

    async fn insert_literal(client: &SqlClient, values: &Vec<(i32, Option<TgDecimalI128>)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            let sql = if let Some(v) = &value.1 {
                format!("insert into test (pk, v) values({}, {})", value.0, v)
            } else {
                format!("insert into test (pk, v) values({}, null)", value.0)
            };
            client.execute(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, Option<TgDecimalI128>)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<TgDecimalI128>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in values {
            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", value.1),
            ];
            client
                .prepared_execute(&transaction, &ps, parameters)
                .await
                .unwrap();
        }

        commit_and_close(client, &transaction).await;

        ps.close().await.unwrap();
    }

    async fn select(client: &SqlClient, expected: &Vec<(i32, Option<TgDecimalI128>)>, skip: bool) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();

        let metadata = query_result.get_metadata().unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::Decimal), c.atom_type());
        assert_eq!(None, c.length());
        assert_eq!(None, c.precision());
        assert_eq!(None, c.scale());
        assert_eq!(None, c.nullable());
        assert_eq!(None, c.varying());

        let mut i = 0;
        while query_result.next_row().await.unwrap() {
            let expected = &expected[i];

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
