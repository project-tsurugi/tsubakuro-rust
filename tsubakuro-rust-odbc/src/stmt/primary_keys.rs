use log::{debug, trace};
use tsubakuro_rust_core::prelude::TableMetadata;

use crate::{
    check_stmt,
    ctype::{
        SqlChar, SqlDataType, SqlLen, SqlNullable::*, SqlReturn, SqlSmallInt, SqlUSmallInt,
        SqlWChar,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::{
        columns::get_table_metadata,
        describe_col::TsurugiOdbcDescribeColumn,
        get_data::{get_data_i32, get_data_null, get_data_string, TsurugiOdbcGetDataArguments},
        TsurugiOdbcStatementProcessor,
    },
    util::{char_to_string_opt, wchar_to_string_opt},
};

#[no_mangle]
pub extern "system" fn SQLPrimaryKeys(
    hstmt: HStmt,
    catalog_name: *const SqlChar,
    catalog_name_length: SqlSmallInt,
    schema_name: *const SqlChar,
    schema_name_length: SqlSmallInt,
    table_name: *const SqlChar,
    table_name_length: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLPrimaryKeys()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, catalog_name={:?}, catalog_name_length={:?}, schema_name={:?}, schema_name_length={:?}, table_name={:?}, table_name_length={:?}",
        hstmt,
        catalog_name,
        catalog_name_length,
        schema_name,
        schema_name_length,
        table_name,
        table_name_length,
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLPrimaryKeys");

    // TODO catalog_name, schema_name are not used in this implementation.

    let table_name = match char_to_string_opt(table_name, table_name_length) {
        Ok(Some(name)) => name,
        Ok(None) => {
            debug!("{stmt}.{FUNCTION_NAME} error. table_name is null");
            stmt.add_diag(TsurugiOdbcError::StringError, "table_name is null");
            return SqlReturn::SQL_ERROR;
        }
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME} table_name error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::StringError,
                format!("table_name error. {}", e),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    // TODO column_name is not used in this implementation.

    let rc = primary_keys(&mut stmt, table_name);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLPrimaryKeysW(
    hstmt: HStmt,
    catalog_name: *const SqlWChar,
    catalog_name_length: SqlSmallInt,
    schema_name: *const SqlWChar,
    schema_name_length: SqlSmallInt,
    table_name: *const SqlWChar,
    table_name_length: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLPrimaryKeysW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, catalog_name={:?}, catalog_name_length={:?}, schema_name={:?}, schema_name_length={:?}, table_name={:?}, table_name_length={:?}",
        hstmt,
        catalog_name,
        catalog_name_length,
        schema_name,
        schema_name_length,
        table_name,
        table_name_length,
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLPrimaryKeysW");

    // TODO catalog_name, schema_name are not used in this implementation.

    let table_name = match wchar_to_string_opt(table_name, table_name_length) {
        Ok(Some(name)) => name,
        Ok(None) => {
            debug!("{stmt}.{FUNCTION_NAME} error. table_name is null");
            stmt.add_diag(TsurugiOdbcError::StringError, "table_name is null");
            return SqlReturn::SQL_ERROR;
        }
        Err(e) => {
            debug!("{stmt}.{FUNCTION_NAME} table_name error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::StringError,
                format!("table_name error. {}", e),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    // TODO column_name is not used in this implementation.

    let rc = primary_keys(&mut stmt, table_name);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn primary_keys(stmt: &mut TsurugiOdbcStmt, table_name: String) -> SqlReturn {
    let metadata = match get_table_metadata(stmt, &table_name) {
        Ok(metadata) => metadata,
        Err(rc) => return rc,
    };

    let processor = TsurugiOdbcPrimaryKeys::new(metadata);
    stmt.set_processor(processor);

    SqlReturn::SQL_SUCCESS
}

struct TsurugiOdbcPrimaryKeys {
    metadata: Option<TableMetadata>,
    row_index: isize,
}

impl TsurugiOdbcPrimaryKeys {
    fn new(metadata: Option<TableMetadata>) -> TsurugiOdbcPrimaryKeys {
        TsurugiOdbcPrimaryKeys {
            metadata,
            row_index: -1,
        }
    }
}

impl TsurugiOdbcStatementProcessor for TsurugiOdbcPrimaryKeys {
    fn number_of_columns(&self) -> SqlUSmallInt {
        6
    }

    fn describe_column(
        &self,
        column_index: SqlUSmallInt,
    ) -> Result<TsurugiOdbcDescribeColumn, SqlReturn> {
        use SqlDataType::*;
        let column = match column_index {
            0 => TsurugiOdbcDescribeColumn::new("TABLE_CAT", SQL_VARCHAR, SQL_NULLABLE),
            1 => TsurugiOdbcDescribeColumn::new("TABLE_SCHEM", SQL_VARCHAR, SQL_NULLABLE),
            2 => TsurugiOdbcDescribeColumn::new("TABLE_NAME", SQL_VARCHAR, SQL_NO_NULLS),
            3 => TsurugiOdbcDescribeColumn::new("COLUMN_NAME", SQL_VARCHAR, SQL_NO_NULLS),
            4 => TsurugiOdbcDescribeColumn::new("KEY_SEQ", SQL_SMALLINT, SQL_NO_NULLS),
            5 => TsurugiOdbcDescribeColumn::new("PK_NAME", SQL_VARCHAR, SQL_NULLABLE),
            _ => unreachable!(),
        };
        Ok(column)
    }

    fn row_count(&self) -> SqlLen {
        match self.metadata {
            Some(ref metadata) => metadata.primary_keys().len() as SqlLen,
            None => 0,
        }
    }

    fn fetch(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        if self.metadata.is_none() {
            return SqlReturn::SQL_NO_DATA;
        }

        let index = self.row_index + 1;
        if index < self.row_count() {
            self.row_index = index;
            SqlReturn::SQL_SUCCESS
        } else {
            SqlReturn::SQL_NO_DATA
        }
    }

    fn get_data(&mut self, stmt: &TsurugiOdbcStmt, arg: &TsurugiOdbcGetDataArguments) -> SqlReturn {
        const FUNCTION_NAME: &str = "TsurugiOdbcPrimaryKeys.get_data()";

        let metadata = match self.metadata {
            Some(ref metadata) => metadata,
            None => {
                return SqlReturn::SQL_NO_DATA;
            }
        };

        let keys = metadata.primary_keys();
        if self.row_index < 0 || self.row_index as usize >= keys.len() {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. index out of bounds. self.row_index={}",
                self.row_index
            );
            return SqlReturn::SQL_NO_DATA;
        }

        match arg.column_index() {
            0 => get_data_string(stmt, arg, metadata.database_name()), // TABLE_CAT varchar
            1 => get_data_string(stmt, arg, metadata.schema_name()),   // TABLE_SCHEM varchar
            2 => get_data_string(stmt, arg, metadata.table_name()),    // TABLE_NAME varchar
            3 => get_data_string(stmt, arg, &keys[self.row_index as usize]), // COLUMN_NAME varchar
            4 => get_data_i32(stmt, arg, self.row_index as i32 + 1),   // KEY_SEQ Smallint
            5 => get_data_null(stmt, arg),                             // PK_NAME varchar
            _ => unreachable!(),
        }
    }

    fn dispose(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        SqlReturn::SQL_SUCCESS
    }
}
