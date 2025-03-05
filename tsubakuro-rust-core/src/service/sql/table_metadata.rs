use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::response::Response as SqlResponseType,
    prelude::{convert_sql_response, SqlColumn},
    session::wire::response::WireResponse,
    sql_service_error,
};

/// Represents metadata of tables.
pub struct TableMetadata {
    describe_table: crate::jogasaki::proto::sql::response::describe_table::Success,
}

impl std::fmt::Debug for TableMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableMetadata")
            .field("database_name", &self.describe_table.database_name)
            .field("schema_name", &self.describe_table.schema_name)
            .field("table_name:", &self.describe_table.table_name)
            .field("columns", &self.describe_table.columns)
            .finish()
    }
}
impl TableMetadata {
    pub(crate) fn new(
        describe_table: crate::jogasaki::proto::sql::response::describe_table::Success,
    ) -> TableMetadata {
        TableMetadata { describe_table }
    }

    /// Returns the database name where the table defined.
    pub fn database_name(&self) -> &String {
        &self.describe_table.database_name
    }

    /// Returns the schema name where the table defined.
    pub fn schema_name(&self) -> &String {
        &self.describe_table.schema_name
    }

    /// Returns simple name of the table.
    pub fn table_name(&self) -> &String {
        &self.describe_table.table_name
    }

    /// Returns the column information of the table.
    pub fn columns(&self) -> &Vec<SqlColumn> {
        &self.describe_table.columns
    }
}

pub(crate) fn table_metadata_processor(response: WireResponse) -> Result<TableMetadata, TgError> {
    const FUNCTION_NAME: &str = "table_metadata_processor()";

    let (sql_response, _) = convert_sql_response(FUNCTION_NAME, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        FUNCTION_NAME,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;
    match message.response {
        Some(SqlResponseType::DescribeTable(describe_table)) => match describe_table.result {
            Some(crate::jogasaki::proto::sql::response::describe_table::Result::Success(
                success,
            )) => Ok(TableMetadata::new(success)),
            Some(crate::jogasaki::proto::sql::response::describe_table::Result::Error(error)) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
            None => Err(invalid_response_error!(
                FUNCTION_NAME,
                format!("response DescribeTable.result is None"),
            )),
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not DescribeTable", message.response),
        )),
    }
}
