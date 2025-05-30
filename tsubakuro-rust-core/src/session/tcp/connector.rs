use std::sync::Arc;
use std::time::Duration;

use crate::error::TgError;
use crate::job::Job;
use crate::prelude::{ConnectionOption, Endpoint};
use crate::session::wire::DelegateWire;
use crate::session::{wire::Wire, Session};
use crate::tateyama::proto::endpoint::request::{
    wire_information::StreamInformation, wire_information::WireInformation as WireInformationType,
    ClientInformation, WireInformation,
};

use crate::service::endpoint::endpoint_broker::EndpointBroker;

use super::link::TcpLink;
use super::wire::TcpWire;

pub(crate) struct TcpConnector {}

impl TcpConnector {
    pub(crate) async fn connect(
        endpoint: &Endpoint,
        connection_option: &ConnectionOption,
        client_information: ClientInformation,
        timeout: Duration,
        default_timeout: Duration,
    ) -> Result<Arc<Session>, TgError> {
        let wire = TcpConnector::create_wire(endpoint, connection_option).await?;

        let wire_information = Self::create_information();
        let session_id =
            EndpointBroker::handshake(&wire, client_information, wire_information, timeout).await?;

        wire.set_session_id(session_id)?;

        Ok(Session::new(wire, connection_option, default_timeout))
    }

    pub(crate) async fn connect_async(
        endpoint: &Endpoint,
        connection_option: &ConnectionOption,
        client_information: ClientInformation,
        default_timeout: Duration,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let wire = TcpConnector::create_wire(endpoint, connection_option).await?;

        let wire_information = Self::create_information();
        let connection_option = connection_option.clone();
        let job = EndpointBroker::handshake_async(
            &wire.clone(),
            client_information,
            wire_information,
            move |session_id| {
                wire.set_session_id(session_id)?;
                Ok(Session::new(
                    wire.clone(),
                    &connection_option,
                    default_timeout,
                ))
            },
            default_timeout,
            false,
        )
        .await?;
        Ok(job)
    }

    async fn create_wire(
        endpoint: &Endpoint,
        connection_option: &ConnectionOption,
    ) -> Result<Arc<Wire>, TgError> {
        let link = TcpLink::connect(endpoint, connection_option).await?;
        let wire = TcpWire::new(link);
        let wire = Wire::new(DelegateWire::Tcp(Arc::new(wire)));
        Ok(wire)
    }

    fn create_information() -> WireInformation {
        let stream_information = StreamInformation {
            maximum_concurrent_result_sets: 127, // TODO maximum_concurrent_result_sets
        };

        WireInformation {
            wire_information: Some(WireInformationType::StreamInformation(stream_information)),
        }
    }
}
