#[cfg(test)]
mod test {
    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use time::{Time, UtcOffset};
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
        assert_eq!(None, c.length());
        assert_eq!(None, c.precision());
        assert_eq!(None, c.scale());
        assert_eq!(Some(true), c.nullable());
        assert_eq!(None, c.varying());
    }

    fn generate_values() -> Vec<(i32, Option<(Time, UtcOffset)>)> {
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

    fn time_with_offset(
        hour: u8,
        min: u8,
        sec: u8,
        nanos: u32,
        offset_hour: i32,
    ) -> (Time, UtcOffset) {
        let time = Time::from_hms_nano(hour, min, sec, nanos).unwrap();
        let offset_seconds = offset_hour * 60 * 60;
        let offset = UtcOffset::from_whole_seconds(offset_seconds).unwrap();
        (time, offset)
    }

    async fn _insert_literal(client: &SqlClient, values: &Vec<(i32, Option<(Time, UtcOffset)>)>) {
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

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, Option<(Time, UtcOffset)>)>) {
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<(Time, UtcOffset)>("value"),
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

    async fn select(
        client: &SqlClient,
        expected: &Vec<(i32, Option<(Time, UtcOffset)>)>,
        skip: bool,
    ) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();

        let metadata = query_result.get_metadata().unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::TimeOfDayWithTimeZone), c.atom_type());
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

    fn to_z(value: Option<(Time, UtcOffset)>) -> Option<(Time, UtcOffset)> {
        if value.is_none() {
            return None;
        }
        let (time, offset) = value.unwrap();

        let date_time = time::OffsetDateTime::new_in_offset(
            time::Date::from_ordinal_date(1970, 1).unwrap(),
            time,
            offset,
        );
        let utc_date_time = date_time.to_offset(UtcOffset::UTC);
        let utc_time = utc_date_time.time();

        Some((utc_time, UtcOffset::UTC))
    }
}
