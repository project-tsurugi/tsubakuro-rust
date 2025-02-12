#[cfg(test)]
mod test {
    use crate::test::{create_table, create_test_sql_client, drop_table_if_exists};
    use tokio::test;
    use tsubakuro_rust_core::{error::TgError, prelude::AtomType};

    #[test]
    async fn table_metadata() {
        let client = create_test_sql_client().await;
        drop_table_if_exists(&client, "test").await;

        {
            let error = client.get_table_metadata("test").await.unwrap_err();
            if let TgError::ServerError(_, _message, code, _server_message) = error {
                assert_eq!("TARGET_NOT_FOUND_EXCEPTION", code.name())
            } else {
                panic!("{:?}", error);
            }
        }

        create_table(
            &client,
            "test",
            "create table test (
        foo int primary key,
        bar bigint,
        zzz varchar(10)
        )",
        )
        .await;

        {
            let metadata = client.get_table_metadata("test").await.unwrap();
            assert_eq!("", metadata.database_name());
            assert_eq!("", metadata.schema_name());
            assert_eq!("test", metadata.table_name());

            let columns = metadata.columns();
            assert_eq!(3, columns.len());
            let c = &columns[0];
            assert_eq!("foo", c.name());
            assert_eq!(Some(AtomType::Int4), c.atom_type());
            let c = &columns[1];
            assert_eq!("bar", c.name());
            assert_eq!(Some(AtomType::Int8), c.atom_type());
            let c = &columns[2];
            assert_eq!("zzz", c.name());
            assert_eq!(Some(AtomType::Character), c.atom_type());
        }
    }

    #[test]
    async fn table_metadata_async() {
        let client = create_test_sql_client().await;
        drop_table_if_exists(&client, "test").await;

        {
            let mut job = client.get_table_metadata_async("test").await.unwrap();
            assert_eq!("TableMetadata", job.name());
            let error = job.take().await.unwrap_err();
            if let TgError::ServerError(_, _message, code, _server_message) = error {
                assert_eq!("TARGET_NOT_FOUND_EXCEPTION", code.name())
            } else {
                panic!("{:?}", error);
            }
        }

        create_table(
            &client,
            "test",
            "create table test (
        foo int primary key,
        bar bigint,
        zzz varchar(10)
        )",
        )
        .await;

        {
            let mut job = client.get_table_metadata_async("test").await.unwrap();
            let metadata = job.take().await.unwrap();
            assert_eq!("", metadata.database_name());
            assert_eq!("", metadata.schema_name());
            assert_eq!("test", metadata.table_name());

            let columns = metadata.columns();
            assert_eq!(3, columns.len());
            let c = &columns[0];
            assert_eq!("foo", c.name());
            assert_eq!(Some(AtomType::Int4), c.atom_type());
            let c = &columns[1];
            assert_eq!("bar", c.name());
            assert_eq!(Some(AtomType::Int8), c.atom_type());
            let c = &columns[2];
            assert_eq!("zzz", c.name());
            assert_eq!(Some(AtomType::Character), c.atom_type());
        }
    }
}
