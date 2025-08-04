use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use log::debug;

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
}

#[derive(Debug, Clone)]
pub(crate) struct LargeObjectPathMappingEntry {
    client_path: PathBuf,
    server_path: String,
}

impl LargeObjectPathMappingEntry {
    pub(crate) fn new<T: AsRef<Path>>(
        client_path: T,
        server_path: &str,
    ) -> LargeObjectPathMappingEntry {
        let mut server_path = server_path.to_string();
        if !server_path.ends_with("/") {
            server_path.push('/');
        }
        LargeObjectPathMappingEntry {
            client_path: client_path.as_ref().to_path_buf(),
            server_path,
        }
    }

    // local_flie: absolute path
    pub(crate) fn convert_to_server_path<T: AsRef<Path>>(&self, local_file: T) -> Option<String> {
        let local_file = local_file.as_ref();
        match local_file.strip_prefix(&self.client_path) {
            Ok(relative_path) => {
                let mut s = self.server_path.clone();
                s.push_str(&client_path_to_server_path(relative_path));
                Some(s)
            }
            Err(_) => None,
        }
    }

    pub(crate) fn convert_to_client_path(&self, server_file: &str) -> Option<PathBuf> {
        if !server_file.starts_with(&self.server_path) {
            return None;
        }

        let relative_path = &server_file[self.server_path.len()..];
        let client_path = self.client_path.join(relative_path);
        Some(client_path)
    }
}

fn client_path_to_server_path(client_path: &Path) -> String {
    client_path.to_string_lossy().replace("\\", "/")
}

fn server_path_to_client_path(server_path: &str) -> PathBuf {
    PathBuf::from(server_path)
}

#[derive(Debug, Clone)]
pub(crate) struct LargeObjectSendPathMapping {
    entries: Vec<LargeObjectPathMappingEntry>,
}

impl LargeObjectSendPathMapping {
    pub fn new() -> LargeObjectSendPathMapping {
        LargeObjectSendPathMapping {
            entries: Vec::new(),
        }
    }

    pub fn add<T: AsRef<Path>>(&mut self, client_path: T, server_path: &str) {
        self.entries
            .push(LargeObjectPathMappingEntry::new(client_path, server_path));
    }

    pub fn contert_to_server_path(&self, client_path: &str) -> Result<String, TgError> {
        let mut client_path = std::fs::canonicalize(client_path)
            .map_err(|e| io_error!("path canonicalize error", e))?;
        if cfg!(windows) {
            client_path = client_path
                .to_string_lossy()
                .trim_start_matches("\\\\?\\")
                .into();
        }

        for entry in &self.entries {
            if let Some(server_path) = entry.convert_to_server_path(&client_path) {
                debug!(
                    "LargeObjectSendPathMapping: client_path={:?} -> server_path={}",
                    client_path, server_path
                );
                return Ok(server_path);
            }
        }

        let server_path = client_path_to_server_path(&client_path);
        debug!(
            "LargeObjectSendPathMapping: client_path={:?} == server_path={}",
            client_path, server_path
        );
        Ok(server_path)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LargeObjectRecvPathMapping {
    entries: Vec<LargeObjectPathMappingEntry>,
}

impl LargeObjectRecvPathMapping {
    pub fn new() -> LargeObjectRecvPathMapping {
        LargeObjectRecvPathMapping {
            entries: Vec::new(),
        }
    }

    pub fn add<T: AsRef<Path>>(&mut self, server_path: &str, client_path: T) {
        self.entries
            .push(LargeObjectPathMappingEntry::new(client_path, server_path));
    }

    pub fn convert_to_client_path(&self, server_path: &str) -> Result<PathBuf, TgError> {
        for entry in &self.entries {
            if let Some(client_path) = entry.convert_to_client_path(server_path) {
                debug!(
                    "LargeObjectRecvPathMapping: server_path={} -> client_path={:?}",
                    server_path, client_path
                );
                return Ok(client_path);
            }
        }

        let client_path = server_path_to_client_path(server_path);
        debug!(
            "LargeObjectRecvPathMapping: server_path={} == client_path={:?}",
            server_path, client_path
        );
        Ok(client_path)
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

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn convert_to_server_path_windows() {
        let target = LargeObjectPathMappingEntry::new("C:/client", "/mnt/client");
        assert_eq!(None, target.convert_to_server_path("C:/tmp/foo/bar.dat"));
        assert_eq!(
            Some("/mnt/client/foo/bar.dat".to_string()),
            target.convert_to_server_path("C:/client/foo/bar.dat")
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn convert_to_server_path_unix() {
        let target = LargeObjectPathMappingEntry::new("/client", "/mnt/client");
        assert_eq!(None, target.convert_to_server_path("/tmp/foo/bar.dat"));
        assert_eq!(
            Some("/mnt/client/foo/bar.dat".to_string()),
            target.convert_to_server_path("/client/foo/bar.dat")
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn convert_to_client_path_windows() {
        let target =
            LargeObjectPathMappingEntry::new("C:/tmp/tsurugi", "/opt/tsurugi/var/data/log");
        assert_eq!(
            None,
            target.convert_to_client_path("/opt/tsurugi/var/data/log1/foo.dat")
        );
        assert_eq!(
            Some(PathBuf::from("C:/tmp/tsurugi\\foo.dat")),
            target.convert_to_client_path("/opt/tsurugi/var/data/log/foo.dat")
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn convert_to_client_path_unix() {
        let target = LargeObjectPathMappingEntry::new("/tmp/tsurugi", "/opt/tsurugi/var/data/log");
        assert_eq!(
            None,
            target.convert_to_client_path("/opt/tsurugi/var/data/log1/foo.dat")
        );
        assert_eq!(
            Some(PathBuf::from("/tmp/tsurugi/foo.dat")),
            target.convert_to_client_path("/opt/tsurugi/var/data/log/foo.dat")
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn client_path_to_server_path_windows() {
        assert_eq!(
            "C:/tmp/foo/bar.dat",
            client_path_to_server_path(&Path::new("C:/tmp/foo/bar.dat"))
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn client_path_to_server_path_unix() {
        assert_eq!(
            "/tmp/foo/bar.dat",
            client_path_to_server_path(&Path::new("/tmp/foo/bar.dat"))
        );
    }

    #[test]
    fn server_path_to_client_path_test() {
        assert_eq!(
            PathBuf::from("/tmp/foo/bar.dat"),
            server_path_to_client_path("/tmp/foo/bar.dat")
        );
    }
}
