use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

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

pub(crate) trait TgLargeObjectReference {
    fn provider(&self) -> LargeObjectProvider;
    fn object_id(&self) -> u64;
    fn reference_tag(&self) -> u64;
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

/// Represents large object cache.
///
/// since 0.5.0
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TgLargeObjectCache {
    path: Option<PathBuf>,
}

impl TgLargeObjectCache {
    fn new(path: Option<PathBuf>) -> Self {
        TgLargeObjectCache { path }
    }

    /// Returns the path of the file that represents the large object, only if it exists.
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

pub(crate) fn lob_cache_processor(
    response: WireResponse,
    session: &Arc<Session>,
) -> Result<TgLargeObjectCache, TgError> {
    const FUNCTION_NAME: &str = "lob_cache_processor()";

    let lob = large_object_data_processor(FUNCTION_NAME, response)?;
    let server_path = match lob {
        LobResult::Path(path) => path,
        _ => return Ok(TgLargeObjectCache::new(None)),
    };

    let lob_recv_path_mapping = session.large_object_path_mapping_on_recv();
    let client_path = lob_recv_path_mapping.convert_to_client_path(&server_path)?;

    Ok(TgLargeObjectCache::new(Some(client_path)))
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
                                Some(lob) => todo!(), //Ok(LobResult::Path(lob.path.clone())),
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
