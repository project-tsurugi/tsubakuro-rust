use log::{debug, warn};
use tsubakuro_rust_core::prelude::*;

use crate::{
    ctype::{SqlDataType, SqlLen, SqlNullable::SQL_NULLABLE_UNKNOWN, SqlReturn, SqlUSmallInt},
    handle::{diag::TsurugiOdbcError, hstmt::TsurugiOdbcStmt},
    stmt::{describe_col::TsurugiOdbcDescribeColumn, get_data::*, TsurugiOdbcStatementProcessor},
};

pub(super) struct TsurugiOdbcQueryResult {
    query_result: SqlQueryResult,
    close_ps: bool,
    column_index: isize,
    closed: bool,
}

impl TsurugiOdbcQueryResult {
    pub(super) fn new(query_result: SqlQueryResult, close_ps: bool) -> TsurugiOdbcQueryResult {
        TsurugiOdbcQueryResult {
            query_result,
            close_ps,
            column_index: -1,
            closed: false,
        }
    }
}

impl TsurugiOdbcStatementProcessor for TsurugiOdbcQueryResult {
    fn number_of_columns(&self) -> SqlUSmallInt {
        match self.query_result.get_metadata() {
            Some(metadata) => metadata.columns().len() as SqlUSmallInt,
            None => 0,
        }
    }

    fn describe_column(
        &self,
        column_index: SqlUSmallInt,
    ) -> Result<TsurugiOdbcDescribeColumn, SqlReturn> {
        let column = &self.query_result.get_metadata().unwrap().columns()[column_index as usize];
        let column = TsurugiOdbcDescribeColumn::new(
            column.name(),
            SqlDataType::from(column),
            0, // TODO SqlQueryResult column_size
            0, // TODO SqlQueryResult decimal_digits
            SQL_NULLABLE_UNKNOWN,
        );
        Ok(column)
    }

    fn row_count(&self) -> SqlLen {
        0 // unsupported
    }

    fn fetch(&mut self, stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        const FUNCTION_NAME: &str = "TsurugiOdbcQueryResult.fetch()";

        let runtime = stmt.runtime();

        let next_row = match runtime.block_on(self.query_result.next_row()) {
            Ok(next) => next,
            Err(e) => {
                warn!("{stmt}.{FUNCTION_NAME}: next_row() error. {:?}", e);
                stmt.add_diag(
                    TsurugiOdbcError::SqlQueryResultNextRowError,
                    format!("next_row error. {}", e),
                );
                return SqlReturn::SQL_ERROR;
            }
        };

        if next_row {
            self.column_index = -1;
            SqlReturn::SQL_SUCCESS
        } else {
            let rc = self.close(stmt, true);
            if rc.is_success() {
                SqlReturn::SQL_NO_DATA
            } else {
                rc
            }
        }
    }

    fn get_data(&mut self, stmt: &TsurugiOdbcStmt, arg: &TsurugiOdbcGetDataArguments) -> SqlReturn {
        const FUNCTION_NAME: &str = "TsurugiOdbcQueryResult.get_data()";

        let column_index = arg.column_index();
        let next_index = self.column_index + 1;
        if column_index < next_index {
            debug!(
                "{stmt}.{FUNCTION_NAME} error: Already fetched. column_index={}, next_index={}",
                column_index, next_index
            );
            stmt.add_diag(
                TsurugiOdbcError::SqlQueryResultFetchError,
                format!(
                    "Already fetched. column_number={}, next_number={}",
                    column_index + 1,
                    next_index + 1
                ),
            );
            return SqlReturn::SQL_ERROR;
        }

        let runtime = stmt.runtime();
        let mut next_column;
        loop {
            next_column = match runtime.block_on(self.query_result.next_column()) {
                Ok(next) => next,
                Err(e) => {
                    warn!("{stmt}.{FUNCTION_NAME}: next_column() error. {:?}", e);
                    stmt.add_diag(
                        TsurugiOdbcError::SqlQueryResultNextColumnError,
                        format!("next_column error. {}", e),
                    );
                    return SqlReturn::SQL_ERROR;
                }
            };
            if next_column {
                self.column_index += 1;
                if self.column_index == column_index {
                    break;
                }
            } else {
                break;
            }
        }

        if !next_column {
            debug!(
                "{stmt}.{FUNCTION_NAME} error. index out of bounds. column_index={}, number_of_columns={}",
                column_index,
                self.number_of_columns()
            );
            let column_number = column_index + 1;
            stmt.add_diag(
                TsurugiOdbcError::ColumnNumberOutOfBounds,
                format!(
                    "column_number must be between 1 and {}, but got {}",
                    self.number_of_columns(),
                    column_number
                ),
            );
            return SqlReturn::SQL_ERROR;
        }

        match self.query_result.is_null() {
            Ok(true) => {
                return get_data_null(stmt, arg);
            }
            Ok(false) => {}
            Err(e) => {
                warn!(
                    "{stmt}.{FUNCTION_NAME}: query_result.is_null() error. {:?}",
                    e
                );
                stmt.add_diag(
                    TsurugiOdbcError::SqlQueryResultIsNullError,
                    format!("is_null error. {}", e),
                );
                return SqlReturn::SQL_ERROR;
            }
        }

        match self.fetch_value(stmt, arg) {
            Ok(rc) => rc,
            Err(e) => {
                warn!("{stmt}.{FUNCTION_NAME}: fetch_value() error. {:?}", e);
                stmt.add_diag(
                    TsurugiOdbcError::SqlQueryResultFetchError,
                    format!("fetch_value error. {}", e),
                );
                SqlReturn::SQL_ERROR
            }
        }
    }

    fn dispose(&mut self, stmt: &mut TsurugiOdbcStmt) -> SqlReturn {
        self.close(stmt, false)
    }
}

impl TsurugiOdbcQueryResult {
    #[allow(clippy::result_large_err)]
    fn fetch_value(
        &mut self,
        stmt: &TsurugiOdbcStmt,
        arg: &TsurugiOdbcGetDataArguments,
    ) -> Result<SqlReturn, TgError> {
        const FUNCTION_NAME: &str = "TsurugiOdbcQueryResult.fetch_value()";

        let runtime = stmt.runtime();

        let column_index = arg.column_index() as usize;
        let column = &self.query_result.get_metadata().unwrap().columns()[column_index];

        use AtomType::*;
        let atom_type = column.atom_type().unwrap_or(Unknown);
        let rc = match atom_type {
            Boolean => {
                let value: bool = runtime.block_on(self.query_result.fetch())?;
                get_data_bool(stmt, arg, value)
            }
            Int4 => {
                let value: i32 = runtime.block_on(self.query_result.fetch())?;
                get_data_i32(stmt, arg, value)
            }
            Int8 => {
                let value: i64 = runtime.block_on(self.query_result.fetch())?;
                get_data_i64(stmt, arg, value)
            }
            Float4 => {
                let value: f32 = runtime.block_on(self.query_result.fetch())?;
                get_data_f32(stmt, arg, value)
            }
            Float8 => {
                let value: f64 = runtime.block_on(self.query_result.fetch())?;
                get_data_f64(stmt, arg, value)
            }
            Decimal => {
                let value: TgDecimalResult = runtime.block_on(self.query_result.fetch())?;
                get_data_decimal(stmt, arg, value)
            }
            Character => {
                let value: String = runtime.block_on(self.query_result.fetch())?;
                get_data_string(stmt, arg, &value)
            }
            Octet => {
                let value: Vec<u8> = runtime.block_on(self.query_result.fetch())?;
                get_data_bytes(stmt, arg, &value)
            }
            // Bit => todo!(),
            Date => {
                let value: time::Date = runtime.block_on(self.query_result.fetch())?;
                get_data_date(stmt, arg, value)
            }
            TimeOfDay => {
                let value: time::Time = runtime.block_on(self.query_result.fetch())?;
                get_data_time(stmt, arg, value)
            }
            TimePoint => {
                let value: time::PrimitiveDateTime = runtime.block_on(self.query_result.fetch())?;
                get_data_timestamp(stmt, arg, value)
            }
            // DatetimeInterval => todo!(),
            TimeOfDayWithTimeZone => {
                let value: (time::Time, time::UtcOffset) =
                    runtime.block_on(self.query_result.fetch())?;
                get_data_time_tz(stmt, arg, value)
            }
            TimePointWithTimeZone => {
                let value: time::OffsetDateTime = runtime.block_on(self.query_result.fetch())?;
                get_data_timestamp_tz(stmt, arg, value)
            }
            // Clob => todo!(),
            // Blob => todo!(),
            _ => {
                debug!(
                    "{stmt}.{FUNCTION_NAME} error. Unsupported AtomType {:?}",
                    atom_type
                );
                stmt.add_diag(
                    TsurugiOdbcError::SqlQueryResultAtomTypeError,
                    format!("Unsupported AtomType {:?}", atom_type),
                );
                return Ok(SqlReturn::SQL_ERROR);
            }
        };

        Ok(rc)
    }

    fn close(&mut self, stmt: &mut TsurugiOdbcStmt, commit: bool) -> SqlReturn {
        const FUNCTION_NAME: &str = "TsurugiOdbcQueryResult.close()";

        if self.closed {
            return SqlReturn::SQL_SUCCESS;
        }
        self.closed = true;

        let runtime = stmt.runtime();
        let rc1 = match runtime.block_on(self.query_result.close()) {
            Ok(_) => {
                debug!("{stmt}.{FUNCTION_NAME}: query_result.close() succeeded");
                SqlReturn::SQL_SUCCESS
            }
            Err(e) => {
                warn!(
                    "{stmt}.{FUNCTION_NAME}: query_result.close() error. {:?}",
                    e
                );
                stmt.add_diag(
                    TsurugiOdbcError::SqlQueryResultCloseError,
                    format!("query_result close error. {}", e),
                );
                SqlReturn::SQL_SUCCESS_WITH_INFO
            }
        };

        let rc = if commit {
            stmt.commit_if_auto_commit()
        } else {
            stmt.rollback_if_auto_commit()
        };

        let rc2 = if self.close_ps {
            stmt.close_prepare()
        } else {
            SqlReturn::SQL_SUCCESS
        };

        rc.or(rc1).or(rc2)
    }
}
