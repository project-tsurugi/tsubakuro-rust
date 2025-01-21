use std::collections::HashMap;

use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::{
        execute_result::CounterType, response::Response as SqlResponseType,
    },
    prelude::convert_sql_response,
    session::wire::response::WireResponse,
    sql_service_error,
};

#[derive(Debug)]
pub struct SqlExecuteResult {
    counters: HashMap<CounterType, i64>,
}

impl SqlExecuteResult {
    pub(crate) fn new(
        success: crate::jogasaki::proto::sql::response::execute_result::Success,
    ) -> SqlExecuteResult {
        let counters: HashMap<CounterType, i64> = success
            .counters
            .iter()
            .map(|entry| (entry.r#type(), entry.value))
            .collect();
        SqlExecuteResult { counters }
    }

    pub fn counters(&self) -> &HashMap<CounterType, i64> {
        &self.counters
    }

    pub fn inserted_rows(&self) -> i64 {
        *self.counters.get(&CounterType::InsertedRows).unwrap_or(&0)
    }

    pub fn updated_rows(&self) -> i64 {
        *self.counters.get(&CounterType::UpdatedRows).unwrap_or(&0)
    }

    pub fn merged_rows(&self) -> i64 {
        *self.counters.get(&CounterType::MergedRows).unwrap_or(&0)
    }

    pub fn deleted_rows(&self) -> i64 {
        *self.counters.get(&CounterType::DeletedRows).unwrap_or(&0)
    }

    pub fn rows(&self) -> i64 {
        self.counters.values().sum()
    }
}

pub(crate) fn execute_result_processor(
    response: WireResponse,
) -> Result<SqlExecuteResult, TgError> {
    const FUNCTION_NAME: &str = "execute_result_processor()";

    let sql_response = convert_sql_response(FUNCTION_NAME, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        FUNCTION_NAME,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;
    match message.response {
        Some(SqlResponseType::ExecuteResult(execute_result)) => match execute_result.result {
            Some(crate::jogasaki::proto::sql::response::execute_result::Result::Success(
                success,
            )) => Ok(SqlExecuteResult::new(success)),
            Some(crate::jogasaki::proto::sql::response::execute_result::Result::Error(error)) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
            None => Err(invalid_response_error!(
                FUNCTION_NAME,
                "response ExecuteResult.result is None",
            )),
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not ExecuteResult", message.response),
        )),
    }
}
