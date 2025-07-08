use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::{AtomType, SqlColumn, TableMetadata};

use crate::{
    check_sql_client_or_err, check_stmt,
    ctype::{
        sql_numeric_struct::SqlNumericStruct, CDataType, SqlChar, SqlDataType, SqlLen,
        SqlNullable::*, SqlPointer, SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::{
        describe_col::TsurugiOdbcDescribeColumn,
        get_data::{
            get_data_i32, get_data_i32_opt, get_data_null, get_data_string, get_data_string_opt,
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
) -> Result<TableMetadata, SqlReturn> {
    const FUNCTION_NAME: &str = "get_table_metadata()";

    let sql_client = check_sql_client_or_err!(stmt);

    let runtime = stmt.runtime();
    let result = runtime.block_on(sql_client.get_table_metadata(table_name));
    let metadata = match result {
        Ok(metadata) => {
            debug!("{stmt}.{FUNCTION_NAME}: get_table_metadata() succeeded");
            metadata
        }
        Err(e) => {
            warn!(
                "{stmt}.{FUNCTION_NAME}: get_table_metadata() error. {:?}",
                e
            );
            stmt.add_diag(
                TsurugiOdbcError::GetTableMetadataError,
                format!("get table metadta error. {}", e),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };

    Ok(metadata)
}

struct TsurugiOdbcColumns {
    metadata: TableMetadata,
    row_index: isize,
}

impl TsurugiOdbcColumns {
    fn new(metadata: TableMetadata) -> Self {
        Self {
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
        let column = match column_index {
            0 => TsurugiOdbcDescribeColumn::new(
                "TABLE_CAT",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            1 => TsurugiOdbcDescribeColumn::new(
                "TABLE_SCHEM",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            2 => TsurugiOdbcDescribeColumn::new(
                "TABLE_NAME",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NO_NULLS,
            ),
            3 => TsurugiOdbcDescribeColumn::new(
                "COLUMN_NAME",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NO_NULLS,
            ),
            4 => TsurugiOdbcDescribeColumn::new(
                "DATA_TYPE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            5 => TsurugiOdbcDescribeColumn::new(
                "TYPE_NAME",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NO_NULLS,
            ),
            6 => TsurugiOdbcDescribeColumn::new(
                "COLUMN_SIZE",
                SqlDataType::SQL_INTEGER,
                0,
                0,
                SQL_NULLABLE,
            ),
            7 => TsurugiOdbcDescribeColumn::new(
                "BUFFER_LENGTH",
                SqlDataType::SQL_INTEGER,
                0,
                0,
                SQL_NULLABLE,
            ),
            8 => TsurugiOdbcDescribeColumn::new(
                "DECIMAL_DIGITS",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            9 => TsurugiOdbcDescribeColumn::new(
                "NUM_PREC_RADIX",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            10 => TsurugiOdbcDescribeColumn::new(
                "NULLABLE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            11 => TsurugiOdbcDescribeColumn::new(
                "REMARKS",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            12 => TsurugiOdbcDescribeColumn::new(
                "COLUMN_DEF",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            13 => TsurugiOdbcDescribeColumn::new(
                "SQL_DATA_TYPE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            14 => TsurugiOdbcDescribeColumn::new(
                "SQL_DATETIME_SUB",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            15 => TsurugiOdbcDescribeColumn::new(
                "CHAR_OCTET_LENGTH",
                SqlDataType::SQL_INTEGER,
                0,
                0,
                SQL_NULLABLE,
            ),
            16 => TsurugiOdbcDescribeColumn::new(
                "ORDINAL_POSITION",
                SqlDataType::SQL_INTEGER,
                0,
                0,
                SQL_NO_NULLS,
            ),
            17 => TsurugiOdbcDescribeColumn::new(
                "IS_NULLABLE",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            _ => unreachable!(),
        };
        Ok(column)
    }

    fn row_count(&self) -> SqlLen {
        let columns = self.metadata.columns();
        columns.len() as SqlLen
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

    fn get_data(
        &mut self,
        stmt: &TsurugiOdbcStmt,
        column_index: SqlUSmallInt,
        target_type: CDataType,
        target_value_ptr: SqlPointer,
        buffer_length: SqlLen,
        str_len_or_ind_ptr: *mut SqlLen,
    ) -> SqlReturn {
        const FUNCTION_NAME: &str = "TsurugiOdbcColumns.get_data()";

        let columns = self.metadata.columns();
        if self.row_index < 0 || self.row_index as usize >= columns.len() {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. index out of bounds. self.row_index={}",
                self.row_index
            );
            return SqlReturn::SQL_NO_DATA;
        }
        let column = &columns[self.row_index as usize];

        // TODO SQLGetColumn() field
        match column_index {
            0 => get_data_string(
                stmt,
                self.metadata.database_name(),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // TABLE_CAT varchar
            1 => get_data_string(
                stmt,
                self.metadata.schema_name(),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // TABLE_SCHEM varchar
            2 => get_data_string(
                stmt,
                self.metadata.table_name(),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // TABLE_NAME varchar
            3 => get_data_string(
                stmt,
                column.name(),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // COLUMN_NAME varchar
            4 => get_data_i32(
                stmt,
                SqlDataType::from(column) as i32,
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // DATA_TYPE SmallInt
            5 => get_data_string_opt(
                stmt,
                column.sql_type(),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // TYPE_NAME varchar
            6 => get_data_i32_opt(
                stmt,
                column_size(column),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // COLUMN_SIZE Integer
            7 => get_data_i32_opt(
                stmt,
                column_buffer_length(column),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // BUFFER_LENGTH Integer
            8 => get_data_i32_opt(
                stmt,
                decimal_digits(column),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // DECIMAL_DIGITS SmallInt
            9 => get_data_i32_opt(
                stmt,
                num_prec_radix(column),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // NUM_PREC_RADIX SmallInt
            10 => get_data_i32_opt(
                stmt,
                column.nullable().map(|b| {
                    if b {
                        SQL_NULLABLE as i32
                    } else {
                        SQL_NO_NULLS as i32
                    }
                }),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // NULLABLE SmallInt
            11 => get_data_string_opt(
                stmt,
                column.description(),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // REMARKS varchar
            12 => not_yet_implemented(stmt, column_index, str_len_or_ind_ptr), // COLUMN_DEF varchar
            13 => get_data_i32(
                stmt,
                SqlDataType::from(column) as i32,
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // SQL_DATA_TYPE SmallInt
            14 => not_yet_implemented(stmt, column_index, str_len_or_ind_ptr), // SQL_DATETIME_SUB SmallInt
            15 => get_data_i32_opt(
                stmt,
                char_octet_length(column),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // CHAR_OCTET_LENGTH Integer
            16 => get_data_i32(
                stmt,
                self.row_index as i32 + 1,
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            ), // ORDINAL_POSITION Integer
            17 => get_data_string_opt(
                stmt,
                column.nullable().map(|b| if b { "YES" } else { "NO" }),
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
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
        // Boolean => todo!(),
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

fn decimal_digits(column: &SqlColumn) -> Option<i32> {
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

fn char_octet_length(column: &SqlColumn) -> Option<i32> {
    use AtomType::*;
    let size = match column.atom_type()? {
        Character | Octet => match column.length() {
            Some((length, false)) => length as i32,
            _ => {
                if column.varying().unwrap_or(false) {
                    2097132 // VARCHAR, VARBINARY
                } else {
                    1 // CHAR, BINARY
                }
            }
        },
        _ => return None,
    };

    Some(size)
}

fn not_yet_implemented(
    stmt: &TsurugiOdbcStmt,
    column_index: SqlUSmallInt,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "TsurugiOdbcColumns.get_data()";

    warn!(
        "{stmt}.{FUNCTION_NAME}: not yet implemented. column_index={}",
        column_index
    );
    get_data_null(stmt, str_len_or_ind_ptr)
}
