use crate::{
    error::DiagnosticCode, service::system::system_client::SERVICE_ID_SYSTEM,
    tateyama::proto::system::diagnostic::ErrorCode,
};

#[doc(hidden)]
#[macro_export]
macro_rules! system_service_error {
    ($function_name:expr, $cause:expr) => {{
        let server_message = if $cause.supplemental_text.is_empty() {
            format!("{}", $cause.message)
        } else {
            format!("{} ({})", $cause.message, $cause.supplemental_text)
        };
        $crate::error::TgError::ServerError(
            format!("{}", $function_name),
            "system service error".to_string(),
            $crate::error::DiagnosticCode::from($cause),
            server_message,
        )
    }};
}

impl From<crate::tateyama::proto::system::response::Error> for DiagnosticCode {
    fn from(value: crate::tateyama::proto::system::response::Error) -> Self {
        let code = value.code();
        let code_number = to_diagnostic_code_number(code);
        let name = code.as_str_name();

        DiagnosticCode::new(SERVICE_ID_SYSTEM, "SYS", code_number, name)
    }
}

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/system/src/main/java/com/tsurugidb/tsubakuro/system/SystemServiceCode.java
fn to_diagnostic_code_number(code: ErrorCode) -> i32 {
    match code {
        ErrorCode::Unknown => 0,
        ErrorCode::NotFound => 1_01,
        _ => -1,
    }
}
