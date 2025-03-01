use crate::prelude::{SqlColumn, SqlQueryResultMetadata};

/// See [SqlQueryResult::get_metadata()](crate::prelude::SqlQueryResult::get_metadata).
impl SqlQueryResultMetadata {
    /// Returns the column information of the query.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// fn example(metadata: &SqlQueryResultMetadata) {
    ///     let columns = metadata.columns();
    ///
    ///     for column in columns {
    ///         let column_name = column.name();
    ///         let atom_type = column.atom_type();
    ///     }
    /// }
    /// ```
    pub fn columns(&self) -> &Vec<SqlColumn> {
        &self.columns
    }
}
