use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use tonic::async_trait;

use crate::{
    data_relay_grpc::proto::blob_relay::blob_reference::BlobReference as RelayLobReference,
    error::TgError,
    io_error,
    job::Job,
    service::{
        lob::{
            lob_transfer_info::LobTransferInfo, privileged::client::PrivilegedLobClient,
            relay::client::RelayLobClient,
        },
        sql::r#type::large_object::TgLargeObjectReference,
    },
    session::Session,
    transaction::Transaction,
};

/// Uploaded large object.
///
/// since 0.10.0
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteLob {
    ServerPath(String),
    RelayLobReference(RelayLobReference),
}

/// Large object client.
///
/// since 0.10.0
#[async_trait]
pub(crate) trait LobClient {
    async fn upload_lob_file(&self, path: &Path, timeout: Duration) -> Result<RemoteLob, TgError>;

    async fn upload_lob(&self, value: &[u8], timeout: Duration) -> Result<RemoteLob, TgError>;

    fn support_download_lob_file(&self) -> bool;

    async fn download_lob_file(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        timeout: Duration,
    ) -> Result<PathBuf, TgError>;

    async fn download_lob_file_async(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<PathBuf>, TgError>;

    async fn download_lob(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        timeout: Duration,
    ) -> Result<Vec<u8>, TgError>;

    async fn download_lob_async(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<Vec<u8>>, TgError>;
}

pub(crate) async fn create_lob_client(
    session: &Arc<Session>,
) -> Result<Box<dyn LobClient>, TgError> {
    let lob_transfer_info = session.lob_transfer_info();
    let lob_client: Box<dyn LobClient> = match lob_transfer_info {
        LobTransferInfo::NotUse => Box::new(NotUseLobClient::new()),
        LobTransferInfo::Privileged => Box::new(PrivilegedLobClient::new(session.clone())),
        LobTransferInfo::Relay(info) => Box::new(RelayLobClient::new(info).await?),
    };
    Ok(lob_client)
}

pub(crate) struct NotUseLobClient {}

impl NotUseLobClient {
    pub(crate) fn new() -> NotUseLobClient {
        NotUseLobClient {}
    }
}

#[async_trait]
impl LobClient for NotUseLobClient {
    async fn upload_lob_file(
        &self,
        _path: &Path,
        _timeout: Duration,
    ) -> Result<RemoteLob, TgError> {
        Err(io_error!("LOB transfer is not available"))
    }

    async fn upload_lob(&self, _value: &[u8], _timeout: Duration) -> Result<RemoteLob, TgError> {
        Err(io_error!("LOB transfer is not available"))
    }

    fn support_download_lob_file(&self) -> bool {
        false
    }

    async fn download_lob_file(
        &self,
        _transaction: &Transaction,
        _lob: &dyn TgLargeObjectReference,
        _timeout: Duration,
    ) -> Result<PathBuf, TgError> {
        Err(io_error!("LOB transfer is not available"))
    }

    async fn download_lob_file_async(
        &self,
        _transaction: &Transaction,
        _lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<PathBuf>, TgError> {
        Err(io_error!("LOB transfer is not available"))
    }

    async fn download_lob(
        &self,
        _transaction: &Transaction,
        _lob: &dyn TgLargeObjectReference,
        _timeout: Duration,
    ) -> Result<Vec<u8>, TgError> {
        Err(io_error!("LOB transfer is not available"))
    }

    async fn download_lob_async(
        &self,
        _transaction: &Transaction,
        _lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<Vec<u8>>, TgError> {
        Err(io_error!("LOB transfer is not available"))
    }
}
