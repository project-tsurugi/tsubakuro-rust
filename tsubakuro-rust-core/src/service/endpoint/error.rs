use crate::{error::DiagnosticCode, service::core::error::to_core_service_diagnostic_code_number};

use super::endpoint_broker::SERVICE_ID_ENDPOINT_BROKER;

#[macro_export]
macro_rules! endpoint_service_error {
    ($function_name:expr, $cause:expr) => {{
        let server_message = format!("{} ({})", $cause.message, $cause.supplemental_text);
        $crate::error::TgError::ServerError(
            format!("{}: endpoint service error", $function_name),
            $crate::error::DiagnosticCode::from($cause),
            server_message,
        )
    }};
}

impl From<crate::tateyama::proto::endpoint::response::Error> for DiagnosticCode {
    fn from(value: crate::tateyama::proto::endpoint::response::Error) -> Self {
        let code = value.code();
        let code_number = to_core_service_diagnostic_code_number(code);
        let name = code.as_str_name();

        DiagnosticCode::new(
            SERVICE_ID_ENDPOINT_BROKER,
            /*FIXME*/ "SCD",
            code_number,
            name,
        )
    }
}
