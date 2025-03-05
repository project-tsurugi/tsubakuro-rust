use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::response::Response as SqlResponseType,
    prelude::{convert_sql_response, SqlColumn},
    session::wire::response::WireResponse,
    sql_service_error,
};

/// Represents an explain result of SQL statement.
#[derive(Debug)]
pub struct SqlExplainResult {
    /// the content format ID.
    format_id: String,
    /// the content format version.
    format_version: u64,
    /// the explain result contents.
    contents: String,
    /// the result set column information, or empty if it does not provided.
    columns: Vec<SqlColumn>,
}

impl SqlExplainResult {
    fn new(
        format_id: String,
        format_version: u64,
        contents: String,
        columns: Vec<SqlColumn>,
    ) -> SqlExplainResult {
        SqlExplainResult {
            format_id,
            format_version,
            contents,
            columns,
        }
    }

    /// Returns the content format ID.
    pub fn format_id(&self) -> &String {
        &self.format_id
    }

    /// Returns the content format version.
    pub fn format_version(&self) -> u64 {
        self.format_version
    }

    /// Returns the explain result contents.
    pub fn contents(&self) -> &String {
        &self.contents
    }

    /// Returns the column information, or empty if it does not provided.
    pub fn columns(&self) -> &Vec<SqlColumn> {
        &self.columns
    }
}

pub(crate) fn explain_processor(response: WireResponse) -> Result<SqlExplainResult, TgError> {
    const FUNCTION_NAME: &str = "explain_processor()";

    let (sql_response, _) = convert_sql_response(FUNCTION_NAME, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        FUNCTION_NAME,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;
    match message.response {
        Some(SqlResponseType::Explain(explain)) => match explain.result {
            Some(crate::jogasaki::proto::sql::response::explain::Result::Success(success)) => {
                Ok(SqlExplainResult::new(
                    success.format_id,
                    success.format_version,
                    success.contents,
                    success.columns,
                ))
            }
            Some(crate::jogasaki::proto::sql::response::explain::Result::Error(error)) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
            _ => Err(invalid_response_error!(
                FUNCTION_NAME,
                format!("response.explain {:?} result is None", explain),
            )),
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not explain", message.response),
        )),
    }
}
