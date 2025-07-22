use log::{debug, trace};
use tsubakuro_rust_core::prelude::{AtomType, SqlColumn};

use crate::{
    check_stmt,
    ctype::{
        SqlChar, SqlDataType, SqlLen, SqlNullable, SqlPointer, SqlReturn, SqlSmallInt, SqlULen,
        SqlUSmallInt, SqlWChar, SQL_NO_TOTAL,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::columns::{char_octet_length, decimal_digits},
    util::{write_char, write_wchar},
};

#[derive(Debug)]
pub(crate) struct TsurugiOdbcDescribeColumn {
    column: Option<SqlColumn>,
    column_name: String,
    data_type: SqlDataType,
    nullable: SqlNullable, // SqlSmallInt
}

impl TsurugiOdbcDescribeColumn {
    pub(crate) fn new(
        column_name: &str,
        data_type: SqlDataType,
        nullable: SqlNullable,
    ) -> TsurugiOdbcDescribeColumn {
        TsurugiOdbcDescribeColumn {
            column: None,
            column_name: column_name.to_string(),
            data_type,
            nullable,
        }
    }
}

impl From<SqlColumn> for TsurugiOdbcDescribeColumn {
    fn from(column: SqlColumn) -> Self {
        let column_name = column.name().into();
        let data_type = SqlDataType::from(&column);
        let nullable = match column.nullable() {
            Some(true) => SqlNullable::SQL_NULLABLE,
            Some(false) => SqlNullable::SQL_NO_NULLS,
            None => SqlNullable::SQL_NULLABLE_UNKNOWN,
        };

        TsurugiOdbcDescribeColumn {
            column: Some(column),
            column_name,
            data_type,
            nullable,
        }
    }
}

impl TsurugiOdbcDescribeColumn {
    pub(crate) fn sql_column(&self) -> Option<&SqlColumn> {
        self.column.as_ref()
    }

    pub(crate) fn column_name(&self) -> &String {
        &self.column_name
    }

    pub(crate) fn data_type(&self) -> SqlDataType {
        self.data_type
    }

    pub(crate) fn column_size(&self) -> SqlLen {
        if let Some(column) = &self.column {
            column_size(column)
        } else {
            0
        }
    }

    pub(crate) fn decimal_digits(&self) -> SqlSmallInt {
        if let Some(column) = &self.column {
            decimal_digits(column).unwrap_or(0) as SqlSmallInt
        } else {
            0
        }
    }

    pub(crate) fn nullable(&self) -> SqlNullable {
        self.nullable
    }

    pub(crate) fn scale(&self) -> u32 {
        if let Some(column) = &self.column {
            match column.scale() {
                Some((scale, false)) => scale,
                _ => 0,
            }
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLDescribeCol(
    hstmt: HStmt,
    column_number: SqlUSmallInt,
    column_name_ptr: *mut SqlChar,
    buffer_length: SqlSmallInt,
    name_length_ptr: *mut SqlSmallInt,
    data_type_ptr: *mut SqlSmallInt,
    column_size_ptr: *mut SqlULen,
    decimal_digits_ptr: *mut SqlSmallInt,
    nullable_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLDescribeCol()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, column_name_ptr={:?}, column_name={:?}, buffer_length={:?}, name_length_ptr={:?}, data_type_ptr={:?}, column_size_ptr={:?}, decimal_digits_ptr={:?}, nullable_ptr={:?}",
        hstmt,
        column_number,
        column_name_ptr,
        buffer_length,
        name_length_ptr,
        data_type_ptr,
        column_size_ptr,
        decimal_digits_ptr,
        nullable_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = describe_col(
        &stmt,
        column_number,
        column_name_ptr as SqlPointer,
        buffer_length,
        name_length_ptr,
        data_type_ptr,
        column_size_ptr,
        decimal_digits_ptr,
        nullable_ptr,
        false,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLDescribeColW(
    hstmt: HStmt,
    column_number: SqlUSmallInt,
    column_name_ptr: *mut SqlWChar,
    buffer_length: SqlSmallInt,
    name_length_ptr: *mut SqlSmallInt,
    data_type_ptr: *mut SqlSmallInt,
    column_size_ptr: *mut SqlULen,
    decimal_digits_ptr: *mut SqlSmallInt,
    nullable_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLDescribeColW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, column_name_ptr={:?}, column_name={:?}, buffer_length={:?}, name_length_ptr={:?}, data_type_ptr={:?}, column_size_ptr={:?}, decimal_digits_ptr={:?}, nullable_ptr={:?}",
        hstmt,
        column_number,
        column_name_ptr,
        buffer_length,
        name_length_ptr,
        data_type_ptr,
        column_size_ptr,
        decimal_digits_ptr,
        nullable_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = describe_col(
        &stmt,
        column_number,
        column_name_ptr as SqlPointer,
        buffer_length,
        name_length_ptr,
        data_type_ptr,
        column_size_ptr,
        decimal_digits_ptr,
        nullable_ptr,
        true,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[allow(clippy::too_many_arguments)]
fn describe_col(
    stmt: &TsurugiOdbcStmt,
    column_number: SqlUSmallInt,
    column_name_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    name_length_ptr: *mut SqlSmallInt,
    data_type_ptr: *mut SqlSmallInt,
    column_size_ptr: *mut SqlULen,
    decimal_digits_ptr: *mut SqlSmallInt,
    nullable_ptr: *mut SqlSmallInt,
    wide_char: bool,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "describe_col()";

    let processor = match stmt.processor(FUNCTION_NAME) {
        Ok(processor) => processor,
        Err(rc) => return rc,
    };
    let processor = processor.borrow();

    let column = {
        let number_of_columns = processor.number_of_columns();
        if column_number < 1 || column_number > number_of_columns {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. index out of bounds: column_number={}",
                column_number
            );
            stmt.add_diag(
                TsurugiOdbcError::ColumnNumberOutOfBounds,
                format!(
                    "column_number must be between 1 and {number_of_columns}, but got {}",
                    column_number
                ),
            );
            return SqlReturn::SQL_ERROR;
        }
        let column_index = column_number - 1;

        match processor.describe_column(column_index) {
            Ok(column) => column,
            Err(rc) => return rc,
        }
    };

    debug!(
        "{stmt}.{FUNCTION_NAME}: column_number={}, column={:?}",
        column_number, column
    );

    let rc = if wide_char {
        write_wchar(
            "SQLDescribeColW.column_name_ptr",
            &column.column_name,
            column_name_ptr as *mut SqlWChar,
            buffer_length,
            name_length_ptr,
            Some(&stmt.diag_collection()),
        )
    } else {
        write_char(
            "SQLDescribeCol.column_name_ptr",
            &column.column_name,
            column_name_ptr as *mut SqlChar,
            buffer_length,
            name_length_ptr,
            Some(&stmt.diag_collection()),
        )
    };

    write_data_type(&column.data_type, data_type_ptr);

    let column_size = column.column_size();
    let column_size = if column_size == SQL_NO_TOTAL {
        0
    } else {
        column_size as SqlULen
    };
    write_column_size(column_size, column_size_ptr);

    write_decimal_digits(column.decimal_digits(), decimal_digits_ptr);

    write_nullable(column.nullable, nullable_ptr);

    rc
}

fn column_size(column: &SqlColumn) -> SqlLen {
    use AtomType::*;
    match column.atom_type() {
        Some(atom_type) => match atom_type {
            Boolean => 1,
            Int4 => 10,
            Int8 => 19,
            Float4 => 7,
            Float8 => 15,
            Decimal => match column.precision() {
                Some((precision, false)) => precision as SqlLen,
                _ => 38,
            },
            Character | Octet => char_octet_length(column).unwrap_or(2097132) as SqlLen,
            Date => 10,             // yyyy-MM-dd
            TimeOfDay => 8 + 1 + 9, // HH:mm:ss.nnnnnnnnn
            TimePoint => 10 + 1 + (8 + 1 + 9),
            TimeOfDayWithTimeZone => (8 + 1 + 9) + 6, // +hh:mm
            TimePointWithTimeZone => 10 + 1 + (8 + 1 + 9) + 6, // +hh:mm
            _ => 0,
        },
        _ => 0,
    }
}

fn write_data_type(data_type: &SqlDataType, data_type_ptr: *mut SqlSmallInt) {
    if !data_type_ptr.is_null() {
        unsafe {
            *data_type_ptr = *data_type as SqlSmallInt;
        }
    }
}

fn write_column_size(value: SqlULen, column_size_ptr: *mut SqlULen) {
    if !column_size_ptr.is_null() {
        unsafe {
            *column_size_ptr = value;
        }
    }
}

fn write_decimal_digits(value: SqlSmallInt, decimal_digits_ptr: *mut SqlSmallInt) {
    if !decimal_digits_ptr.is_null() {
        unsafe {
            *decimal_digits_ptr = value;
        }
    }
}

fn write_nullable(value: SqlNullable, nullable_ptr: *mut SqlSmallInt) {
    if !nullable_ptr.is_null() {
        unsafe {
            *nullable_ptr = value as SqlSmallInt;
        }
    }
}
