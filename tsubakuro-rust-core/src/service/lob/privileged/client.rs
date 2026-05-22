use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use prost::Message;
use tonic::async_trait;

use crate::{
    error::TgError,
    invalid_response_error, io_error,
    job::Job,
    jogasaki::proto::sql::common::Transaction as ProtoTransaction,
    lob_relay_privileged_service_error, prost_decode_error,
    service::{
        lob::{
            lob_client::{LobClient, LobClientMethod, RemoteLob},
            privileged::path_mapping::LargeObjectRecvPathMapping,
            storage_id,
            uploader::LobUploader,
        },
        sql::r#type::large_object::TgLargeObjectReference,
        ServiceMessageVersion,
    },
    session::{
        wire::{response::WireResponse, response_box::SlotEntryHandle},
        Session,
    },
    tateyama::proto::blob_relay_privilege::request::{request::Command, Request},
    transaction::Transaction,
};

/// The symbolic ID of this implementation.
pub(crate) const BLOB_RELAY_PRIVILEGE_SERVICE_SYMBOLIC_ID: &str = "blob_relay_privilege";

/// The major service message version for blob relay privilege.
const BLOB_RELAY_PRIVILEGE_SERVICE_MESSAGE_VERSION_MAJOR: u64 = 0;

/// The minor service message version for blob relay privilege.
const BLOB_RELAY_PRIVILEGE_SERVICE_MESSAGE_VERSION_MINOR: u64 = 0;

/// The service id for blob relay privilege.
pub(crate) const SERVICE_ID_BLOB_RELAY_PRIVILEGE: i32 = 13;

pub(crate) struct PrivilegedLobClient {
    session: Arc<Session>,
}

impl PrivilegedLobClient {
    pub(crate) fn new(session: Arc<Session>) -> PrivilegedLobClient {
        PrivilegedLobClient { session }
    }
}

impl ServiceMessageVersion for PrivilegedLobClient {
    fn service_message_version() -> String {
        format!(
            "{}-{}.{}",
            BLOB_RELAY_PRIVILEGE_SERVICE_SYMBOLIC_ID,
            BLOB_RELAY_PRIVILEGE_SERVICE_MESSAGE_VERSION_MAJOR,
            BLOB_RELAY_PRIVILEGE_SERVICE_MESSAGE_VERSION_MINOR
        )
    }
}

#[async_trait]
impl LobClient for PrivilegedLobClient {
    fn supports_method(&self, method: LobClientMethod) -> bool {
        use LobClientMethod::*;
        match method {
            UploadLobFile => true,
            UploadLob | CreateLobUploader => false,
            DownloadLobFile | DownloadLob => true,
        }
    }

    async fn upload_lob_file(&self, path: &Path, _timeout: Duration) -> Result<RemoteLob, TgError> {
        let server_path = self
            .session
            .large_object_path_mapping_on_send()
            .convert_to_server_path(path)?;
        Ok(RemoteLob::ServerPath(server_path))
    }

    async fn upload_lob_file_async(&self, path: &Path) -> Result<Job<RemoteLob>, TgError> {
        let server_path = self
            .session
            .large_object_path_mapping_on_send()
            .convert_to_server_path(path)?;
        let remote_lob = RemoteLob::ServerPath(server_path);
        Ok(Job::returns("UploadLobFile", remote_lob))
    }

    async fn upload_lob(&self, _value: &[u8], _timeout: Duration) -> Result<RemoteLob, TgError> {
        Err(io_error!("Not supported in privileged mode"))
    }

    async fn upload_lob_async(&self, _value: &[u8]) -> Result<Job<RemoteLob>, TgError> {
        Err(io_error!("Not supported in privileged mode"))
    }

    async fn create_lob_uploader(&self) -> Result<Arc<dyn LobUploader + Send + Sync>, TgError> {
        Err(io_error!("Not supported in privileged mode"))
    }

    async fn download_lob_file(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        timeout: Duration,
    ) -> Result<PathBuf, TgError> {
        self.get_file_path(transaction, lob, timeout).await
    }

    async fn download_lob_file_async(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<PathBuf>, TgError> {
        self.get_file_path_async(transaction, lob).await
    }

    async fn download_lob(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        timeout: Duration,
    ) -> Result<Vec<u8>, TgError> {
        let client_path = self.get_file_path(transaction, lob, timeout).await?;
        std::fs::read(client_path).map_err(|e| io_error!("Failed to read lob file: {}", e))
    }

    async fn download_lob_async(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<Vec<u8>>, TgError> {
        let job = self.get_file_path_async(transaction, lob).await?;
        let job = job.convert(
            "Bytes",
            Arc::new(|client_path| {
                std::fs::read(client_path).map_err(|e| io_error!("Failed to read lob file: {}", e))
            }),
        );
        Ok(job)
    }
}

impl PrivilegedLobClient {
    async fn get_file_path(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
        timeout: Duration,
    ) -> Result<PathBuf, TgError> {
        let tx_handle = transaction.transaction_handle()?;

        let command = Self::get_lob_command(&tx_handle, lob)?;
        let (_, response) = self.send_and_pull_response(command, timeout).await?;
        let client_path = get_lob_processor(
            response,
            self.session.large_object_path_mapping_on_recv().clone(),
        )?;

        Ok(client_path)
    }

    async fn get_file_path_async(
        &self,
        transaction: &Transaction,
        lob: &dyn TgLargeObjectReference,
    ) -> Result<Job<PathBuf>, TgError> {
        let tx_handle = transaction.transaction_handle()?;

        let command = Self::get_lob_command(&tx_handle, lob)?;
        let lob_recv_path_mapping = self.session.large_object_path_mapping_on_recv().clone();
        let job = self
            .send_and_pull_async(
                "LobFilePath",
                command,
                Arc::new(move |_, response| {
                    get_lob_processor(response, lob_recv_path_mapping.clone())
                }),
            )
            .await?;

        Ok(job)
    }

    fn get_lob_command(
        transaction_handle: &ProtoTransaction,
        lob: &dyn TgLargeObjectReference,
    ) -> Result<Command, TgError> {
        use crate::tateyama::proto::blob_relay_privilege::request::get_blob::ContextId;
        let context_id = ContextId::TransactionHandle(transaction_handle.handle);

        let lob_reference = crate::tateyama::proto::blob_relay_privilege::request::BlobReference {
            storage_id: storage_id(lob)?,
            object_id: lob.object_id(),
            tag: lob.reference_tag(),
        };

        let get_lob = crate::tateyama::proto::blob_relay_privilege::request::GetBlob {
            blob_reference: Some(lob_reference),
            context_id: Some(context_id),
        };
        Ok(Command::GetBlob(get_lob))
    }
}

fn get_lob_processor(
    response: WireResponse,
    lob_recv_path_mapping: Arc<LargeObjectRecvPathMapping>,
) -> Result<PathBuf, TgError> {
    const FUNCTION_NAME: &str = "get_lob_processor()";

    use crate::tateyama::proto::blob_relay_privilege::response::{get_blob::Result, GetBlob};

    match response {
        WireResponse::ResponseSessionPayload(_slot, payload, _lobs, error) => {
            if let Some(e) = error {
                return Err(e.to_tg_error());
            }
            if payload.is_none() {
                return Err(invalid_response_error!(FUNCTION_NAME, "payload is None"));
            }
            // let payload = payload.as_deref().unwrap();
            let payload = &payload.as_ref().unwrap()[..];
            let response = GetBlob::decode_length_delimited(payload)
                .map_err(|e| prost_decode_error!(FUNCTION_NAME, "GetBlob", e))?;
            match response.result {
                Some(Result::Success(success)) => {
                    let server_path = success.server_file_path;
                    let client_path = lob_recv_path_mapping.convert_to_client_path(&server_path)?;
                    Ok(client_path)
                }
                Some(Result::Error(error)) => {
                    Err(lob_relay_privileged_service_error!(FUNCTION_NAME, error))
                }
                _ => Err(invalid_response_error!(
                    FUNCTION_NAME,
                    "GetBlob.result is None"
                )),
            }
        }
        _ => Err(invalid_response_error!(
            FUNCTION_NAME,
            "response is not ResponseSessionPayload",
        )),
    }
}

impl PrivilegedLobClient {
    async fn send_and_pull_response(
        &self,
        command: Command,
        timeout: Duration,
    ) -> Result<(Arc<SlotEntryHandle>, WireResponse), TgError> {
        let request = Self::new_request(command);
        self.session
            .wire()
            .send_and_pull_response(SERVICE_ID_BLOB_RELAY_PRIVILEGE, request, None, timeout)
            .await
    }

    async fn send_and_pull_async<T: Send + Sync + 'static>(
        &self,
        job_name: &str,
        command: Command,
        converter: Arc<
            dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send + Sync,
        >,
    ) -> Result<Job<T>, TgError> {
        let request = Self::new_request(command);
        self.session
            .wire()
            .send_and_pull_async(
                job_name,
                SERVICE_ID_BLOB_RELAY_PRIVILEGE,
                request,
                None,
                converter,
                self.session.default_timeout(),
                self.session.fail_on_drop_error(),
            )
            .await
    }

    fn new_request(command: Command) -> Request {
        Request {
            service_message_version_major: BLOB_RELAY_PRIVILEGE_SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: BLOB_RELAY_PRIVILEGE_SERVICE_MESSAGE_VERSION_MINOR,
            command: Some(command),
        }
    }
}
