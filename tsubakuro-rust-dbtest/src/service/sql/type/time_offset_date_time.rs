#[cfg(test)]
mod test {
    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use time::{OffsetDateTime, UtcOffset};
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
            "create table test (pk int primary key, v timestamp with time zone, r int default 999)",
        )
        .await;

        let metadata = client.get_table_metadata("test").await.unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::TimePointWithTimeZone), c.atom_type());
    }

    fn generate_values(minus: bool) -> Vec<(i32, Option<OffsetDateTime>)> {
        let mut values = vec![];

        values.push((0, None));
        values.push((1, Some(date_time(2025, 1, 16, 18, 9, 30, 123456789, 9))));
        values.push((2, Some(date_time(1970, 1, 1, 0, 0, 0, 0, 0))));
        values.push((3, Some(date_time(1969, 12, 31, 23, 59, 59, 999999999, 0))));
        values.push((4, Some(date_time(1, 1, 1, 0, 0, 0, 0, 0))));
        values.push((5, Some(date_time(9999, 12, 31, 23, 59, 59, 999999999, 0))));
        if minus {
            values.push((10, Some(date_time(0, 1, 1, 0, 0, 0, 0, 9))));
            values.push((11, Some(date_time(-1, 1, 1, 0, 0, 0, 0, 9))));
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
        offset_hour: i32,
    ) -> OffsetDateTime {
        OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day)
                .unwrap(),
            time::Time::from_hms_nano(hour, min, sec, nanos).unwrap(),
            UtcOffset::from_whole_seconds(offset_hour * 60 * 60).unwrap(),
        )
    }

    async fn insert_literal(client: &SqlClient, values: &Vec<(i32, Option<OffsetDateTime>)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            let sql = if let Some(v) = &value.1 {
                let (h, m, _s) = v.offset().as_hms();
                let offset = if h >= 0 {
                    format!("+{:02}:{:02}", h, m)
                } else {
                    format!("-{:02}:{:02}", h.abs(), m)
                };
                format!(
                    "insert into test (pk, v) values({}, timestamp with time zone'{} {}{}')",
                    value.0,
                    v.date(),
                    v.time(),
                    offset
                )
            } else {
                format!("insert into test (pk, v) values({}, null)", value.0)
            };
            client.execute(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, Option<OffsetDateTime>)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<OffsetDateTime>("value"),
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

    async fn select(client: &SqlClient, expected: &Vec<(i32, Option<OffsetDateTime>)>, skip: bool) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();

        let metadata = query_result.get_metadata().unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::TimePointWithTimeZone), c.atom_type());

        let mut i = 0;
        while query_result.next_row().await.unwrap() {
            let expected = &expected[i];

            assert_eq!(true, query_result.next_column().await.unwrap());
            let pk = query_result.fetch().await.unwrap();
            assert_eq!(expected.0, pk);

            assert_eq!(true, query_result.next_column().await.unwrap());
            if !skip {
                let v = query_result.fetch().await.unwrap();
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

    fn to_z(value: Option<OffsetDateTime>) -> Option<OffsetDateTime> {
        if value.is_none() {
            return None;
        }
        let value = value.unwrap();

        let utc = value.to_offset(UtcOffset::UTC);

        Some(utc)
    }
}
