#[cfg(test)]
mod test {
    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[test]
    async fn last_empty1() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (pk int primary key, value varchar(10))",
        )
        .await;

        insert(&client).await;
        select(&client).await;
    }

    async fn insert(client: &SqlClient) {
        let transaction = start_occ(&client).await;

        client
            .execute(
                &transaction,
                "insert into test values(1, 'Hello'), (2, null), (3, 'Hi'), (4, '')",
            )
            .await
            .unwrap();

        commit_and_close(client, &transaction).await;
    }

    async fn select(client: &SqlClient) {
        let sql = "select * from test order by pk";

        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();

        {
            assert_eq!(true, query_result.next_row().await.unwrap());

            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(1, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            let value: String = query_result.fetch().await.unwrap();
            assert_eq!("Hello", value);
        }
        {
            assert_eq!(true, query_result.next_row().await.unwrap());

            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(2, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(true, query_result.is_null().unwrap());
        }
        {
            assert_eq!(true, query_result.next_row().await.unwrap());

            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(3, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            let value: String = query_result.fetch().await.unwrap();
            assert_eq!("Hi", value);
        }
        {
            assert_eq!(true, query_result.next_row().await.unwrap());

            assert_eq!(true, query_result.next_column().await.unwrap());
            assert_eq!(4, query_result.fetch().await.unwrap());
            assert_eq!(true, query_result.next_column().await.unwrap());
            let value: String = query_result.fetch().await.unwrap();
            assert_eq!("", value);
        }
        assert_eq!(false, query_result.next_row().await.unwrap());

        query_result.close().await.unwrap();

        commit_and_close(client, &transaction).await;
    }

    #[test]
    async fn last_empty2() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (pk int primary key, value varchar(10))",
        )
        .await;

        insert(&client).await;

        let transaction = start_occ(&client).await;

        let sql = "select * from test order by pk";
        let mut query_result = client.query(&transaction, sql).await.unwrap();

        while query_result.next_row().await.unwrap() {
            // do nothing
        }

        query_result.close().await.unwrap();

        commit_and_close(&client, &transaction).await;
    }
}
