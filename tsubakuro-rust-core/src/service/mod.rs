use std::sync::Arc;

use crate::prelude::Session;

pub mod core;
pub(crate) mod endpoint;
pub mod sql;
pub mod system;

/// client of service on tsurugidb.
pub trait ServiceClient {
    /// Creates a new instance.
    ///
    /// See [Session::make_client]
    fn new(session: Arc<Session>) -> Self;
}

/// Service Message Version.
///
/// since 0.7.0
pub trait ServiceMessageVersion {
    /// Returns the service message version.
    fn service_message_version() -> String;
}
