use crate::error::DiagnosticCode;

#[doc(hidden)]
#[macro_export]
macro_rules! core_service_error {
    ($function_name:expr, $cause:expr) => {{
        let server_message = format!("{}", $cause.message);
        $crate::error::TgError::ServerError(
            format!("{}", $function_name),
            "core service error".to_string(),
            $crate::error::DiagnosticCode::from($cause),
            server_message,
        )
    }};
    ($function_name:expr, $cause:expr, $server_message:expr) => {{
        $crate::error::TgError::ServerError(
            format!("{}", $function_name),
            "core service error".to_string(),
            $crate::error::DiagnosticCode::from($cause),
            $server_message,
        )
    }};
}

impl From<crate::tateyama::proto::diagnostics::Record> for DiagnosticCode {
    fn from(value: crate::tateyama::proto::diagnostics::Record) -> Self {
        DiagnosticCode::from(&value)
    }
}

impl From<&crate::tateyama::proto::diagnostics::Record> for DiagnosticCode {
    fn from(value: &crate::tateyama::proto::diagnostics::Record) -> Self {
        let code = value.code();
        let code_number = to_core_service_diagnostic_code_number(code);

        if code as i32 == value.code {
            let name = code.as_str_name();
            DiagnosticCode::new(/*FIXME*/ 0, "SCD", code_number, name)
        } else {
            let name = format!("UnknownCoreError{}", value.code);
            DiagnosticCode::new(/*FIXME*/ 0, "SCD", code_number, &name)
        }
    }
}

impl From<crate::tateyama::proto::core::response::UnknownError> for DiagnosticCode {
    fn from(_value: crate::tateyama::proto::core::response::UnknownError) -> Self {
        let code = crate::tateyama::proto::diagnostics::Code::Unknown;
        let code_number = to_core_service_diagnostic_code_number(code);
        let name = code.as_str_name();

        DiagnosticCode::new(/*FIXME*/ 0, "SCD", code_number, name)
    }
}

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/common/src/main/java/com/tsurugidb/tsubakuro/exception/CoreServiceCode.java
pub(crate) fn to_core_service_diagnostic_code_number(
    code: crate::tateyama::proto::diagnostics::Code,
) -> i32 {
    match code {
        crate::tateyama::proto::diagnostics::Code::Unknown => 0,
        crate::tateyama::proto::diagnostics::Code::SystemError => 1_00,
        crate::tateyama::proto::diagnostics::Code::UnsupportedOperation => 1_01,
        crate::tateyama::proto::diagnostics::Code::IllegalState => 1_02,
        crate::tateyama::proto::diagnostics::Code::IoError => 1_03,
        crate::tateyama::proto::diagnostics::Code::OutOfMemory => 1_04,
        crate::tateyama::proto::diagnostics::Code::ResourceLimitReached => 1_05,
        crate::tateyama::proto::diagnostics::Code::AuthenticationError => 2_01,
        crate::tateyama::proto::diagnostics::Code::PermissionError => 2_02,
        crate::tateyama::proto::diagnostics::Code::AccessExpired => 2_03,
        crate::tateyama::proto::diagnostics::Code::RefreshExpired => 2_04,
        crate::tateyama::proto::diagnostics::Code::BrokenCredential => 2_05,
        crate::tateyama::proto::diagnostics::Code::SessionClosed => 3_01,
        crate::tateyama::proto::diagnostics::Code::SessionExpired => 3_02,
        crate::tateyama::proto::diagnostics::Code::ServiceNotFound => 4_01,
        crate::tateyama::proto::diagnostics::Code::ServiceUnavailable => 4_02,
        crate::tateyama::proto::diagnostics::Code::OperationCanceled => 4_03,
        crate::tateyama::proto::diagnostics::Code::InvalidRequest => 5_01,
    }
}
