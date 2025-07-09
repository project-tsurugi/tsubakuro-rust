use crate::ctype::{sql_date_struct::SqlDateStruct, sql_timestamp_struct::SqlTimestampStruct};

use super::*;

pub(crate) fn get_data_date(
    stmt: &TsurugiOdbcStmt,
    arg: GetDataArguments,
    value: time::Date,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_date()";

    if let Err(rc) = check_target_value_ptr(FUNCTION_NAME, stmt, &arg) {
        return rc;
    }

    use CDataType::*;
    let target_type = arg.target_type;
    match target_type {
        SQL_C_TYPE_DATE | SQL_C_DATE => {
            let value = SqlDateStruct::from(value);
            write_date_struct(arg, value)
        }
        SQL_C_TYPE_TIMESTAMP | SQL_C_TIMESTAMP => {
            let value = SqlTimestampStruct::from(value);
            write_timestamp_struct(arg, value)
        }
        SQL_C_CHAR | SQL_C_WCHAR => {
            let value = value.to_string();
            do_get_data_string(stmt, arg, value)
        }
        _ => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported target type {:?}",
                target_type
            );
            stmt.add_diag(
                TsurugiOdbcError::GetDataUnsupportedTargetType,
                format!("Unsupported target type {:?}", target_type),
            );
            SqlReturn::SQL_ERROR
        }
    }
}
