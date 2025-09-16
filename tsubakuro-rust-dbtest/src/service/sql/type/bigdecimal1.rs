#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use bigdecimal::{num_bigint::BigInt, BigDecimal, Zero};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    const DECIMAL_SIZE: u32 = 19;

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
        assert_eq!(Some((DECIMAL_SIZE, false)), c.precision());
        assert_eq!(Some((1, false)), c.scale());
        assert_eq!(Some(true), c.nullable());
        assert_eq!(None, c.varying());
    }

    fn generate_values() -> Vec<(i32, Option<BigDecimal>)> {
        let mut values = vec![];

        values.push((0, None));
        values.push((1, Some(BigDecimal::zero())));
        values.push((2, Some(BigDecimal::from_str("1").unwrap())));
        values.push((3, Some(BigDecimal::from_str("-1").unwrap())));
        values.push((4, Some(BigDecimal::from_str("1.0").unwrap())));
        values.push((5, Some(BigDecimal::from_str("-1.0").unwrap())));
        values.push((6, Some(BigDecimal::from_str("1.2").unwrap())));
        values.push((7, Some(BigDecimal::from_str("-1.2").unwrap())));

        let mut i = 10;
        let max = BigDecimal::new(BigInt::from(10).pow(DECIMAL_SIZE) - 1, 0);
        let mut v = -&max;
        let step: BigDecimal = &max / 50 * 2;
        let step = step.with_scale(0);
        loop {
            values.push((i, Some(&v / 10)));

            if v > &max - &step {
                break;
            }
            i += 1;
            v += &step;
        }

        values
    }

    async fn insert_literal(client: &SqlClient, values: &Vec<(i32, Option<BigDecimal>)>) {
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

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, Option<BigDecimal>)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<BigDecimal>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in values {
            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", value.1.as_ref()),
            ];
            client
                .prepared_execute(&transaction, &ps, parameters)
                .await
                .unwrap();
        }

        commit_and_close(client, &transaction).await;

        ps.close().await.unwrap();
    }

    async fn select(client: &SqlClient, expected: &Vec<(i32, Option<BigDecimal>)>, skip: bool) {
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
        assert_eq!(Some((DECIMAL_SIZE, false)), c.precision());
        assert_eq!(Some((1, false)), c.scale());
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
                if let Some(value) = v {
                    let scale = value.as_bigint_and_exponent().1;
                    assert_eq!(1, scale);
                }
            }

            assert_eq!(true, query_result.next_column().await.unwrap());
            let r = query_result.fetch().await.unwrap();
            assert_eq!(999, r);

            i += 1;
        }
        assert_eq!(expected.len(), i);

        query_result.close().await.unwrap();

        commit_and_close(client, &transaction).await;
    }
}
