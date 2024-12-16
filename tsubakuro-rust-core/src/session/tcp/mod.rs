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

use super::{
    wire::{DelegateWire, Wire},
    Session,
};
use crate::service::endpoint::EndpointBroker;

pub(crate) mod r#enum;
pub(crate) mod link;
pub(crate) mod result_set_box;
pub(crate) mod result_set_wire;
pub(crate) mod wire;

pub(crate) struct TcpConnector {}

impl TcpConnector {
    pub(crate) async fn connect(
        endpoint: &Endpoint,
        client_information: ClientInformation,
        timeout: Duration,
        default_timeout: Duration,
    ) -> Result<Arc<Session>, TgError> {
        let wire = TcpConnector::create_wire(endpoint).await?;

        let wire_information = Self::create_information();
        let session_id =
            EndpointBroker::handshake(wire.clone(), client_information, wire_information, timeout)
                .await?;

        wire.set_session_id(session_id)?;

        Ok(Session::new(wire, default_timeout))
    }

    pub(crate) async fn connect_async(
        endpoint: &Endpoint,
        client_information: ClientInformation,
        timeout: Duration,
        default_timeout: Duration,
    ) -> Result<Job<Arc<Session>>, TgError> {
        let wire = TcpConnector::create_wire(endpoint).await?;

        let wire_information = Self::create_information();
        let handshake_job = EndpointBroker::handshake_async(
            wire.clone(),
            client_information,
            wire_information,
            timeout,
            default_timeout,
        )
        .await?;

        let job = Job::new(
            move |timeout| {
                Box::pin(async move {
                    let session_id = handshake_job.take_for(timeout).await?;

                    wire.set_session_id(session_id)?;

                    Ok(Session::new(wire, default_timeout))
                })
            },
            timeout,
        );
        Ok(job)
    }

    async fn create_wire(endpoint: &Endpoint) -> Result<Arc<Wire>, TgError> {
        let link = TcpLink::connect(endpoint).await?;
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
