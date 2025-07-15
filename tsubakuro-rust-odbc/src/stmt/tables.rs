use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::TableList;

use crate::{
    check_sql_client, check_stmt,
    ctype::{
        SqlChar, SqlDataType, SqlLen, SqlNullable::*, SqlReturn, SqlSmallInt, SqlUSmallInt,
        SqlWChar,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::{
        describe_col::TsurugiOdbcDescribeColumn,
        get_data::{
            get_data_null, get_data_string, get_data_string_opt, TsurugiOdbcGetDataArguments,
        },
        TsurugiOdbcStatementProcessor,
    },
};

#[no_mangle]
pub extern "system" fn SQLTables(
    hstmt: HStmt,
    catalog_name: *const SqlChar,
    name_length_1: SqlSmallInt,
    schema_name: *const SqlChar,
    name_length_2: SqlSmallInt,
    table_name: *const SqlChar,
    name_length_3: SqlSmallInt,
    table_type: *const SqlChar,
    name_length_4: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLTables()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, catalog_name={:?}, name_length_1={:?}, schema_name={:?}, name_length_2={:?}, table_name={:?}, name_length_3={:?}, table_type={:?}, name_length_4={:?}",
        hstmt,
        catalog_name,
        name_length_1,
        schema_name,
        name_length_2,
        table_name,
        name_length_3,
        table_type,
        name_length_4
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLTables");

    // TODO SQLTables(): table_type, table_name の判定
    let rc = tables(&mut stmt);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLTablesW(
    hstmt: HStmt,
    catalog_name: *const SqlWChar,
    name_length_1: SqlSmallInt,
    schema_name: *const SqlWChar,
    name_length_2: SqlSmallInt,
    table_name: *const SqlWChar,
    name_length_3: SqlSmallInt,
    table_type: *const SqlWChar,
    name_length_4: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLTablesW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, catalog_name={:?}, name_length_1={:?}, schema_name={:?}, name_length_2={:?}, table_name={:?}, name_length_3={:?}, table_type={:?}, name_length_4={:?}",
        hstmt,
        catalog_name,
        name_length_1,
        schema_name,
        name_length_2,
        table_name,
        name_length_3,
        table_type,
        name_length_4
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLTablesW");

    // TODO SQLTables(): table_type, table_name の判定
    let rc = tables(&mut stmt);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn tables(stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
    const FUNCTION_NAME: &str = "ables()";

    let sql_client = check_sql_client!(stmt);
    let runtime = stmt.runtime();
    let result = runtime.block_on(sql_client.list_tables());
    let table_list = match result {
        Ok(tables) => {
            debug!("{stmt}.{FUNCTION_NAME}: list_tables() succeeded");
            tables
        }
        Err(e) => {
            warn!("{stmt}.{FUNCTION_NAME}: list_tables() error. {:?}", e);
            stmt.add_diag(
                TsurugiOdbcError::ListTablesError,
                format!("list tables error. {}", e),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    let processor = TsurugiOdbcTableList::new(table_list);
    stmt.set_processor(processor);

    SqlReturn::SQL_SUCCESS
}

struct TsurugiOdbcTableList {
    table_list: TableList,
    row_index: isize,
}

impl TsurugiOdbcTableList {
    fn new(table_list: TableList) -> Self {
        Self {
            table_list,
            row_index: -1,
        }
    }
}

impl TsurugiOdbcStatementProcessor for TsurugiOdbcTableList {
    fn number_of_columns(&self) -> SqlUSmallInt {
        5
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
            3 => TsurugiOdbcDescribeColumn::new("TABLE_TYPE", SQL_VARCHAR, SQL_NO_NULLS),
            4 => TsurugiOdbcDescribeColumn::new("REMARKS", SQL_VARCHAR, SQL_NULLABLE),
            _ => unreachable!(),
        };
        Ok(column)
    }

    fn row_count(&self) -> SqlLen {
        let table_names = self.table_list.table_names();
        table_names.len() as SqlLen
    }

    fn fetch(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        let index = self.row_index + 1;
        if index < self.row_count() {
            self.row_index = index;
            SqlReturn::SQL_SUCCESS
        } else {
            SqlReturn::SQL_NO_DATA
        }
    }

    fn get_data(&mut self, stmt: &TsurugiOdbcStmt, arg: &TsurugiOdbcGetDataArguments) -> SqlReturn {
        const FUNCTION_NAME: &str = "TsurugiOdbcTableList.get_data()";

        let table_names = self.table_list.table_names();
        if self.row_index < 0 || self.row_index as usize >= table_names.len() {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. index out of bounds. self.row_index={}",
                self.row_index
            );
            return SqlReturn::SQL_NO_DATA;
        }
        let name = &table_names[self.row_index as usize];

        match arg.column_index() {
            0 => get_data_string_opt(stmt, arg, name.database_name()), // TABLE_CAT varchar
            1 => get_data_string_opt(stmt, arg, name.schema_name()),   // TABLE_SCHEM varchar
            2 => get_data_string_opt(stmt, arg, name.last_name()),     // TABLE_NAME varchar
            3 => get_data_string(stmt, arg, "TABLE"),                  // TABLE_TYPE varchar
            4 => get_data_null(stmt, arg),                             // REMARKS varchar
            _ => unreachable!(),
        }
    }

    fn dispose(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        SqlReturn::SQL_SUCCESS
    }
}
