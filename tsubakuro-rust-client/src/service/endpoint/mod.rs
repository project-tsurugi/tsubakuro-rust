use std::{sync::Arc, time::Duration};

use log::trace;
use prost::Message as ProstMessage;

use crate::{
    endpoint_service_error,
    error::TgError,
    invalid_response_error,
    job::Job,
    prost_decode_error,
    session::wire::{Wire, WireResponse},
    tateyama::proto::endpoint::{
        request::{
            request::Command as EndointRequestCommand, ClientInformation,
            Handshake as HandshakeRequest, Request as EndpointRequest, WireInformation,
        },
        response::{handshake::Result as HandshakeResult, Handshake as HandshakeResponse},
    },
};

mod error;

/// The service id for endpoint broker.
const SERVICE_ID_ENDPOINT_BROKER: i32 = 1;

/// The major service message version for EndpointRequest.
const ENDPOINT_BROKER_SERVICE_MESSAGE_VERSION_MAJOR: u64 = 0;

/// The minor service message version for EndpointRequest.
const ENDPOINT_BROKER_SERVICE_MESSAGE_VERSION_MINOR: u64 = 0;

pub struct EndpointBroker;

impl EndpointBroker {
    pub(crate) async fn handshake(
        wire: Arc<Wire>,
        client_information: ClientInformation,
        wire_information: WireInformation,
        timeout: Duration,
    ) -> Result<i64, TgError> {
        const FUNCTION_NAME: &str = "handshake()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::handshake_command(client_information, wire_information);
        let request = Self::new_request(command);

        let response = wire
            .send_and_pull_response(SERVICE_ID_ENDPOINT_BROKER, request, timeout)
            .await?;
        let session_id = handshake_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(session_id)
    }

    pub(crate) async fn handshake_async(
        wire: Arc<Wire>,
        client_information: ClientInformation,
        wire_information: WireInformation,
        timeout: Duration,
        default_timeout: Duration,
    ) -> Result<Job<i64>, TgError> {
        const FUNCTION_NAME: &str = "handshake_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::handshake_command(client_information, wire_information);
        let request = Self::new_request(command);

        let job = wire
            .send_and_pull_async(
                SERVICE_ID_ENDPOINT_BROKER,
                request,
                Box::new(move |response| {
                    let session_id = handshake_processor(response)?;
                    Ok(session_id)
                }),
                timeout,
                default_timeout,
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn handshake_command(
        client_information: ClientInformation,
        wire_information: WireInformation,
    ) -> EndointRequestCommand {
        let handshake = HandshakeRequest {
            client_information: Some(client_information),
            wire_information: Some(wire_information),
        };
        EndointRequestCommand::Handshake(handshake)
    }

    fn new_request(command: EndointRequestCommand) -> EndpointRequest {
        EndpointRequest {
            service_message_version_major: ENDPOINT_BROKER_SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: ENDPOINT_BROKER_SERVICE_MESSAGE_VERSION_MINOR,
            command: Some(command),
        }
    }
}

fn handshake_processor(wire_response: WireResponse) -> Result<i64, TgError> {
    const FUNCTION_NAME: &str = "handshake_processor()";

    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload) = wire_response {
        payload.unwrap()
    } else {
        return Err(invalid_response_error!(
            FUNCTION_NAME,
            "response is not ResponseSessionPayload",
        ));
    };

    let message = HandshakeResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "HandshakeResponse", e))?;
    match message.result {
        Some(result) => match result {
            HandshakeResult::Success(success) => Ok(success.session_id as i64),
            HandshakeResult::Error(error) => Err(endpoint_service_error!(FUNCTION_NAME, error)),
        },
        None => Err(invalid_response_error!(
            FUNCTION_NAME,
            "HandshakeResponse.result is None",
        )),
    }
}
