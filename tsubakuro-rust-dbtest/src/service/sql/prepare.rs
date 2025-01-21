#[cfg(test)]
mod test {
    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn prepare() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        insert(&client).await;
        select(&client).await;
    }

    async fn insert(client: &SqlClient) {
        let sql = "insert into test values(:foo, :bar, :zzz)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("foo"),
            SqlPlaceholder::of::<i64>("bar"),
            SqlPlaceholder::of::<String>("zzz"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();
        assert_eq!(false, ps.has_result_records());

        let transaction = start_occ(&client).await;

        let parameters = vec![
            SqlParameter::of("foo", 1),
            SqlParameter::of("bar", 11_i64),
            SqlParameter::of("zzz", "abc"),
        ];
        let r = client
            .prepared_execute(&transaction, &ps, parameters)
            .await
            .unwrap();
        assert_eq!(1, r.inserted_rows());

        let parameters = vec![
            SqlParameter::of("foo", 2),
            SqlParameter::of("bar", 22_i64),
            SqlParameter::of("zzz", "def"),
        ];
        let r = client
            .prepared_execute(&transaction, &ps, parameters)
            .await
            .unwrap();
        assert_eq!(1, r.inserted_rows());

        let parameters = vec![
            SqlParameter::of("foo", 3),
            SqlParameter::of("bar", 33_i64),
            SqlParameter::null("zzz"),
        ];
        let r = client
            .prepared_execute(&transaction, &ps, parameters)
            .await
            .unwrap();
        assert_eq!(1, r.inserted_rows());

        commit_and_close(client, &transaction).await;

        assert_eq!(false, ps.is_closed());
        ps.close().await.unwrap();
        assert_eq!(true, ps.is_closed());
    }

    async fn select(client: &SqlClient) {
        let sql = "select * from test where foo = :foo";
        let placeholders = vec![SqlPlaceholder::of::<i32>("foo")];
        let ps = client.prepare(sql, placeholders).await.unwrap();
        assert_eq!(true, ps.has_result_records());

        let transaction = start_occ(&client).await;

        {
            let parameters = vec![SqlParameter::of("foo", 1)];
            let mut query_result = client
                .prepared_query(&transaction, &ps, parameters)
                .await
                .unwrap();

            assert_eq!(true, query_result.next_row().await.unwrap());

            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(1, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(11_i64, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            let zzz: String = query_result.fetch().await.unwrap();
            assert_eq!("abc", zzz);
            assert_eq!(false, query_result.next_column().await.unwrap());

            assert_eq!(false, query_result.next_row().await.unwrap());
        }

        {
            let parameters = vec![SqlParameter::of("foo", 3)];
            let mut query_result = client
                .prepared_query(&transaction, &ps, parameters)
                .await
                .unwrap();

            assert_eq!(true, query_result.next_row().await.unwrap());

            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(3, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(33_i64, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(true, query_result.is_null().unwrap());
            assert_eq!(false, query_result.next_column().await.unwrap());

            assert_eq!(false, query_result.next_row().await.unwrap());
        }

        commit_and_close(client, &transaction).await;

        assert_eq!(false, ps.is_closed());
        ps.close().await.unwrap();
        assert_eq!(true, ps.is_closed());
    }

    #[test]
    async fn prepare_async() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        insert_async(&client).await;
        select_async(&client).await;
    }

    async fn insert_async(client: &SqlClient) {
        let sql = "insert into test values(:foo, :bar, :zzz)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("foo"),
            SqlPlaceholder::of::<i64>("bar"),
            SqlPlaceholder::of::<String>("zzz"),
        ];
        let mut job = client.prepare_async(sql, placeholders).await.unwrap();
        let ps = job.take().await.unwrap();
        assert_eq!(false, ps.has_result_records());

        let transaction = start_occ(&client).await;

        let parameters = vec![
            SqlParameter::of("foo", 1),
            SqlParameter::of("bar", 11_i64),
            SqlParameter::of("zzz", "abc"),
        ];
        let mut job = client
            .prepared_execute_async(&transaction, &ps, parameters)
            .await
            .unwrap();
        let r = job.take().await.unwrap();
        assert_eq!(1, r.inserted_rows());

        let parameters = vec![
            SqlParameter::of("foo", 2),
            SqlParameter::of("bar", 22_i64),
            SqlParameter::of("zzz", "def"),
        ];
        let mut job = client
            .prepared_execute_async(&transaction, &ps, parameters)
            .await
            .unwrap();
        let r = job.take().await.unwrap();
        assert_eq!(1, r.inserted_rows());

        let parameters = vec![
            SqlParameter::of("foo", 3),
            SqlParameter::of("bar", 33_i64),
            SqlParameter::null("zzz"),
        ];
        let mut job = client
            .prepared_execute_async(&transaction, &ps, parameters)
            .await
            .unwrap();
        let r = job.take().await.unwrap();
        assert_eq!(1, r.inserted_rows());

        commit_and_close(client, &transaction).await;

        assert_eq!(false, ps.is_closed());
        ps.close().await.unwrap();
        assert_eq!(true, ps.is_closed());
    }

    async fn select_async(client: &SqlClient) {
        let sql = "select * from test where foo = :foo";
        let placeholders = vec![SqlPlaceholder::of::<i32>("foo")];
        let mut job = client.prepare_async(sql, placeholders).await.unwrap();
        let ps = job.take().await.unwrap();
        assert_eq!(true, ps.has_result_records());

        let transaction = start_occ(&client).await;

        {
            let parameters = vec![SqlParameter::of("foo", 1)];
            let mut job = client
                .prepared_query_async(&transaction, &ps, parameters)
                .await
                .unwrap();
            let mut query_result = job.take().await.unwrap();

            assert_eq!(true, query_result.next_row().await.unwrap());

            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(1, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(11_i64, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            let zzz: String = query_result.fetch().await.unwrap();
            assert_eq!("abc", zzz);
            assert_eq!(false, query_result.next_column().await.unwrap());

            assert_eq!(false, query_result.next_row().await.unwrap());
        }

        {
            let parameters = vec![SqlParameter::of("foo", 3)];
            let mut job = client
                .prepared_query_async(&transaction, &ps, parameters)
                .await
                .unwrap();
            let mut query_result = job.take().await.unwrap();

            assert_eq!(true, query_result.next_row().await.unwrap());

            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(3, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(33_i64, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(true, query_result.is_null().unwrap());
            assert_eq!(false, query_result.next_column().await.unwrap());

            assert_eq!(false, query_result.next_row().await.unwrap());
        }

        commit_and_close(client, &transaction).await;

        assert_eq!(false, ps.is_closed());
        ps.close().await.unwrap();
        assert_eq!(true, ps.is_closed());
    }

    #[test]
    async fn prepare_error() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        let sql = "insert into test values(:foo, :bar, :zzz)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("foo"),
            SqlPlaceholder::of::<i64>("bar"),
            // not defined SqlPlaceholder::of::<String>("zzz"),
        ];
        let error = client.prepare(sql, placeholders).await.unwrap_err();
        match error {
            TgError::ServerError(_message, code, _server_message) => {
                assert_eq!("SYMBOL_ANALYZE_EXCEPTION", code.name());
            }
            _ => panic!("{:?}", error),
        };
    }

    #[test]
    async fn prepare_close() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        for _i in 0..30 {
            prepare_dispose_test(&client, true, false).await;
        }
    }

    async fn prepare_dispose_test(client: &SqlClient, close: bool, logger_init: bool) {
        let sql = "insert into test values(:foo, :bar, :zzz)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("foo"),
            SqlPlaceholder::of::<i64>("bar"),
            SqlPlaceholder::of::<String>("zzz"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        if logger_init {
            env_logger::builder().is_test(true).try_init().unwrap();
        }
        if close {
            ps.close().await.unwrap();
        }
    }

    #[test]
    #[ignore]
    async fn prepare_drop() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        for i in 0..10 {
            println!("prepare_drop {}", i);
            prepare_dispose_test(&client, false, false).await;
        }
    }

    #[test]
    #[ignore]
    async fn prepare_drop1() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (foo int primary key, bar bigint, zzz varchar(10))",
        )
        .await;

        prepare_dispose_test(&client, false, true).await;
    }
}
