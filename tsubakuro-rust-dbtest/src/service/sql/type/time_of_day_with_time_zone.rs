#[cfg(test)]
mod test {
    use crate::{
        service::sql::r#type::{seconds_of_day, seconds_of_day_to_hms},
        test::{commit_and_close, create_table, create_test_sql_client, start_occ},
    };
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    // #[test] // TODO 'time with time zone' literal
    async fn _literal() {
        let client = create_test_sql_client().await;

        create_test_table(&client).await;

        let values = generate_values();

        _insert_literal(&client, &values).await;
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
            "create table test (pk int primary key, v time with time zone, r int default 999)",
        )
        .await;

        let metadata = client.get_table_metadata("test").await.unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::TimeOfDayWithTimeZone), c.atom_type());
    }

    fn generate_values() -> Vec<(i32, Option<(u64, i32)>)> {
        let mut values = vec![];

        values.push((0, None));
        values.push((1, Some(time_with_offset(0, 0, 0, 0, 0))));
        values.push((2, Some(time_with_offset(0, 0, 0, 0, 9))));
        values.push((3, Some(time_with_offset(0, 0, 0, 0, -9))));
        values.push((4, Some(time_with_offset(23, 59, 59, 999_999_999, 0))));
        values.push((5, Some(time_with_offset(23, 59, 59, 999_999_999, 9))));
        values.push((6, Some(time_with_offset(23, 59, 59, 999_999_999, -9))));

        values
    }

    fn time_with_offset(hour: u8, min: u8, sec: u8, nanos: u64, offset_hour: i32) -> (u64, i32) {
        let value = seconds_of_day(hour, min, sec) as u64 * 1_000_000_000 + nanos;
        (value, offset_hour * 60)
    }

    async fn _insert_literal(client: &SqlClient, values: &Vec<(i32, Option<(u64, i32)>)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            let sql = if let Some((v, offset)) = &value.1 {
                format!(
                    "insert into test (pk, v) values({}, time with time zone'{}{}')",
                    value.0, v, offset
                )
            } else {
                format!("insert into test (pk, v) values({}, null)", value.0)
            };
            client.execute(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, Option<(u64, i32)>)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of_atom_type("value", AtomType::TimeOfDayWithTimeZone),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in values {
            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of_time_of_day_with_time_zone_opt("value", value.1),
            ];
            client
                .prepared_execute(&transaction, &ps, parameters)
                .await
                .unwrap();
        }

        commit_and_close(client, &transaction).await;

        ps.close().await.unwrap();
    }

    async fn select(client: &SqlClient, expected: &Vec<(i32, Option<(u64, i32)>)>, skip: bool) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();

        let metadata = query_result.get_metadata().unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::TimeOfDayWithTimeZone), c.atom_type());

        let mut i = 0;
        while query_result.next_row().await.unwrap() {
            let expected = &expected[i];

            assert_eq!(true, query_result.next_column().await.unwrap());
            let pk = query_result.fetch().await.unwrap();
            assert_eq!(expected.0, pk);

            assert_eq!(true, query_result.next_column().await.unwrap());
            if !skip {
                let v = query_result
                    .fetch_time_of_day_with_time_zone_opt()
                    .await
                    .unwrap();
                assert_eq!(to_z(expected.1), v);
            }

            assert_eq!(true, query_result.next_column().await.unwrap());
            let r = query_result.fetch().await.unwrap();
            assert_eq!(999, r);

            i += 1;
        }
        assert_eq!(expected.len(), i);

        commit_and_close(client, &transaction).await;
    }

    fn to_z(value: Option<(u64, i32)>) -> Option<(u64, i32)> {
        if value.is_none() {
            return None;
        }
        let (time, offset) = value.unwrap();

        if offset == 0 {
            return value;
        }

        let nanos = time % 1_000_000_000;
        let seconds = time / 1_000_000_000;
        let (hour, min, sec) = seconds_of_day_to_hms(seconds as u32);
        let mut hour = hour as i32;
        hour -= offset / 60;
        if hour < 0 {
            hour += 24;
        } else if hour >= 24 {
            hour -= 24;
        }

        Some(time_with_offset(hour as u8, min, sec, nanos, 0))
    }
}
