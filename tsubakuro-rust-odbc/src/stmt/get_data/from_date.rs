use crate::ctype::{sql_date_struct::SqlDateStruct, sql_timestamp_struct::SqlTimestampStruct};

use super::*;

pub(crate) fn get_data_date(
    stmt: &TsurugiOdbcStmt,
    value: time::Date,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "get_data_date()";

    if target_value_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. target_value_ptr is null");
        stmt.add_diag(
            TsurugiOdbcError::InvalidValuePtr,
            "target_value_ptr is null",
        );
        return SqlReturn::SQL_ERROR;
    }

    match target_type {
        CDataType::SQL_C_TYPE_DATE | CDataType::SQL_C_DATE => {
            let value = SqlDateStruct::from(value);
            write_date_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        CDataType::SQL_C_TYPE_TIMESTAMP | CDataType::SQL_C_TIMESTAMP => {
            let value = SqlTimestampStruct::from(value);
            write_timestamp_struct(value, target_value_ptr, str_len_or_ind_ptr)
        }
        CDataType::SQL_C_CHAR | CDataType::SQL_C_WCHAR => {
            let value = value.to_string();
            do_get_data_string(
                stmt,
                &value,
                target_type,
                target_value_ptr,
                buffer_length,
                str_len_or_ind_ptr,
            )
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
