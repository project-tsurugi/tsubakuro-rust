use crate::{
    ctype::{CDataType, SqlLen, SqlPointer, SqlReturn, SqlUSmallInt},
    handle::hstmt::TsurugiOdbcStmt,
    stmt::describe_col::TsurugiOdbcDescribeColumn,
};

pub mod bind_col;
pub mod bind_parameter;
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

    fn get_data(
        &mut self,
        stmt: &TsurugiOdbcStmt,
        column_index: SqlUSmallInt,
        target_type: CDataType,
        target_value_ptr: SqlPointer,
        buffer_length: SqlLen,
        str_len_or_ind_ptr: *mut SqlLen,
    ) -> SqlReturn;

    fn dispose(&mut self, stmt: &mut TsurugiOdbcStmt) -> SqlReturn;
}
