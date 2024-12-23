use std::sync::Arc;
use std::time::Duration;

use link::TcpLink;
use wire::TcpWire;

use crate::error::TgError;
use crate::job::Job;
use crate::prelude::Endpoint;
use crate::tateyama::proto::endpoint::request::{
    wire_information::StreamInformation, wire_information::WireInformation as WireInformationType,
    ClientInformation, WireInformation,
};

use super::option::ConnectionOption;
use super::{
    wire::{DelegateWire, Wire},
    Session,
};
use crate::service::endpoint::EndpointBroker;

pub(crate) mod data_channel_box;
pub(crate) mod data_channel_wire;
pub(crate) mod r#enum;
pub(crate) mod link;
pub(crate) mod wire;

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

        Ok(Session::new(wire, default_timeout))
    }

    pub(crate) async fn connect_async(
        endpoint: &Endpoint,
        connection_option: &ConnectionOption,
        client_information: ClientInformation,
        default_timeout: Duration,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let wire = TcpConnector::create_wire(endpoint, connection_option).await?;

        let wire_information = Self::create_information();
        let job = EndpointBroker::handshake_async(
            &wire.clone(),
            client_information,
            wire_information,
            move |session_id| {
                wire.set_session_id(session_id)?;
                Ok(Session::new(wire.clone(), default_timeout))
            },
            default_timeout,
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
