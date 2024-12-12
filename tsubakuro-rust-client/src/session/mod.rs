use std::{sync::Arc, time::Duration};

use endpoint::Endpoint;
use option::ConnectionOption;
use tcp::TcpConnector;
use wire::Wire;

use crate::{
    error::TgError, illegal_argument_error, job::Job, prelude::ServiceClient,
    tateyama::proto::endpoint::request::ClientInformation, util::string_to_prost_string,
};

pub mod endpoint;
pub mod option;
pub(crate) mod tcp;
pub(crate) mod wire;

#[derive(Debug)]
pub struct Session {
    wire: Arc<Wire>,
    default_timeout: Duration,
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
                TcpConnector::connect(endpoint, client_information, timeout, default_timeout).await
            }
            _ => Err(illegal_argument_error!("unsupported endpoint")),
        }
    }

    pub async fn connect_async(
        connection_option: &ConnectionOption,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let timeout = connection_option.default_timeout();
        Self::connect_async_for(connection_option, timeout).await
    }

    pub async fn connect_async_for(
        connection_option: &ConnectionOption,
        timeout: Duration,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let (endpoint, client_information) = Self::create_information(connection_option)?;
        let default_timeout = connection_option.default_timeout();

        let job = match endpoint {
            Endpoint::Tcp(_, _) => {
                TcpConnector::connect_async(endpoint, client_information, timeout, default_timeout)
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

    // TODO Session::shutdown()

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
        })
    }

    pub(crate) fn wire(&self) -> Arc<Wire> {
        self.wire.clone()
    }
}
