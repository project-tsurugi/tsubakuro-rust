use log::{debug, trace};

use crate::{
    check_stmt,
    ctype::{
        SqlChar, SqlDataType, SqlNullable, SqlPointer, SqlReturn, SqlSmallInt, SqlULen,
        SqlUSmallInt, SqlWChar,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    util::{write_char, write_wchar},
};

#[derive(Debug)]
pub(crate) struct TsurugiOdbcDescribeColumn {
    column_name: String,
    data_type: SqlDataType,
    column_size: SqlULen,
    decimal_digits: SqlSmallInt,
    nullable: SqlNullable, // SqlSmallInt
}

impl TsurugiOdbcDescribeColumn {
    pub(crate) fn new(
        column_name: &str,
        data_type: SqlDataType,
        column_size: SqlULen,
        decimal_digits: SqlSmallInt,
        nullable: SqlNullable,
    ) -> TsurugiOdbcDescribeColumn {
        TsurugiOdbcDescribeColumn {
            column_name: column_name.to_string(),
            data_type,
            column_size,
            decimal_digits,
            nullable,
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
            &column.column_name,
            column_name_ptr as *mut SqlWChar,
            buffer_length,
            name_length_ptr,
            Some(&stmt.diag_collection()),
        )
    } else {
        write_char(
            &column.column_name,
            column_name_ptr as *mut SqlChar,
            buffer_length,
            name_length_ptr,
            Some(&stmt.diag_collection()),
        )
    };
    write_data_type(&column.data_type, data_type_ptr);
    write_column_size(column.column_size, column_size_ptr);
    write_decimal_digits(column.decimal_digits, decimal_digits_ptr);
    write_nullable(column.nullable, nullable_ptr);

    rc
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
