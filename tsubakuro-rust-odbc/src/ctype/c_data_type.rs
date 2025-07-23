const SQL_SIGNED_OFFSET: i16 = -20;
const SQL_UNSIGNED_OFFSET: i16 = -22;

/// C data type
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub(crate) enum CDataType {
    /// CHAR, VARCHAR
    SQL_C_CHAR = 1,
    SQL_C_WCHAR = -8,
    /// INTEGER
    SQL_C_LONG = 4,
    /// SMALLINT
    SQL_C_SHORT = 5,
    /// REAL
    SQL_C_FLOAT = 7,
    /// DOUBLE
    SQL_C_DOUBLE = 8,
    SQL_C_NUMERIC = 2,
    SQL_C_DEFAULT = 99,

    SQL_C_DATE = 9,
    SQL_C_TIME = 10,
    SQL_C_TIMESTAMP = 11,

    SQL_C_TYPE_DATE = 91,
    SQL_C_TYPE_TIME = 92,
    SQL_C_TYPE_TIMESTAMP = 93,
    SQL_C_INTERVAL_YEAR = -80,
    SQL_C_INTERVAL_MONTH = -81,
    SQL_C_INTERVAL_DAY = -83,
    SQL_C_INTERVAL_HOUR = -84,
    SQL_C_INTERVAL_MINUTE = -85,
    SQL_C_INTERVAL_SECOND = -86,
    SQL_C_INTERVAL_YEAR_TO_MONTH = -82,
    SQL_C_INTERVAL_DAY_TO_HOUR = -87,
    SQL_C_INTERVAL_DAY_TO_MINUTE = -88,
    SQL_C_INTERVAL_DAY_TO_SECOND = -89,
    SQL_C_INTERVAL_HOUR_TO_MINUTE = -90,
    SQL_C_INTERVAL_HOUR_TO_SECOND = -91,
    SQL_C_INTERVAL_MINUTE_TO_SECOND = -92,

    SQL_C_BINARY = -2,
    SQL_C_BIT = -7,

    SQL_C_SBIGINT = -5 + SQL_SIGNED_OFFSET,
    SQL_C_UBIGINT = -5 + SQL_UNSIGNED_OFFSET,

    SQL_C_TINYINT = -6,
    SQL_C_SLONG = CDataType::SQL_C_LONG as i16 + SQL_SIGNED_OFFSET,
    SQL_C_SSHORT = CDataType::SQL_C_SHORT as i16 + SQL_SIGNED_OFFSET,
    SQL_C_STINYINT = CDataType::SQL_C_TINYINT as i16 + SQL_SIGNED_OFFSET,
    SQL_C_ULONG = CDataType::SQL_C_LONG as i16 + SQL_UNSIGNED_OFFSET,
    SQL_C_USHORT = CDataType::SQL_C_SHORT as i16 + SQL_UNSIGNED_OFFSET,
    SQL_C_UTINYINT = CDataType::SQL_C_TINYINT as i16 + SQL_UNSIGNED_OFFSET,
}

impl TryFrom<i16> for CDataType {
    type Error = i16;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        use CDataType::*;
        match value {
            1 => Ok(SQL_C_CHAR),
            -8 => Ok(SQL_C_WCHAR),
            4 => Ok(SQL_C_LONG),
            5 => Ok(SQL_C_SHORT),
            7 => Ok(SQL_C_FLOAT),
            8 => Ok(SQL_C_DOUBLE),
            2 => Ok(SQL_C_NUMERIC),
            99 => Ok(SQL_C_DEFAULT),
            9 => Ok(SQL_C_DATE),
            10 => Ok(SQL_C_TIME),
            11 => Ok(SQL_C_TIMESTAMP),
            91 => Ok(SQL_C_TYPE_DATE),
            92 => Ok(SQL_C_TYPE_TIME),
            93 => Ok(SQL_C_TYPE_TIMESTAMP),
            -80 => Ok(SQL_C_INTERVAL_YEAR),
            -81 => Ok(SQL_C_INTERVAL_MONTH),
            -82 => Ok(SQL_C_INTERVAL_YEAR_TO_MONTH),
            -83 => Ok(SQL_C_INTERVAL_DAY),
            -84 => Ok(SQL_C_INTERVAL_HOUR),
            -85 => Ok(SQL_C_INTERVAL_MINUTE),
            -86 => Ok(SQL_C_INTERVAL_SECOND),
            -87 => Ok(SQL_C_INTERVAL_DAY_TO_HOUR),
            -88 => Ok(SQL_C_INTERVAL_DAY_TO_MINUTE),
            -89 => Ok(SQL_C_INTERVAL_DAY_TO_SECOND),
            -90 => Ok(SQL_C_INTERVAL_HOUR_TO_MINUTE),
            -91 => Ok(SQL_C_INTERVAL_HOUR_TO_SECOND),
            -92 => Ok(SQL_C_INTERVAL_MINUTE_TO_SECOND),
            -2 => Ok(SQL_C_BINARY),
            -7 => Ok(SQL_C_BIT),
            x if x == -5 + SQL_SIGNED_OFFSET => Ok(SQL_C_SBIGINT),
            x if x == -5 + SQL_UNSIGNED_OFFSET => Ok(SQL_C_UBIGINT),
            -6 => Ok(SQL_C_TINYINT),
            x if x == 4 + SQL_SIGNED_OFFSET => Ok(SQL_C_SLONG),
            x if x == 5 + SQL_SIGNED_OFFSET => Ok(SQL_C_SSHORT),
            x if x == -6 + SQL_SIGNED_OFFSET => Ok(SQL_C_STINYINT),
            x if x == 4 + SQL_UNSIGNED_OFFSET => Ok(SQL_C_ULONG),
            x if x == 5 + SQL_UNSIGNED_OFFSET => Ok(SQL_C_USHORT),
            x if x == -6 + SQL_UNSIGNED_OFFSET => Ok(SQL_C_UTINYINT),
            e => Err(e),
        }
    }
}
