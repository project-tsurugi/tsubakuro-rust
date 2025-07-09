use tsubakuro_rust_core::prelude::*;

use crate::{
    ctype::{SqlLen, SqlReturn, SqlUSmallInt},
    handle::hstmt::TsurugiOdbcStmt,
    stmt::{
        describe_col::TsurugiOdbcDescribeColumn, get_data::TsurugiOdbcGetDataArguments,
        TsurugiOdbcStatementProcessor,
    },
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
        _arg: &TsurugiOdbcGetDataArguments,
    ) -> SqlReturn {
        SqlReturn::SQL_NO_DATA
    }

    fn dispose(&mut self, _stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        SqlReturn::SQL_SUCCESS
    }
}
