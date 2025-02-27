use crate::{jogasaki::proto::sql::response::ResultSetMetadata, prelude::SqlColumn};

impl ResultSetMetadata {
    /// Returns the column information of the query.
    pub fn columns(&self) -> &Vec<SqlColumn> {
        &self.columns
    }
}
