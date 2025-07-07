use log::{debug, trace};

use crate::{
    check_stmt,
    ctype::{CDataType, SqlLen, SqlPointer, SqlReturn, SqlSmallInt, SqlUSmallInt},
    handle::{
        diag::TsurugiOdbcError,
        hstmt::{HStmt, TsurugiOdbcStmt},
    },
};

#[no_mangle]
pub extern "system" fn SQLBindCol(
    hstmt: HStmt,
    column_number: SqlUSmallInt,
    target_type: SqlSmallInt,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLBindCol()";
    trace!(
        "{FUNCTION_NAME} start. hstmt={:?}, column_number={:?}, target_type={:?}, target_value_ptr={:?}, buffer_length={:?}, str_len_or_ind_ptr={:?}",
        hstmt,
        column_number,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr
    );

    let stmt = check_stmt!(hstmt);
    let mut stmt = stmt.lock().unwrap();
    stmt.clear_diag();

    let rc = bind_col(
        &mut stmt,
        column_number,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    );

    trace!("{FUNCTION_NAME} end. rc={:?}", rc);
    rc
}

fn bind_col(
    stmt: &mut TsurugiOdbcStmt,
    column_number: SqlUSmallInt,
    target_type: SqlSmallInt,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
) -> SqlReturn {
    const FUNCTION_NAME: &str = "SQLBindCol()";

    let target_type = match CDataType::try_from(target_type) {
        Ok(value) => value,
        Err(e) => {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. Unsupported CDataType {:?}",
                target_type
            );
            stmt.add_diag(e, format!("Unsupported CDataType {:?}", target_type));
            return SqlReturn::SQL_ERROR;
        }
    };

    if str_len_or_ind_ptr.is_null() {
        debug!("{stmt}.{FUNCTION_NAME} error. str_len_or_ind_ptr is null");
        stmt.add_diag(TsurugiOdbcError::BindColError, "str_len_or_ind_ptr is null");
        return SqlReturn::SQL_ERROR;
    };

    let bind_column = TsurugiOdbcBindColumn::new(
        column_number,
        target_type,
        target_value_ptr,
        buffer_length,
        str_len_or_ind_ptr,
    );

    stmt.set_bind_column(bind_column);

    SqlReturn::SQL_SUCCESS
}

#[derive(Debug)]
pub(crate) struct TsurugiOdbcBindColumn {
    column_number: SqlUSmallInt,
    target_type: CDataType,
    target_value_ptr: SqlPointer,
    buffer_length: SqlLen,
    str_len_or_ind_ptr: *mut SqlLen,
}

impl TsurugiOdbcBindColumn {
    fn new(
        column_number: SqlUSmallInt,
        target_type: CDataType,
        target_value_ptr: SqlPointer,
        buffer_length: SqlLen,
        str_len_or_ind_ptr: *mut SqlLen,
    ) -> TsurugiOdbcBindColumn {
        TsurugiOdbcBindColumn {
            column_number,
            target_type,
            target_value_ptr,
            buffer_length,
            str_len_or_ind_ptr,
        }
    }

    pub(crate) fn column_number(&self) -> SqlUSmallInt {
        self.column_number
    }

    pub(crate) fn target_type(&self) -> CDataType {
        self.target_type
    }

    pub(crate) fn target_value_ptr(&self) -> SqlPointer {
        self.target_value_ptr
    }

    pub(crate) fn buffer_length(&self) -> SqlLen {
        self.buffer_length
    }

    pub(crate) fn str_len_or_ind_ptr(&self) -> *mut SqlLen {
        self.str_len_or_ind_ptr
    }
}
