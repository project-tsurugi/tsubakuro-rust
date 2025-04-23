use std::{path::Path, sync::Arc};

use crate::{
    error::TgError,
    invalid_response_error, io_error,
    jogasaki::proto::sql::{
        common::LargeObjectProvider, response::response::Response as SqlResponseType,
    },
    prelude::{convert_sql_response, Session},
    session::wire::response::WireResponse,
    sql_service_error,
};

use super::large_object::TgLargeObjectReference;

/// BLOB.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TgBlob {
    /// BLOB with path.
    Path(String),
    /// BLOB with bytes.
    Contents(Vec<u8>),
}

impl TgBlob {
    pub fn new(path: &str) -> TgBlob {
        TgBlob::Path(path.to_string())
    }
}

impl From<Vec<u8>> for TgBlob {
    fn from(value: Vec<u8>) -> Self {
        TgBlob::Contents(value)
    }
}

/// BLOB for [SqlQueryResult](crate::prelude::SqlQueryResult).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgBlobReference {
    provider: LargeObjectProvider,
    object_id: u64,
}

impl TgBlobReference {
    pub(crate) fn new(provider: LargeObjectProvider, object_id: u64) -> TgBlobReference {
        TgBlobReference {
            provider,
            object_id,
        }
    }
}

impl TgLargeObjectReference for TgBlobReference {
    fn provider(&self) -> LargeObjectProvider {
        self.provider
    }

    fn object_id(&self) -> u64 {
        self.object_id
    }
}

pub(crate) fn lob_open_processor(
    response: WireResponse,
    session: &Arc<Session>,
) -> Result<std::fs::File, TgError> {
    const FUNCTION_NAME: &str = "lob_open_processor()";

    let lob = large_object_data_processor(FUNCTION_NAME, response)?;
    let server_path = match lob {
        LobResult::Path(path) => path,
        _ => {
            return Err(invalid_response_error!(
                FUNCTION_NAME,
                "unsupported LobResult"
            ))
        }
    };

    let lob_recv_path_mapping = session.large_object_path_mapping_on_recv();
    let client_path = lob_recv_path_mapping.convert_to_client_path(&server_path)?;

    match std::fs::File::open(client_path) {
        Ok(value) => Ok(value),
        Err(e) => Err(io_error!("lob file open error", e)),
    }
}

pub(crate) fn lob_copy_to_processor<T: AsRef<Path>>(
    response: WireResponse,
    session: &Arc<Session>,
    destination: T,
) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "lob_copy_to_processor()";

    let lob = large_object_data_processor(FUNCTION_NAME, response)?;
    let server_path = match lob {
        LobResult::Path(path) => path,
        _ => {
            return Err(invalid_response_error!(
                FUNCTION_NAME,
                "unsupported LobResult"
            ))
        }
    };

    let lob_recv_path_mapping = session.large_object_path_mapping_on_recv();
    let client_path = lob_recv_path_mapping.convert_to_client_path(&server_path)?;

    if let Err(e) = std::fs::copy(client_path, destination) {
        return Err(io_error!("file copy error", e));
    }
    Ok(())
}

enum LobResult {
    Path(String),
    #[allow(dead_code)]
    Contents(Vec<u8>),
}

fn large_object_data_processor(
    function_name: &str,
    response: WireResponse,
) -> Result<LobResult, TgError> {
    let (sql_response, lobs) = convert_sql_response(function_name, &response)?;
    let message = sql_response.ok_or(invalid_response_error!(
        function_name,
        format!("response {:?} is not ResponseSessionPayload", response),
    ))?;

    use crate::jogasaki::proto::sql::response::get_large_object_data::Result;
    match message.response {
        Some(SqlResponseType::GetLargeObjectData(data)) => match data.result {
            Some(Result::Success(success)) => {
                use crate::jogasaki::proto::sql::response::get_large_object_data::success::Data;
                match success.data {
                    Some(Data::ChannelName(channel_name)) => {
                        if let Some(lobs) = lobs {
                            match lobs.get(&channel_name) {
                                Some(lob) => Ok(LobResult::Path(lob.path.clone())),
                                None => Err(invalid_response_error!(
                                    function_name,
                                    "channel_name not found in BlobOpt",
                                )),
                            }
                        } else {
                            Err(invalid_response_error!(function_name, "BlobOpt not found"))
                        }
                    }
                    Some(Data::Contents(contents)) => Ok(LobResult::Contents(contents)),
                    None => Err(invalid_response_error!(
                        function_name,
                        "response GetLargeObjectData.result.success is None",
                    )),
                }
            }
            Some(Result::Error(error)) => Err(sql_service_error!(function_name, error)),
            None => Err(invalid_response_error!(
                function_name,
                "response GetLargeObjectData.result is None",
            )),
        },
        _ => Err(invalid_response_error!(
            function_name,
            format!("response {:?} is not ExecuteResult", message.response),
        )),
    }
}
