use std::{sync::Arc, time::Duration};

use log::trace;

use crate::{
    error::TgError,
    job::Job,
    prelude::{Session, SystemInfo},
    service::{system::system_info::system_info_processor, ServiceClient, ServiceMessageVersion},
    session::wire::{response::WireResponse, response_box::SlotEntryHandle, Wire},
    tateyama::proto::system::request::{
        request::Command as SystemCommand, Request as SystemRequest,
    },
};

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/system/src/main/java/com/tsurugidb/tsubakuro/system/SystemClient.java
/// The major service message version which this client requests.
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 0;
/// The minor service message version which this client requests.
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 0;

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/system/src/main/java/com/tsurugidb/tsubakuro/system/impl/Constants.java
/// The service ID of system service.
pub(crate) const SERVICE_ID_SYSTEM: i32 = 12;

/// Client of system service.
///
/// # Examples
/// ```
/// use std::sync::Arc;
/// use tsubakuro_rust_core::prelude::*;
///
/// async fn example(session: &Arc<Session>) -> Result<(), TgError> {
///     let client: SystemClient = session.make_client();
///
///     let system_info = client.get_system_info().await?;
///
///     Ok(())
/// }
/// ```
///
/// since 0.7.0
pub struct SystemClient {
    session: Arc<Session>,
    default_timeout: Duration,
}

impl ServiceClient for SystemClient {
    fn new(session: Arc<Session>) -> Self {
        let default_timeout = session.default_timeout();
        SystemClient {
            session,
            default_timeout,
        }
    }
}
impl ServiceMessageVersion for SystemClient {
    fn service_message_version() -> String {
        format!(
            "system-{}.{}",
            SERVICE_MESSAGE_VERSION_MAJOR, SERVICE_MESSAGE_VERSION_MINOR
        )
    }
}

impl SystemClient {
    fn wire(&self) -> Arc<Wire> {
        self.session.wire()
    }

    async fn send_and_pull_response(
        &self,
        command: SystemCommand,
        timeout: Duration,
    ) -> Result<(Arc<SlotEntryHandle>, WireResponse), TgError> {
        let request = Self::new_request(command);
        self.wire()
            .send_and_pull_response(SERVICE_ID_SYSTEM, request, None, timeout)
            .await
    }

    async fn send_and_pull_async<T: 'static>(
        &self,
        job_name: &str,
        command: SystemCommand,
        converter: Box<dyn Fn(Arc<SlotEntryHandle>, WireResponse) -> Result<T, TgError> + Send>,
    ) -> Result<Job<T>, TgError> {
        let request = Self::new_request(command);
        self.wire()
            .send_and_pull_async(
                job_name,
                SERVICE_ID_SYSTEM,
                request,
                None,
                converter,
                self.default_timeout,
                self.session.fail_on_drop_error(),
            )
            .await
    }

    fn new_request(command: SystemCommand) -> SystemRequest {
        SystemRequest {
            service_message_version_major: SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: SERVICE_MESSAGE_VERSION_MINOR,
            command: Some(command),
        }
    }
}

impl SystemClient {
    /// Set default timeout.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Get default timeout.
    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }
}

impl SystemClient {
    /// Retrieves system info.
    ///
    /// # Examples
    /// ```
    /// use tsubakuro_rust_core::prelude::*;
    ///
    /// async fn example(client: &SystemClient) -> Result<(), TgError> {
    ///     let system_info = client.get_system_info().await?;
    ///     println!("version: {}", system_info.version());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_system_info(&self) -> Result<SystemInfo, TgError> {
        let timeout = self.default_timeout;
        self.get_system_info_for(timeout).await
    }

    /// Retrieve system info.
    pub async fn get_system_info_for(&self, timeout: Duration) -> Result<SystemInfo, TgError> {
        const FUNCTION_NAME: &str = "get_system_info()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::get_system_info_command();
        let (slot_handle, response) = self.send_and_pull_response(command, timeout).await?;
        let system_info = system_info_processor(slot_handle, response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(system_info)
    }

    /// Retrieve system info .
    pub async fn get_system_info_async(&self) -> Result<Job<SystemInfo>, TgError> {
        const FUNCTION_NAME: &str = "get_system_info_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::get_system_info_command();
        let job = self
            .send_and_pull_async("GetSystemInfo", command, Box::new(system_info_processor))
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn get_system_info_command() -> SystemCommand {
        let request = crate::tateyama::proto::system::request::GetSystemInfo {};
        SystemCommand::GetSystemInfo(request)
    }
}
