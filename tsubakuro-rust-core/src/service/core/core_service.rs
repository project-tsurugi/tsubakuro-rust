use std::{sync::Arc, time::Duration};

use log::{error, trace, warn};
use prost::Message;

use crate::{
    core_service_error,
    error::TgError,
    invalid_response_error,
    job::Job,
    prost_decode_error,
    session::wire::{response::WireResponse, Wire},
    tateyama::proto::core::{
        request::{request::Command as CoreCommand, Request as CoreRequest, ShutdownType},
        response::{
            Shutdown as ShutdownResponse, UpdateExpirationTime as UpdateExpirationTimeResponse,
        },
    },
};

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/common/impl/SessionImpl.java
const SERVICE_ID_ROUTING: i32 = 0;
// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/common/Session.java
const SERVICE_MESSAGE_VERSION_MAJOR: u64 = 0;
const SERVICE_MESSAGE_VERSION_MINOR: u64 = 1;

pub(crate) struct CoreService;

impl CoreService {
    pub(crate) async fn update_expiration_time(
        wire: &Arc<Wire>,
        expiration_time: Option<Duration>,
        timeout: Duration,
    ) -> Result<(), TgError> {
        const FUNCTION_NAME: &str = "update_expiration_time()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::update_expiration_time_command(expiration_time);
        let request = Self::new_request(command);

        let response = wire
            .send_and_pull_response(SERVICE_ID_ROUTING, request, timeout)
            .await?;
        update_expiration_time_processor(response)?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    pub(crate) async fn update_expiration_time_async(
        wire: &Arc<Wire>,
        expiration_time: Option<Duration>,
        default_timeout: Duration,
        fail_on_drop_error: bool,
    ) -> Result<Job<()>, TgError> {
        const FUNCTION_NAME: &str = "update_expiration_time_async()";
        trace!("{} start", FUNCTION_NAME);

        let command = Self::update_expiration_time_command(expiration_time);
        let request = Self::new_request(command);

        let job = wire
            .send_and_pull_async(
                "updateExpirationTime",
                SERVICE_ID_ROUTING,
                request,
                Box::new(update_expiration_time_processor),
                default_timeout,
                fail_on_drop_error,
            )
            .await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(job)
    }

    fn update_expiration_time_command(expiration_time: Option<Duration>) -> CoreCommand {
        use crate::tateyama::proto::core::request::update_expiration_time::ExpirationTimeOpt;
        let expiration_time_opt =
            expiration_time.map(|time| ExpirationTimeOpt::ExpirationTime(time.as_millis() as u64));
        let update_expiration_time = crate::tateyama::proto::core::request::UpdateExpirationTime {
            expiration_time_opt,
        };
        CoreCommand::UpdateExpirationTime(update_expiration_time)
    }

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
        shutdown_processor(wire, response).await?;

        trace!("{} end", FUNCTION_NAME);
        Ok(())
    }

    pub(crate) async fn shutdown_async(
        wire: &Arc<Wire>,
        shutdown_type: ShutdownType,
        default_timeout: Duration,
        fail_on_drop_error: bool,
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
                Box::new(move |response| shutdown_processor_for_job(&wire_clone, response)),
                default_timeout,
                fail_on_drop_error,
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

fn update_expiration_time_processor(wire_response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "update_expiration_time_processor()";

    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload, error) = wire_response
    {
        if let Some(e) = error {
            return Err(e.to_tg_error());
        }
        if let Some(payload) = payload {
            payload
        } else {
            return Err(invalid_response_error!(FUNCTION_NAME, "payload is None"));
        }
    } else {
        return Err(invalid_response_error!(
            FUNCTION_NAME,
            "response is not ResponseSessionPayload",
        ));
    };

    let message = UpdateExpirationTimeResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "UpdateExpirationTimeResponse", e))?;
    if let Some(result) = message.result {
        match result {
            crate::tateyama::proto::core::response::update_expiration_time::Result::Success(_) => {
                Ok(())
            }
            crate::tateyama::proto::core::response::update_expiration_time::Result::UnknownError(error) => {
                let server_message = error.message.clone();
                Err(core_service_error!(FUNCTION_NAME, error, server_message))
            }
        }
    } else {
        Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response.result {:?} is None", message),
        ))
    }
}

fn shutdown_processor_main(
    function_name: &str,
    wire_response: WireResponse,
) -> Result<(), TgError> {
    let payload = if let WireResponse::ResponseSessionPayload(_slot, payload, error) = wire_response
    {
        if let Some(e) = error {
            return Err(e.to_tg_error());
        }
        if let Some(payload) = payload {
            payload
        } else {
            return Err(invalid_response_error!(function_name, "payload is None"));
        }
    } else {
        return Err(invalid_response_error!(
            function_name,
            "response is not ResponseSessionPayload",
        ));
    };

    let _message = ShutdownResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(function_name, "ShutdownResponse", e))?;

    Ok(())
}

async fn shutdown_processor(wire: &Arc<Wire>, wire_response: WireResponse) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "shutdown_processor()";

    shutdown_processor_main(FUNCTION_NAME, wire_response)?;

    wire.close().await
}

fn shutdown_processor_for_job(
    wire: &Arc<Wire>,
    wire_response: WireResponse,
) -> Result<(), TgError> {
    const FUNCTION_NAME: &str = "shutdown_processor()";

    shutdown_processor_main(FUNCTION_NAME, wire_response)?;

    std::thread::scope(|scope| {
        scope.spawn(move || {
            let runtime = {
                match tokio::runtime::Runtime::new() {
                    Ok(runtime) => runtime,
                    Err(e) => {
                        error!("{FUNCTION_NAME}() runtime::new error. {}", e);
                        return;
                    }
                }
            };
            runtime.block_on(async {
                if let Err(e) = wire.close().await {
                    warn!("{FUNCTION_NAME}() wire.close error. {}", e);
                }
            })
        });
    });
    Ok(())
}
