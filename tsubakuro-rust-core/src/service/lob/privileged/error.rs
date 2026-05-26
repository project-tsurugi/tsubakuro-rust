use crate::{
    error::DiagnosticCode,
    service::{
        core::error::to_core_service_diagnostic_code_number,
        lob::privileged::client::{
            BLOB_RELAY_PRIVILEGE_SERVICE_SYMBOLIC_ID, SERVICE_ID_BLOB_RELAY_PRIVILEGE,
        },
    },
};

#[doc(hidden)]
#[macro_export]
macro_rules! lob_relay_privileged_service_error {
    ($function_name:expr, $cause:expr) => {{
        let server_message = if $cause.supplemental_text.is_empty() {
            format!("{}", $cause.message)
        } else {
            format!("{} ({})", $cause.message, $cause.supplemental_text)
        };
        $crate::error::TgError::ServerError(
            format!("{}", $function_name),
            "blob relay privileged service error".to_string(),
            $crate::error::DiagnosticCode::from($cause),
            server_message,
        )
    }};
}

impl From<crate::tateyama::proto::blob_relay_privilege::response::Error> for DiagnosticCode {
    fn from(value: crate::tateyama::proto::blob_relay_privilege::response::Error) -> Self {
        let code = value.code();
        let code_number = to_core_service_diagnostic_code_number(code);
        let name = code.as_str_name();

        DiagnosticCode::new(
            SERVICE_ID_BLOB_RELAY_PRIVILEGE,
            BLOB_RELAY_PRIVILEGE_SERVICE_SYMBOLIC_ID,
            code_number,
            name,
        )
    }
}
