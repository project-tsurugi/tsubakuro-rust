#[cfg(test)]
mod test {
    use std::{
        io::{Read, Write},
        time::Duration,
    };

    use crate::test::{commit_and_close, create_table, create_test_connection_option, start_occ};
    use log::warn;
    use tempfile::NamedTempFile;
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    async fn create_sql_client() -> Result<Option<SqlClient>, TgError> {
        let mut option = create_test_connection_option();
        option.set_lob_transfer_type(LobTransferType::Relay);
        let session = match Session::connect(&option).await {
            Ok(session) => session,
            Err(e) => {
                let _ = env_logger::builder().is_test(true).try_init();
                warn!("clob_relay::test: Failed to connect: {e}");
                return Ok(None);
            }
        };
        session.set_fail_on_drop_error(true);

        assert_eq!(LobTransferType::Relay, session.lob_transfer_type());

        let client: SqlClient = session.make_client();
        assert_eq!(
            true,
            client
                .allows_lob_operation(LobOperation::UploadLobFile)
                .await?
        );
        assert_eq!(
            true,
            client.allows_lob_operation(LobOperation::UploadLob).await?
        );
        assert_eq!(
            true,
            client
                .allows_lob_operation(LobOperation::CreateLobUploader)
                .await?
        );
        assert_eq!(
            false,
            client.allows_lob_operation(LobOperation::OpenLob).await?
        );
        assert_eq!(
            true,
            client
                .allows_lob_operation(LobOperation::GetLobCache)
                .await?
        );
        assert_eq!(
            true,
            client.allows_lob_operation(LobOperation::ReadLob).await?
        );
        assert_eq!(
            true,
            client.allows_lob_operation(LobOperation::CopyLobTo).await?
        );
        assert_eq!(
            true,
            client
                .allows_lob_operation(LobOperation::CreateLobDownloader)
                .await?
        );

        Ok(Some(client))
    }

    async fn create_test_table(client: &SqlClient) {
        create_table(
            client,
            "test",
            "create table test (pk int primary key, v clob)",
        )
        .await;
    }

    fn generate_values() -> Vec<(i32, Option<String>)> {
        let mut values = vec![];

        values.push((0, None));
        values.push((1, Some(String::new())));
        values.push((2, Some(String::from("a"))));
        values.push((3, Some(String::from("ABC"))));
        values.push((
            4,
            Some(String::from_utf8(vec!['X' as u8; 1024 * 1024]).unwrap()),
        ));
        values.push((
            5,
            Some(String::from_utf8(vec!['Y' as u8; 1024 * 1024 - 1]).unwrap()),
        ));
        values.push((
            6,
            Some(String::from_utf8(vec!['Z' as u8; 1024 * 1024 + 1]).unwrap()),
        ));

        values
    }

    #[test]
    async fn upload_lob_file() -> Result<(), TgError> {
        let client = match create_sql_client().await? {
            Some(client) => client,
            None => return Ok(()),
        };
        create_test_table(&client).await;
        let values = generate_values();
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<TgClob>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in &values {
            let mut file;
            let clob = match &value.1 {
                Some(value) => {
                    file = NamedTempFile::new().unwrap();
                    file.write_all(value.as_bytes()).unwrap();
                    let clob = client.upload_clob_file(&file.path()).await?;
                    Some(clob)
                }
                None => None,
            };

            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", clob),
            ];
            let result = client
                .prepared_execute(&transaction, &ps, parameters)
                .await?;
            assert_eq!(1, result.rows());
        }

        commit_and_close(&client, &transaction).await;

        ps.close().await?;

        select(&client, &values).await
    }

    #[test]
    async fn upload_lob_file_async() -> Result<(), TgError> {
        let client = match create_sql_client().await? {
            Some(client) => client,
            None => return Ok(()),
        };
        create_test_table(&client).await;
        let values = generate_values();
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<TgClob>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in &values {
            let mut file;
            let clob = match &value.1 {
                Some(value) => {
                    file = NamedTempFile::new().unwrap();
                    file.write_all(value.as_bytes()).unwrap();
                    let mut job = client.upload_clob_file_async(&file.path()).await?;
                    let clob = job.take().await?;
                    Some(clob)
                }
                None => None,
            };

            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", clob),
            ];
            let result = client
                .prepared_execute(&transaction, &ps, parameters)
                .await?;
            assert_eq!(1, result.rows());
        }

        commit_and_close(&client, &transaction).await;

        ps.close().await?;

        select(&client, &values).await
    }

    #[test]
    async fn upload_lob() -> Result<(), TgError> {
        let client = match create_sql_client().await? {
            Some(client) => client,
            None => return Ok(()),
        };
        create_test_table(&client).await;
        let values = generate_values();
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<TgClob>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in &values {
            let clob = match &value.1 {
                Some(value) => {
                    let clob = client.upload_clob(&value).await?;
                    Some(clob)
                }
                None => None,
            };

            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", clob),
            ];
            let result = client
                .prepared_execute(&transaction, &ps, parameters)
                .await?;
            assert_eq!(1, result.rows());
        }

        commit_and_close(&client, &transaction).await;

        ps.close().await?;

        select(&client, &values).await
    }

    #[test]
    async fn upload_lob_async() -> Result<(), TgError> {
        let client = match create_sql_client().await? {
            Some(client) => client,
            None => return Ok(()),
        };
        create_test_table(&client).await;
        let values = generate_values();
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<TgClob>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in &values {
            let clob = match &value.1 {
                Some(value) => {
                    let mut job = client.upload_clob_async(&value).await?;
                    let clob = job.take().await?;
                    Some(clob)
                }
                None => None,
            };

            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", clob),
            ];
            let result = client
                .prepared_execute(&transaction, &ps, parameters)
                .await?;
            assert_eq!(1, result.rows());
        }

        commit_and_close(&client, &transaction).await;

        ps.close().await?;

        select(&client, &values).await
    }

    #[test]
    async fn uploader() -> Result<(), TgError> {
        let client = match create_sql_client().await? {
            Some(client) => client,
            None => return Ok(()),
        };
        create_test_table(&client).await;
        let values = generate_values();
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<TgClob>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in &values {
            let clob = match &value.1 {
                Some(value) => {
                    let mut uploader = client.create_clob_uploader().await?;
                    let timeout = Duration::from_secs(10);
                    for chunk in value.as_bytes().chunks(1024) {
                        uploader.upload_chunk(chunk, timeout).await?;
                    }
                    let clob = uploader.finish(timeout).await?;
                    Some(clob)
                }
                None => None,
            };

            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", clob),
            ];
            let result = client
                .prepared_execute(&transaction, &ps, parameters)
                .await?;
            assert_eq!(1, result.rows());
        }

        commit_and_close(&client, &transaction).await;

        ps.close().await?;

        select(&client, &values).await
    }

    async fn select(
        client: &SqlClient,
        expected: &Vec<(i32, Option<String>)>,
    ) -> Result<(), TgError> {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await?;

        let mut i = 0;
        while query_result.next_row().await? {
            let expected = &expected[i];

            assert_eq!(true, query_result.next_column().await?);
            let pk: i32 = query_result.fetch().await?;
            assert_eq!(expected.0, pk);

            assert_eq!(true, query_result.next_column().await?);
            let v: Option<TgClobReference> = query_result.fetch().await?;
            if let Some(clob) = v {
                // let mut file = client.open_clob(&transaction, &clob).await?;
                // let mut v = Vec::new();
                // file.read_to_end(&mut v).unwrap();
                // assert_eq!(expected.1, Some(v));

                let cache = client.get_clob_cache(&transaction, &clob).await?;
                assert_eq!(None, cache.path());

                let mut job = client.get_clob_cache_async(&transaction, &clob).await?;
                let cache = job.take().await?;
                assert_eq!(None, cache.path());

                let v = client.read_clob(&transaction, &clob).await?;
                assert_eq!(expected.1, Some(v));

                let mut job = client.read_clob_async(&transaction, &clob).await?;
                let v = job.take().await?;
                assert_eq!(expected.1, Some(v));

                let mut file = NamedTempFile::new().unwrap();
                client
                    .copy_clob_to(&transaction, &clob, file.path())
                    .await?;
                let mut v = String::new();
                file.read_to_string(&mut v).unwrap();
                assert_eq!(expected.1, Some(v));

                let mut file = NamedTempFile::new().unwrap();
                let mut job = client
                    .copy_clob_to_async(&transaction, &clob, file.path())
                    .await?;
                let _ = job.take().await?;
                let mut v = String::new();
                file.read_to_string(&mut v).unwrap();
                assert_eq!(expected.1, Some(v));

                let timeout = Duration::from_secs(10);
                let mut v = Vec::new();
                let mut downloader = client
                    .create_clob_downloader(&transaction, &clob, timeout)
                    .await?;
                while let Some(chunk) = downloader.download_chunk_utf8(1024, timeout).await? {
                    v.extend_from_slice(&chunk);
                }
                assert_eq!(expected.1, Some(String::from_utf8(v).unwrap()));

                let mut v = Vec::new();
                let mut downloader = client
                    .create_clob_downloader(&transaction, &clob, timeout)
                    .await?;
                let mut chunk = [0u8; 1024];
                loop {
                    let len = downloader
                        .download_chunk_into_utf8(&mut chunk, timeout)
                        .await?;
                    if len == 0 {
                        break;
                    }
                    v.extend_from_slice(&chunk[..len]);
                }
                assert_eq!(expected.1, Some(String::from_utf8(v).unwrap()));
            } else {
                assert_eq!(expected.1, None);
            }

            i += 1;
        }
        assert_eq!(expected.len(), i);

        query_result.close().await?;

        commit_and_close(client, &transaction).await;
        Ok(())
    }
}
