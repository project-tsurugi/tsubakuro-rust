use std::{sync::Arc, time::Duration};

use log::trace;
use prost::Message as ProstMessage;

use crate::{
    endpoint_service_error,
    error::TgError,
    invalid_response_error,
    job::Job,
    prost_decode_error,
    session::wire::{response::WireResponse, Wire},
    tateyama::proto::endpoint::{
        request::{
            request::Command as EndointCommand, Cancel as CancelRequest, ClientInformation,
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

pub(crate) struct EndpointBroker;

impl EndpointBroker {
    pub(crate) async fn handshake(
        wire: &Arc<Wire>,
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

    pub(crate) async fn handshake_async<F, T: 'static>(
        wire: &Arc<Wire>,
        client_information: ClientInformation,
        wire_information: WireInformation,
        converter: F,
        default_timeout: Duration,
    ) -> Result<Job<T>, TgError>
    where
        F: Fn(/*session_id*/ i64) -> Result<T, TgError> + Send + 'static,
    {
        const FUNCTION_NAME: &str = "handshake_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::handshake_command(client_information, wire_information);
        let request = Self::new_request(command);

        let job = wire
            .send_and_pull_async(
                "Handshake",
                SERVICE_ID_ENDPOINT_BROKER,
                request,
                Box::new(move |response| {
                    let session_id = handshake_processor(response)?;
                    let result = converter(session_id);
                    result
                }),
                default_timeout,
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn handshake_command(
        client_information: ClientInformation,
        wire_information: WireInformation,
    ) -> EndointCommand {
        let handshake = HandshakeRequest {
            client_information: Some(client_information),
            wire_information: Some(wire_information),
        };
        EndointCommand::Handshake(handshake)
    }

    pub(crate) async fn cancel(wire: &Arc<Wire>, slot: i32) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "cancel()";
        trace!("{} (slot={} start)", FUNCTION_NAME, slot);

        let command = Self::cancel_command();
        let request = Self::new_request(command);

        wire.send_to_slot(slot, SERVICE_ID_ENDPOINT_BROKER, request)
            .await?;

        trace!("{} (slot={}) send end", FUNCTION_NAME, slot);
        Ok(())
    }

    fn cancel_command() -> EndointCommand {
        let cancel = CancelRequest {};
        EndointCommand::Cancel(cancel)
    }

    fn new_request(command: EndointCommand) -> EndpointRequest {
        EndpointRequest {
            service_message_version_major: ENDPOINT_BROKER_SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: ENDPOINT_BROKER_SERVICE_MESSAGE_VERSION_MINOR,
            command: Some(command),
        }
    }
}

fn handshake_processor(wire_response: WireResponse) -> Result<i64, TgError> {
    const FUNCTION_NAME: &str = "handshake_processor()";

    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload, error) = wire_response
    {
        if let Some(e) = error {
            return Err(e.to_tg_error());
        }
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
