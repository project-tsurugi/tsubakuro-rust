use prost::Message;
use std::sync::Arc;

use crate::{
    error::TgError,
    invalid_response_error,
    prelude::SystemInfo,
    prost_decode_error,
    session::wire::{response::WireResponse, response_box::SlotEntryHandle},
    system_service_error,
};

impl SystemInfo {
    /// Returns the name of the database system.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the version of the database system.
    pub fn version(&self) -> &String {
        &self.version
    }
}

pub(crate) fn system_info_processor(
    _: Arc<SlotEntryHandle>,
    wire_response: WireResponse,
) -> Result<SystemInfo, TgError> {
    const FUNCTION_NAME: &str = "system_info_processor()";
    let payload =
        if let WireResponse::ResponseSessionPayload(_slot, payload, _, error) = wire_response {
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

    use crate::tateyama::proto::system::response::{
        get_system_info::Result, GetSystemInfo as SystemInfoResponse,
    };

    let message = SystemInfoResponse::decode_length_delimited(payload)
        .map_err(|e| prost_decode_error!(FUNCTION_NAME, "SystemInfoResponse", e))?;
    if let Some(result) = message.result {
        match result {
            Result::Success(success) => Ok(success.system_info.unwrap_or_default()),
            Result::Error(error) => Err(system_service_error!(FUNCTION_NAME, error)),
        }
    } else {
        Err(invalid_response_error!(
            FUNCTION_NAME,
            format!("response.result {:?} is None", message),
        ))
    }
}
