use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::{response::Response as SqlResponseType, Name},
    prelude::convert_sql_response,
    session::wire::response::WireResponse,
    sql_service_error,
};

use super::name::TName;

#[derive(Debug)]
pub struct TableList {
    table_names: Vec<TName>,
}

impl TableList {
    pub(crate) fn new(
        success: crate::jogasaki::proto::sql::response::list_tables::Success,
    ) -> TableList {
        let table_names: Vec<Name> = success.table_path_names;
        let table_names: Vec<TName> = table_names.iter().map(TName::from).collect();
        TableList { table_names }
    }

    pub fn table_names(&self) -> &Vec<TName> {
        &self.table_names
    }
}

pub(crate) fn list_tables_processor(response: WireResponse) -> Result<TableList, TgError> {
    const FUNCTION_NAME: &str = "list_tables_processor()";

    let sql_response = convert_sql_response(FUNCTION_NAME, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        FUNCTION_NAME,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;
    match message.response {
        Some(SqlResponseType::ListTables(list_tables)) => match list_tables.result {
            Some(crate::jogasaki::proto::sql::response::list_tables::Result::Success(success)) => {
                Ok(TableList::new(success))
            }
            Some(crate::jogasaki::proto::sql::response::list_tables::Result::Error(error)) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
            None => Err(invalid_response_error!(
                FUNCTION_NAME,
                format!("response ListTables.result is None"),
            )),
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not ListTables", message.response),
        )),
    }
}
