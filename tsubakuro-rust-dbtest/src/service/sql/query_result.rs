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

    #[test]
    async fn close_server_error() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (pk int primary key, value varchar(10))",
        )
        .await;

        insert(&client).await;

        let transaction = start_occ(&client).await;
        let error = client
            .execute(&transaction, "insert into test values(1, 'dup')")
            .await
            .unwrap_err();
        if let TgError::ServerError(_, _message, code, _server_message) = error {
            assert_eq!("UNIQUE_CONSTRAINT_VIOLATION_EXCEPTION", code.name())
        } else {
            panic!("{:?}", error);
        }

        let sql = "select * from test order by pk";
        let mut query_result = client.query(&transaction, sql).await.unwrap();

        // first close
        let error = query_result.close().await.unwrap_err();
        if let TgError::ServerError(_, _message, code, _server_message) = error {
            assert_eq!("INACTIVE_TRANSACTION_EXCEPTION", code.name())
        } else {
            panic!("{:?}", error);
        }

        // second close
        query_result.close().await.unwrap();

        let commit_option = CommitOption::new();
        let error = client
            .commit(&transaction, &commit_option)
            .await
            .unwrap_err();
        if let TgError::ServerError(_, _message, code, _server_message) = error {
            assert_eq!("INACTIVE_TRANSACTION_EXCEPTION", code.name())
        } else {
            panic!("{:?}", error);
        }

        transaction.close().await.unwrap();
    }

    #[test]
    async fn select_server_error() {
        let client = create_test_sql_client().await;

        create_table(
            &client,
            "test",
            "create table test (pk int primary key, value varchar(10))",
        )
        .await;

        insert(&client).await;

        let transaction = start_occ(&client).await;
        let error = client
            .execute(&transaction, "insert into test values(1, 'dup')")
            .await
            .unwrap_err();
        if let TgError::ServerError(_, _message, code, _server_message) = error {
            assert_eq!("UNIQUE_CONSTRAINT_VIOLATION_EXCEPTION", code.name())
        } else {
            panic!("{:?}", error);
        }

        let sql = "select * from test order by pk";
        let mut query_result = client.query(&transaction, sql).await.unwrap();

        // first next_row()
        let error = get_from_query_result(&mut query_result).await.unwrap_err();
        if let TgError::ServerError(_, _message, code, _server_message) = error {
            assert_eq!("INACTIVE_TRANSACTION_EXCEPTION", code.name())
        } else {
            panic!("{:?}", error);
        }

        // second next_row()
        let error = query_result.next_row().await.unwrap_err();
        if let TgError::ServerError(_, _message, code, _server_message) = error {
            assert_eq!("INACTIVE_TRANSACTION_EXCEPTION", code.name())
        } else {
            panic!("{:?}", error);
        }

        query_result.close().await.unwrap();

        let commit_option = CommitOption::new();
        let error = client
            .commit(&transaction, &commit_option)
            .await
            .unwrap_err();
        if let TgError::ServerError(_, _message, code, _server_message) = error {
            assert_eq!("INACTIVE_TRANSACTION_EXCEPTION", code.name())
        } else {
            panic!("{:?}", error);
        }

        transaction.close().await.unwrap();
    }

    async fn get_from_query_result(query_result: &mut SqlQueryResult) -> Result<(), TgError> {
        while query_result.next_row().await? {
            assert_eq!(true, query_result.next_column().await?);
            let _pk: i32 = query_result.fetch().await?;

            assert_eq!(true, query_result.next_column().await?);
            let _value: String = query_result.fetch().await?;

            assert_eq!(false, query_result.next_column().await?);
        }
        Ok(())
    }
}
