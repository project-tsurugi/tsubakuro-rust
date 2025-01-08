use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use endpoint::Endpoint;
use option::ConnectionOption;
use tcp::TcpConnector;
use wire::Wire;

use crate::{
    error::TgError,
    illegal_argument_error,
    job::Job,
    prelude::{core::CoreService, ServiceClient, ShutdownType},
    tateyama::proto::endpoint::request::ClientInformation,
    util::string_to_prost_string,
};

pub mod endpoint;
pub mod option;
pub(crate) mod tcp;
pub(crate) mod wire;

#[derive(Debug)]
pub struct Session {
    wire: Arc<Wire>,
    default_timeout: Duration,
    shutodowned: AtomicBool,
}

impl Session {
    pub async fn connect(connection_option: &ConnectionOption) -> Result<Arc<Session>, TgError> {
        let timeout = connection_option.default_timeout();
        Self::connect_for(connection_option, timeout).await
    }

    pub async fn connect_for(
        connection_option: &ConnectionOption,
        timeout: Duration,
    ) -> Result<Arc<Session>, TgError> {
        let (endpoint, client_information) = Self::create_information(connection_option)?;
        let default_timeout = connection_option.default_timeout();

        match endpoint {
            Endpoint::Tcp(_, _) => {
                TcpConnector::connect(
                    endpoint,
                    connection_option,
                    client_information,
                    timeout,
                    default_timeout,
                )
                .await
            }
            _ => Err(illegal_argument_error!("unsupported endpoint")),
        }
    }

    pub async fn connect_async(
        connection_option: &ConnectionOption,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let (endpoint, client_information) = Self::create_information(connection_option)?;
        let default_timeout = connection_option.default_timeout();

        let job = match endpoint {
            Endpoint::Tcp(_, _) => {
                TcpConnector::connect_async(
                    endpoint,
                    connection_option,
                    client_information,
                    default_timeout,
                )
                .await?
            }
            _ => return Err(illegal_argument_error!("unsupported endpoint")),
        };

        Ok(job)
    }

    fn create_information(
        option: &ConnectionOption,
    ) -> Result<(&Endpoint, ClientInformation), TgError> {
        let endpoint = option
            .endpoint()
            .ok_or(illegal_argument_error!("endpoint not specified"))?;

        let client_information = ClientInformation {
            connection_label: string_to_prost_string(option.label()),
            application_name: string_to_prost_string(option.application_name()),
            credential: None, // TODO Crendential
        };
        Ok((endpoint, client_information))
    }

    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    pub fn default_timeout(&self) -> Duration {
        self.default_timeout
    }

    pub fn make_client<T: ServiceClient>(self: &Arc<Session>) -> T {
        T::new(self.clone())
    }

    pub async fn shutdown(&self, shutdown_type: ShutdownType) -> Result<(), TgError> {
        let timeout = self.default_timeout;
        self.shutdown_for(shutdown_type, timeout).await
    }

    pub async fn shutdown_for(
        &self,
        shutdown_type: ShutdownType,
        timeout: Duration,
    ) -> Result<(), TgError> {
        self.set_shutdown();
        CoreService::shutdown(&self.wire, shutdown_type, timeout).await
    }

    pub async fn shutdown_async(&self, shutdown_type: ShutdownType) -> Result<Job<()>, TgError> {
        self.set_shutdown();
        CoreService::shutdown_async(&self.wire, shutdown_type, self.default_timeout).await
    }

    fn set_shutdown(&self) {
        self.shutodowned
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_shutdowned(&self) -> bool {
        self.shutodowned.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub async fn close(&self) -> Result<(), TgError> {
        self.wire.close().await
    }

    pub fn is_closed(&self) -> bool {
        self.wire.is_closed()
    }
}

impl Session {
    fn new(wire: Arc<Wire>, default_timeout: Duration) -> Arc<Self> {
        Arc::new(Session {
            wire,
            default_timeout,
            shutodowned: AtomicBool::new(false),
        })
    }

    pub(crate) fn wire(&self) -> Arc<Wire> {
        self.wire.clone()
    }
}
