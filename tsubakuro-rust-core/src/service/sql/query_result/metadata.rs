use crate::{jogasaki::proto::sql::response::ResultSetMetadata, prelude::SqlColumn};

impl ResultSetMetadata {
    pub fn columns(&self) -> &Vec<SqlColumn> {
        &self.columns
    }
}
