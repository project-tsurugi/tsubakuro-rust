use crate::{
    ctype::{SqlLen, SqlReturn, SqlUSmallInt},
    handle::hstmt::TsurugiOdbcStmt,
    stmt::{describe_col::TsurugiOdbcDescribeColumn, get_data::TsurugiOdbcGetDataArguments},
};

pub mod bind_col;
pub mod bind_parameter;
pub mod col_attribute;
pub mod columns;
pub mod describe_col;
pub mod exec_direct;
pub mod execute;
pub mod fetch;
pub mod get_data;
pub mod get_type_info;
pub mod num_result_cols;
pub mod prepare;
pub mod primary_keys;
pub mod row_count;
pub mod stmt_attr;
pub mod tables;

pub(crate) trait TsurugiOdbcStatementProcessor {
    fn number_of_columns(&self) -> SqlUSmallInt;

    fn describe_column(
        &self,
        column_index: SqlUSmallInt,
    ) -> Result<TsurugiOdbcDescribeColumn, SqlReturn>;

    fn row_count(&self) -> SqlLen;

    fn fetch(&mut self, stmt: &mut TsurugiOdbcStmt) -> SqlReturn;

    fn get_data(&mut self, stmt: &TsurugiOdbcStmt, arg: &TsurugiOdbcGetDataArguments) -> SqlReturn;

    fn dispose(&mut self, stmt: &mut TsurugiOdbcStmt) -> SqlReturn;
}
