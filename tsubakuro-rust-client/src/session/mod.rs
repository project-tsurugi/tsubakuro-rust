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
}

impl Session {
    pub async fn connect(
        connection_option: &ConnectionOption,
        timeout: Duration,
    ) -> Result<Arc<Session>, TgError> {
        let (endpoint, client_information) = Self::create_information(connection_option)?;
        match endpoint {
            Endpoint::Tcp(_, _) => {
                TcpConnector::connect(endpoint, client_information, timeout).await
            }
            _ => Err(illegal_argument_error!("unsupported endpoint")),
        }
    }

    pub async fn connect_async(
        connection_option: &ConnectionOption,
        timeout: Duration,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let (endpoint, client_information) = Self::create_information(connection_option)?;

        let job = match endpoint {
            Endpoint::Tcp(_, _) => {
                TcpConnector::connect_async(endpoint, client_information, timeout).await?
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

    pub fn make_client<T: ServiceClient>(self: &Arc<Session>) -> T {
        T::new(self.clone())
    }

    // TODO Session::shutdown()
}

impl Session {
    fn new(wire: Arc<Wire>) -> Arc<Self> {
        Arc::new(Session { wire })
    }

    pub(crate) fn wire(&self) -> Arc<Wire> {
        self.wire.clone()
    }
}
