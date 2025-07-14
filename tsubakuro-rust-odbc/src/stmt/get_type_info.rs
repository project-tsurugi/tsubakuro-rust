use log::{debug, trace, warn};

use crate::{
    check_stmt,
    ctype::{
        SqlDataType, SqlLen, SqlNullable::*, SqlReturn, SqlSmallInt, SqlUSmallInt, SQL_FALSE,
        SQL_PRED_BASIC, SQL_PRED_CHAR, SQL_TRUE,
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
};

const SQL_ALL_TYPES: SqlSmallInt = 0;

#[no_mangle]
pub extern "system" fn SQLGetTypeInfo(hstmt: HStmt, data_type: SqlSmallInt) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetTypeInfo()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, data_type={:?}",
        hstmt,
        data_type
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLGetTypeInfo");

    let data_types = match target_data_type(&mut stmt, data_type) {
        Ok(value) => value,
        Err(rc) => return rc,
    };

    let rc = get_type_info(&mut stmt, data_types);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLGetTypeInfoW(hstmt: HStmt, data_type: SqlSmallInt) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetTypeInfoW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, data_type={:?}",
        hstmt,
        data_type
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();
    stmt.set_name("SQLGetTypeInfoW");

    let data_types = match target_data_type(&mut stmt, data_type) {
        Ok(value) => value,
        Err(rc) => return rc,
    };

    let rc = get_type_info(&mut stmt, data_types);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn target_data_type(
    stmt: &mut TsurugiOdbcStmt,
    data_type: SqlSmallInt,
) -> Result<Vec<SqlDataType>, SqlReturn> {
    const FUNCTION_NAME: &str = "target_data_type()";

    if data_type == SQL_ALL_TYPES {
        use SqlDataType::*;
        let list = vec![
            SQL_INTEGER,
            SQL_BIGINT,
            SQL_REAL,
            SQL_DOUBLE,
            SQL_DECIMAL,
            SQL_CHAR,
            SQL_VARCHAR,
            SQL_BINARY,
            SQL_VARBINARY,
        ];
        return Ok(list);
    }

    let data_type = match SqlDataType::try_from(data_type) {
        Ok(value) => value,
        Err(e) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported SqlDataType {}",
                data_type
            );
            stmt.add_diag(e, format!("Unsupported SqlDataType {}", data_type));
            let rc = SqlReturn::SQL_ERROR;
            trace!("{FUNCTION_NAME} end. rc={:?}", rc);
            return Err(rc);
        }
    };

    Ok(vec![data_type])
}

fn get_type_info(stmt: &mut TsurugiOdbcStmt, data_types: Vec<SqlDataType>) -> SqlReturn {
    let processor = TsurugiOdbcTypeInfo::new(data_types);
    stmt.set_processor(processor);

    SqlReturn::SQL_SUCCESS
}

struct TsurugiOdbcTypeInfo {
    data_types: Vec<SqlDataType>,
    row_index: isize,
}

impl TsurugiOdbcTypeInfo {
    fn new(data_types: Vec<SqlDataType>) -> Self {
        Self {
            data_types,
            row_index: -1,
        }
    }
}

impl TsurugiOdbcStatementProcessor for TsurugiOdbcTypeInfo {
    fn number_of_columns(&self) -> SqlUSmallInt {
        19
    }
    fn describe_column(
        &self,
        column_index: SqlUSmallInt,
    ) -> Result<TsurugiOdbcDescribeColumn, SqlReturn> {
        let column = match column_index {
            0 => TsurugiOdbcDescribeColumn::new(
                "TYPE_NAME",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NO_NULLS,
            ),
            1 => TsurugiOdbcDescribeColumn::new(
                "DATA_TYPE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            2 => TsurugiOdbcDescribeColumn::new(
                "COLUMN_SIZE",
                SqlDataType::SQL_INTEGER,
                0,
                0,
                SQL_NULLABLE,
            ),
            3 => TsurugiOdbcDescribeColumn::new(
                "LITERAL_PREFIX",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            4 => TsurugiOdbcDescribeColumn::new(
                "LITERAL_SUFFIX",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            5 => TsurugiOdbcDescribeColumn::new(
                "CREATE_PARAMS",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            6 => TsurugiOdbcDescribeColumn::new(
                "NULLABLE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            7 => TsurugiOdbcDescribeColumn::new(
                "CASE_SENSITIVE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            8 => TsurugiOdbcDescribeColumn::new(
                "SEARCHABLE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            9 => TsurugiOdbcDescribeColumn::new(
                "UNSIGNED_ATTRIBUTE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            10 => TsurugiOdbcDescribeColumn::new(
                "FIXED_PREC_SCALE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            11 => TsurugiOdbcDescribeColumn::new(
                "AUTO_UNIQUE_VALUE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            12 => TsurugiOdbcDescribeColumn::new(
                "LOCAL_TYPE_NAME",
                SqlDataType::SQL_VARCHAR,
                0,
                0,
                SQL_NULLABLE,
            ),
            13 => TsurugiOdbcDescribeColumn::new(
                "MINIMUM_SCALE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            14 => TsurugiOdbcDescribeColumn::new(
                "MAXIMUM_SCALE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            15 => TsurugiOdbcDescribeColumn::new(
                "SQL_DATA_TYPE",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NO_NULLS,
            ),
            16 => TsurugiOdbcDescribeColumn::new(
                "SQL_DATETIME_SUB",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            17 => TsurugiOdbcDescribeColumn::new(
                "NUM_PREC_RADIX",
                SqlDataType::SQL_INTEGER,
                0,
                0,
                SQL_NULLABLE,
            ),
            18 => TsurugiOdbcDescribeColumn::new(
                "INTERVAL_PRECISION",
                SqlDataType::SQL_SMALLINT,
                0,
                0,
                SQL_NULLABLE,
            ),
            _ => unreachable!(),
        };
        Ok(column)
    }

    fn row_count(&self) -> SqlLen {
        self.data_types.len() as SqlLen
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
        let data_types = &self.data_types;
        if self.row_index < 0 || self.row_index as usize >= data_types.len() {
            debug!(
                "get_data() index out of bounds: self.row_index={}",
                self.row_index
            );
            return SqlReturn::SQL_NO_DATA;
        }
        let data_type = &data_types[self.row_index as usize];

        let column_index = arg.column_index();
        match column_index {
            0 => match type_name(stmt, data_type) {
                Ok(value) => get_data_string(stmt, arg, value),
                Err(rc) => rc,
            }, // TYPE_NAME varchar
            1 => get_data_i32(stmt, arg, *data_type as i32), // DATA_TYPE Smallint
            2 => get_data_i32_opt(stmt, arg, column_size(data_type)), // COLUMN_SIZE Integer
            3 => get_data_string_opt(stmt, arg, literal_prefix(data_type)), // LITERAL_PREFIX varchar
            4 => get_data_string_opt(stmt, arg, literal_suffix(data_type)), // LITERAL_SUFFIX varchar
            5 => get_data_string_opt(stmt, arg, create_params(data_type)),  // CREATE_PARAMS varchar
            6 => get_data_i32(stmt, arg, SQL_NULLABLE as i32),              // NULLABLE SmallInt
            7 => get_data_i32(stmt, arg, case_sensitive(data_type)), // CASE_SENSITIVE SmallInt
            8 => get_data_i32(stmt, arg, searchable(data_type)),     // SEARCHABLE SmallInt
            9 => get_data_i32_opt(stmt, arg, unsigned_attribute(data_type)), // UNSIGNED_ATTRIBUTE SmallInt
            10 => get_data_i32(stmt, arg, SQL_FALSE), // FIXED_PREC_SCALE SmallInt
            11 => get_data_i32_opt(stmt, arg, auto_unique_value(data_type)), // AUTO_UNIQUE_VALUE SmallInt
            12 => get_data_null(stmt, arg), // LOCAL_TYPE_NAME varchar
            13 => get_data_i32_opt(stmt, arg, minimum_scale(data_type)), // MINIMUM_SCALE SmallInt
            14 => get_data_i32_opt(stmt, arg, maximum_scale(data_type)), // MAXIMUM_SCALE SmallInt
            15 => get_data_i32(
                stmt,
                arg,
                *data_type as i32, // TODO SQL_DATETIME
            ), // SQL_DATA_TYPE SmallInt
            16 => not_yet_implemented(stmt, arg), // SQL_DATETIME_SUB SmallInt
            17 => get_data_i32_opt(stmt, arg, num_prec_radix(data_type)), // NUM_PREC_RADIX Integer
            18 => not_yet_implemented(stmt, arg), // INTERVAL_PRECISION SmallInt
            _ => unreachable!(),
        }
    }

    fn dispose(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        SqlReturn::SQL_SUCCESS
    }
}

fn type_name(stmt: &TsurugiOdbcStmt, data_type: &SqlDataType) -> Result<&'static str, SqlReturn> {
    const FUNCTION_NAME: &str = "type_name()";

    use SqlDataType::*;
    let type_name = match data_type {
        SQL_CHAR | SQL_WCHAR => "CHAR",
        SQL_NUMERIC | SQL_DECIMAL => "DECIMAL",
        SQL_INTEGER => "INT",
        // SQL_SMALLINT => todo!(),
        SQL_FLOAT | SQL_REAL => "REAL",
        SQL_DOUBLE => "DOUBLE",
        SQL_VARCHAR | SQL_WVARCHAR => "VARCHAR",
        SQL_TYPE_DATE => "DATE",
        SQL_TYPE_TIME => "TIME",
        SQL_TYPE_TIMESTAMP | SQL_DATETIME => "TIMESTAMP",
        // SQL_INTERVAL_YEAR => todo!(),
        // SQL_INTERVAL_MONTH => todo!(),
        // SQL_INTERVAL_DAY => todo!(),
        // SQL_INTERVAL_HOUR => todo!(),
        // SQL_INTERVAL_MINUTE => todo!(),
        // SQL_INTERVAL_SECOND => todo!(),
        // SQL_INTERVAL_YEAR_TO_MONTH => todo!(),
        // SQL_INTERVAL_DAY_TO_HOUR => todo!(),
        // SQL_INTERVAL_DAY_TO_MINUTE => todo!(),
        // SQL_INTERVAL_DAY_TO_SECOND => todo!(),
        // SQL_INTERVAL_HOUR_TO_MINUTE => todo!(),
        // SQL_INTERVAL_HOUR_TO_SECOND => todo!(),
        // SQL_INTERVAL_MINUTE_TO_SECOND => todo!(),
        SQL_LONGVARCHAR | SQL_WLONGVARCHAR => "CLOB",
        SQL_BINARY => "BINARY",
        SQL_VARBINARY => "VARBINARY",
        SQL_LONGVARBINARY => "BLOB",
        SQL_BIGINT => "BIGINT",
        // SQL_TINYINT => todo!(),
        SQL_BIT => "BOOLEAN",
        _ => {
            warn!(
                "{stmt}.{FUNCTION_NAME}: Unsupported data_type {:?}",
                data_type
            );
            stmt.add_diag(
                TsurugiOdbcError::UnsupportedSqlDataType,
                format!("Unsupported data_type {:?}", data_type),
            );
            return Err(SqlReturn::SQL_ERROR);
        }
    };
    Ok(type_name)
}

fn column_size(data_type: &SqlDataType) -> Option<i32> {
    use SqlDataType::*;
    let size = match data_type {
        SQL_CHAR | SQL_WCHAR | SQL_VARCHAR | SQL_WVARCHAR => 2097132,
        SQL_NUMERIC | SQL_DECIMAL => 38,
        SQL_INTEGER => 4 * 8,
        SQL_SMALLINT => 2 * 8,
        SQL_FLOAT | SQL_REAL => 38,
        SQL_DOUBLE => 308,
        // SQL_TYPE_DATE => todo!(),
        // SQL_TYPE_TIME => todo!(),
        // SQL_TYPE_TIMESTAMP | SQL_DATETIME => todo!(),
        // SQL_INTERVAL_YEAR => todo!(),
        // SQL_INTERVAL_MONTH => todo!(),
        // SQL_INTERVAL_DAY => todo!(),
        // SQL_INTERVAL_HOUR => todo!(),
        // SQL_INTERVAL_MINUTE => todo!(),
        // SQL_INTERVAL_SECOND => todo!(),
        // SQL_INTERVAL_YEAR_TO_MONTH => todo!(),
        // SQL_INTERVAL_DAY_TO_HOUR => todo!(),
        // SQL_INTERVAL_DAY_TO_MINUTE => todo!(),
        // SQL_INTERVAL_DAY_TO_SECOND => todo!(),
        // SQL_INTERVAL_HOUR_TO_MINUTE => todo!(),
        // SQL_INTERVAL_HOUR_TO_SECOND => todo!(),
        // SQL_INTERVAL_MINUTE_TO_SECOND => todo!(),
        SQL_BINARY | SQL_VARBINARY => 2097132,
        SQL_LONGVARBINARY => todo!(),
        SQL_BIGINT => 8 * 8,
        SQL_TINYINT => 8,
        SQL_BIT => 1,
        _ => return None,
    };

    Some(size)
}

fn literal_prefix(data_type: &SqlDataType) -> Option<&'static str> {
    use SqlDataType::*;
    let ret = match data_type {
        SQL_CHAR | SQL_WCHAR | SQL_VARCHAR | SQL_WVARCHAR => "'",
        SQL_TYPE_DATE => "DATE'",
        SQL_TYPE_TIME => "TIME'",
        SQL_TYPE_TIMESTAMP | SQL_DATETIME => "TIMESTAMP'",
        // SQL_INTERVAL_YEAR => todo!(),
        // SQL_INTERVAL_MONTH => todo!(),
        // SQL_INTERVAL_DAY => todo!(),
        // SQL_INTERVAL_HOUR => todo!(),
        // SQL_INTERVAL_MINUTE => todo!(),
        // SQL_INTERVAL_SECOND => todo!(),
        // SQL_INTERVAL_YEAR_TO_MONTH => todo!(),
        // SQL_INTERVAL_DAY_TO_HOUR => todo!(),
        // SQL_INTERVAL_DAY_TO_MINUTE => todo!(),
        // SQL_INTERVAL_DAY_TO_SECOND => todo!(),
        // SQL_INTERVAL_HOUR_TO_MINUTE => todo!(),
        // SQL_INTERVAL_HOUR_TO_SECOND => todo!(),
        // SQL_INTERVAL_MINUTE_TO_SECOND => todo!(),
        SQL_BINARY | SQL_VARBINARY | SQL_LONGVARBINARY => "X'",
        _ => return None,
    };

    Some(ret)
}

fn literal_suffix(data_type: &SqlDataType) -> Option<&'static str> {
    use SqlDataType::*;
    let ret = match data_type {
        SQL_CHAR | SQL_WCHAR | SQL_VARCHAR | SQL_WVARCHAR => "'",
        SQL_TYPE_DATE => "'",
        SQL_TYPE_TIME => "'",
        SQL_TYPE_TIMESTAMP | SQL_DATETIME => "'",
        // SQL_INTERVAL_YEAR => todo!(),
        // SQL_INTERVAL_MONTH => todo!(),
        // SQL_INTERVAL_DAY => todo!(),
        // SQL_INTERVAL_HOUR => todo!(),
        // SQL_INTERVAL_MINUTE => todo!(),
        // SQL_INTERVAL_SECOND => todo!(),
        // SQL_INTERVAL_YEAR_TO_MONTH => todo!(),
        // SQL_INTERVAL_DAY_TO_HOUR => todo!(),
        // SQL_INTERVAL_DAY_TO_MINUTE => todo!(),
        // SQL_INTERVAL_DAY_TO_SECOND => todo!(),
        // SQL_INTERVAL_HOUR_TO_MINUTE => todo!(),
        // SQL_INTERVAL_HOUR_TO_SECOND => todo!(),
        // SQL_INTERVAL_MINUTE_TO_SECOND => todo!(),
        SQL_BINARY | SQL_VARBINARY | SQL_LONGVARBINARY => "'",
        _ => return None,
    };

    Some(ret)
}

fn create_params(data_type: &SqlDataType) -> Option<&'static str> {
    use SqlDataType::*;
    let ret = match data_type {
        SQL_CHAR | SQL_WCHAR | SQL_VARCHAR | SQL_WVARCHAR => "length",
        SQL_NUMERIC | SQL_DECIMAL => "precision,scale",
        SQL_BINARY | SQL_VARBINARY => "length",
        _ => return None,
    };

    Some(ret)
}

fn case_sensitive(data_type: &SqlDataType) -> i32 {
    use SqlDataType::*;
    match data_type {
        SQL_CHAR | SQL_WCHAR | SQL_VARCHAR | SQL_WVARCHAR | SQL_LONGVARBINARY => SQL_TRUE,
        _ => SQL_FALSE,
    }
}

//TODO searchable()
fn searchable(data_type: &SqlDataType) -> i32 {
    use SqlDataType::*;
    match data_type {
        SQL_CHAR | SQL_WCHAR | SQL_VARCHAR | SQL_WVARCHAR | SQL_LONGVARBINARY => SQL_PRED_CHAR,
        _ => SQL_PRED_BASIC,
    }
}

pub(crate) fn unsigned_attribute(data_type: &SqlDataType) -> Option<i32> {
    use SqlDataType::*;
    let ret = match data_type {
        SQL_NUMERIC | SQL_DECIMAL => SQL_FALSE,
        SQL_INTEGER => SQL_FALSE,
        SQL_SMALLINT => SQL_FALSE,
        SQL_FLOAT | SQL_REAL => SQL_FALSE,
        SQL_DOUBLE => SQL_FALSE,
        SQL_BIGINT => SQL_FALSE,
        SQL_TINYINT => SQL_FALSE,
        SQL_BIT => SQL_TRUE,
        _ => return None,
    };
    Some(ret)
}

fn auto_unique_value(data_type: &SqlDataType) -> Option<i32> {
    use SqlDataType::*;
    let ret = match data_type {
        SQL_INTEGER => SQL_FALSE,
        SQL_BIGINT => SQL_FALSE,
        _ => return None,
    };
    Some(ret)
}

fn minimum_scale(data_type: &SqlDataType) -> Option<i32> {
    use SqlDataType::*;
    let ret = match data_type {
        SQL_TYPE_TIME => 9,
        SQL_TYPE_TIMESTAMP => 9,
        _ => return None,
    };
    Some(ret)
}

fn maximum_scale(data_type: &SqlDataType) -> Option<i32> {
    use SqlDataType::*;
    let ret = match data_type {
        SQL_TYPE_TIME => 9,
        SQL_TYPE_TIMESTAMP => 9,
        _ => return None,
    };
    Some(ret)
}

fn num_prec_radix(data_type: &SqlDataType) -> Option<i32> {
    use SqlDataType::*;
    let ret = match data_type {
        SQL_NUMERIC | SQL_DECIMAL => 10,
        SQL_INTEGER => 2,
        SQL_SMALLINT => 2,
        SQL_FLOAT | SQL_REAL => 10,
        SQL_DOUBLE => 10,
        SQL_BIGINT => 2,
        SQL_TINYINT => 2,
        SQL_BIT => 2,
        _ => return None,
    };
    Some(ret)
}

fn not_yet_implemented(stmt: &TsurugiOdbcStmt, arg: &TsurugiOdbcGetDataArguments) -> SqlReturn {
    const FUNCTION_NAME: &str = "TsurugiOdbcTypeInfo.get_data()";

    warn!(
        "{stmt}.{FUNCTION_NAME}: not yet implemented. column_index={}",
        arg.column_index()
    );
    get_data_null(stmt, arg)
}
