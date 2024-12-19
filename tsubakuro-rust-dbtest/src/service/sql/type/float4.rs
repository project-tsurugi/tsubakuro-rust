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
            "create table test (pk int primary key, v real)",
        )
        .await;

        let values = generate_values(false);

        insert_literal(&client, &values).await;
        select(&client, &values).await;
    }

    #[test]
    async fn prepare() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (pk int primary key, v real)",
        )
        .await;

        let values = generate_values(true);

        insert_prepared(&client, &values).await;
        select(&client, &values).await;
    }

    fn generate_values(inf: bool) -> Vec<(i32, f32)> {
        // TODO let min = f32::MIN;
        // TODO let max = f32::MAX;
        let min = -3.40282e+38;
        let max = 3.40282e+38;

        let mut values = vec![];

        values.push((0, 0_f32));
        values.push((1, min));
        values.push((2, max));
        if inf {
            values.push((3, f32::NAN));
            values.push((4, f32::NEG_INFINITY));
            values.push((5, f32::INFINITY));
        }

        let mut i = 6;
        let mut v = min + 1e+30;
        let step = max / 500_f32 * 2_f32;
        loop {
            values.push((i, v));

            if v > max - step {
                break;
            }
            i += 1;
            v += step;
        }

        values
    }

    async fn insert_literal(client: &SqlClient, values: &Vec<(i32, f32)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            let sql = format!("insert into test values({}, {:e})", value.0, value.1);
            client.execute(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, f32)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test values(:pk, :value)";
        let placeholders = vec![i32::placeholder("pk"), f32::placeholder("value")];
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

    async fn select(client: &SqlClient, expected: &Vec<(i32, f32)>) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();
        let mut i = 0;
        while query_result.next_row().await.unwrap() {
            let expected = expected[i];

            assert_eq!(true, query_result.next_column().await.unwrap());
            let pk = query_result.fetch().await.unwrap();
            assert_eq!(expected.0, pk);

            assert_eq!(true, query_result.next_column().await.unwrap());
            let v: f32 = query_result.fetch().await.unwrap();
            if expected.1.is_nan() {
                assert_eq!(true, v.is_nan());
            } else {
                assert_eq!(expected.1, v);
            }

            i += 1;
        }
        assert_eq!(expected.len(), i);

        commit_and_close(client, &transaction).await;
    }
}
