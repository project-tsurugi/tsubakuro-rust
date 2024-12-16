use std::collections::HashMap;

use prost::Message;

use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::{
        execute_result::CounterType, response::Response as SqlResponseCase, Response as SqlResponse,
    },
    prost_decode_error,
    session::wire::WireResponse,
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

    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload) = response {
        payload.unwrap()
    } else {
        return Err(invalid_response_error!(
            FUNCTION_NAME,
            "response is not ResponseSessionPayload",
        ));
    };

    let message = SqlResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "SqlResponse", e))?;
    match message.response {
        Some(SqlResponseCase::ExecuteResult(execute_result)) => {
            match execute_result.result.unwrap() {
                crate::jogasaki::proto::sql::response::execute_result::Result::Success(success) => {
                    Ok(SqlExecuteResult::new(success))
                }
                crate::jogasaki::proto::sql::response::execute_result::Result::Error(error) => {
                    Err(sql_service_error!(FUNCTION_NAME, error))
                }
            }
        }
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not ExecuteResult", message.response),
        )),
    }
}
