use std::path::{Path, PathBuf};

use log::debug;

use crate::{error::TgError, io_error, jogasaki::proto::sql::common::LargeObjectProvider};

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
