#[cfg(test)]
mod test {
    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_client::prelude::*;

    #[test]
    async fn test() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (pk int primary key, v double)",
        )
        .await;

        let values = generate_values(false);

        insert(&client, &values).await;
        select(&client, &values).await;
    }

    fn generate_values(inf: bool) -> Vec<(i32, f64)> {
        let min = f64::MIN;
        let max = f64::MAX;

        let mut values = vec![];

        values.push((0, 0_f64));
        values.push((1, min));
        values.push((2, max));
        if inf {
            values.push((3, f64::NAN));
            values.push((4, f64::NEG_INFINITY));
            values.push((5, f64::INFINITY));
        }

        let mut i = 6;
        let mut v = min + 1e+30;
        let step = max / 500_f64 * 2_f64;
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

    async fn insert(client: &SqlClient, values: &Vec<(i32, f64)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            // TODO prepraed statement
            let sql = format!("insert into test values({}, {:e})", value.0, value.1);
            // println!("{sql}");
            client.execute_statement(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    async fn select(client: &SqlClient, expected: &Vec<(i32, f64)>) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.execute_query(&transaction, sql).await.unwrap();
        let mut i = 0;
        while query_result.next_row().await.unwrap() {
            let expected = expected[i];

            assert_eq!(true, query_result.next_column().await.unwrap());
            let pk = query_result.fetch().await.unwrap();
            assert_eq!(expected.0, pk);

            assert_eq!(true, query_result.next_column().await.unwrap());
            let v = query_result.fetch().await.unwrap();
            assert_eq!(expected.1, v);

            // println!("pk={pk}, v={v}");

            i += 1;
        }
        assert_eq!(expected.len(), i);

        commit_and_close(client, &transaction).await;
    }
}
