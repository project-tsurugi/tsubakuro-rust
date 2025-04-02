#[cfg(test)]
mod test {
    use crate::{
        service::sql::r#type::{
            epoch_days, epoch_days_to_string, seconds_of_day, seconds_of_day_to_string,
        },
        test::{commit_and_close, create_table, create_test_sql_client, start_occ},
    };
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;
    #[test]

    async fn literal() {
        let client = create_test_sql_client().await;

        create_test_table(&client).await;

        let values = generate_values(false);

        insert_literal(&client, &values).await;
        select(&client, &values, false).await;
        select(&client, &values, true).await;
    }

    #[test]
    async fn prepare() {
        let client = create_test_sql_client().await;

        create_test_table(&client).await;

        let values = generate_values(true);

        insert_prepared(&client, &values).await;
        select(&client, &values, false).await;
        select(&client, &values, true).await;
    }

    async fn create_test_table(client: &SqlClient) {
        create_table(
            &client,
            "test",
            "create table test (pk int primary key, v timestamp, r int default 999)",
        )
        .await;

        let metadata = client.get_table_metadata("test").await.unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::TimePoint), c.atom_type());
        assert_eq!(None, c.length());
        assert_eq!(None, c.precision());
        assert_eq!(None, c.scale());
        assert_eq!(None, c.nullable());
        assert_eq!(None, c.varying());
    }

    fn generate_values(minus: bool) -> Vec<(i32, Option<TgTimePoint>)> {
        let mut values = vec![];

        values.push((0, None));
        values.push((1, Some(date_time(2025, 1, 16, 18, 9, 30, 123456789))));
        values.push((2, Some(date_time(1970, 1, 1, 0, 0, 0, 0))));
        values.push((3, Some(date_time(1969, 12, 31, 23, 59, 59, 999999999))));
        values.push((4, Some(date_time(1, 1, 1, 0, 0, 0, 0))));
        values.push((5, Some(date_time(9999, 12, 31, 23, 59, 59, 999999999))));
        if minus {
            values.push((10, Some(date_time(0, 1, 1, 0, 0, 0, 0))));
            values.push((11, Some(date_time(-1, 1, 1, 0, 0, 0, 0))));
        }

        values
    }

    fn date_time(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        min: u8,
        sec: u8,
        nanos: u32,
    ) -> TgTimePoint {
        let days = epoch_days(year, month, day);
        let seconds = seconds_of_day(hour, min, sec) as i64;
        TgTimePoint::new(days * 86400 + seconds, nanos)
    }

    async fn insert_literal(client: &SqlClient, values: &Vec<(i32, Option<TgTimePoint>)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            let sql = if let Some(v) = &value.1 {
                let s = date_time_to_string(*v);
                format!(
                    "insert into test (pk, v) values({}, timestamp'{}')",
                    value.0, s
                )
            } else {
                format!("insert into test (pk, v) values({}, null)", value.0)
            };
            client.execute(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    fn date_time_to_string(value: TgTimePoint) -> String {
        let mut days = value.offset_seconds / 86400;
        let mut seconds = value.offset_seconds % 86400;
        if seconds < 0 {
            seconds += 86400;
            days -= 1;
        }
        format!(
            "{} {}",
            epoch_days_to_string(days),
            seconds_of_day_to_string(seconds as u32, value.nano_adjustment)
        )
    }

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, Option<TgTimePoint>)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<TgTimePoint>("value"),
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

    async fn select(client: &SqlClient, expected: &Vec<(i32, Option<TgTimePoint>)>, skip: bool) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();

        let metadata = query_result.get_metadata().unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::TimePoint), c.atom_type());
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
