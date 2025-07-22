use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::{AtomType, SqlColumn, TableMetadata};

use crate::{
    check_sql_client_or_err, check_stmt,
    ctype::{
        sql_numeric_struct::SqlNumericStruct, SqlChar, SqlDataType, SqlLen, SqlNullable::*,
        SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::{
        describe_col::TsurugiOdbcDescribeColumn,
        get_data::{
            get_data_i32, get_data_i32_opt, get_data_null, get_data_string, get_data_string_opt,
            TsurugiOdbcGetDataArguments,
        },
        TsurugiOdbcStatementProcessor,
    },
    util::{char_to_string_opt, wchar_to_string_opt},
};

#[no_mangle]
pub extern "system" fn SQLColumns(
    hstmt: HStmt,
    catalog_name: *const SqlChar,
    catalog_name_length: SqlSmallInt,
    schema_name: *const SqlChar,
    schema_name_length: SqlSmallInt,
    table_name: *const SqlChar,
    table_name_length: SqlSmallInt,
    column_name: *const SqlChar,
    column_name_length: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLColumns()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, catalog_name={:?}, catalog_name_length={:?}, schema_name={:?}, schema_name_length={:?}, table_name={:?}, table_name_length={:?}, column_name={:?}, column_name_length={:?}",
        hstmt,
        catalog_name,
        catalog_name_length,
        schema_name,
        schema_name_length,
        table_name,
        table_name_length,
        column_name,
        column_name_length
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLGetColumns");

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

    let rc = columns(&mut stmt, table_name);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLColumnsW(
    hstmt: HStmt,
    catalog_name: *const SqlWChar,
    catalog_name_length: SqlSmallInt,
    schema_name: *const SqlWChar,
    schema_name_length: SqlSmallInt,
    table_name: *const SqlWChar,
    table_name_length: SqlSmallInt,
    column_name: *const SqlWChar,
    column_name_length: SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLColumnsW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, catalog_name={:?}, catalog_name_length={:?}, schema_name={:?}, schema_name_length={:?}, table_name={:?}, table_name_length={:?}, column_name={:?}, column_name_length={:?}",
        hstmt,
        catalog_name,
        catalog_name_length,
        schema_name,
        schema_name_length,
        table_name,
        table_name_length,
        column_name,
        column_name_length
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLGetColumns");

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

    let rc = columns(&mut stmt, table_name);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn columns(stmt: &mut TsurugiOdbcStmt, table_name: String) -> SqlReturn {
    let metadata = match get_table_metadata(stmt, &table_name) {
        Ok(metadata) => metadata,
        Err(rc) => return rc,
    };

    let processor = TsurugiOdbcColumns::new(metadata);
    stmt.set_processor(processor);

    SqlReturn::SQL_SUCCESS
}

pub(crate) fn get_table_metadata(
    stmt: &TsurugiOdbcStmt,
    table_name: &str,
) -> Result<Option<TableMetadata>, SqlReturn> {
    const FUNCTION_NAME: &str = "get_table_metadata()";

    let sql_client = check_sql_client_or_err!(stmt);

    let runtime = stmt.runtime();
    let result = runtime.block_on(sql_client.get_table_metadata(table_name));
    match result {
        Ok(metadata) => {
            debug!("{stmt}.{FUNCTION_NAME}: sql_client.get_table_metadata() succeeded");
            Ok(Some(metadata))
        }
        Err(e) => {
            if let Some(code) = e.diagnostic_code() {
                if code.category_number() == 3 && code.code_number() == 2014 {
                    // SQL-02014 TARGET_NOT_FOUND_EXCEPTION
                    debug!(
                        "{stmt}.{FUNCTION_NAME}: sql_client.get_table_metadata() table not found"
                    );
                    return Ok(None);
                }
            }

            warn!(
                "{stmt}.{FUNCTION_NAME}: sql_client.get_table_metadata() error. {:?}",
                e
            );
            stmt.add_diag(
                TsurugiOdbcError::GetTableMetadataError,
                format!("get table metadta error. {}", e),
            );
            Err(SqlReturn::SQL_ERROR)
        }
    }
}

struct TsurugiOdbcColumns {
    metadata: Option<TableMetadata>,
    row_index: isize,
}

impl TsurugiOdbcColumns {
    fn new(metadata: Option<TableMetadata>) -> TsurugiOdbcColumns {
        TsurugiOdbcColumns {
            metadata,
            row_index: -1,
        }
    }
}

impl TsurugiOdbcStatementProcessor for TsurugiOdbcColumns {
    fn number_of_columns(&self) -> SqlUSmallInt {
        18
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
            4 => TsurugiOdbcDescribeColumn::new("DATA_TYPE", SQL_SMALLINT, SQL_NO_NULLS),
            5 => TsurugiOdbcDescribeColumn::new("TYPE_NAME", SQL_VARCHAR, SQL_NO_NULLS),
            6 => TsurugiOdbcDescribeColumn::new("COLUMN_SIZE", SQL_INTEGER, SQL_NULLABLE),
            7 => TsurugiOdbcDescribeColumn::new("BUFFER_LENGTH", SQL_INTEGER, SQL_NULLABLE),
            8 => TsurugiOdbcDescribeColumn::new("DECIMAL_DIGITS", SQL_SMALLINT, SQL_NULLABLE),
            9 => TsurugiOdbcDescribeColumn::new("NUM_PREC_RADIX", SQL_SMALLINT, SQL_NULLABLE),
            10 => TsurugiOdbcDescribeColumn::new("NULLABLE", SQL_SMALLINT, SQL_NO_NULLS),
            11 => TsurugiOdbcDescribeColumn::new("REMARKS", SQL_VARCHAR, SQL_NULLABLE),
            12 => TsurugiOdbcDescribeColumn::new("COLUMN_DEF", SQL_VARCHAR, SQL_NULLABLE),
            13 => TsurugiOdbcDescribeColumn::new("SQL_DATA_TYPE", SQL_SMALLINT, SQL_NO_NULLS),
            14 => TsurugiOdbcDescribeColumn::new("SQL_DATETIME_SUB", SQL_SMALLINT, SQL_NULLABLE),
            15 => TsurugiOdbcDescribeColumn::new("CHAR_OCTET_LENGTH", SQL_INTEGER, SQL_NULLABLE),
            16 => TsurugiOdbcDescribeColumn::new("ORDINAL_POSITION", SQL_INTEGER, SQL_NO_NULLS),
            17 => TsurugiOdbcDescribeColumn::new("IS_NULLABLE", SQL_VARCHAR, SQL_NULLABLE),
            _ => unreachable!(),
        };
        Ok(column)
    }

    fn row_count(&self) -> SqlLen {
        match self.metadata {
            Some(ref metadata) => metadata.columns().len() as SqlLen,
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
        const FUNCTION_NAME: &str = "TsurugiOdbcColumns.get_data()";

        let metadata = match self.metadata {
            Some(ref metadata) => metadata,
            None => {
                return SqlReturn::SQL_NO_DATA;
            }
        };

        let columns = metadata.columns();
        if self.row_index < 0 || self.row_index as usize >= columns.len() {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. index out of bounds. self.row_index={}",
                self.row_index
            );
            return SqlReturn::SQL_NO_DATA;
        }
        let column = &columns[self.row_index as usize];

        // TODO SQLGetColumn() field
        let column_index = arg.column_index();
        match column_index {
            0 => get_data_string(stmt, arg, metadata.database_name()), // TABLE_CAT varchar
            1 => get_data_string(stmt, arg, metadata.schema_name()),   // TABLE_SCHEM varchar
            2 => get_data_string(stmt, arg, metadata.table_name()),    // TABLE_NAME varchar
            3 => get_data_string(stmt, arg, column.name()),            // COLUMN_NAME varchar
            4 => get_data_i32(stmt, arg, SqlDataType::from(column) as i32), // DATA_TYPE SmallInt
            5 => get_data_string_opt(stmt, arg, column.sql_type()),    // TYPE_NAME varchar
            6 => get_data_i32_opt(stmt, arg, column_size(column)),     // COLUMN_SIZE Integer
            7 => get_data_i32_opt(stmt, arg, column_buffer_length(column)), // BUFFER_LENGTH Integer
            8 => get_data_i32_opt(stmt, arg, decimal_digits(column)),  // DECIMAL_DIGITS SmallInt
            9 => get_data_i32_opt(stmt, arg, num_prec_radix(column)),  // NUM_PREC_RADIX SmallInt
            10 => get_data_i32_opt(
                stmt,
                arg,
                column.nullable().map(|b| {
                    if b {
                        SQL_NULLABLE as i32
                    } else {
                        SQL_NO_NULLS as i32
                    }
                }),
            ), // NULLABLE SmallInt
            11 => get_data_string_opt(stmt, arg, column.description()), // REMARKS varchar
            12 => not_yet_implemented(stmt, arg),                      // COLUMN_DEF varchar
            13 => get_data_i32(stmt, arg, SqlDataType::from(column) as i32), // SQL_DATA_TYPE SmallInt
            14 => not_yet_implemented(stmt, arg), // SQL_DATETIME_SUB SmallInt
            15 => get_data_i32_opt(stmt, arg, char_octet_length(column)), // CHAR_OCTET_LENGTH Integer
            16 => get_data_i32(stmt, arg, self.row_index as i32 + 1), // ORDINAL_POSITION Integer
            17 => get_data_string_opt(
                stmt,
                arg,
                column.nullable().map(|b| if b { "YES" } else { "NO" }),
            ), // IS_NULLABLE varchar
            _ => unreachable!(),
        }
    }

    fn dispose(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        SqlReturn::SQL_SUCCESS
    }
}

fn column_size(column: &SqlColumn) -> Option<i32> {
    use AtomType::*;
    let size = match column.atom_type()? {
        Boolean => 1,  // bits
        Int4 => 8 * 4, // bits
        Int8 => 8 * 8, // bits
        Float4 => 38,
        Float8 => 308,
        Decimal => return None,
        Character | Octet => return char_octet_length(column),
        Date => 10,             // yyyy-MM-dd
        TimeOfDay => 8 + 1 + 9, // HH:mm:ss.nnnnnnnnn
        TimePoint => 10 + 1 + (8 + 1 + 9),
        TimeOfDayWithTimeZone => (8 + 1 + 9) + 6, // +hh:mm
        TimePointWithTimeZone => 10 + 1 + (8 + 1 + 9) + 6, // +hh:mm
        _ => return None,
    };

    Some(size)
}

fn column_buffer_length(column: &SqlColumn) -> Option<i32> {
    use AtomType::*;
    let length = match column.atom_type()? {
        Boolean => 1,
        Int4 => 4,
        Int8 => 8,
        Float4 => 4,
        Float8 => 8,
        Decimal => std::mem::size_of::<SqlNumericStruct>() as i32,
        Character | Octet => return char_octet_length(column),
        // Bit => todo!(),
        // Date => todo!(),
        // TimeOfDay => todo!(),
        // TimePoint => todo!(),
        // DatetimeInterval => todo!(),
        // TimeOfDayWithTimeZone => todo!(),
        // TimePointWithTimeZone => todo!(),
        // Clob => todo!(),
        // Blob => todo!(),
        _ => return None,
    };

    Some(length)
}

pub(crate) fn decimal_digits(column: &SqlColumn) -> Option<i32> {
    use AtomType::*;
    let scale = match column.atom_type()? {
        Decimal => match column.scale() {
            Some((scale, arbitrary)) => {
                if arbitrary {
                    return None;
                } else {
                    scale as i32
                }
            }
            None => 0,
        },
        TimeOfDay => 9, // 秒の小数以下の桁数
        TimePoint => 9,
        TimeOfDayWithTimeZone => 9,
        TimePointWithTimeZone => 9,
        _ => return None,
    };

    Some(scale)
}

fn num_prec_radix(column: &SqlColumn) -> Option<i32> {
    use AtomType::*;
    let radix = match column.atom_type()? {
        Boolean => 2,
        Int4 => 2,
        Int8 => 2,
        Float4 => 10,
        Float8 => 10,
        Decimal => 10,
        _ => return None,
    };

    Some(radix)
}

pub(crate) fn char_octet_length(column: &SqlColumn) -> Option<i32> {
    use AtomType::*;
    let size = match column.atom_type()? {
        Character | Octet => match column.length() {
            Some((length, false)) => length as i32,
            _ => 2097132,
        },
        _ => return None,
    };

    Some(size)
}

fn not_yet_implemented(stmt: &TsurugiOdbcStmt, arg: &TsurugiOdbcGetDataArguments) -> SqlReturn {
    const FUNCTION_NAME: &str = "TsurugiOdbcColumns.get_data()";

    warn!(
        "{stmt}.{FUNCTION_NAME}: not yet implemented. column_index={}",
        arg.column_index()
    );
    get_data_null(stmt, arg)
}
