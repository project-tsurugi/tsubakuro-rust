/// Error for tsubakuro-rust-core.
pub enum TgError {
    /// Client error.
    ClientError(
        /*message*/ String,
        /*cause*/ Option<Box<dyn std::error::Error>>,
    ),
    /// Timeout error.
    TimeoutError(/*message*/ String),
    /// I/O error.
    IoError(
        /*message*/ String,
        /*cause*/ Option<Box<dyn std::error::Error>>,
    ),

    /// Server error.
    ServerError(
        /*function_name*/ String,
        /*message*/ String,
        /*code*/ DiagnosticCode,
        /*server_message*/ String,
    ),
}

impl std::fmt::Debug for TgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClientError(message, cause) => f
                .debug_tuple("ClientError")
                .field(message)
                .field(cause)
                .finish(),
            Self::TimeoutError(message) => f.debug_tuple("TimeoutError").field(message).finish(),
            Self::IoError(message, cause) => f
                .debug_tuple("IoError")
                .field(message)
                .field(cause)
                .finish(),
            Self::ServerError(function_name, message, code, server_message) => f
                .debug_tuple("ServerError")
                .field(function_name)
                .field(message)
                .field(&code.to_string())
                .field(server_message)
                .finish(),
        }
    }
}

impl std::fmt::Display for TgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TgError::ClientError(message, cause) => match cause {
                Some(cause) => write!(f, "{message} ({cause})"),
                _ => write!(f, "{message}"),
            },
            TgError::TimeoutError(message) => write!(f, "{message}"),
            TgError::IoError(message, cause) => match cause {
                Some(cause) => write!(f, "{message} ({cause})"),
                _ => write!(f, "{message}"),
            },
            TgError::ServerError(_function_name, message, code, server_message) => {
                write!(f, "{message} ({code}) {server_message}")
            }
        }
    }
}

impl std::error::Error for TgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TgError::ClientError(_, cause) => cause.as_deref(),
            TgError::TimeoutError(_) => None,
            TgError::IoError(_, cause) => cause.as_deref(),
            TgError::ServerError(_, _, _, _) => None,
        }
    }
}

impl TgError {
    /// get error message.
    pub fn message(&self) -> &String {
        match self {
            TgError::ClientError(message, _cause) => message,
            TgError::TimeoutError(message) => message,
            TgError::IoError(message, _cause) => message,
            TgError::ServerError(_function_name, message, _code, _server_message) => message,
        }
    }

    /// get diagnostic code of ServerError.
    pub fn diagnostic_code(&self) -> Option<&DiagnosticCode> {
        match self {
            TgError::ClientError(_, _) => None,
            TgError::TimeoutError(_) => None,
            TgError::IoError(_, _) => None,
            TgError::ServerError(_, _, code, _) => Some(code),
        }
    }
}

/// Diagnostic code of ServerError.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticCode {
    category_number: i32,
    category_str: String,
    code_number: i32,
    name: String,
}

impl DiagnosticCode {
    pub(crate) fn new(
        category_number: i32,
        category_str: &str,
        code_number: i32,
        name: &str,
    ) -> DiagnosticCode {
        DiagnosticCode {
            category_number,
            category_str: category_str.to_string(),
            code_number,
            name: name.to_string(),
        }
    }

    /// get error category.
    pub fn category_number(&self) -> i32 {
        self.category_number
    }

    /// get error category.
    pub fn category_str(&self) -> &String {
        &self.category_str
    }

    /// get error code.
    pub fn code_number(&self) -> i32 {
        self.code_number
    }

    /// get structured error code.
    pub fn structured_code(&self) -> String {
        format!("{}-{:05}", self.category_str, self.code_number)
    }

    /// get error name.
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl std::fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.structured_code(), self.name)
    }
}

#[macro_export]
macro_rules! client_error {
    ($message:expr) => {
        $crate::error::TgError::ClientError(format!("{}", $message), None)
    };
    ($message:expr, $cause:expr) => {
        $crate::error::TgError::ClientError(format!("{}", $message), Some(Box::new($cause)))
    };
}

#[macro_export]
macro_rules! illegal_argument_error {
    ($message:expr) => {
        $crate::error::TgError::ClientError(format!("{}", $message), None)
    };
}

#[macro_export]
macro_rules! io_error {
    ($message:expr) => {
        $crate::error::TgError::IoError(format!("{}", $message), None)
    };
    ($message:expr, $cause:expr) => {
        $crate::error::TgError::IoError(format!("{}", $message), Some(Box::new($cause)))
    };
}

#[macro_export]
macro_rules! timeout_error {
    ($function_name:expr) => {
        $crate::error::TgError::TimeoutError(format!("{} timeout", $function_name))
    };
}

#[macro_export]
macro_rules! invalid_response_error {
    ($function_name:expr, $message:expr $(,)?) => {
        $crate::error::TgError::ClientError(format!("{}: {}", $function_name, $message), None)
    };
}

#[macro_export]
macro_rules! prost_decode_error {
    ($function_name:expr, $data_name:expr, $cause:expr) => {
        $crate::error::TgError::ClientError(
            format!("{}: {} decode error", $function_name, $data_name),
            Some(Box::new($cause)),
        )
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn diagnostic_code() {
        let code = DiagnosticCode::new(123, "TST", 456, "TEST_EXCEPTION");
        assert_eq!(123, code.category_number());
        assert_eq!("TST", code.category_str());
        assert_eq!(456, code.code_number());
        assert_eq!("TST-00456", code.structured_code());
        assert_eq!("TEST_EXCEPTION", code.name());
        assert_eq!("TST-00456 (TEST_EXCEPTION)", code.to_string());
    }
}
