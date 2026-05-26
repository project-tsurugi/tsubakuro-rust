use std::sync::Arc;

use crate::{prelude::Session, service::lob::privileged::client::PrivilegedLobClient};

pub mod core;
pub(crate) mod endpoint;
pub(crate) mod lob;
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

/// Returns the service message version for PrivilegedLobClient.
/// for internal use only.
///
/// since 0.10.0
pub fn privileged_lob_client_service_message_version() -> String {
    PrivilegedLobClient::service_message_version()
}
