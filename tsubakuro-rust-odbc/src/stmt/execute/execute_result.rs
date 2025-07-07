use tsubakuro_rust_core::prelude::*;

use crate::{
    ctype::{CDataType, SqlLen, SqlPointer, SqlReturn, SqlUSmallInt},
    handle::hstmt::TsurugiOdbcStmt,
    stmt::{describe_col::TsurugiOdbcDescribeColumn, TsurugiOdbcStatementProcessor},
};

pub(super) struct TsurugiOdbcSqlExecuteResult {
    execute_result: SqlExecuteResult,
}

impl TsurugiOdbcSqlExecuteResult {
    pub(super) fn new(execute_result: SqlExecuteResult) -> TsurugiOdbcSqlExecuteResult {
        TsurugiOdbcSqlExecuteResult { execute_result }
    }
}

impl TsurugiOdbcStatementProcessor for TsurugiOdbcSqlExecuteResult {
    fn number_of_columns(&self) -> SqlUSmallInt {
        0
    }

    fn describe_column(
        &self,
        _column_index: SqlUSmallInt,
    ) -> Result<TsurugiOdbcDescribeColumn, SqlReturn> {
        unreachable!()
    }

    fn row_count(&self) -> SqlLen {
        self.execute_result.rows() as SqlLen
    }

    fn fetch(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        SqlReturn::SQL_NO_DATA
    }

    fn get_data(
        &mut self,
        _stmt: &TsurugiOdbcStmt,
        _column_index: SqlUSmallInt,
        _target_type: CDataType,
        _target_value_ptr: SqlPointer,
        _buffer_length: SqlLen,
        _str_len_or_ind_ptr: *mut SqlLen,
    ) -> SqlReturn {
        SqlReturn::SQL_NO_DATA
    }

    fn dispose(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        SqlReturn::SQL_SUCCESS
    }
}
