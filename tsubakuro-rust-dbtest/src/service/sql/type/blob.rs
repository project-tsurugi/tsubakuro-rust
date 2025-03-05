#[cfg(test)]
mod test {
    use std::io::{Read, Write};

    use crate::test::{commit_and_close, create_table, create_test_sql_client, start_occ};
    use tempfile::NamedTempFile;
    use tokio::test;
    use tsubakuro_rust_core::prelude::*;

    #[allow(dead_code)]
    //TODO #[test]
    async fn literal() {
        let client = create_test_sql_client().await;

        create_test_table(&client).await;

        let values = generate_values();

        insert_literal(&client, &values).await;
        select(&client, &values, false).await;
        select(&client, &values, true).await;
    }

    #[test]
    async fn prepare() {
        let client = create_test_sql_client().await;

        create_test_table(&client).await;

        let values = generate_values();

        let inserted = insert_prepared(&client, &values).await;
        if inserted {
            select(&client, &values, false).await;
            select(&client, &values, true).await;
        } else {
            println!("(skip select)");
        }
    }

    async fn create_test_table(client: &SqlClient) {
        create_table(
            client,
            "test",
            "create table test (pk int primary key, v blob, r int default 999)",
        )
        .await;

        let metadata = client.get_table_metadata("test").await.unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::Blob), c.atom_type());
    }

    fn generate_values() -> Vec<(i32, Option<Vec<u8>>)> {
        let mut values = vec![];

        values.push((0, None));
        values.push((1, Some(vec![1, 2, 3])));
        values.push((2, Some(vec![0x11, 0x22, 0x33])));

        values
    }

    async fn insert_literal(client: &SqlClient, values: &Vec<(i32, Option<Vec<u8>>)>) {
        let transaction = start_occ(&client).await;

        for value in values {
            let sql = if let Some(v) = &value.1 {
                let v = v.iter().map(|b| format!("{:02x}", b)).collect::<String>();
                format!("insert into test (pk, v) values({}, X'{}')", value.0, v)
            } else {
                format!("insert into test (pk, v) values({}, null)", value.0)
            };
            client.execute(&transaction, &sql).await.unwrap();
        }

        commit_and_close(client, &transaction).await;
    }

    async fn insert_prepared(client: &SqlClient, values: &Vec<(i32, Option<Vec<u8>>)>) -> bool {
        let transaction = start_occ(&client).await;

        let sql = "insert into test (pk, v) values(:pk, :value)";
        let placeholders = vec![
            SqlPlaceholder::of::<i32>("pk"),
            SqlPlaceholder::of::<TgBlob>("value"),
        ];
        let ps = client.prepare(sql, placeholders).await.unwrap();

        for value in values {
            let mut file;
            let blob = match &value.1 {
                Some(value) => {
                    file = NamedTempFile::new().unwrap();
                    file.write_all(value).unwrap();
                    Some(TgBlob::new(file.path().to_str().unwrap()))
                }
                None => None,
            };

            let parameters = vec![
                SqlParameter::of("pk", value.0),
                SqlParameter::of("value", blob),
            ];
            let result = client.prepared_execute(&transaction, &ps, parameters).await;
            if let Err(ref e) = result {
                transaction.close().await.unwrap();
                match e.diagnostic_code() {
                    Some(code) => {
                        if code.name() == "IO_ERROR" {
                            return false;
                        }
                    }
                    None => {}
                }
                result.unwrap();
            }
        }

        commit_and_close(client, &transaction).await;

        ps.close().await.unwrap();
        true
    }

    async fn select(client: &SqlClient, expected: &Vec<(i32, Option<Vec<u8>>)>, skip: bool) {
        let sql = "select * from test order by pk";
        let transaction = start_occ(&client).await;

        let mut query_result = client.query(&transaction, sql).await.unwrap();

        let metadata = query_result.get_metadata().unwrap();
        let columns = metadata.columns();
        assert_eq!(3, columns.len());
        let c = &columns[1];
        assert_eq!("v", c.name());
        assert_eq!(Some(AtomType::Blob), c.atom_type());

        let mut i = 0;
        while query_result.next_row().await.unwrap() {
            let expected = &expected[i];

            assert_eq!(true, query_result.next_column().await.unwrap());
            let pk = query_result.fetch().await.unwrap();
            assert_eq!(expected.0, pk);

            assert_eq!(true, query_result.next_column().await.unwrap());
            if !skip {
                let v: Option<TgBlobReference> = query_result.fetch().await.unwrap();
                if let Some(blob) = v {
                    let mut file = client.open_blob(&transaction, &blob).await.unwrap();
                    let mut v = Vec::new();
                    file.read_to_end(&mut v).unwrap();
                    assert_eq!(expected.1, Some(v));

                    let mut file = NamedTempFile::new().unwrap();
                    client
                        .copy_blob_to(&transaction, &blob, file.path().to_str().unwrap())
                        .await
                        .unwrap();
                    let mut v = Vec::new();
                    file.read_to_end(&mut v).unwrap();
                    assert_eq!(expected.1, Some(v));
                } else {
                    assert_eq!(expected.1, None);
                }
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
