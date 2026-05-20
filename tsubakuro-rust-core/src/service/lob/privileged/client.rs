use std::{path::Path, time::Duration};

use tonic::async_trait;

use crate::{
    client_error,
    error::TgError,
    service::lob::{
        lob_client::{LobClient, RemoteLob},
        privileged::path_mapping::LargeObjectSendPathMapping,
    },
    session::Session,
};

pub(crate) struct PrivilegedLobClient {
    lob_send_path_mapping: LargeObjectSendPathMapping,
}

impl PrivilegedLobClient {
    pub(crate) fn new(session: &Session) -> PrivilegedLobClient {
        let lob_send_path_mapping = session.large_object_path_mapping_on_send().clone();
        PrivilegedLobClient {
            lob_send_path_mapping,
        }
    }
}

#[async_trait]
impl LobClient for PrivilegedLobClient {
    async fn upload_lob_file(&self, path: &Path, _timeout: Duration) -> Result<RemoteLob, TgError> {
        let server_path = self.lob_send_path_mapping.convert_to_server_path(path)?;
        Ok(RemoteLob::ServerPath(server_path))
    }

    async fn upload_lob(&self, _value: &[u8], _timeout: Duration) -> Result<RemoteLob, TgError> {
        Err(client_error!("Not supported in privileged mode"))
    }
}
