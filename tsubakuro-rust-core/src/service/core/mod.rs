use std::{sync::Arc, time::Duration};

use log::{debug, trace};
use prost::Message;

use crate::{
    error::TgError,
    invalid_response_error,
    job::Job,
    prost_decode_error,
    session::wire::{response::WireResponse, Wire},
    tateyama::proto::core::{
        request::{request::Command as CoreCommand, Request as CoreRequest, ShutdownType},
        response::Shutdown as ShutdownResponse,
    },
};

pub(crate) mod error;

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/common/impl/SessionImpl.java
const SERVICE_ID_ROUTING: i32 = 0;
// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/common/Session.java
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 0;
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 1;

pub(crate) struct CoreService;

impl CoreService {
    pub(crate) async fn shutdown(
        wire: &Arc<Wire>,
        shutdown_type: ShutdownType,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "shutdown()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::shutdown_command(shutdown_type);
        let request = Self::new_request(command);

        let response = wire
            .send_and_pull_response(SERVICE_ID_ROUTING, request, timeout)
            .await?;
        shutdown_processor(wire, response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    pub(crate) async fn shutdown_async(
        wire: &Arc<Wire>,
        shutdown_type: ShutdownType,
        default_timeout: Duration,
    ) -> Result<Job<()>, TgError> {
        const FUNCTION_NAME: &str = "shutdown_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::shutdown_command(shutdown_type);
        let request = Self::new_request(command);

        let wire_clone = wire.clone();
        let job = wire
            .send_and_pull_async(
                "Shutdown",
                SERVICE_ID_ROUTING,
                request,
                Box::new(move |response| shutdown_processor(&wire_clone, response)),
                default_timeout,
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn shutdown_command(shutdown_type: ShutdownType) -> CoreCommand {
        let shutdown = crate::tateyama::proto::core::request::Shutdown {
            r#type: shutdown_type.into(),
        };
        CoreCommand::Shutdown(shutdown)
    }

    fn new_request(command: CoreCommand) -> CoreRequest {
        CoreRequest {
            service_message_version_major: SERVICE_MESSAGE_VERSION_MAJOR,
            service_message_version_minor: SERVICE_MESSAGE_VERSION_MINOR,
            command: Some(command),
        }
    }
}

fn shutdown_processor(wire: &Arc<Wire>, wire_response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "shutdown_processor()";

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

    let _message = ShutdownResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "ShutdownResponse", e))?;

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = {
                match tokio::runtime::Runtime::new() {
                    Ok(runtime) => runtime,
                    Err(e) => {
                        debug!("shutdown_processor() error. {}", e);
                        return;
                    }
                }
            };
            runtime.block_on(async {
                if let Err(e) = wire.close().await {
                    debug!("shutdown_processor() error. {}", e);
                }
            })
        });
    });
    Ok(())
}
