use std::sync::Arc;

use log::{debug, trace};

use crate::{
    check_dbc,
    ctype::{SqlChar, SqlPointer, SqlReturn, SqlSmallInt, SqlUInteger, SqlUSmallInt, SqlWChar},
    handle::{
        diag::TsurugiOdbcError,
        hdbc::{HDbc, TsurugiOdbcDbc},
    },
    odbc_driver_version,
    util::{write_char, write_wchar_bytes},
    ODBC_DRIVER_FILE_NAME, TSURUGI_VERSION,
};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum InfoType {
    SQL_MAXIMUM_DRIVER_CONNECTIONS = 0, // SQL_MAX_DRIVER_CONNECTIONS
    SQL_MAXIMUM_CONCURRENT_ACTIVITIES = 1, // SQL_MAX_CONCURRENT_ACTIVITIES
    SQL_DATA_SOURCE_NAME = 2,
    SQL_DRIVER_HDBC = 3,
    SQL_DRIVER_HENV = 4,
    SQL_DRIVER_HSTMT = 5,
    SQL_DRIVER_NAME = 6,
    SQL_DRIVER_VER = 7,
    SQL_FETCH_DIRECTION = 8,
    SQL_ODBC_API_CONFORMANCE = 9,
    SQL_ODBC_VER = 10,
    SQL_ROW_UPDATES = 11,
    SQL_ODBC_SAG_CLI_CONFORMANCE = 12,
    SQL_SERVER_NAME = 13,
    SQL_SEARCH_PATTERN_ESCAPE = 14,
    SQL_ODBC_SQL_CONFORMANCE = 15,
    SQL_DATABASE_NAME = 16,
    SQL_DBMS_NAME = 17,
    SQL_DBMS_VER = 18,
    SQL_ACCESSIBLE_TABLES = 19,
    SQL_ACCESSIBLE_PROCEDURES = 20,
    SQL_PROCEDURES = 21,
    SQL_CONCAT_NULL_BEHAVIOR = 22,
    SQL_CURSOR_COMMIT_BEHAVIOR = 23,
    SQL_CURSOR_ROLLBACK_BEHAVIOR = 24,
    SQL_DATA_SOURCE_READ_ONLY = 25,
    SQL_DEFAULT_TXN_ISOLATION = 26,
    SQL_EXPRESSIONS_IN_ORDERBY = 27,
    SQL_IDENTIFIER_CASE = 28,
    SQL_IDENTIFIER_QUOTE_CHAR = 29,
    SQL_MAXIMUM_COLUMN_NAME_LENGTH = 30, // SQL_MAX_COLUMN_NAME_LEN
    SQL_MAXIMUM_CURSOR_NAME_LENGTH = 31, // SQL_MAX_CURSOR_NAME_LEN
    SQL_MAXIMUM_SCHEMA_NAME_LENGTH = 32, // SQL_MAX_SCHEMA_NAME_LEN
    SQL_MAX_PROCEDURE_NAME_LEN = 33,
    SQL_MAXIMUM_CATALOG_NAME_LENGTH = 34, // SQL_MAX_CATALOG_NAME_LEN
    SQL_MAX_TABLE_NAME_LEN = 35,
    SQL_MULT_RESULT_SETS = 36,
    SQL_MULTIPLE_ACTIVE_TXN = 37,
    SQL_OUTER_JOINS = 38,
    // SQL_OWNER_TERM = 39,
    SQL_PROCEDURE_TERM = 40,
    // SQL_QUALIFIER_NAME_SEPARATOR = 41,
    // SQL_QUALIFIER_TERM = 42,
    SQL_SCROLL_CONCURRENCY = 43,
    SQL_SCROLL_OPTIONS = 44,
    SQL_TABLE_TERM = 45,
    SQL_TRANSACTION_CAPABLE = 46, // SQL_TXN_CAPABLE
    SQL_USER_NAME = 47,
    SQL_CONVERT_FUNCTIONS = 48,
    SQL_NUMERIC_FUNCTIONS = 49,
    SQL_STRING_FUNCTIONS = 50,
    SQL_SYSTEM_FUNCTIONS = 51,
    SQL_TIMEDATE_FUNCTIONS = 52,
    SQL_CONVERT_BIGINT = 53,
    SQL_CONVERT_BINARY = 54,
    SQL_CONVERT_BIT = 55,
    SQL_CONVERT_CHAR = 56,
    SQL_CONVERT_DATE = 57,
    SQL_CONVERT_DECIMAL = 58,
    SQL_CONVERT_DOUBLE = 59,
    SQL_CONVERT_FLOAT = 60,
    SQL_CONVERT_INTEGER = 61,
    SQL_CONVERT_LONGVARCHAR = 62,
    SQL_CONVERT_NUMERIC = 63,
    SQL_CONVERT_REAL = 64,
    SQL_CONVERT_SMALLINT = 65,
    SQL_CONVERT_TIME = 66,
    SQL_CONVERT_TIMESTAMP = 67,
    SQL_CONVERT_TINYINT = 68,
    SQL_CONVERT_VARBINARY = 69,
    SQL_CONVERT_VARCHAR = 70,
    SQL_CONVERT_LONGVARBINARY = 71,
    SQL_TRANSACTION_ISOLATION_OPTION = 72, // SQL_TXN_ISOLATION_OPTION
    SQL_INTEGRITY = 73,
    SQL_CORRELATION_NAME = 74,
    SQL_NON_NULLABLE_COLUMNS = 75,
    SQL_DRIVER_HLIB = 76,
    SQL_DRIVER_ODBC_VER = 77,
    SQL_LOCK_TYPES = 78,
    SQL_POS_OPERATIONS = 79,
    SQL_POSITIONED_STATEMENTS = 80,
    SQL_GETDATA_EXTENSIONS = 81,
    SQL_BOOKMARK_PERSISTENCE = 82,
    SQL_STATIC_SENSITIVITY = 83,
    SQL_FILE_USAGE = 84,
    SQL_NULL_COLLATION = 85,
    SQL_ALTER_TABLE = 86,
    SQL_COLUMN_ALIAS = 87,
    SQL_GROUP_BY = 88,
    SQL_KEYWORDS = 89,
    SQL_ORDER_BY_COLUMNS_IN_SELECT = 90,
    // SQL_OWNER_USAGE = 91,
    // SQL_QUALIFIER_USAGE = 92,
    SQL_QUOTED_IDENTIFIER_CASE = 93,
    SQL_SPECIAL_CHARACTERS = 94,
    SQL_SUBQUERIES = 95,
    // SQL_UNION = 96,
    SQL_MAXIMUM_COLUMNS_IN_GROUP_BY = 97, // SQL_MAX_COLUMNS_IN_GROUP_BY
    SQL_MAXIMUM_COLUMNS_IN_INDEX = 98,    // SQL_MAX_COLUMNS_IN_INDEX
    SQL_MAXIMUM_COLUMNS_IN_ORDER_BY = 99, // SQL_MAX_COLUMNS_IN_ORDER_BY
    SQL_MAXIMUM_COLUMNS_IN_SELECT = 100,  // SQL_MAX_COLUMNS_IN_SELECT
    SQL_MAX_COLUMNS_IN_TABLE = 101,
    SQL_MAXIMUM_INDEX_SIZE = 102, // SQL_MAX_INDEX_SIZE
    SQL_MAX_ROW_SIZE_INCLUDES_LONG = 103,
    SQL_MAXIMUM_ROW_SIZE = 104,         // SQL_MAX_ROW_SIZE
    SQL_MAXIMUM_STATEMENT_LENGTH = 105, // SQL_MAX_STATEMENT_LEN
    SQL_MAXIMUM_TABLES_IN_SELECT = 106, // SQL_MAX_TABLES_IN_SELECT
    SQL_MAXIMUM_USER_NAME_LENGTH = 107, // SQL_MAX_USER_NAME_LEN
    SQL_MAX_CHAR_LITERAL_LEN = 108,
    SQL_TIMEDATE_ADD_INTERVALS = 109,
    SQL_TIMEDATE_DIFF_INTERVALS = 110,
    SQL_NEED_LONG_DATA_LEN = 111,
    SQL_MAX_BINARY_LITERAL_LEN = 112,
    SQL_LIKE_ESCAPE_CLAUSE = 113,
    // SQL_QUALIFIER_LOCATION = 114,
    SQL_OUTER_JOIN_CAPABILITIES = 115,

    SQL_ACTIVE_ENVIRONMENTS = 116,
    SQL_ALTER_DOMAIN = 117,
    SQL_SQL_CONFORMANCE = 118,
    SQL_DATETIME_LITERALS = 119,
    SQL_ASYNC_MODE = 10021, /* new X/Open spec */
    SQL_BATCH_ROW_COUNT = 120,
    SQL_BATCH_SUPPORT = 121,
    SQL_CATALOG_LOCATION = 114,      // SQL_QUALIFIER_LOCATION
    SQL_CATALOG_NAME_SEPARATOR = 41, // SQL_QUALIFIER_NAME_SEPARATOR
    SQL_CATALOG_TERM = 42,           // SQL_QUALIFIER_TERM
    SQL_CATALOG_USAGE = 92,          // SQL_QUALIFIER_USAGE
    SQL_CONVERT_WCHAR = 122,
    SQL_CONVERT_INTERVAL_DAY_TIME = 123,
    SQL_CONVERT_INTERVAL_YEAR_MONTH = 124,
    SQL_CONVERT_WLONGVARCHAR = 125,
    SQL_CONVERT_WVARCHAR = 126,
    SQL_CREATE_ASSERTION = 127,
    SQL_CREATE_CHARACTER_SET = 128,
    SQL_CREATE_COLLATION = 129,
    SQL_CREATE_DOMAIN = 130,
    SQL_CREATE_SCHEMA = 131,
    SQL_CREATE_TABLE = 132,
    SQL_CREATE_TRANSLATION = 133,
    SQL_CREATE_VIEW = 134,
    SQL_DRIVER_HDESC = 135,
    SQL_DROP_ASSERTION = 136,
    SQL_DROP_CHARACTER_SET = 137,
    SQL_DROP_COLLATION = 138,
    SQL_DROP_DOMAIN = 139,
    SQL_DROP_SCHEMA = 140,
    SQL_DROP_TABLE = 141,
    SQL_DROP_TRANSLATION = 142,
    SQL_DROP_VIEW = 143,
    SQL_DYNAMIC_CURSOR_ATTRIBUTES1 = 144,
    SQL_DYNAMIC_CURSOR_ATTRIBUTES2 = 145,
    SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1 = 146,
    SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2 = 147,
    SQL_INDEX_KEYWORDS = 148,
    SQL_INFO_SCHEMA_VIEWS = 149,
    SQL_KEYSET_CURSOR_ATTRIBUTES1 = 150,
    SQL_KEYSET_CURSOR_ATTRIBUTES2 = 151,
    SQL_MAX_ASYNC_CONCURRENT_STATEMENTS = 10022, /* new X/Open spec */
    SQL_ODBC_INTERFACE_CONFORMANCE = 152,
    SQL_PARAM_ARRAY_ROW_COUNTS = 153,
    SQL_PARAM_ARRAY_SELECTS = 154,
    SQL_SCHEMA_TERM = 39,  // SQL_OWNER_TERM
    SQL_SCHEMA_USAGE = 91, //SQL_OWNER_USAGE
    SQL_SQL92_DATETIME_FUNCTIONS = 155,
    SQL_SQL92_FOREIGN_KEY_DELETE_RULE = 156,
    SQL_SQL92_FOREIGN_KEY_UPDATE_RULE = 157,
    SQL_SQL92_GRANT = 158,
    SQL_SQL92_NUMERIC_VALUE_FUNCTIONS = 159,
    SQL_SQL92_PREDICATES = 160,
    SQL_SQL92_RELATIONAL_JOIN_OPERATORS = 161,
    SQL_SQL92_REVOKE = 162,
    SQL_SQL92_ROW_VALUE_CONSTRUCTOR = 163,
    SQL_SQL92_STRING_FUNCTIONS = 164,
    SQL_SQL92_VALUE_EXPRESSIONS = 165,
    SQL_STANDARD_CLI_CONFORMANCE = 166,
    SQL_STATIC_CURSOR_ATTRIBUTES1 = 167,
    SQL_STATIC_CURSOR_ATTRIBUTES2 = 168,
    SQL_AGGREGATE_FUNCTIONS = 169,
    SQL_DDL_INDEX = 170,
    SQL_DM_VER = 171,
    SQL_INSERT_STATEMENT = 172,
    SQL_CONVERT_GUID = 173,
    SQL_UNION_STATEMENT = 96, // SQL_UNION

    SQL_XOPEN_CLI_YEAR = 10000,
    SQL_CURSOR_SENSITIVITY = 10001,
    SQL_DESCRIBE_PARAMETER = 10002,
    SQL_CATALOG_NAME = 10003,
    SQL_COLLATION_SEQ = 10004,
    SQL_MAXIMUM_IDENTIFIER_LENGTH = 10005, // SQL_MAX_IDENTIFIER_LEN
}
impl TryFrom<u16> for InfoType {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use InfoType::*;
        match value {
            0 => Ok(SQL_MAXIMUM_DRIVER_CONNECTIONS),
            1 => Ok(SQL_MAXIMUM_CONCURRENT_ACTIVITIES),
            2 => Ok(SQL_DATA_SOURCE_NAME),
            3 => Ok(SQL_DRIVER_HDBC),
            4 => Ok(SQL_DRIVER_HENV),
            5 => Ok(SQL_DRIVER_HSTMT),
            6 => Ok(SQL_DRIVER_NAME),
            7 => Ok(SQL_DRIVER_VER),
            8 => Ok(SQL_FETCH_DIRECTION),
            9 => Ok(SQL_ODBC_API_CONFORMANCE),
            10 => Ok(SQL_ODBC_VER),
            11 => Ok(SQL_ROW_UPDATES),
            12 => Ok(SQL_ODBC_SAG_CLI_CONFORMANCE),
            13 => Ok(SQL_SERVER_NAME),
            14 => Ok(SQL_SEARCH_PATTERN_ESCAPE),
            15 => Ok(SQL_ODBC_SQL_CONFORMANCE),
            17 => Ok(SQL_DBMS_NAME),
            18 => Ok(SQL_DBMS_VER),
            19 => Ok(SQL_ACCESSIBLE_TABLES),
            20 => Ok(SQL_ACCESSIBLE_PROCEDURES),
            21 => Ok(SQL_PROCEDURES),
            22 => Ok(SQL_CONCAT_NULL_BEHAVIOR),
            23 => Ok(SQL_CURSOR_COMMIT_BEHAVIOR),
            24 => Ok(SQL_CURSOR_ROLLBACK_BEHAVIOR),
            25 => Ok(SQL_DATA_SOURCE_READ_ONLY),
            26 => Ok(SQL_DEFAULT_TXN_ISOLATION),
            27 => Ok(SQL_EXPRESSIONS_IN_ORDERBY),
            28 => Ok(SQL_IDENTIFIER_CASE),
            29 => Ok(SQL_IDENTIFIER_QUOTE_CHAR),
            30 => Ok(SQL_MAXIMUM_COLUMN_NAME_LENGTH),
            31 => Ok(SQL_MAXIMUM_CURSOR_NAME_LENGTH),
            32 => Ok(SQL_MAXIMUM_SCHEMA_NAME_LENGTH),
            33 => Ok(SQL_MAX_PROCEDURE_NAME_LEN),
            34 => Ok(SQL_MAXIMUM_CATALOG_NAME_LENGTH),
            35 => Ok(SQL_MAX_TABLE_NAME_LEN),
            36 => Ok(SQL_MULT_RESULT_SETS),
            37 => Ok(SQL_MULTIPLE_ACTIVE_TXN),
            38 => Ok(SQL_OUTER_JOINS),
            // 39 => Ok(SQL_OWNER_TERM),
            40 => Ok(SQL_PROCEDURE_TERM),
            // 41 => Ok(SQL_QUALIFIER_NAME_SEPARATOR),
            // 42 => Ok(SQL_QUALIFIER_TERM),
            43 => Ok(SQL_SCROLL_CONCURRENCY),
            44 => Ok(SQL_SCROLL_OPTIONS),
            45 => Ok(SQL_TABLE_TERM),
            46 => Ok(SQL_TRANSACTION_CAPABLE),
            47 => Ok(SQL_USER_NAME),
            48 => Ok(SQL_CONVERT_FUNCTIONS),
            49 => Ok(SQL_NUMERIC_FUNCTIONS),
            50 => Ok(SQL_STRING_FUNCTIONS),
            51 => Ok(SQL_SYSTEM_FUNCTIONS),
            52 => Ok(SQL_TIMEDATE_FUNCTIONS),
            53 => Ok(SQL_CONVERT_BIGINT),
            54 => Ok(SQL_CONVERT_BINARY),
            55 => Ok(SQL_CONVERT_BIT),
            56 => Ok(SQL_CONVERT_CHAR),
            57 => Ok(SQL_CONVERT_DATE),
            58 => Ok(SQL_CONVERT_DECIMAL),
            59 => Ok(SQL_CONVERT_DOUBLE),
            60 => Ok(SQL_CONVERT_FLOAT),
            61 => Ok(SQL_CONVERT_INTEGER),
            62 => Ok(SQL_CONVERT_LONGVARCHAR),
            63 => Ok(SQL_CONVERT_NUMERIC),
            64 => Ok(SQL_CONVERT_REAL),
            65 => Ok(SQL_CONVERT_SMALLINT),
            66 => Ok(SQL_CONVERT_TIME),
            67 => Ok(SQL_CONVERT_TIMESTAMP),
            68 => Ok(SQL_CONVERT_TINYINT),
            69 => Ok(SQL_CONVERT_VARBINARY),
            70 => Ok(SQL_CONVERT_VARCHAR),
            71 => Ok(SQL_CONVERT_LONGVARBINARY),
            72 => Ok(SQL_TRANSACTION_ISOLATION_OPTION),
            73 => Ok(SQL_INTEGRITY),
            74 => Ok(SQL_CORRELATION_NAME),
            75 => Ok(SQL_NON_NULLABLE_COLUMNS),
            76 => Ok(SQL_DRIVER_HLIB),
            77 => Ok(SQL_DRIVER_ODBC_VER),
            78 => Ok(SQL_LOCK_TYPES),
            79 => Ok(SQL_POS_OPERATIONS),
            80 => Ok(SQL_POSITIONED_STATEMENTS),
            81 => Ok(SQL_GETDATA_EXTENSIONS),
            82 => Ok(SQL_BOOKMARK_PERSISTENCE),
            83 => Ok(SQL_STATIC_SENSITIVITY),
            84 => Ok(SQL_FILE_USAGE),
            85 => Ok(SQL_NULL_COLLATION),
            86 => Ok(SQL_ALTER_TABLE),
            87 => Ok(SQL_COLUMN_ALIAS),
            88 => Ok(SQL_GROUP_BY),
            89 => Ok(SQL_KEYWORDS),
            90 => Ok(SQL_ORDER_BY_COLUMNS_IN_SELECT),
            // 91 => Ok(SQL_OWNER_USAGE),
            // 92 => Ok(SQL_QUALIFIER_USAGE),
            93 => Ok(SQL_QUOTED_IDENTIFIER_CASE),
            94 => Ok(SQL_SPECIAL_CHARACTERS),
            95 => Ok(SQL_SUBQUERIES),
            // 96 => Ok(SQL_UNION),
            97 => Ok(SQL_MAXIMUM_COLUMNS_IN_GROUP_BY),
            98 => Ok(SQL_MAXIMUM_COLUMNS_IN_INDEX),
            99 => Ok(SQL_MAXIMUM_COLUMNS_IN_ORDER_BY),
            100 => Ok(SQL_MAXIMUM_COLUMNS_IN_SELECT),
            101 => Ok(SQL_MAX_COLUMNS_IN_TABLE),
            102 => Ok(SQL_MAXIMUM_INDEX_SIZE),
            103 => Ok(SQL_MAX_ROW_SIZE_INCLUDES_LONG),
            104 => Ok(SQL_MAXIMUM_ROW_SIZE),
            105 => Ok(SQL_MAXIMUM_STATEMENT_LENGTH),
            106 => Ok(SQL_MAXIMUM_TABLES_IN_SELECT),
            107 => Ok(SQL_MAXIMUM_USER_NAME_LENGTH),
            108 => Ok(SQL_MAX_CHAR_LITERAL_LEN),
            109 => Ok(SQL_TIMEDATE_ADD_INTERVALS),
            110 => Ok(SQL_TIMEDATE_DIFF_INTERVALS),
            111 => Ok(SQL_NEED_LONG_DATA_LEN),
            112 => Ok(SQL_MAX_BINARY_LITERAL_LEN),
            113 => Ok(SQL_LIKE_ESCAPE_CLAUSE),
            // 114 => Ok(SQL_QUALIFIER_LOCATION),
            115 => Ok(SQL_OUTER_JOIN_CAPABILITIES),
            116 => Ok(SQL_ACTIVE_ENVIRONMENTS),
            117 => Ok(SQL_ALTER_DOMAIN),
            118 => Ok(SQL_SQL_CONFORMANCE),
            119 => Ok(SQL_DATETIME_LITERALS),
            10021 => Ok(SQL_ASYNC_MODE),
            120 => Ok(SQL_BATCH_ROW_COUNT),
            121 => Ok(SQL_BATCH_SUPPORT),
            114 => Ok(SQL_CATALOG_LOCATION),
            41 => Ok(SQL_CATALOG_NAME_SEPARATOR),
            42 => Ok(SQL_CATALOG_TERM),
            92 => Ok(SQL_CATALOG_USAGE),
            122 => Ok(SQL_CONVERT_WCHAR),
            123 => Ok(SQL_CONVERT_INTERVAL_DAY_TIME),
            124 => Ok(SQL_CONVERT_INTERVAL_YEAR_MONTH),
            125 => Ok(SQL_CONVERT_WLONGVARCHAR),
            126 => Ok(SQL_CONVERT_WVARCHAR),
            127 => Ok(SQL_CREATE_ASSERTION),
            128 => Ok(SQL_CREATE_CHARACTER_SET),
            129 => Ok(SQL_CREATE_COLLATION),
            130 => Ok(SQL_CREATE_DOMAIN),
            131 => Ok(SQL_CREATE_SCHEMA),
            132 => Ok(SQL_CREATE_TABLE),
            133 => Ok(SQL_CREATE_TRANSLATION),
            134 => Ok(SQL_CREATE_VIEW),
            135 => Ok(SQL_DRIVER_HDESC),
            136 => Ok(SQL_DROP_ASSERTION),
            137 => Ok(SQL_DROP_CHARACTER_SET),
            138 => Ok(SQL_DROP_COLLATION),
            139 => Ok(SQL_DROP_DOMAIN),
            140 => Ok(SQL_DROP_SCHEMA),
            141 => Ok(SQL_DROP_TABLE),
            142 => Ok(SQL_DROP_TRANSLATION),
            143 => Ok(SQL_DROP_VIEW),
            144 => Ok(SQL_DYNAMIC_CURSOR_ATTRIBUTES1),
            145 => Ok(SQL_DYNAMIC_CURSOR_ATTRIBUTES2),
            146 => Ok(SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1),
            147 => Ok(SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2),
            148 => Ok(SQL_INDEX_KEYWORDS),
            149 => Ok(SQL_INFO_SCHEMA_VIEWS),
            150 => Ok(SQL_KEYSET_CURSOR_ATTRIBUTES1),
            151 => Ok(SQL_KEYSET_CURSOR_ATTRIBUTES2),
            10022 => Ok(SQL_MAX_ASYNC_CONCURRENT_STATEMENTS),
            152 => Ok(SQL_ODBC_INTERFACE_CONFORMANCE),
            153 => Ok(SQL_PARAM_ARRAY_ROW_COUNTS),
            154 => Ok(SQL_PARAM_ARRAY_SELECTS),
            39 => Ok(SQL_SCHEMA_TERM),
            91 => Ok(SQL_SCHEMA_USAGE),
            155 => Ok(SQL_SQL92_DATETIME_FUNCTIONS),
            156 => Ok(SQL_SQL92_FOREIGN_KEY_DELETE_RULE),
            157 => Ok(SQL_SQL92_FOREIGN_KEY_UPDATE_RULE),
            158 => Ok(SQL_SQL92_GRANT),
            159 => Ok(SQL_SQL92_NUMERIC_VALUE_FUNCTIONS),
            160 => Ok(SQL_SQL92_PREDICATES),
            161 => Ok(SQL_SQL92_RELATIONAL_JOIN_OPERATORS),
            162 => Ok(SQL_SQL92_REVOKE),
            163 => Ok(SQL_SQL92_ROW_VALUE_CONSTRUCTOR),
            164 => Ok(SQL_SQL92_STRING_FUNCTIONS),
            165 => Ok(SQL_SQL92_VALUE_EXPRESSIONS),
            166 => Ok(SQL_STANDARD_CLI_CONFORMANCE),
            167 => Ok(SQL_STATIC_CURSOR_ATTRIBUTES1),
            168 => Ok(SQL_STATIC_CURSOR_ATTRIBUTES2),
            169 => Ok(SQL_AGGREGATE_FUNCTIONS),
            170 => Ok(SQL_DDL_INDEX),
            171 => Ok(SQL_DM_VER),
            172 => Ok(SQL_INSERT_STATEMENT),
            173 => Ok(SQL_CONVERT_GUID),
            96 => Ok(SQL_UNION_STATEMENT),
            10000 => Ok(SQL_XOPEN_CLI_YEAR),
            10001 => Ok(SQL_CURSOR_SENSITIVITY),
            10002 => Ok(SQL_DESCRIBE_PARAMETER),
            10003 => Ok(SQL_CATALOG_NAME),
            10004 => Ok(SQL_COLLATION_SEQ),
            10005 => Ok(SQL_MAXIMUM_IDENTIFIER_LENGTH),
            e => Err(e),
        }
    }
}

const SQL_TXN_SERIALIZABLE: SqlUInteger = 0x00000008;

#[no_mangle]
pub extern "system" fn SQLGetInfo(
    hdbc: HDbc,
    info_type: SqlUSmallInt,
    info_value_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetInfo()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, info_type={:?}, info_value_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        hdbc, info_type, info_value_ptr, buffer_length, string_length_ptr
    );

    let dbc = check_dbc!(hdbc);

    let info = SqlGetInfo::new(dbc, info_value_ptr, buffer_length, string_length_ptr, false);
    let rc = info.get_info(info_type);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[no_mangle]
pub extern "system" fn SQLGetInfoW(
    hdbc: HDbc,
    info_type: SqlUSmallInt,
    info_value_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLGetInfoW()";
    trace!(
        "{FUNCTION_NAME} start. hdbc={:?}, info_type={:?}, info_value_ptr={:?}, buffer_length={:?}, string_length_ptr={:?}",
        hdbc, info_type, info_value_ptr, buffer_length, string_length_ptr
    );

    let dbc = check_dbc!(hdbc);

    let info = SqlGetInfo::new(dbc, info_value_ptr, buffer_length, string_length_ptr, true);
    let rc = info.get_info(info_type);

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum SqlCursorBehavior {
    SQL_CB_DELETE = 0,
    SQL_CB_CLOSE = 1,
    SQL_CB_PRESERVE = 2,
}

struct SqlGetInfo {
    dbc: Arc<TsurugiOdbcDbc>,
    info_value_ptr: SqlPointer,
    buffer_length: SqlSmallInt,
    string_length_ptr: *mut SqlSmallInt,
    wide_char: bool,
}

impl SqlGetInfo {
    fn new(
        dbc: Arc<TsurugiOdbcDbc>,
        info_value_ptr: SqlPointer,
        buffer_length: SqlSmallInt,
        string_length_ptr: *mut SqlSmallInt,
        wide_char: bool,
    ) -> SqlGetInfo {
        SqlGetInfo {
            dbc,
            info_value_ptr,
            buffer_length,
            string_length_ptr,
            wide_char,
        }
    }

    fn odbc_function_name(&self) -> &str {
        if self.wide_char {
            "SQLGetInfoW()"
        } else {
            "SQLGetInfo()"
        }
    }

    fn get_info(&self, info_type: SqlUSmallInt) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_info()";

        let dbc = &self.dbc;

        let info_type = match InfoType::try_from(info_type) {
            Ok(value) => value,
            Err(info_type) => {
                debug!(
                    "{dbc}.{FUNCTION_NAME} error. Unsupported info_type: {}",
                    info_type
                );
                let odbc_function_name = self.odbc_function_name();
                dbc.add_diag(
                    TsurugiOdbcError::GetInfoUnsupportedInfoType,
                    format!("{odbc_function_name}: Unsupported info_type {}", info_type),
                );
                return SqlReturn::SQL_ERROR;
            }
        };

        use InfoType::*;
        match info_type {
            SQL_DRIVER_NAME => self.write_string(info_type, ODBC_DRIVER_FILE_NAME),
            SQL_DRIVER_VER => self.write_string(info_type, odbc_driver_version()),
            SQL_DRIVER_ODBC_VER => self.write_string(info_type, "03.51"),
            SQL_DBMS_NAME => self.write_string(info_type, "Tsurugi"),
            SQL_DBMS_VER => self.write_string(info_type, TSURUGI_VERSION), // TODO 接続中のTsurugiのバージョン

            SQL_ACCESSIBLE_PROCEDURES => self.write_string(info_type, "N"),
            SQL_ACCESSIBLE_TABLES => self.write_string(info_type, "Y"),
            SQL_ACTIVE_ENVIRONMENTS => self.write_usmallint(info_type, 0),
            SQL_AGGREGATE_FUNCTIONS => self.write_uinteger(info_type, aggregate_functions()),
            SQL_ALTER_DOMAIN | SQL_ALTER_TABLE => self.write_uinteger(info_type, 0),
            // SQL_ASYNC_DBC_FUNCTIONS
            SQL_ASYNC_MODE => {
                self.write_uinteger(info_type, 0 /* SQL_AM_NONE */)
            }
            // SQL_ASYNC_NOTIFICATION
            // TODO SQL_BATCH_ROW_COUNT
            SQL_BATCH_SUPPORT => self.write_uinteger(info_type, 0),
            SQL_BOOKMARK_PERSISTENCE => self.write_uinteger(info_type, 0),
            SQL_CATALOG_LOCATION => self.write_usmallint(info_type, 0), // TODO カタログがサポートされたらSQL_CL_STARTを返す
            SQL_CATALOG_NAME => self.write_string(info_type, "N"), // TODO カタログがサポートされたら"Y"を返す
            SQL_CATALOG_NAME_SEPARATOR => self.write_string(info_type, "."),
            SQL_CATALOG_TERM => self.write_string(info_type, "database"),
            SQL_CATALOG_USAGE => self.write_uinteger(info_type, 0), // TODO カタログがサポートされたらビットマスクを返す
            SQL_COLLATION_SEQ => self.write_string(info_type, "UTF-8"),
            SQL_COLUMN_ALIAS => self.write_string(info_type, "Y"),
            SQL_CONCAT_NULL_BEHAVIOR => self.write_usmallint(info_type, 0 /* SQL_CB_NULL */),
            SQL_CONVERT_BIGINT
            | SQL_CONVERT_BINARY
            | SQL_CONVERT_BIT
            | SQL_CONVERT_CHAR
            | SQL_CONVERT_GUID
            | SQL_CONVERT_DATE
            | SQL_CONVERT_DECIMAL
            | SQL_CONVERT_DOUBLE
            | SQL_CONVERT_FLOAT
            | SQL_CONVERT_INTEGER
            | SQL_CONVERT_INTERVAL_YEAR_MONTH
            | SQL_CONVERT_INTERVAL_DAY_TIME
            | SQL_CONVERT_LONGVARBINARY
            | SQL_CONVERT_LONGVARCHAR
            | SQL_CONVERT_NUMERIC
            | SQL_CONVERT_REAL
            | SQL_CONVERT_SMALLINT
            | SQL_CONVERT_TIME
            | SQL_CONVERT_TIMESTAMP
            | SQL_CONVERT_TINYINT
            | SQL_CONVERT_VARBINARY
            | SQL_CONVERT_VARCHAR => self.write_uinteger(info_type, 0),
            SQL_CONVERT_FUNCTIONS => self.write_uinteger(info_type, 0),
            // TODO SQL_CORRELATION_NAME
            SQL_CREATE_ASSERTION
            | SQL_CREATE_CHARACTER_SET
            | SQL_CREATE_COLLATION
            | SQL_CREATE_DOMAIN
            | SQL_CREATE_SCHEMA
            | SQL_CREATE_TRANSLATION
            | SQL_CREATE_VIEW => self.write_uinteger(info_type, 0),
            SQL_CREATE_TABLE => {
                const SQL_CT_CREATE_TABLE: SqlUInteger = 0x00000001;
                const SQL_CT_COLUMN_DEFAULT: SqlUInteger = 0x00000400;
                self.write_uinteger(info_type, SQL_CT_CREATE_TABLE | SQL_CT_COLUMN_DEFAULT)
            }
            SQL_CURSOR_COMMIT_BEHAVIOR => {
                self.write_usmallint(info_type, SqlCursorBehavior::SQL_CB_DELETE as SqlUSmallInt)
            }
            SQL_CURSOR_ROLLBACK_BEHAVIOR => {
                self.write_usmallint(info_type, SqlCursorBehavior::SQL_CB_DELETE as SqlUSmallInt)
            }
            SQL_CURSOR_SENSITIVITY => self.write_uinteger(info_type, 1 /* SQL_INSENSITIVE  */),
            // TODO SQL_DATA_SOURCE_NAME
            SQL_DATA_SOURCE_READ_ONLY => self.write_string(info_type, "N"),
            SQL_DATABASE_NAME => self.write_string(info_type, ""), // TODO データベース名
            SQL_DATETIME_LITERALS => {
                const SQL_DL_SQL92_DATE: SqlUInteger = 0x00000001;
                const SQL_DL_SQL92_TIME: SqlUInteger = 0x00000002;
                const SQL_DL_SQL92_TIMESTAMP: SqlUInteger = 0x00000004;
                self.write_uinteger(
                    info_type,
                    SQL_DL_SQL92_DATE | SQL_DL_SQL92_TIME | SQL_DL_SQL92_TIMESTAMP,
                )
            }
            SQL_DDL_INDEX => {
                const SQL_DI_CREATE_INDEX: SqlUInteger = 0x00000001;
                const SQL_DI_DROP_INDEX: SqlUInteger = 0x00000002;
                self.write_uinteger(info_type, SQL_DI_CREATE_INDEX | SQL_DI_DROP_INDEX)
            }
            SQL_DEFAULT_TXN_ISOLATION => self.write_uinteger(info_type, SQL_TXN_SERIALIZABLE),
            // TODO SQL_DESCRIBE_PARAMETER
            SQL_DROP_ASSERTION
            | SQL_DROP_CHARACTER_SET
            | SQL_DROP_COLLATION
            | SQL_DROP_DOMAIN
            | SQL_DROP_SCHEMA
            | SQL_DROP_TRANSLATION
            | SQL_DROP_VIEW => self.write_uinteger(info_type, 0),
            SQL_DROP_TABLE => {
                const SQL_DT_DROP_TABLE: SqlUInteger = 0x00000001;
                self.write_uinteger(info_type, SQL_DT_DROP_TABLE)
            }
            SQL_DYNAMIC_CURSOR_ATTRIBUTES1 | SQL_DYNAMIC_CURSOR_ATTRIBUTES2 => {
                self.write_uinteger(info_type, 0)
            }
            SQL_EXPRESSIONS_IN_ORDERBY => self.write_string(info_type, "Y"),
            SQL_FILE_USAGE => self.write_usmallint(info_type, 0 /* SQL_FILE_NOT_SUPPORTED */),
            SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1 | SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2 => {
                self.write_uinteger(info_type, 0)
            }
            SQL_GETDATA_EXTENSIONS => self.write_uinteger(info_type, 0),
            SQL_GROUP_BY => {
                self.write_usmallint(info_type, 1 /* SQL_GB_GROUP_BY_EQUALS_SELECT */)
            }
            SQL_IDENTIFIER_CASE => {
                self.write_usmallint(info_type, 3 /* SQL_IC_SENSITIVE */)
            }
            SQL_IDENTIFIER_QUOTE_CHAR => self.write_string(info_type, "\""),
            SQL_INDEX_KEYWORDS => {
                const SQL_IK_ASC: SqlUInteger = 0x00000001;
                const SQL_IK_DESC: SqlUInteger = 0x00000002;
                const SQL_IK_ALL: SqlUInteger = SQL_IK_ASC | SQL_IK_DESC;
                self.write_uinteger(info_type, SQL_IK_ALL)
            }
            SQL_INFO_SCHEMA_VIEWS => self.write_uinteger(info_type, 0),
            SQL_INSERT_STATEMENT => {
                const SQL_IS_INSERT_LITERALS: SqlUInteger = 0x00000001;
                const SQL_IS_INSERT_SEARCHED: SqlUInteger = 0x00000002;
                // const SQL_IS_SELECT_INTO: SqlUInteger = 0x00000004;
                self.write_uinteger(info_type, SQL_IS_INSERT_LITERALS | SQL_IS_INSERT_SEARCHED)
            }
            SQL_KEYSET_CURSOR_ATTRIBUTES1 | SQL_KEYSET_CURSOR_ATTRIBUTES2 => {
                self.write_uinteger(info_type, 0)
            }
            SQL_INTEGRITY => self.write_string(info_type, "N"),
            // TODO SQL_KEYWORDS
            SQL_LIKE_ESCAPE_CLAUSE => self.write_string(info_type, "Y"),
            SQL_MAX_ASYNC_CONCURRENT_STATEMENTS
            | SQL_MAX_BINARY_LITERAL_LEN
            | SQL_MAX_CHAR_LITERAL_LEN
            | SQL_MAXIMUM_INDEX_SIZE
            | SQL_MAXIMUM_ROW_SIZE
            | SQL_MAXIMUM_STATEMENT_LENGTH => self.write_uinteger(info_type, 0),
            SQL_MAXIMUM_CATALOG_NAME_LENGTH
            | SQL_MAXIMUM_COLUMN_NAME_LENGTH
            | SQL_MAXIMUM_COLUMNS_IN_GROUP_BY
            | SQL_MAXIMUM_COLUMNS_IN_INDEX
            | SQL_MAXIMUM_COLUMNS_IN_ORDER_BY
            | SQL_MAXIMUM_COLUMNS_IN_SELECT
            | SQL_MAX_COLUMNS_IN_TABLE
            | SQL_MAXIMUM_CONCURRENT_ACTIVITIES
            | SQL_MAXIMUM_CURSOR_NAME_LENGTH
            | SQL_MAXIMUM_DRIVER_CONNECTIONS
            | SQL_MAXIMUM_IDENTIFIER_LENGTH
            | SQL_MAX_PROCEDURE_NAME_LEN
            | SQL_MAXIMUM_SCHEMA_NAME_LENGTH
            | SQL_MAX_TABLE_NAME_LEN
            | SQL_MAXIMUM_TABLES_IN_SELECT
            | SQL_MAXIMUM_USER_NAME_LENGTH => self.write_usmallint(info_type, 0),
            SQL_MULT_RESULT_SETS => self.write_string(info_type, "N"),
            SQL_MULTIPLE_ACTIVE_TXN => self.write_string(info_type, "Y"),
            SQL_NEED_LONG_DATA_LEN => self.write_string(info_type, "N"),
            SQL_NON_NULLABLE_COLUMNS => {
                self.write_usmallint(info_type, 1 /* SQL_NNC_NON_NULL */)
            }
            SQL_NULL_COLLATION => {
                self.write_usmallint(info_type, 1 /* SQL_NC_LOW */)
            }
            SQL_NUMERIC_FUNCTIONS => self.write_uinteger(info_type, numeric_functions()),
            SQL_ODBC_INTERFACE_CONFORMANCE => {
                self.write_uinteger(info_type, 1 /* SQL_OIC_CORE */)
            }
            SQL_ORDER_BY_COLUMNS_IN_SELECT => self.write_string(info_type, "N"),
            SQL_PROCEDURES => self.write_string(info_type, "N"),
            SQL_SCHEMA_TERM => self.write_string(info_type, "schema"),
            SQL_SCHEMA_USAGE => self.write_uinteger(info_type, 0), // TODO スキーマがサポートされたらビットマスクを返す
            // TODO SQL_SCROLL_OPTIONS
            SQL_SEARCH_PATTERN_ESCAPE => self.write_string(info_type, ""),
            // TODO SQL_SERVER_NAME
            SQL_SPECIAL_CHARACTERS => self.write_string(info_type, ""),
            SQL_SQL_CONFORMANCE => self.write_uinteger(info_type, 1 /* SQL_SC_SQL92_ENTRY */),
            SQL_SQL92_DATETIME_FUNCTIONS => {
                self.write_uinteger(info_type, sql92_datetime_functions())
            }
            SQL_SQL92_FOREIGN_KEY_DELETE_RULE | SQL_SQL92_FOREIGN_KEY_UPDATE_RULE => {
                self.write_uinteger(info_type, 0)
            }
            SQL_SQL92_GRANT => self.write_uinteger(info_type, 0),
            SQL_SQL92_NUMERIC_VALUE_FUNCTIONS => {
                self.write_uinteger(info_type, numeric_value_functions())
            }
            SQL_SQL92_PREDICATES => self.write_uinteger(info_type, predicates()),
            SQL_SQL92_RELATIONAL_JOIN_OPERATORS => {
                self.write_uinteger(info_type, relational_join_operators())
            }
            SQL_SQL92_REVOKE => self.write_uinteger(info_type, 0),
            SQL_SQL92_ROW_VALUE_CONSTRUCTOR => self.write_uinteger(info_type, 0),
            SQL_SQL92_STRING_FUNCTIONS => self.write_uinteger(info_type, sql92_string_functions()),
            SQL_SQL92_VALUE_EXPRESSIONS => self.write_uinteger(info_type, value_expressions()),
            SQL_STANDARD_CLI_CONFORMANCE => self.write_uinteger(info_type, 0),
            SQL_STATIC_CURSOR_ATTRIBUTES1 | SQL_STATIC_CURSOR_ATTRIBUTES2 => {
                self.write_uinteger(info_type, 0)
            }
            SQL_STRING_FUNCTIONS => self.write_uinteger(info_type, string_functions()),
            SQL_SUBQUERIES => self.write_uinteger(info_type, 0), // TODO サブクエリー
            SQL_SYSTEM_FUNCTIONS => self.write_uinteger(info_type, system_functions()),
            SQL_TABLE_TERM => self.write_string(info_type, "table"),
            SQL_TIMEDATE_ADD_INTERVALS => self.write_uinteger(info_type, 0),
            SQL_TIMEDATE_DIFF_INTERVALS => self.write_uinteger(info_type, 0),
            SQL_TIMEDATE_FUNCTIONS => self.write_uinteger(info_type, datetime_functions()),
            SQL_TRANSACTION_CAPABLE => self.write_usmallint(info_type, 2 /* SQL_TC_ALL */),
            SQL_TRANSACTION_ISOLATION_OPTION => {
                self.write_uinteger(info_type, SQL_TXN_SERIALIZABLE)
            }
            SQL_UNION_STATEMENT => {
                self.write_uinteger(info_type, 3 /* SQL_U_UNION|SQL_U_UNION_ALL */)
            }
            _ => {
                debug!(
                    "{dbc}.{FUNCTION_NAME} error. Unsupported info_type {:?}",
                    info_type
                );
                let odbc_function_name = self.odbc_function_name();
                dbc.add_diag(
                    TsurugiOdbcError::GetInfoUnsupportedInfoType,
                    format!(
                        "{odbc_function_name}: Unsupported info_type {:?}",
                        info_type
                    ),
                );
                SqlReturn::SQL_ERROR
            }
        }
    }

    fn write_string(&self, info_type: InfoType, value: &str) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_info().write_string()";
        let dbc = &self.dbc;
        debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", info_type, value);

        if self.wide_char {
            write_wchar_bytes(
                "SQLGetInfoW.info_value_ptr",
                value,
                self.info_value_ptr as *mut SqlWChar,
                self.buffer_length,
                self.string_length_ptr,
                Some(&self.dbc.diag_collection()),
            )
        } else {
            write_char(
                "SQLGetInfo.info_value_ptr",
                value,
                self.info_value_ptr as *mut SqlChar,
                self.buffer_length,
                self.string_length_ptr,
                Some(&self.dbc.diag_collection()),
            )
        }
    }

    fn write_usmallint(&self, info_type: InfoType, value: SqlUSmallInt) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_info().write_usmallint()";
        let dbc = &self.dbc;
        debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", info_type, value);

        let int_ptr = self.info_value_ptr as *mut SqlUSmallInt;
        unsafe {
            *int_ptr = value;
        }
        SqlReturn::SQL_SUCCESS
    }

    fn write_uinteger(&self, info_type: InfoType, value: SqlUInteger) -> SqlReturn {
        const FUNCTION_NAME: &str = "get_info().write_uinteger()";
        let dbc = &self.dbc;
        debug!("{dbc}.{FUNCTION_NAME}: {:?}={}", info_type, value);

        let int_ptr = self.info_value_ptr as *mut SqlUInteger;
        unsafe {
            *int_ptr = value;
        }
        SqlReturn::SQL_SUCCESS
    }
}

fn aggregate_functions() -> SqlUInteger {
    const SQL_AF_AVG: SqlUInteger = 0x00000001;
    const SQL_AF_COUNT: SqlUInteger = 0x00000002;
    const SQL_AF_MAX: SqlUInteger = 0x00000004;
    const SQL_AF_MIN: SqlUInteger = 0x00000008;
    const SQL_AF_SUM: SqlUInteger = 0x00000010;
    const SQL_AF_DISTINCT: SqlUInteger = 0x00000020;
    const SQL_AF_ALL: SqlUInteger = 0x00000040;

    SQL_AF_AVG | SQL_AF_COUNT | SQL_AF_MAX | SQL_AF_MIN | SQL_AF_SUM | SQL_AF_DISTINCT | SQL_AF_ALL
}

#[allow(dead_code)]
fn numeric_functions() -> SqlUInteger {
    const SQL_FN_NUM_ABS: SqlUInteger = 0x00000001;
    const SQL_FN_NUM_ACOS: SqlUInteger = 0x00000002;
    const SQL_FN_NUM_ASIN: SqlUInteger = 0x00000004;
    const SQL_FN_NUM_ATAN: SqlUInteger = 0x00000008;
    const SQL_FN_NUM_ATAN2: SqlUInteger = 0x00000010;
    const SQL_FN_NUM_CEILING: SqlUInteger = 0x00000020;
    const SQL_FN_NUM_COS: SqlUInteger = 0x00000040;
    const SQL_FN_NUM_COT: SqlUInteger = 0x00000080;
    const SQL_FN_NUM_EXP: SqlUInteger = 0x00000100;
    const SQL_FN_NUM_FLOOR: SqlUInteger = 0x00000200;
    const SQL_FN_NUM_LOG: SqlUInteger = 0x00000400;
    const SQL_FN_NUM_MOD: SqlUInteger = 0x00000800;
    const SQL_FN_NUM_SIGN: SqlUInteger = 0x00001000;
    const SQL_FN_NUM_SIN: SqlUInteger = 0x00002000;
    const SQL_FN_NUM_SQRT: SqlUInteger = 0x00004000;
    const SQL_FN_NUM_TAN: SqlUInteger = 0x00008000;
    const SQL_FN_NUM_PI: SqlUInteger = 0x00010000;
    const SQL_FN_NUM_RAND: SqlUInteger = 0x00020000;
    const SQL_FN_NUM_DEGREES: SqlUInteger = 0x00040000;
    const SQL_FN_NUM_LOG10: SqlUInteger = 0x00080000;
    const SQL_FN_NUM_POWER: SqlUInteger = 0x00100000;
    const SQL_FN_NUM_RADIANS: SqlUInteger = 0x00200000;
    const SQL_FN_NUM_ROUND: SqlUInteger = 0x00400000;
    const SQL_FN_NUM_TRUNCATE: SqlUInteger = 0x00800000;

    SQL_FN_NUM_ABS | SQL_FN_NUM_CEILING | SQL_FN_NUM_FLOOR | SQL_FN_NUM_MOD | SQL_FN_NUM_ROUND
}

#[allow(dead_code)]
fn string_functions() -> SqlUInteger {
    const SQL_FN_STR_CONCAT: SqlUInteger = 0x00000001;
    const SQL_FN_STR_INSERT: SqlUInteger = 0x00000002;
    const SQL_FN_STR_LEFT: SqlUInteger = 0x00000004;
    const SQL_FN_STR_LTRIM: SqlUInteger = 0x00000008;
    const SQL_FN_STR_LENGTH: SqlUInteger = 0x00000010;
    const SQL_FN_STR_LOCATE: SqlUInteger = 0x00000020;
    const SQL_FN_STR_LCASE: SqlUInteger = 0x00000040;
    const SQL_FN_STR_REPEAT: SqlUInteger = 0x00000080;
    const SQL_FN_STR_REPLACE: SqlUInteger = 0x00000100;
    const SQL_FN_STR_RIGHT: SqlUInteger = 0x00000200;
    const SQL_FN_STR_RTRIM: SqlUInteger = 0x00000400;
    const SQL_FN_STR_SUBSTRING: SqlUInteger = 0x00000800;
    const SQL_FN_STR_UCASE: SqlUInteger = 0x00001000;
    const SQL_FN_STR_ASCII: SqlUInteger = 0x00002000;
    const SQL_FN_STR_CHAR: SqlUInteger = 0x00004000;
    const SQL_FN_STR_DIFFERENCE: SqlUInteger = 0x00008000;
    const SQL_FN_STR_LOCATE_2: SqlUInteger = 0x00010000;
    const SQL_FN_STR_SOUNDEX: SqlUInteger = 0x00020000;
    const SQL_FN_STR_SPACE: SqlUInteger = 0x00040000;
    const SQL_FN_STR_BIT_LENGTH: SqlUInteger = 0x00080000;
    const SQL_FN_STR_CHAR_LENGTH: SqlUInteger = 0x00100000;
    const SQL_FN_STR_CHARACTER_LENGTH: SqlUInteger = 0x00200000;
    const SQL_FN_STR_OCTET_LENGTH: SqlUInteger = 0x00400000;
    const SQL_FN_STR_POSITION: SqlUInteger = 0x00800000;

    SQL_FN_STR_CHAR_LENGTH
        | SQL_FN_STR_CHARACTER_LENGTH
        | SQL_FN_STR_OCTET_LENGTH
        | SQL_FN_STR_POSITION
        | SQL_FN_STR_SUBSTRING
}

#[allow(dead_code)]
fn sql92_string_functions() -> SqlUInteger {
    const SQL_SSF_CONVERT: SqlUInteger = 0x00000001;
    const SQL_SSF_LOWER: SqlUInteger = 0x00000002;
    const SQL_SSF_UPPER: SqlUInteger = 0x00000004;
    const SQL_SSF_SUBSTRING: SqlUInteger = 0x00000008;
    const SQL_SSF_TRANSLATE: SqlUInteger = 0x00000010;
    const SQL_SSF_TRIM_BOTH: SqlUInteger = 0x00000020;
    const SQL_SSF_TRIM_LEADING: SqlUInteger = 0x00000040;
    const SQL_SSF_TRIM_TRAILING: SqlUInteger = 0x00000080;

    SQL_SSF_LOWER | SQL_SSF_SUBSTRING | SQL_SSF_UPPER
}

#[allow(dead_code)]
fn numeric_value_functions() -> SqlUInteger {
    const SQL_SNVF_BIT_LENGTH: SqlUInteger = 0x00000001;
    const SQL_SNVF_CHAR_LENGTH: SqlUInteger = 0x00000002;
    const SQL_SNVF_CHARACTER_LENGTH: SqlUInteger = 0x00000004;
    const SQL_SNVF_EXTRACT: SqlUInteger = 0x00000008;
    const SQL_SNVF_OCTET_LENGTH: SqlUInteger = 0x00000010;
    const SQL_SNVF_POSITION: SqlUInteger = 0x00000020;

    SQL_SNVF_CHAR_LENGTH | SQL_SNVF_CHARACTER_LENGTH | SQL_SNVF_OCTET_LENGTH | SQL_SNVF_POSITION
}

#[allow(dead_code)]
fn datetime_functions() -> SqlUInteger {
    const SQL_FN_TD_NOW: SqlUInteger = 0x00000001;
    const SQL_FN_TD_CURDATE: SqlUInteger = 0x00000002;
    const SQL_FN_TD_DAYOFMONTH: SqlUInteger = 0x00000004;
    const SQL_FN_TD_DAYOFWEEK: SqlUInteger = 0x00000008;
    const SQL_FN_TD_DAYOFYEAR: SqlUInteger = 0x00000010;
    const SQL_FN_TD_MONTH: SqlUInteger = 0x00000020;
    const SQL_FN_TD_QUARTER: SqlUInteger = 0x00000040;
    const SQL_FN_TD_WEEK: SqlUInteger = 0x00000080;
    const SQL_FN_TD_YEAR: SqlUInteger = 0x00000100;
    const SQL_FN_TD_CURTIME: SqlUInteger = 0x00000200;
    const SQL_FN_TD_HOUR: SqlUInteger = 0x00000400;
    const SQL_FN_TD_MINUTE: SqlUInteger = 0x00000800;
    const SQL_FN_TD_SECOND: SqlUInteger = 0x00001000;
    const SQL_FN_TD_TIMESTAMPADD: SqlUInteger = 0x00002000;
    const SQL_FN_TD_TIMESTAMPDIFF: SqlUInteger = 0x00004000;
    const SQL_FN_TD_DAYNAME: SqlUInteger = 0x00008000;
    const SQL_FN_TD_MONTHNAME: SqlUInteger = 0x00010000;
    const SQL_FN_TD_CURRENT_DATE: SqlUInteger = 0x00020000;
    const SQL_FN_TD_CURRENT_TIME: SqlUInteger = 0x00040000;
    const SQL_FN_TD_CURRENT_TIMESTAMP: SqlUInteger = 0x00080000;
    const SQL_FN_TD_EXTRACT: SqlUInteger = 0x00100000;

    SQL_FN_TD_CURRENT_DATE | SQL_FN_TD_CURRENT_TIMESTAMP
}

#[allow(dead_code)]
fn sql92_datetime_functions() -> SqlUInteger {
    const SQL_SDF_CURRENT_DATE: SqlUInteger = 0x00000001;
    const SQL_SDF_CURRENT_TIME: SqlUInteger = 0x00000002;
    const SQL_SDF_CURRENT_TIMESTAMP: SqlUInteger = 0x00000004;

    SQL_SDF_CURRENT_DATE | SQL_SDF_CURRENT_TIMESTAMP
}

#[allow(dead_code)]
fn system_functions() -> SqlUInteger {
    const SQL_FN_SYS_USERNAME: SqlUInteger = 0x00000001;
    const SQL_FN_SYS_DBNAME: SqlUInteger = 0x00000002;
    const SQL_FN_SYS_IFNULL: SqlUInteger = 0x00000004;

    0
}

#[allow(dead_code)]
fn predicates() -> SqlUInteger {
    const SQL_SP_EXISTS: SqlUInteger = 0x00000001;
    const SQL_SP_ISNOTNULL: SqlUInteger = 0x00000002;
    const SQL_SP_ISNULL: SqlUInteger = 0x00000004;
    const SQL_SP_MATCH_FULL: SqlUInteger = 0x00000008;
    const SQL_SP_MATCH_PARTIAL: SqlUInteger = 0x00000010;
    const SQL_SP_MATCH_UNIQUE_FULL: SqlUInteger = 0x00000020;
    const SQL_SP_MATCH_UNIQUE_PARTIAL: SqlUInteger = 0x00000040;
    const SQL_SP_OVERLAPS: SqlUInteger = 0x00000080;
    const SQL_SP_UNIQUE: SqlUInteger = 0x00000100;
    const SQL_SP_LIKE: SqlUInteger = 0x00000200;
    const SQL_SP_IN: SqlUInteger = 0x00000400;
    const SQL_SP_BETWEEN: SqlUInteger = 0x00000800;
    const SQL_SP_COMPARISON: SqlUInteger = 0x00001000;
    const SQL_SP_QUANTIFIED_COMPARISON: SqlUInteger = 0x00002000;

    SQL_SP_ISNOTNULL | SQL_SP_ISNULL | SQL_SP_LIKE | SQL_SP_IN | SQL_SP_BETWEEN | SQL_SP_COMPARISON
}

fn value_expressions() -> SqlUInteger {
    const SQL_SVE_CASE: SqlUInteger = 0x00000001;
    const SQL_SVE_CAST: SqlUInteger = 0x00000002;
    const SQL_SVE_COALESCE: SqlUInteger = 0x00000004;
    const SQL_SVE_NULLIF: SqlUInteger = 0x00000008;

    SQL_SVE_CASE | SQL_SVE_CAST | SQL_SVE_COALESCE | SQL_SVE_NULLIF
}

#[allow(dead_code)]
fn relational_join_operators() -> SqlUInteger {
    const SQL_SRJO_CORRESPONDING_CLAUSE: SqlUInteger = 0x00000001;
    const SQL_SRJO_CROSS_JOIN: SqlUInteger = 0x00000002;
    const SQL_SRJO_EXCEPT_JOIN: SqlUInteger = 0x00000004;
    const SQL_SRJO_FULL_OUTER_JOIN: SqlUInteger = 0x00000008;
    const SQL_SRJO_INNER_JOIN: SqlUInteger = 0x00000010;
    const SQL_SRJO_INTERSECT_JOIN: SqlUInteger = 0x00000020;
    const SQL_SRJO_LEFT_OUTER_JOIN: SqlUInteger = 0x00000040;
    const SQL_SRJO_NATURAL_JOIN: SqlUInteger = 0x00000080;
    const SQL_SRJO_RIGHT_OUTER_JOIN: SqlUInteger = 0x00000100;
    const SQL_SRJO_UNION_JOIN: SqlUInteger = 0x00000200;

    SQL_SRJO_INNER_JOIN
        | SQL_SRJO_LEFT_OUTER_JOIN
        | SQL_SRJO_RIGHT_OUTER_JOIN
        | SQL_SRJO_FULL_OUTER_JOIN
        | SQL_SRJO_CROSS_JOIN
}
