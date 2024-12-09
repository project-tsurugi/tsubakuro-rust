use prost::Message;

use crate::{
    error::TgError,
    invalid_response_error,
    jogasaki::proto::sql::response::{
        response::Response as SqlResponseCase, Name, Response as SqlResponse,
    },
    prost_decode_error,
    session::wire::WireResponse,
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
        let table_names: Vec<TName> = table_names.iter().map(|name| TName::from(name)).collect();
        TableList {
            table_names: table_names,
        }
    }

    pub fn get_table_names(&self) -> &Vec<TName> {
        &self.table_names
    }
}

pub(crate) fn list_tables_processor(response: WireResponse) -> Result<TableList, TgError> {
    const FUNCTION_NAME: &str = "list_tables_processor()";

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
        Some(SqlResponseCase::ListTables(list_tables)) => match list_tables.result.unwrap() {
            crate::jogasaki::proto::sql::response::list_tables::Result::Success(success) => {
                Ok(TableList::new(success))
            }
            crate::jogasaki::proto::sql::response::list_tables::Result::Error(error) => {
                Err(sql_service_error!(FUNCTION_NAME, error))
            }
        },
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response {:?} is not ListTables", message.response),
        )),
    }
}
