use crate::error::DiagnosticCode;

use super::SERVICE_ID_SQL;

#[doc(hidden)]
#[macro_export]
macro_rules! sql_service_error {
    ($function_name:expr, $cause:expr) => {{
        let server_message = format!("{} ({})", $cause.detail, $cause.supplemental_text);
        $crate::error::TgError::ServerError(
            format!("{}", $function_name),
            "SQL service error".to_string(),
            $crate::error::DiagnosticCode::from($cause),
            server_message,
        )
    }};
}

impl From<crate::jogasaki::proto::sql::response::Error> for DiagnosticCode {
    fn from(value: crate::jogasaki::proto::sql::response::Error) -> Self {
        let mut code = value.code();
        let mut code_number = to_sql_service_diagnostic_code_number(code);
        let mut value_code = value.code;
        if code_number < 0 {
            code = crate::jogasaki::proto::sql::error::Code::SqlServiceException;
            code_number = to_sql_service_diagnostic_code_number(code);
            value_code = code as i32;
        }

        if code as i32 == value_code {
            let name = code.as_str_name();
            DiagnosticCode::new(SERVICE_ID_SQL, "SQL", code_number, name)
        } else {
            let name = format!("UnknownSqlError{}", value_code);
            DiagnosticCode::new(SERVICE_ID_SQL, "SQL", code_number, &name)
        }
    }
}

// https://github.com/project-tsurugi/tsubakuro/blob/master/modules/session/src/main/java/com/tsurugidb/tsubakuro/sql/SqlServiceCode.java
fn to_sql_service_diagnostic_code_number(code: crate::jogasaki::proto::sql::error::Code) -> i32 {
    match code {
        crate::jogasaki::proto::sql::error::Code::SqlServiceException => 1000,
        crate::jogasaki::proto::sql::error::Code::SqlExecutionException => 2000,
        crate::jogasaki::proto::sql::error::Code::ConstraintViolationException => 2001,
        crate::jogasaki::proto::sql::error::Code::UniqueConstraintViolationException => 2002,
        crate::jogasaki::proto::sql::error::Code::NotNullConstraintViolationException => 2003,
        crate::jogasaki::proto::sql::error::Code::ReferentialIntegrityConstraintViolationException => 2004,
        crate::jogasaki::proto::sql::error::Code::CheckConstraintViolationException => 2005,
        crate::jogasaki::proto::sql::error::Code::EvaluationException => 2010,
        crate::jogasaki::proto::sql::error::Code::ValueEvaluationException => 2011,
        crate::jogasaki::proto::sql::error::Code::ScalarSubqueryEvaluationException => 2012,
        crate::jogasaki::proto::sql::error::Code::TargetNotFoundException => 2014,
        crate::jogasaki::proto::sql::error::Code::TargetAlreadyExistsException => 100,
        crate::jogasaki::proto::sql::error::Code::InconsistentStatementException => 2018,
        crate::jogasaki::proto::sql::error::Code::RestrictedOperationException => 2020,
        crate::jogasaki::proto::sql::error::Code::DependenciesViolationException => 2021,
        crate::jogasaki::proto::sql::error::Code::WriteOperationByRtxException => 2022,
        crate::jogasaki::proto::sql::error::Code::LtxWriteOperationWithoutWritePreserveException => 2023,
        crate::jogasaki::proto::sql::error::Code::ReadOperationOnRestrictedReadAreaException => 2024,
        crate::jogasaki::proto::sql::error::Code::InactiveTransactionException => 2025,
        crate::jogasaki::proto::sql::error::Code::ParameterException => 2027,
        crate::jogasaki::proto::sql::error::Code::UnresolvedPlaceholderException => 2028,
        crate::jogasaki::proto::sql::error::Code::LoadFileException => 2030,
        crate::jogasaki::proto::sql::error::Code::LoadFileNotFoundException => 2031,
        crate::jogasaki::proto::sql::error::Code::LoadFileFormatException => 2032,
        crate::jogasaki::proto::sql::error::Code::DumpFileException => 2033,
        crate::jogasaki::proto::sql::error::Code::DumpDirectoryInaccessibleException => 2034,
        crate::jogasaki::proto::sql::error::Code::SqlLimitReachedException => 2036,
        crate::jogasaki::proto::sql::error::Code::TransactionExceededLimitException => 2037,
        crate::jogasaki::proto::sql::error::Code::SqlRequestTimeoutException => 2039,
        crate::jogasaki::proto::sql::error::Code::DataCorruptionException => 2041,
        crate::jogasaki::proto::sql::error::Code::SecondaryIndexCorruptionException => 2042,
        crate::jogasaki::proto::sql::error::Code::RequestFailureException => 2044,
        crate::jogasaki::proto::sql::error::Code::TransactionNotFoundException => 2045,
        crate::jogasaki::proto::sql::error::Code::StatementNotFoundException => 2046,
        crate::jogasaki::proto::sql::error::Code::InternalException => 2048,
        crate::jogasaki::proto::sql::error::Code::UnsupportedRuntimeFeatureException => 2050,
        crate::jogasaki::proto::sql::error::Code::BlockedByHighPriorityTransactionException => 2052,
        crate::jogasaki::proto::sql::error::Code::InvalidRuntimeValueException => 2054,
        crate::jogasaki::proto::sql::error::Code::ValueOutOfRangeException => 2056,
        crate::jogasaki::proto::sql::error::Code::ValueTooLongException => 2058,
        crate::jogasaki::proto::sql::error::Code::InvalidDecimalValueException => 2060,
        crate::jogasaki::proto::sql::error::Code::CompileException => 3000,
        crate::jogasaki::proto::sql::error::Code::SyntaxException => 3001,
        crate::jogasaki::proto::sql::error::Code::AnalyzeException => 3002,
        crate::jogasaki::proto::sql::error::Code::TypeAnalyzeException => 3003,
        crate::jogasaki::proto::sql::error::Code::SymbolAnalyzeException => 3004,
        crate::jogasaki::proto::sql::error::Code::ValueAnalyzeException => 3005,
        crate::jogasaki::proto::sql::error::Code::UnsupportedCompilerFeatureException => 3010,
        crate::jogasaki::proto::sql::error::Code::CcException => 4000,
        crate::jogasaki::proto::sql::error::Code::OccException => 4001,
        crate::jogasaki::proto::sql::error::Code::OccReadException => 4010,
        crate::jogasaki::proto::sql::error::Code::ConflictOnWritePreserveException => 4015,
        crate::jogasaki::proto::sql::error::Code::OccWriteException => 4011,
        crate::jogasaki::proto::sql::error::Code::LtxException => 4003,
        crate::jogasaki::proto::sql::error::Code::LtxReadException => 4013,
        crate::jogasaki::proto::sql::error::Code::LtxWriteException => 4014,
        crate::jogasaki::proto::sql::error::Code::RtxException => 4005,
        crate::jogasaki::proto::sql::error::Code::BlockedByConcurrentOperationException => 4007,
        // other
        crate::jogasaki::proto::sql::error::Code::Unspecified => -1,
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! broken_relation_error {
    ($function_name:expr, $message:expr) => {
        $crate::error::TgError::ClientError(format!("{}: {}", $function_name, $message), None)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! broken_encoding_error {
    ($function_name:expr, $message:expr) => {
        $crate::error::TgError::ClientError(format!("{}: {}", $function_name, $message), None)
    };
}
