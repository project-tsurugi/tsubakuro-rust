use log::{debug, trace, warn};
use tsubakuro_rust_core::prelude::{AtomType, SqlColumn};

use crate::{
    check_stmt,
    ctype::{
        SqlChar, SqlLen, SqlPointer, SqlReturn, SqlSmallInt, SqlUSmallInt, SqlWChar, SQL_FALSE,
        SQL_TRUE,
    },
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
    stmt::{
        columns::char_octet_length,
        get_type_info::{
            literal_prefix, literal_suffix, num_prec_radix, searchable, type_name,
            unsigned_attribute,
        },
    },
    util::{write_char, write_wchar_bytes},
};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub(crate) enum FieldIdentifier {
    SQL_COLUMN_COUNT = 0,
    SQL_COLUMN_NAME = 1,
    // SQL_COLUMN_TYPE = 2,
    SQL_COLUMN_LENGTH = 3,
    SQL_COLUMN_PRECISION = 4,
    SQL_COLUMN_SCALE = 5,
    // SQL_COLUMN_DISPLAY_SIZE = 6,
    SQL_COLUMN_NULLABLE = 7,
    // SQL_COLUMN_UNSIGNED = 8,
    // SQL_COLUMN_MONEY = 9,
    // SQL_COLUMN_UPDATABLE = 10,
    // SQL_COLUMN_AUTO_INCREMENT = 11,
    // SQL_COLUMN_CASE_SENSITIVE = 12,
    // SQL_COLUMN_SEARCHABLE = 13,
    // SQL_COLUMN_TYPE_NAME = 14,
    // SQL_COLUMN_TABLE_NAME = 15,
    // SQL_COLUMN_OWNER_NAME = 16,
    // SQL_COLUMN_QUALIFIER_NAME = 17,
    // SQL_COLUMN_LABEL = 18,
    SQL_DESC_ARRAY_SIZE = 20,
    SQL_DESC_ARRAY_STATUS_PTR = 21,
    SQL_DESC_AUTO_UNIQUE_VALUE = 11, // SQL_COLUMN_AUTO_INCREMENT
    SQL_DESC_BASE_COLUMN_NAME = 22,
    SQL_DESC_BASE_TABLE_NAME = 23,
    SQL_DESC_BIND_OFFSET_PTR = 24,
    SQL_DESC_BIND_TYPE = 25,
    SQL_DESC_CASE_SENSITIVE = 12, // SQL_COLUMN_CASE_SENSITIVE
    SQL_DESC_CATALOG_NAME = 17,   // SQL_COLUMN_QUALIFIER_NAME
    SQL_DESC_CONCISE_TYPE = 2,    // SQL_COLUMN_TYPE
    SQL_DESC_DATETIME_INTERVAL_PRECISION = 26,
    SQL_DESC_DISPLAY_SIZE = 6,     // SQL_COLUMN_DISPLAY_SIZE
    SQL_DESC_FIXED_PREC_SCALE = 9, // SQL_COLUMN_MONEY
    SQL_DESC_LABEL = 18,           // SQL_COLUMN_LABEL
    SQL_DESC_LITERAL_PREFIX = 27,
    SQL_DESC_LITERAL_SUFFIX = 28,
    SQL_DESC_LOCAL_TYPE_NAME = 29,
    SQL_DESC_MAXIMUM_SCALE = 30,
    SQL_DESC_MINIMUM_SCALE = 31,
    SQL_DESC_NUM_PREC_RADIX = 32,
    SQL_DESC_PARAMETER_TYPE = 33,
    SQL_DESC_ROWS_PROCESSED_PTR = 34,
    SQL_DESC_ROWVER = 35,
    SQL_DESC_SCHEMA_NAME = 16, // SQL_COLUMN_OWNER_NAME
    SQL_DESC_SEARCHABLE = 13,  // SQL_COLUMN_SEARCHABLE
    SQL_DESC_TYPE_NAME = 14,   // SQL_COLUMN_TYPE_NAME
    SQL_DESC_TABLE_NAME = 15,  // SQL_COLUMN_TABLE_NAME
    SQL_DESC_UNSIGNED = 8,     // SQL_COLUMN_UNSIGNED
    SQL_DESC_UPDATABLE = 10,   // SQL_COLUMN_UPDATABLE
    SQL_DESC_COUNT = 1001,
    SQL_DESC_TYPE = 1002,
    SQL_DESC_LENGTH = 1003,
    SQL_DESC_OCTET_LENGTH_PTR = 1004,
    SQL_DESC_PRECISION = 1005,
    SQL_DESC_SCALE = 1006,
    SQL_DESC_DATETIME_INTERVAL_CODE = 1007,
    SQL_DESC_NULLABLE = 1008,
    SQL_DESC_INDICATOR_PTR = 1009,
    SQL_DESC_DATA_PTR = 1010,
    SQL_DESC_NAME = 1011,
    SQL_DESC_UNNAMED = 1012,
    SQL_DESC_OCTET_LENGTH = 1013,
    SQL_DESC_ALLOC_TYPE = 1099,
}

impl TryFrom<u16> for FieldIdentifier {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use FieldIdentifier::*;
        match value {
            0 => Ok(SQL_COLUMN_COUNT),
            1 => Ok(SQL_COLUMN_NAME),
            3 => Ok(SQL_COLUMN_LENGTH),
            4 => Ok(SQL_COLUMN_PRECISION),
            5 => Ok(SQL_COLUMN_SCALE),
            7 => Ok(SQL_COLUMN_NULLABLE),
            20 => Ok(SQL_DESC_ARRAY_SIZE),
            21 => Ok(SQL_DESC_ARRAY_STATUS_PTR),
            11 => Ok(SQL_DESC_AUTO_UNIQUE_VALUE),
            22 => Ok(SQL_DESC_BASE_COLUMN_NAME),
            23 => Ok(SQL_DESC_BASE_TABLE_NAME),
            24 => Ok(SQL_DESC_BIND_OFFSET_PTR),
            25 => Ok(SQL_DESC_BIND_TYPE),
            12 => Ok(SQL_DESC_CASE_SENSITIVE),
            17 => Ok(SQL_DESC_CATALOG_NAME),
            2 => Ok(SQL_DESC_CONCISE_TYPE),
            26 => Ok(SQL_DESC_DATETIME_INTERVAL_PRECISION),
            6 => Ok(SQL_DESC_DISPLAY_SIZE),
            9 => Ok(SQL_DESC_FIXED_PREC_SCALE),
            18 => Ok(SQL_DESC_LABEL),
            27 => Ok(SQL_DESC_LITERAL_PREFIX),
            28 => Ok(SQL_DESC_LITERAL_SUFFIX),
            29 => Ok(SQL_DESC_LOCAL_TYPE_NAME),
            30 => Ok(SQL_DESC_MAXIMUM_SCALE),
            31 => Ok(SQL_DESC_MINIMUM_SCALE),
            32 => Ok(SQL_DESC_NUM_PREC_RADIX),
            33 => Ok(SQL_DESC_PARAMETER_TYPE),
            34 => Ok(SQL_DESC_ROWS_PROCESSED_PTR),
            35 => Ok(SQL_DESC_ROWVER),
            16 => Ok(SQL_DESC_SCHEMA_NAME),
            13 => Ok(SQL_DESC_SEARCHABLE),
            14 => Ok(SQL_DESC_TYPE_NAME),
            15 => Ok(SQL_DESC_TABLE_NAME),
            8 => Ok(SQL_DESC_UNSIGNED),
            10 => Ok(SQL_DESC_UPDATABLE),
            1001 => Ok(SQL_DESC_COUNT),
            1002 => Ok(SQL_DESC_TYPE),
            1003 => Ok(SQL_DESC_LENGTH),
            1004 => Ok(SQL_DESC_OCTET_LENGTH_PTR),
            1005 => Ok(SQL_DESC_PRECISION),
            1006 => Ok(SQL_DESC_SCALE),
            1007 => Ok(SQL_DESC_DATETIME_INTERVAL_CODE),
            1008 => Ok(SQL_DESC_NULLABLE),
            1009 => Ok(SQL_DESC_INDICATOR_PTR),
            1010 => Ok(SQL_DESC_DATA_PTR),
            1011 => Ok(SQL_DESC_NAME),
            1012 => Ok(SQL_DESC_UNNAMED),
            1013 => Ok(SQL_DESC_OCTET_LENGTH),
            1099 => Ok(SQL_DESC_ALLOC_TYPE),
            e => Err(e),
        }
    }
}

#[no_mangle]
pub extern "system" fn SQLColAttribute(
    hstmt: HStmt,
    column_number: SqlUSmallInt,
    field_identifier: SqlUSmallInt,
    character_attribute_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
    numeric_attribute_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLColAttribute()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, column_number={:?}, field_identifier={:?}, character_attribute_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}, numeric_attribute_ptr={:?}",
        hstmt,
        column_number,
        field_identifier,
        character_attribute_ptr,
        buffer_length,
        string_length_ptr,
        numeric_attribute_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let field_identifier = match FieldIdentifier::try_from(field_identifier) {
        Ok(value) => value,
        Err(field_identifier) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported field_identifier: {}",
                field_identifier
            );
            stmt.add_diag(
                TsurugiOdbcError::ColAttributeUnsupportedFieldIdentifier,
                format!(
                    "{FUNCTION_NAME}: Unsupported field_identifier: {}",
                    field_identifier
                ),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    let arg = TsurugiOdbcColAttributeArguments::new(
        column_number,
        field_identifier,
        character_attribute_ptr,
        buffer_length,
        string_length_ptr,
        numeric_attribute_ptr,
        false,
    );
    let rc = col_attribute(&stmt, arg);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLColAttributeW(
    hstmt: HStmt,
    column_number: SqlUSmallInt,
    field_identifier: SqlUSmallInt,
    character_attribute_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
    numeric_attribute_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLColAttributeW()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, column_number={:?}, field_identifier={:?}, character_attribute_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}, numeric_attribute_ptr={:?}",
        hstmt,
        column_number,
        field_identifier,
        character_attribute_ptr,
        buffer_length,
        string_length_ptr,
        numeric_attribute_ptr
    );

    let stmt = check_stmt!(hstmt);
    let stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let field_identifier = match FieldIdentifier::try_from(field_identifier) {
        Ok(value) => value,
        Err(field_identifier) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported field_identifier: {}",
                field_identifier
            );
            stmt.add_diag(
                TsurugiOdbcError::ColAttributeUnsupportedFieldIdentifier,
                format!(
                    "{FUNCTION_NAME}: Unsupported field_identifier: {}",
                    field_identifier
                ),
            );
            return SqlReturn::SQL_ERROR;
        }
    };

    let arg = TsurugiOdbcColAttributeArguments::new(
        column_number,
        field_identifier,
        character_attribute_ptr,
        buffer_length,
        string_length_ptr,
        numeric_attribute_ptr,
        true,
    );
    let rc = col_attribute(&stmt, arg);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

pub(crate) struct TsurugiOdbcColAttributeArguments {
    column_number: SqlUSmallInt,
    field_identifier: FieldIdentifier,
    character_attribute_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
    numeric_attribute_ptr: *mut SqlLen,
    wide_char: bool,
}

impl TsurugiOdbcColAttributeArguments {
    pub fn new(
        column_number: SqlUSmallInt,
        field_identifier: FieldIdentifier,
        character_attribute_ptr: SqlPointer,
        buffer_length: SqlSmallInt,
        string_length_ptr: *mut SqlSmallInt,
        numeric_attribute_ptr: *mut SqlLen,
        wide_char: bool,
    ) -> TsurugiOdbcColAttributeArguments {
        TsurugiOdbcColAttributeArguments {
            column_number,
            field_identifier,
            character_attribute_ptr,
            buffer_length,
            string_length_ptr,
            numeric_attribute_ptr,
            wide_char,
        }
    }

    fn odbc_function_name(&self) -> &'static str {
        if self.wide_char {
            "SQLColAttributeW()"
        } else {
            "SQLColAttribute()"
        }
    }
}

fn col_attribute(stmt: &TsurugiOdbcStmt, arg: TsurugiOdbcColAttributeArguments) -> SqlReturn {
    const FUNCTION_NAME: &str = "col_attribute()";

    let field_identifier = arg.field_identifier;

    let processor = match stmt.processor(FUNCTION_NAME) {
        Ok(processor) => processor,
        Err(rc) => return rc,
    };
    let processor = processor.borrow();

    use FieldIdentifier::*;
    match field_identifier {
        SQL_DESC_COUNT | SQL_COLUMN_COUNT => {
            return write_numeric(stmt, &arg, processor.number_of_columns() as SqlLen);
        }
        _ => {}
    }

    let column_number = arg.column_number;
    let column = {
        let number_of_columns = processor.number_of_columns();
        if column_number < 1 || column_number > number_of_columns {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. index out of bounds: column_number={}",
                column_number
            );
            stmt.add_diag(
                TsurugiOdbcError::ColAttributeInvalidColumnNumber,
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

    match field_identifier {
        SQL_DESC_AUTO_UNIQUE_VALUE => write_numeric(stmt, &arg, SQL_FALSE as SqlLen),
        SQL_DESC_BASE_COLUMN_NAME | SQL_DESC_LABEL | SQL_DESC_NAME | SQL_COLUMN_NAME => {
            write_string(stmt, &arg, column.column_name())
        }
        SQL_DESC_BASE_TABLE_NAME | SQL_DESC_TABLE_NAME => write_string(stmt, &arg, ""), // TODO table name
        SQL_DESC_CASE_SENSITIVE => write_numeric(stmt, &arg, SQL_TRUE as SqlLen),
        SQL_DESC_CATALOG_NAME => write_string(stmt, &arg, ""), // TODO catalog name
        SQL_DESC_CONCISE_TYPE | SQL_DESC_TYPE => {
            write_numeric(stmt, &arg, column.data_type() as SqlLen)
        }
        SQL_DESC_COUNT | SQL_COLUMN_COUNT => unreachable!(),
        SQL_DESC_DISPLAY_SIZE => write_numeric(stmt, &arg, column.column_size() as SqlLen),
        SQL_DESC_FIXED_PREC_SCALE => {
            let value = if column.scale() != 0 {
                SQL_TRUE
            } else {
                SQL_FALSE
            };
            write_numeric(stmt, &arg, value as SqlLen)
        }
        SQL_DESC_LOCAL_TYPE_NAME => write_string(stmt, &arg, ""),
        SQL_DESC_LENGTH | SQL_COLUMN_LENGTH => {
            write_numeric(stmt, &arg, column.column_size() as SqlLen)
        }
        SQL_DESC_LITERAL_PREFIX => write_string(
            stmt,
            &arg,
            literal_prefix(&column.data_type()).unwrap_or(""),
        ),
        SQL_DESC_LITERAL_SUFFIX => write_string(
            stmt,
            &arg,
            literal_suffix(&column.data_type()).unwrap_or(""),
        ),
        SQL_DESC_NULLABLE | SQL_COLUMN_NULLABLE => {
            write_numeric(stmt, &arg, column.nullable() as SqlLen)
        }
        SQL_DESC_NUM_PREC_RADIX => write_numeric(
            stmt,
            &arg,
            num_prec_radix(&column.data_type()).unwrap_or(0) as SqlLen,
        ),
        SQL_DESC_OCTET_LENGTH => {
            let value = if let Some(column) = column.sql_column() {
                char_octet_length(column).unwrap_or(0) as SqlLen
            } else {
                0
            };
            write_numeric(stmt, &arg, value)
        }
        SQL_DESC_PRECISION | SQL_COLUMN_PRECISION => {
            let value = if let Some(column) = column.sql_column() {
                precision(column) as SqlLen
            } else {
                0
            };
            write_numeric(stmt, &arg, value)
        }
        SQL_DESC_SCALE | SQL_COLUMN_SCALE => write_numeric(stmt, &arg, column.scale() as SqlLen),
        SQL_DESC_SCHEMA_NAME => write_string(stmt, &arg, ""), // TODO schema name
        SQL_DESC_SEARCHABLE => write_numeric(stmt, &arg, searchable(&column.data_type()) as SqlLen),
        SQL_DESC_TYPE_NAME => {
            let value = column
                .sql_column()
                .map(|c| c.sql_type_name())
                .unwrap_or(None);
            let value = if let Some(value) = value {
                value
            } else {
                type_name(&column.data_type()).unwrap_or("")
            };
            write_string(stmt, &arg, value)
        }
        SQL_DESC_UNNAMED => {
            let name = column.column_name();
            let value = if name.is_empty() {
                1 // SQL_UNNAMED
            } else {
                0 // SQL_NAMED
            };
            write_numeric(stmt, &arg, value)
        }
        SQL_DESC_UNSIGNED => {
            let value = match unsigned_attribute(&column.data_type()) {
                Some(value) => value,
                None => SQL_TRUE, // not numeric
            };
            write_numeric(stmt, &arg, value as SqlLen)
        }
        SQL_DESC_UPDATABLE => write_numeric(stmt, &arg, 0 /* SQL_ATTR_READONLY */),
        _ => {
            warn!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported field_identifier: {:?}",
                field_identifier
            );
            let odbc_function_name = arg.odbc_function_name();
            stmt.add_diag(
                TsurugiOdbcError::ColAttributeUnsupportedFieldIdentifier,
                format!(
                    "{odbc_function_name}: Unsupported field_identifier: {:?}",
                    field_identifier
                ),
            );
            SqlReturn::SQL_ERROR
        }
    }
}

fn precision(column: &SqlColumn) -> u32 {
    use AtomType::*;
    match column.atom_type() {
        Some(atom_type) => {
            match atom_type {
                Boolean => 1,  // bits
                Int4 => 8 * 4, // bits
                Int8 => 8 * 8, // bits
                Float4 => 38,
                Float8 => 308,
                Decimal => match column.precision() {
                    Some((_, true)) => 38,
                    Some((precision, false)) => precision,
                    _ => 0,
                },
                TimeOfDay | TimePoint | TimeOfDayWithTimeZone | TimePointWithTimeZone => 9,
                _ => 0,
            }
        }
        _ => 0,
    }
}

fn write_string(
    stmt: &TsurugiOdbcStmt,
    arg: &TsurugiOdbcColAttributeArguments,
    value: &str,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "col_attribute().write_string()";
    debug!(
        "{stmt}.{FUNCTION_NAME}: column_number={}, {:?}={}",
        arg.column_number, arg.field_identifier, value
    );

    if arg.wide_char {
        write_wchar_bytes(
            "SQLColAttributeW.character_attribute_ptr",
            value,
            arg.character_attribute_ptr as *mut SqlWChar,
            arg.buffer_length,
            arg.string_length_ptr,
            Some(&stmt.diag_collection()),
        )
    } else {
        write_char(
            "SQLColAttribute.character_attribute_ptr",
            value,
            arg.character_attribute_ptr as *mut SqlChar,
            arg.buffer_length,
            arg.string_length_ptr,
            Some(&stmt.diag_collection()),
        )
    }
}

fn write_numeric(
    stmt: &TsurugiOdbcStmt,
    arg: &TsurugiOdbcColAttributeArguments,
    value: SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "col_attribute().write_numeric()";
    debug!(
        "{stmt}.{FUNCTION_NAME}: column_number={}, {:?}={}",
        arg.column_number, arg.field_identifier, value
    );

    let numeric_attribute_ptr = arg.numeric_attribute_ptr;
    if !numeric_attribute_ptr.is_null() {
        unsafe {
            *numeric_attribute_ptr = value;
        }
    }

    SqlReturn::SQL_SUCCESS
}
