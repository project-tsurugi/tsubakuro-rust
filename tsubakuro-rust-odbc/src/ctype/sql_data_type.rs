use tsubakuro_rust_core::prelude::{AtomType, SqlColumn};

use crate::handle::diag::TsurugiOdbcError;

/// SQL data type
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum SqlDataType {
    SQL_UNKNOWN_TYPE = 0,
    SQL_CHAR = 1,
    SQL_NUMERIC = 2,
    SQL_DECIMAL = 3,
    SQL_INTEGER = 4,
    SQL_SMALLINT = 5,
    SQL_FLOAT = 6,
    SQL_REAL = 7,
    SQL_DOUBLE = 8,
    SQL_DATETIME = 9,
    SQL_VARCHAR = 12,

    SQL_TYPE_DATE = 91,
    SQL_TYPE_TIME = 92,
    SQL_TYPE_TIMESTAMP = 93,

    SQL_INTERVAL_YEAR = 101,
    SQL_INTERVAL_MONTH = 102,
    SQL_INTERVAL_DAY = 103,
    SQL_INTERVAL_HOUR = 104,
    SQL_INTERVAL_MINUTE = 105,
    SQL_INTERVAL_SECOND = 106,
    SQL_INTERVAL_YEAR_TO_MONTH = 107,
    SQL_INTERVAL_DAY_TO_HOUR = 108,
    SQL_INTERVAL_DAY_TO_MINUTE = 109,
    SQL_INTERVAL_DAY_TO_SECOND = 110,
    SQL_INTERVAL_HOUR_TO_MINUTE = 111,
    SQL_INTERVAL_HOUR_TO_SECOND = 112,
    SQL_INTERVAL_MINUTE_TO_SECOND = 113,

    SQL_LONGVARCHAR = -1,
    SQL_BINARY = -2,
    SQL_VARBINARY = -3,
    SQL_LONGVARBINARY = -4,
    SQL_BIGINT = -5,
    SQL_TINYINT = -6,
    SQL_BIT = -7,

    SQL_WCHAR = -8,
    SQL_WVARCHAR = -9,
    SQL_WLONGVARCHAR = -10,

    SQL_GUID = -11,
}
impl TryFrom<i16> for SqlDataType {
    type Error = TsurugiOdbcError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        use SqlDataType::*;
        match value {
            0 => Ok(SQL_UNKNOWN_TYPE),
            1 => Ok(SQL_CHAR),
            2 => Ok(SQL_NUMERIC),
            3 => Ok(SQL_DECIMAL),
            4 => Ok(SQL_INTEGER),
            5 => Ok(SQL_SMALLINT),
            6 => Ok(SQL_FLOAT),
            7 => Ok(SQL_REAL),
            8 => Ok(SQL_DOUBLE),
            9 => Ok(SQL_DATETIME),
            12 => Ok(SQL_VARCHAR),
            91 => Ok(SQL_TYPE_DATE),
            92 => Ok(SQL_TYPE_TIME),
            93 => Ok(SQL_TYPE_TIMESTAMP),
            101 => Ok(SQL_INTERVAL_YEAR),
            102 => Ok(SQL_INTERVAL_MONTH),
            103 => Ok(SQL_INTERVAL_DAY),
            104 => Ok(SQL_INTERVAL_HOUR),
            105 => Ok(SQL_INTERVAL_MINUTE),
            106 => Ok(SQL_INTERVAL_SECOND),
            107 => Ok(SQL_INTERVAL_YEAR_TO_MONTH),
            108 => Ok(SQL_INTERVAL_DAY_TO_HOUR),
            109 => Ok(SQL_INTERVAL_DAY_TO_MINUTE),
            110 => Ok(SQL_INTERVAL_DAY_TO_SECOND),
            111 => Ok(SQL_INTERVAL_HOUR_TO_MINUTE),
            112 => Ok(SQL_INTERVAL_HOUR_TO_SECOND),
            113 => Ok(SQL_INTERVAL_MINUTE_TO_SECOND),
            -1 => Ok(SQL_LONGVARCHAR),
            -2 => Ok(SQL_BINARY),
            -3 => Ok(SQL_VARBINARY),
            -4 => Ok(SQL_LONGVARBINARY),
            -5 => Ok(SQL_BIGINT),
            -6 => Ok(SQL_TINYINT),
            -7 => Ok(SQL_BIT),
            -8 => Ok(SQL_WCHAR),
            -9 => Ok(SQL_WVARCHAR),
            -10 => Ok(SQL_WLONGVARCHAR),
            -11 => Ok(SQL_GUID),
            _ => Err(TsurugiOdbcError::UnsupportedSqlDataType),
        }
    }
}

impl From<&SqlColumn> for SqlDataType {
    fn from(value: &SqlColumn) -> Self {
        use SqlDataType::*;
        match value.atom_type().unwrap_or(AtomType::Unknown) {
            AtomType::Boolean => SQL_BIT,
            AtomType::Int4 => SQL_INTEGER,
            AtomType::Int8 => SQL_BIGINT,
            AtomType::Float4 => SQL_REAL,
            AtomType::Float8 => SQL_DOUBLE,
            AtomType::Decimal => SQL_DECIMAL,
            AtomType::Character => {
                if value.length().is_none() || value.varying().unwrap_or(true) {
                    SQL_VARCHAR
                } else {
                    SQL_CHAR
                }
            }
            AtomType::Octet => {
                if value.length().is_none() || value.varying().unwrap_or(true) {
                    SQL_VARBINARY
                } else {
                    SQL_BINARY
                }
            }
            // AtomType::Bit => todo!(),
            AtomType::Date => SQL_TYPE_DATE,
            AtomType::TimeOfDay => SQL_TYPE_TIME,
            AtomType::TimePoint => SQL_TYPE_TIMESTAMP,
            // AtomType::DatetimeInterval => todo!(),
            AtomType::TimeOfDayWithTimeZone => SQL_TYPE_TIME,
            AtomType::TimePointWithTimeZone => SQL_TYPE_TIMESTAMP,
            AtomType::Clob => SQL_LONGVARCHAR,
            AtomType::Blob => SQL_LONGVARBINARY,
            _ => SQL_UNKNOWN_TYPE,
        }
    }
}

impl TryFrom<SqlDataType> for AtomType {
    type Error = ();

    fn try_from(value: SqlDataType) -> Result<Self, Self::Error> {
        match value {
            SqlDataType::SQL_BIT => Ok(AtomType::Boolean),
            SqlDataType::SQL_INTEGER => Ok(AtomType::Int4),
            SqlDataType::SQL_BIGINT => Ok(AtomType::Int8),
            SqlDataType::SQL_FLOAT | SqlDataType::SQL_REAL => Ok(AtomType::Float4),
            SqlDataType::SQL_DOUBLE => Ok(AtomType::Float8),
            SqlDataType::SQL_NUMERIC | SqlDataType::SQL_DECIMAL => Ok(AtomType::Decimal),
            SqlDataType::SQL_CHAR | SqlDataType::SQL_VARCHAR => Ok(AtomType::Character),
            SqlDataType::SQL_WCHAR | SqlDataType::SQL_WVARCHAR => Ok(AtomType::Character),
            SqlDataType::SQL_BINARY | SqlDataType::SQL_VARBINARY => Ok(AtomType::Octet),
            SqlDataType::SQL_TYPE_DATE => Ok(AtomType::Date),
            SqlDataType::SQL_TYPE_TIME => Ok(AtomType::TimeOfDay),
            SqlDataType::SQL_TYPE_TIMESTAMP | SqlDataType::SQL_DATETIME => Ok(AtomType::TimePoint),
            SqlDataType::SQL_LONGVARBINARY => Ok(AtomType::Blob),
            SqlDataType::SQL_LONGVARCHAR => Ok(AtomType::Clob),
            SqlDataType::SQL_WLONGVARCHAR => Ok(AtomType::Clob),
            _ => Err(()),
        }
    }
}
